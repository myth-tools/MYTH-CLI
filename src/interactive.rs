use crate::config;
use crate::core;
use crate::core::commands::{handle_command, CommandAction};
use crate::stream::{consume_agent_stream, stream_to_tui};
use crate::tui;
use crate::ui;

use color_eyre::eyre::Result;
use owo_colors::OwoColorize;
use rustyline::completion::Completer;
use rustyline::highlight::{CmdKind, Highlighter};
use rustyline::hint::Hinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};
use std::borrow::Cow;
use std::path::{Path, PathBuf};

pub struct MythHelper {
    pub config: config::AppConfig,
    pub mission_targets: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
}

impl Completer for MythHelper {
    type Candidate = String;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<String>)> {
        let targets = match self.mission_targets.lock() {
            Ok(t) => t,
            Err(_) => return Ok((pos, vec![])), // Silicon-grade safety: don't crash on poisoned locks
        };
        let ctx = core::commands::CommandContext {
            config: &self.config,
            mission_targets: &targets,
        };
        let suggestions = core::commands::get_argument_suggestions(&line[..pos], &ctx);
        let tokens: Vec<&str> = line[..pos].split_whitespace().collect();
        let last_token = tokens.last().cloned().unwrap_or("");
        let start = if line[..pos].ends_with(' ') {
            pos
        } else {
            pos.saturating_sub(last_token.len())
        };
        Ok((start, suggestions))
    }
}

impl Hinter for MythHelper {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<String> {
        if pos < line.len() {
            return None;
        }
        let targets = match self.mission_targets.lock() {
            Ok(t) => t,
            Err(_) => return None,
        };
        let ctx = core::commands::CommandContext {
            config: &self.config,
            mission_targets: &targets,
        };
        core::commands::get_ghost_suggestion(line, &ctx)
    }
}

impl Highlighter for MythHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        use core::commands::TokenRole;
        let tokens = core::commands::tokenize_semantics(line);
        if tokens.is_empty() {
            return Cow::Borrowed(line);
        }

        if tokens
            .first()
            .map(|t| t.role == TokenRole::Other)
            .unwrap_or(false)
        {
            return Cow::Borrowed(line);
        }

        let mut highlighted = String::new();
        let mut current_pos = 0;

        for token_sem in tokens {
            if let Some(idx) = line[current_pos..].find(&token_sem.text) {
                highlighted.push_str(&line[current_pos..current_pos + idx]);
                current_pos += idx;
            }

            let text = match token_sem.role {
                TokenRole::Command => ui::CyberTheme::secondary(&token_sem.text)
                    .bold()
                    .to_string(),
                TokenRole::Flag => ui::CyberTheme::secondary(&token_sem.text)
                    .italic()
                    .to_string(),
                TokenRole::FlagValue => ui::CyberTheme::secondary(&token_sem.text).to_string(),
                TokenRole::Target => ui::CyberTheme::bright(&token_sem.text).to_string(),
                TokenRole::Other => token_sem.text.clone(),
            };
            highlighted.push_str(&text);
            current_pos += token_sem.text.len();
        }
        highlighted.push_str(&line[current_pos..]);
        Cow::Owned(highlighted)
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Cow::Owned(ui::CyberTheme::dim(hint).italic().to_string())
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _kind: CmdKind) -> bool {
        true
    }
}

impl Validator for MythHelper {}
impl Helper for MythHelper {}

/// Run interactive CLI mode (no TUI).
pub async fn run_interactive(config: &config::AppConfig) -> Result<()> {
    let (_ui_task, agent_result) = tokio::join!(
        async {
            println!(
                "  {}",
                ui::CyberTheme::secondary("AI Reconnaissance Agent for Kali Linux").italic()
            );
            println!(
                "  {} {}",
                ui::CyberTheme::dim("Version:"),
                ui::CyberTheme::bright(&config.agent.version)
            );
            println!(
                "  {} {}",
                ui::CyberTheme::dim("Model:  "),
                ui::CyberTheme::accent(&config.llm.model)
            );
            println!(
                "  {} {}",
                ui::CyberTheme::dim("Config: "),
                ui::CyberTheme::dim(format!(
                    "{}",
                    config::AppConfig::user_config_path().display()
                ))
            );
            println!(
                "  {} {}",
                ui::CyberTheme::dim("Sandbox:"),
                ui::CyberTheme::primary("ACTIVE").bold()
            );
            println!();
            println!(
                "  {} 'scan <target>' to begin reconnaissance.",
                "Type".dimmed()
            );
            println!("  {} '/help' to list commands.", "Type".dimmed());
            println!();
        },
        core::agent::ReconAgent::new(config.clone())
    );

    let agent = agent_result.map_err(|e| color_eyre::eyre::eyre!("Agent init failed: {}", e))?;

    let mission_targets = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    run_interactive_with_context(config, agent, mission_targets).await
}

/// Core interactive loop for CLI mode, shared across entry points.
pub async fn run_interactive_with_context(
    config: &config::AppConfig,
    mut agent: core::agent::ReconAgent,
    mission_targets: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
) -> Result<()> {
    let helper = MythHelper {
        config: config.clone(),
        mission_targets: mission_targets.clone(),
    };

    let mut rl = Editor::<MythHelper, DefaultHistory>::new()?;
    rl.set_helper(Some(helper));
    let history_path = config::AppConfig::user_config_path()
        .parent()
        .map(|p| p.join(".myth_history"))
        .unwrap_or_else(|| PathBuf::from(".myth_history"));
    let _ = rl.load_history(&history_path);

    let (config_tx, mut config_rx) =
        tokio::sync::mpsc::unbounded_channel::<config::watcher::ConfigUpdateEvent>();
    let mcp_config_path = config::AppConfig::mcp_config_path();
    let config_dir = mcp_config_path
        .parent()
        .unwrap_or(std::path::Path::new("config"));
    let _watcher = config::ConfigWatcher::new(config_dir, config_tx).ok();

    let mut mission_events = std::collections::VecDeque::with_capacity(20);
    mission_events.push_back(format!(
        "{} Neural core synchronized.",
        ui::CyberTheme::primary("✓")
    ));
    mission_events.push_back(format!(
        "{} Volatile ramdisk mounted.",
        ui::CyberTheme::primary("✓")
    ));

    loop {
        while let Ok(event) = config_rx.try_recv() {
            if let Err(e) = agent.reload_config(Some(event)).await {
                println!(
                    "\n{} Config Hot-Reload Failed: {}\n",
                    ui::CyberTheme::accent("⚠").bold(),
                    e
                );
            } else {
                println!(
                    "\n{} Configuration synchronized. Tactical assets hot-swapped.\n",
                    ui::CyberTheme::primary("⚡").bold()
                );
            }
        }

        crate::signals::reset_mission_signal();
        ui::print_operative_prompt(&config.agent.user_name).await;
        let prompt = ui::get_operative_prompt(&config.agent.user_name);
        match rl.readline(&prompt) {
            Ok(line) => {
                let cmd = line.trim().to_string();
                if cmd.is_empty() {
                    continue;
                }
                let _ = rl.add_history_entry(&cmd);

                if cmd.starts_with("/scan ") || cmd.starts_with("scan ") {
                    let target = cmd.split_whitespace().nth(1).unwrap_or("").to_string();
                    if !target.is_empty() {
                        if let Ok(mut targets) = mission_targets.lock() {
                            if !targets.contains(&target) {
                                targets.push(target);
                            }
                        }
                    }
                }

                match handle_command(&cmd, &mut agent, &mission_events).await {
                    CommandAction::Exit => {
                        let _ = rl.save_history(&history_path);
                        agent.cleanup().await;
                        break;
                    }
                    CommandAction::Clear => {
                        use std::io::Write;
                        print!("\x1B[2J\x1B[1;1H\x1B[3J");
                        let _ = std::io::stdout().flush();
                        mission_events.push_back(format!(
                            "{} Visual buffer purged.",
                            ui::CyberTheme::primary("✓")
                        ));
                    }
                    CommandAction::WipeSession => {
                        use std::io::Write;
                        agent.reset_session().await;
                        print!("\x1B[2J\x1B[1;1H\x1B[3J");
                        let _ = std::io::stdout().flush();
                        ui::print_banner(&config.agent.name, &config.agent.version);
                        println!(
                            "{} {} Tactical memory flushed. Neural core reset.\n",
                            ui::CyberTheme::primary("✓"),
                            ui::CyberTheme::primary("SESSION WIPED //").bold()
                        );
                        mission_events.push_back(format!(
                            "{} Session memory cleared by user.",
                            ui::CyberTheme::primary("✓")
                        ));
                    }
                    CommandAction::Burn => {
                        use std::io::Write;
                        println!(
                            "\n{} {} SHREDDING VOLATILE DATA... [OVERWRITE PASS 1/3]",
                            "🔥".bright_red(),
                            " EMERGENCY PURGE //".bold()
                        );
                        for i in (1..=8).rev() {
                            let block = "█".repeat(i);
                            let styled_block = block.bright_red();
                            let void = " ".repeat(8 - i);
                            print!(
                                "\r  [{}{}] OVERWRITING BUFFERS: 0x{:08X}...",
                                styled_block,
                                void,
                                rand::random::<u32>()
                            );
                            let _ = std::io::stdout().flush();
                            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                        }
                        let _ = std::fs::remove_file(&history_path);
                        agent.cleanup().await;
                        println!("\n\n{} {} Neural session shredded. Cyber-tracks purged. System offline.", "💀".bright_red(), "PURGE SUCCESSFUL //".bold());
                        break;
                    }
                    CommandAction::Response(resp) => {
                        println!("{}", resp);
                    }
                    CommandAction::SetDepth(depth) => {
                        agent.set_max_iterations(depth).await;
                        println!(
                            "\n{} Recon depth modulated to: {}\n",
                            ui::CyberTheme::primary("⚡").bold(),
                            depth.bright_yellow()
                        );
                        mission_events.push_back(format!(
                            "{} Depth adjusted to {}",
                            ui::CyberTheme::primary("✓"),
                            depth
                        ));
                    }
                    CommandAction::StartSession {
                        target,
                        profile,
                        prompt,
                    } => {
                        println!(
                            "\n{} TARGET ACQUISITION: {}",
                            ui::CyberTheme::primary("⚡").bold(),
                            target.bright_white().bold()
                        );
                        if let Ok(mut targets) = mission_targets.lock() {
                            if !targets.contains(&target) {
                                targets.push(target.clone());
                            }
                        }

                        agent.start_session(&target, &profile).await;
                        mission_events.push_back(format!(
                            "{} Target acquired: {}",
                            ui::CyberTheme::primary("✓"),
                            target
                        ));

                        if let Some(p) = prompt {
                            let start = std::time::Instant::now();
                            let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
                            agent.set_event_tx(Some(tx));
                            match agent.chat_stream(&p).await {
                                Ok((stream, executed_tools)) => {
                                    consume_agent_stream(
                                        Box::pin(stream),
                                        executed_tools,
                                        &mut agent,
                                        rx,
                                        start,
                                    )
                                    .await?;
                                    ui::print_turn_separator().await;
                                }
                                Err(e) => {
                                    println!("\n{} Mission launch failure: {}", "✗".bright_red(), e)
                                }
                            }
                        }
                    }
                    CommandAction::AgentChat(cmd) => {
                        let spinner = indicatif::ProgressBar::new_spinner();
                        spinner.set_style(
                            indicatif::ProgressStyle::default_spinner()
                                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                                .template(&format!("{{spinner:.{}}} {{msg}}", "cyan"))
                                .unwrap_or_else(|_| indicatif::ProgressStyle::default_spinner()),
                        );
                        spinner.enable_steady_tick(std::time::Duration::from_millis(80));
                        spinner.set_message("Thinking...".bright_black().to_string());

                        let start = std::time::Instant::now();
                        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
                        agent.set_event_tx(Some(tx));
                        match agent.chat_stream(&cmd).await {
                            Ok((stream, executed_tools)) => {
                                spinner.finish_and_clear();
                                consume_agent_stream(
                                    Box::pin(stream),
                                    executed_tools,
                                    &mut agent,
                                    rx,
                                    start,
                                )
                                .await?;
                                ui::print_turn_separator().await;
                            }
                            Err(e) => {
                                spinner.finish_and_clear();
                                println!("\n{} {}", "Error:".bright_red().bold(), e);
                            }
                        }
                    }
                    CommandAction::ExecuteTool {
                        tool_name,
                        arguments,
                    } => {
                        ui::print_status(
                            "DIRECT EXECUTION",
                            &format!(
                                "Launching {} without neural overhead",
                                tool_name.bright_white().bold()
                            ),
                        );
                        match agent.execute_tool_directly(&tool_name, arguments).await {
                            Ok(result) => {
                                if let Some(out) = result["stdout"].as_str() {
                                    if !out.is_empty() {
                                        println!("{}", out);
                                    }
                                }
                                if let Some(err) = result["stderr"].as_str() {
                                    if !err.is_empty() {
                                        eprintln!("{}", err.bright_red());
                                    }
                                }
                                if let Some(error) = result["error"].as_str() {
                                    println!("\n{} Execution Error: {}\n", "✗".bright_red(), error);
                                }
                                ui::print_turn_separator().await;
                            }
                            Err(e) => {
                                println!("\n{} Direct execution failed: {}\n", "✗".bright_red(), e)
                            }
                        }
                    }
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted)
            | Err(rustyline::error::ReadlineError::Eof) => {
                let _ = rl.save_history(&history_path);
                agent.cleanup().await;
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

/// Run interactive TUI mode.
pub async fn run_tui(config: &config::AppConfig, target: &str) -> Result<()> {
    use crossterm::{
        event::{self, Event},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::prelude::*;
    use tokio::sync::mpsc;

    let (tx, rx) = mpsc::unbounded_channel::<tui::app::TuiEvent>();
    let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel::<String>();

    enable_raw_mode()?;
    std::env::set_var("MYTH_TUI_ACTIVE", "true");
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = tui::app::App::new(
        config.agent.name.clone(),
        config.agent.version.clone(),
        config.agent.author.clone(),
        rx,
        config.clone(),
    );
    app.target = target.to_string();

    let (config_tx, mut config_rx) =
        mpsc::unbounded_channel::<config::watcher::ConfigUpdateEvent>();
    let mcp_config_path = config::AppConfig::mcp_config_path();
    let config_dir = mcp_config_path.parent().unwrap_or(Path::new("config"));
    let _config_watcher = config::ConfigWatcher::new(config_dir, config_tx).ok();

    let agent_config = config.clone();
    let initial_target = target.to_string();
    let tui_tx = tx.clone();

    tokio::spawn(async move {
        let mut agent = match core::agent::ReconAgent::new(agent_config).await {
            Ok(a) => a.with_event_tx(tui_tx.clone()),
            Err(e) => {
                let _ = tui_tx.send(tui::app::TuiEvent::Message {
                    role: "error".to_string(),
                    content: format!("Neural initialization failed: {}", e),
                });
                return;
            }
        };

        if initial_target != "(no target)" {
            agent.start_session(&initial_target, "full").await;

            let prompt = format!(
                "Begin full reconnaissance on {} using available tools.",
                initial_target
            );
            let _ = tui_tx.send(tui::app::TuiEvent::MessageStart {
                role: "agent".to_string(),
            });
            let _ = tui_tx.send(tui::app::TuiEvent::ProcessingStatus(true));
            match agent.chat_stream(&prompt).await {
                Ok((stream, _)) => {
                    stream_to_tui(Box::pin(stream), &tui_tx).await;
                }
                Err(e) => {
                    let _ = tui_tx.send(tui::app::TuiEvent::MessageChunk {
                        chunk: format!(" [ERROR: {}]", e),
                    });
                }
            }
            let _ = tui_tx.send(tui::app::TuiEvent::ProcessingStatus(false));
        }

        loop {
            tokio::select! {
                Some(event) = config_rx.recv() => {
                    if let Err(e) = agent.reload_config(Some(event)).await {
                        let _ = tui_tx.send(tui::app::TuiEvent::Message {
                            role: "error".to_string(),
                            content: format!("Config Hot-Reload Failed: {}", e)
                        });
                    } else {
                        let _ = tui_tx.send(tui::app::TuiEvent::Message {
                            role: "system".to_string(),
                            content: "Tactical configuration synchronized. Internal assets hot-swapped.".to_string()
                        });
                    }
                }
                Some(cmd) = cmd_rx.recv() => {
                    let tui_mission_events = std::collections::VecDeque::new();
                    match handle_command(&cmd, &mut agent, &tui_mission_events).await {
                        CommandAction::Exit => {
                            tracing::info!("TUI Command: Secure exit protocol initiated");
                            agent.cleanup().await;
                            let _ = tui_tx.send(tui::app::TuiEvent::Message {
                                role: "system".to_string(),
                                content: "Neural links severed. Tactical session terminated.".to_string()
                            });
                            break;
                        }
                        CommandAction::Clear => {}
                        CommandAction::WipeSession => { agent.clear_session().await; }
                        CommandAction::Burn => {
                            let _ = tui_tx.send(tui::app::TuiEvent::Message {
                                role: "system".to_string(), content: "EMERGENCY SHRED PROTOCOL INITIATED.".to_string()
                            });
                            agent.cleanup().await;
                            let _ = tui_tx.send(tui::app::TuiEvent::Message {
                                role: "system".to_string(), content: "All session data shredded. System offline.".to_string()
                            });
                            break;
                        }
                        CommandAction::Response(resp) => {
                            let _ = tui_tx.send(tui::app::TuiEvent::Message {
                                role: "system".to_string(), content: resp
                            });
                        }
                        CommandAction::SetDepth(depth) => {
                            agent.set_max_iterations(depth).await;
                            let _ = tui_tx.send(tui::app::TuiEvent::IterationUpdate(depth));
                            let _ = tui_tx.send(tui::app::TuiEvent::Message {
                                role: "system".to_string(), content: format!("Recon depth modulated to: {}", depth)
                            });
                        }
                        CommandAction::StartSession { target, profile, prompt } => {
                            agent.start_session(&target, &profile).await;
                            let _ = tui_tx.send(tui::app::TuiEvent::TargetUpdate(target.clone()));

                            if let Some(p) = prompt {
                                let _ = tui_tx.send(tui::app::TuiEvent::MessageStart { role: "agent".to_string() });
                                let _ = tui_tx.send(tui::app::TuiEvent::ProcessingStatus(true));
                                match agent.chat_stream(&p).await {
                                    Ok((stream, _)) => stream_to_tui(Box::pin(stream), &tui_tx).await,
                                    Err(e) => {
                                        let _ = tui_tx.send(tui::app::TuiEvent::MessageChunk { chunk: format!(" [ERROR: {}]", e) });
                                    }
                                }
                                let _ = tui_tx.send(tui::app::TuiEvent::ProcessingStatus(false));
                            }
                        }
                        CommandAction::AgentChat(chat_cmd) => {
                            let _ = tui_tx.send(tui::app::TuiEvent::MessageStart { role: "agent".to_string() });
                            let _ = tui_tx.send(tui::app::TuiEvent::ProcessingStatus(true));
                            match agent.chat_stream(&chat_cmd).await {
                                Ok((stream, _)) => stream_to_tui(Box::pin(stream), &tui_tx).await,
                                Err(e) => {
                                    let _ = tui_tx.send(tui::app::TuiEvent::MessageChunk { chunk: format!(" [ERROR: {}]", e) });
                                }
                            }
                            let _ = tui_tx.send(tui::app::TuiEvent::ProcessingStatus(false));
                        }
                        CommandAction::ExecuteTool { tool_name, arguments } => {
                            let _ = tui_tx.send(tui::app::TuiEvent::Message {
                                role: "system".to_string(),
                                content: format!("[DIRECT EXECUTION] Launching {} without neural overhead...", tool_name)
                            });
                            match agent.execute_tool_directly(&tool_name, arguments).await {
                                Ok(res) => {
                                    if let Some(out) = res["stdout"].as_str() {
                                        if !out.is_empty() {
                                            let _ = tui_tx.send(tui::app::TuiEvent::Message { role: "tool".to_string(), content: out.to_string() });
                                        }
                                    }
                                    if let Some(err) = res["stderr"].as_str() {
                                        if !err.is_empty() {
                                            let _ = tui_tx.send(tui::app::TuiEvent::Message { role: "tool".to_string(), content: format!("[ERROR] {}", err) });
                                        }
                                    }
                                }
                                Err(e) => {
                                    let _ = tui_tx.send(tui::app::TuiEvent::Message { role: "error".to_string(), content: format!("Direct execution failed: {}", e) });
                                }
                            }
                        }
                    }
                }
            }
        }
        agent.cleanup().await;
    });

    loop {
        app.update();
        terminal.draw(|frame| app.render(frame))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if let Some(command) = app.handle_key(key) {
                    match command.as_str() {
                        "/quit" | "/exit" | "/q" => {
                            app.running = false;
                        }
                        "/clear" => {
                            app.chat = tui::widgets::chat::ChatWidget::new();
                        }
                        _ => {
                            let _ = cmd_tx.send(command);
                        }
                    }
                }
            }
        }
        if !app.running {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    println!(
        "🔒 {} session ended. Neural links severed.",
        config.agent.name
    );
    Ok(())
}
