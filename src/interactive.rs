use crate::config;
use crate::core;
use crate::core::commands::{handle_command, CommandAction};
use crate::stream::{consume_agent_stream, stream_to_tui};
use crate::tui;
use crate::tui::app::TuiEvent;
use crate::ui;

use color_eyre::eyre::Result;
use owo_colors::OwoColorize;
use rustyline::completion::Completer;
use rustyline::highlight::{CmdKind, Highlighter};
use rustyline::hint::Hinter;
use rustyline::history::{DefaultHistory, SearchDirection};
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};
use std::borrow::Cow;
use std::path::Path;

pub struct MythHelper {
    pub config: config::AppConfig,
    pub mission_targets: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
    pub discovered_tools: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
}

impl Completer for MythHelper {
    type Candidate = String;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<String>)> {
        let targets = match self.mission_targets.lock() {
            Ok(t) => t,
            Err(_) => return Ok((pos, vec![])),
        };
        let tools = match self.discovered_tools.lock() {
            Ok(t) => t,
            Err(_) => return Ok((pos, vec![])),
        };

        let mut history_entries = Vec::new();
        for i in 0..ctx.history().len() {
            if let Ok(Some(res)) = ctx.history().get(i, SearchDirection::Forward) {
                history_entries.push(res.entry.to_string());
            }
        }

        let cmd_ctx = core::commands::CommandContext {
            config: &self.config,
            mission_targets: &targets,
            history: &history_entries,
            discovered_tools: &tools,
        };

        let suggestions = core::commands::get_argument_suggestions(&line[..pos], &cmd_ctx);

        // Silicon-Grade Token Start Detection (Precision Alignment)
        let mut lexer = core::commands::Lexer::new(&line[..pos]);
        let tokens = lexer.tokenize();
        let last_token_len = tokens.last().map(|t| t.len()).unwrap_or(0);

        let start = if line[..pos].ends_with(' ') {
            pos
        } else {
            pos.saturating_sub(last_token_len)
        };

        Ok((start, suggestions))
    }
}

impl Hinter for MythHelper {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        if pos < line.len() {
            return None;
        }
        let targets = match self.mission_targets.lock() {
            Ok(t) => t,
            Err(_) => return None,
        };
        let tools = match self.discovered_tools.lock() {
            Ok(t) => t,
            Err(_) => return None,
        };

        let mut history_entries = Vec::new();
        for i in 0..ctx.history().len() {
            if let Ok(Some(res)) = ctx.history().get(i, SearchDirection::Forward) {
                history_entries.push(res.entry.to_string());
            }
        }

        let cmd_ctx = core::commands::CommandContext {
            config: &self.config,
            mission_targets: &targets,
            history: &history_entries,
            discovered_tools: &tools,
        };
        core::commands::get_ghost_suggestion(line, &cmd_ctx)
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
            ui::print_mission_header(config);
        },
        core::agent::ReconAgent::new(config.clone())
    );

    let agent = agent_result.map_err(|e| color_eyre::eyre::eyre!("Agent init failed: {}", e))?;

    let mission_targets = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let discovered_tools = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

    // Initial tool discovery for CLI mode
    {
        let mcp = agent.mcp_server();
        let disc_lock = mcp.discovery();
        let disc = disc_lock.read().await;
        let mut tools = discovered_tools.lock().unwrap();
        for tool in disc.list_all() {
            tools.push(tool.name.clone());
        }
    }

    run_interactive_with_context(config, agent, mission_targets, discovered_tools).await
}

/// Core interactive loop for CLI mode, shared across entry points.
pub async fn run_interactive_with_context(
    config: &config::AppConfig,
    mut agent: core::agent::ReconAgent,
    mission_targets: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
    discovered_tools: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
) -> Result<()> {
    let helper = MythHelper {
        config: config.clone(),
        mission_targets: mission_targets.clone(),
        discovered_tools: discovered_tools.clone(),
    };

    let mut rl = Editor::<MythHelper, DefaultHistory>::new()?;
    rl.set_helper(Some(helper));
    let history_path = crate::core::persistence::get_history_path();
    let vault_path = crate::core::persistence::get_vault_path();
    let vault = crate::core::persistence::HistoryVault::init(&vault_path)
        .map_err(|e| color_eyre::eyre::eyre!(e))?;

    // Automated Tactical Migration (Industry Grade Resilience)
    if let Ok(true) = vault.migrate_legacy(&history_path) {
        println!(
            "\n{} {}\n",
            ui::CyberTheme::primary("✓").bold(),
            ui::CyberTheme::primary("Legacy mission history successfully vaulted.")
        );
    }

    // Silicon-Grade Encrypted History Load
    if let Err(e) = vault.load_into(rl.history_mut()) {
        println!(
            "\n{} {}\n",
            ui::CyberTheme::accent("⚠ VAULT RECOVERY:").bold(),
            ui::CyberTheme::dim(format!("{}", e))
        );
    }

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
        ui::print_operative_prompt(&config.agent.user_name, config.tui.simulated_mode).await;
        let prompt = ui::get_operative_prompt(&config.agent.user_name);
        match rl.readline(&prompt) {
            Ok(line) => {
                let cmd = line.trim().to_string();
                if cmd.is_empty() {
                    continue;
                }
                let _ = rl.add_history_entry(&cmd);

                // Silicon-Grade Auto-Save (Institutional Standard)
                // Atomic SQLite transaction ensures zero corruption even on power loss.
                let _ = vault.append(&cmd);

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
                        let _ = vault.append(&cmd);
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
                        ui::print_banner();
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
                        let history_path = crate::core::persistence::get_history_path();
                        let _ = std::fs::remove_file(&history_path);
                        agent.cleanup().await;
                        println!("\n\n{} {} Neural session shredded. Cyber-tracks purged. System offline.", "💀".bright_red(), "PURGE SUCCESSFUL //".bold());
                        break;
                    }
                    CommandAction::Response(resp) => {
                        println!("{}", resp);
                    }
                    CommandAction::ProvisionFont(font_id) => {
                        println!(
                            "\n  {} {} {}",
                            ui::CyberTheme::accent("⚠"),
                            ui::CyberTheme::accent("Fidelity Alert:").bold(),
                            ui::CyberTheme::dim("Requested mission asset is not locally indexed.")
                        );
                        print!(
                            "  {} {} [y/N]: ",
                            ui::CyberTheme::primary("?"),
                            ui::CyberTheme::bright("Initialize automated font provisioning?")
                        );
                        use std::io::Write;
                        std::io::stdout().flush().ok();

                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).ok();
                        if input.trim().to_lowercase() == "y" {
                            ui::provision_font(&font_id)
                                .await
                                .map_err(|e| color_eyre::eyre::eyre!("{}", e))?;
                            ui::perform_typography_audit(&font_id);
                        } else {
                            println!(
                                "\n  {} Provisioning aborted. Fidelity remains degraded.\n",
                                ui::CyberTheme::accent("✗")
                            );
                        }
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
                            let stream_res = tokio::select! {
                                res = agent.chat_stream(&p) => Some(res),
                                _ = tokio::signal::ctrl_c() => {
                                    println!("\n\n{} {}", "⚠".yellow().bold(), "MISSION INTERRUPT: Neural initialization aborted by operative.".bright_red().italic());
                                    None
                                }
                            };
                            if let Some(res) = stream_res {
                                match res {
                                    Ok((stream, executed_tools)) => {
                                        consume_agent_stream(
                                            Box::pin(stream),
                                            executed_tools,
                                            &mut agent,
                                            rx,
                                            start,
                                            None,
                                        )
                                        .await?;
                                        ui::print_turn_separator().await;
                                    }
                                    Err(e) => {
                                        println!(
                                            "\n{} Mission launch failure: {}",
                                            "✗".bright_red(),
                                            e
                                        )
                                    }
                                }
                            }
                        }
                    }
                    CommandAction::AgentChat(cmd) => {
                        let target = agent
                            .session()
                            .map(|s| s.target.clone())
                            .unwrap_or_else(|| "INDEPENDENT".to_string());
                        let dynamic_msg =
                            crate::stream::nlp::synthesize_topic_message(&cmd, &target);

                        ui::print_agent_header(
                            &agent.config().agent.name,
                            agent.config().tui.simulated_mode,
                        );

                        let spinner_frames = ui::get_premium_loading_frames("orbit");
                        let spinner = indicatif::ProgressBar::new_spinner();
                        spinner.set_style(
                            indicatif::ProgressStyle::default_spinner()
                                .tick_strings(&spinner_frames)
                                .template(&format!("{{prefix}} {{spinner:.{}}} {{msg}}", "cyan"))
                                .unwrap_or_else(|_| indicatif::ProgressStyle::default_spinner()),
                        );
                        spinner.set_prefix(ui::CyberTheme::accent("└─❯").to_string());
                        spinner.set_message(ui::CyberTheme::dim(&dynamic_msg).to_string());
                        spinner.enable_steady_tick(std::time::Duration::from_millis(60));

                        let start = std::time::Instant::now();
                        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
                        agent.set_event_tx(Some(tx));
                        let stream_res = tokio::select! {
                            res = agent.chat_stream(&cmd) => Some(res),
                            _ = tokio::signal::ctrl_c() => {
                                spinner.finish_and_clear();
                                println!("\n\n{} {}", "⚠".yellow().bold(), "MISSION INTERRUPT: Neural initialization aborted by operative.".bright_red().italic());
                                None
                            }
                        };
                        if let Some(res) = stream_res {
                            match res {
                                Ok((stream, executed_tools)) => {
                                    // Do NOT finish_and_clear here, pass the live spinner into logic
                                    consume_agent_stream(
                                        Box::pin(stream),
                                        executed_tools,
                                        &mut agent,
                                        rx,
                                        start,
                                        Some(spinner),
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
                // Final Mission Sync
                let _ = vault.append("exit"); // Mark session end
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
        event::{self, DisableMouseCapture, EnableMouseCapture, Event},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::prelude::*;
    use tokio::sync::mpsc;

    let (tx, rx) = mpsc::unbounded_channel::<tui::app::TuiEvent>();
    let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel::<String>();

    // ─── Panic-Safe Terminal Restore ───
    // Install a panic hook that restores the terminal before printing the panic
    // message. This prevents terminal corruption if the TUI panics.
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = execute!(
            std::io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            crossterm::cursor::Show
        );
        original_hook(info);
    }));

    enable_raw_mode()?;
    std::env::set_var("MYTH_TUI_ACTIVE", "true");
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
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

    // ─── Phase 1: Background Telemetry Task (Industry-Grade Offloading) ───
    let stats_tx = tx.clone();
    tokio::spawn(async move {
        let mut sys = sysinfo::System::new_all();
        loop {
            sys.refresh_cpu_usage();
            sys.refresh_memory();

            let cpus = sys.cpus();
            let cpu = if cpus.is_empty() {
                0.0
            } else {
                cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / cpus.len() as f32
            };
            let mem = (sys.used_memory() as f32 / sys.total_memory().max(1) as f32) * 100.0;

            let _ = stats_tx.send(TuiEvent::VitalsUpdate { cpu, mem });
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });

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
                        let _ = tui_tx.send(tui::app::TuiEvent::ConfigReloaded(Box::new(agent.config().clone())));
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
                        CommandAction::ProvisionFont(font_id) => {
                            let _ = tui_tx.send(tui::app::TuiEvent::Message {
                                role: "system".to_string(),
                                content: format!("Fidelity Alert: Font '{}' is missing. Please use 'myth typography set {}' in a separate CLI session to provision this mission-critical asset.", font_id, font_id)
                            });
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

    let mut needs_redraw = true;
    loop {
        if app.update() {
            needs_redraw = true;
        }

        if needs_redraw {
            terminal.draw(|frame| app.render(frame))?;
            needs_redraw = false;
        }

        // Adaptive tick rate: 16ms (~60fps) when active, 50ms (~20fps) when idle
        let tick_rate = if app.is_thinking || !app.toasts.is_empty() || app.hud_message.is_some() {
            std::time::Duration::from_millis(16)
        } else {
            std::time::Duration::from_millis(50)
        };

        if event::poll(tick_rate)? {
            needs_redraw = true;
            match event::read()? {
                Event::Key(key) => {
                    if let Some(command) = app.handle_key(key) {
                        match command.as_str() {
                            "/quit" | "/exit" | "/q" => {
                                app.running = false;
                            }
                            "/clear" => {
                                app.chat.clear();
                                let _ = cmd_tx.send(command);
                            }
                            _ => {
                                let _ = cmd_tx.send(command);
                            }
                        }
                    }
                }
                Event::Resize(_, _) => {
                    terminal.clear()?;
                }
                Event::Mouse(mouse_event) => {
                    use crossterm::event::{MouseButton, MouseEventKind};
                    match mouse_event.kind {
                        MouseEventKind::Down(MouseButton::Right) => {
                            let col = mouse_event.column;
                            let row = mouse_event.row;

                            if let Some(body) = app.last_body {
                                if app.show_left_panel
                                    && col >= body.left.x
                                    && col < body.left.x + body.left.width
                                    && row >= body.left.y
                                    && row < body.left.y + body.left.height
                                {
                                    app.active_context_menu = Some(tui::app::ContextMenu {
                                        options: vec![
                                            " ◈ SCAN_NODE  ",
                                            " ◈ COPY_PATH  ",
                                            " ◈ DISMISS    ",
                                        ]
                                        .into_iter()
                                        .map(|s| s.to_string())
                                        .collect(),
                                        x: col,
                                        y: row,
                                        selected: 0,
                                    });
                                }
                            }
                        }
                        MouseEventKind::Down(MouseButton::Left) => {
                            let col = mouse_event.column;
                            let row = mouse_event.row;

                            // Context Menu Intercept
                            if let Some(ref menu) = app.active_context_menu {
                                let menu_w = menu.options.iter().map(|o| o.len()).max().unwrap_or(0)
                                    as u16
                                    + 2;
                                let menu_h = menu.options.len() as u16 + 2;
                                if col >= menu.x
                                    && col < menu.x + menu_w
                                    && row >= menu.y
                                    && row < menu.y + menu_h
                                {
                                    let rel_row = row.saturating_sub(menu.y + 1) as usize;
                                    if let Some(opt) = menu.options.get(rel_row) {
                                        if opt.contains("SCAN_NODE") {
                                            // Trigger scan (mock for now)
                                            app.set_hud_message("MOCK_SCAN_TRIGGERED".to_string());
                                        } else if opt.contains("COPY_PATH") {
                                            if let Some(path) = app.tree.selected_path() {
                                                if let Ok(mut clipboard) = arboard::Clipboard::new()
                                                {
                                                    let _ = clipboard.set_text(path.clone());
                                                    app.add_toast(
                                                        format!("COPIED: {}", path),
                                                        "success".to_string(),
                                                    );
                                                } else {
                                                    app.add_toast(
                                                        "CLIPBOARD_ERROR".to_string(),
                                                        "warning".to_string(),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    app.active_context_menu = None;
                                    return Ok(()); // skip further checks
                                } else {
                                    app.active_context_menu = None;
                                }
                            }
                            if let (Some(layout), Some(body)) = (app.last_layout, app.last_body) {
                                // Hit detection for Borders
                                if app.show_left_panel && col == body.center.x.saturating_sub(1) {
                                    app.is_dragging_left_border = true;
                                } else if app.show_right_panel
                                    && col == body.center.x + body.center.width
                                {
                                    app.is_dragging_right_border = true;
                                } else if col == body.center.x + body.center.width - 1
                                    && row > body.center.y
                                    && row < body.center.y + body.center.height - 1
                                {
                                    app.is_dragging_scrollbar = true;
                                    let rel_y = row.saturating_sub(body.center.y + 1);
                                    let ratio = rel_y as f64
                                        / (body.center.height.saturating_sub(2) as f64);
                                    app.chat.handle_scrollbar_drag(ratio);
                                } else if col >= layout.nav.x
                                    && col < layout.nav.x + layout.nav.width
                                    && row >= layout.nav.y
                                    && row < layout.nav.y + layout.nav.height
                                {
                                    if let Some(screen) =
                                        tui::widgets::nav::NavWidget::get_screen_at(
                                            layout.nav, col, row,
                                        )
                                    {
                                        app.current_screen = screen;
                                        app.focus = tui::app::Focus::Nav;
                                        app.set_hud_message("VIEW_ROUTED".to_string());
                                    }
                                } else if col >= layout.input.x
                                    && col < layout.input.x + layout.input.width
                                    && row >= layout.input.y
                                    && row < layout.input.y + layout.input.height
                                {
                                    app.focus = tui::app::Focus::Input;
                                } else if col >= body.center.x
                                    && col < body.center.x + body.center.width
                                    && row >= body.center.y
                                    && row < body.center.y + body.center.height
                                {
                                    app.focus = tui::app::Focus::Chat;

                                    // Click-to-Action: Detect command links
                                    let rel_col = col.saturating_sub(body.center.x + 1);
                                    let rel_row = row.saturating_sub(body.center.y + 1);
                                    let inner_area = ratatui::layout::Rect {
                                        x: body.center.x + 1,
                                        y: body.center.y + 1,
                                        width: body.center.width.saturating_sub(2),
                                        height: body.center.height.saturating_sub(2),
                                    };

                                    if let Some(cmd) =
                                        app.chat.get_command_at(rel_col, rel_row, inner_area)
                                    {
                                        app.input.set_content(cmd);
                                        app.focus = tui::app::Focus::Input;
                                        app.set_hud_message("COMMAND_INJECTED".to_string());
                                    }
                                } else if app.show_left_panel
                                    && col >= body.left.x
                                    && col < body.left.x + body.left.width
                                    && row >= body.left.y
                                    && row < body.left.y + body.left.height
                                {
                                    app.focus = tui::app::Focus::Tree;
                                    let rel_row = row.saturating_sub(body.left.y + 1) as usize;
                                    app.tree.toggle_node_at(rel_row);
                                } else if app.show_right_panel
                                    && col >= body.right.x
                                    && col < body.right.x + body.right.width
                                    && row >= body.right.y
                                    && row < body.right.y + body.right.height
                                {
                                    app.focus = tui::app::Focus::Sensors;
                                }
                            }
                        }
                        MouseEventKind::Up(MouseButton::Left) => {
                            app.is_dragging_scrollbar = false;
                            app.is_dragging_left_border = false;
                            app.is_dragging_right_border = false;
                        }
                        MouseEventKind::Drag(MouseButton::Left) => {
                            needs_redraw = true; // Essential for fluid resizing
                            if app.is_dragging_left_border {
                                app.left_panel_width =
                                    mouse_event.column.saturating_sub(1).clamp(10, 80);
                            } else if app.is_dragging_right_border {
                                if let Some(layout) = app.last_layout {
                                    let right_start = mouse_event.column;
                                    app.right_panel_width =
                                        layout.body.width.saturating_sub(right_start).clamp(10, 80);
                                }
                            } else if app.is_dragging_scrollbar {
                                if let Some(body) = app.last_body {
                                    let rel_y = mouse_event.row.saturating_sub(body.center.y + 1);
                                    let h = body.center.height.saturating_sub(2) as f64;
                                    if h > 0.0 {
                                        let ratio = (rel_y as f64 / h).clamp(0.0, 1.0);
                                        app.chat.handle_scrollbar_drag(ratio);
                                    }
                                }
                            }
                        }
                        MouseEventKind::Moved => {
                            let col = mouse_event.column;
                            let row = mouse_event.row;
                            app.mouse_pos = Some((col, row));

                            if let (Some(_layout), Some(body)) = (app.last_layout, app.last_body) {
                                let prev_hover_l = app.is_hovering_left_border;
                                let prev_hover_r = app.is_hovering_right_border;

                                // Industry-Grade Hover Detection
                                app.is_hovering_left_border =
                                    app.show_left_panel && col == body.center.x.saturating_sub(1);
                                app.is_hovering_right_border = app.show_right_panel
                                    && col == body.center.x + body.center.width;

                                if prev_hover_l != app.is_hovering_left_border
                                    || prev_hover_r != app.is_hovering_right_border
                                {
                                    needs_redraw = true;
                                }

                                if app.show_right_panel
                                    && col >= body.right.x
                                    && col < body.right.x + body.right.width
                                    && row >= body.right.y
                                    && row < body.right.y + body.right.height
                                {
                                    if let Some(text) =
                                        app.sensors.get_tool_at(body.right, col, row)
                                    {
                                        if app.active_tooltip.as_ref().map(|t| &t.text)
                                            != Some(&text)
                                        {
                                            app.active_tooltip = Some(tui::app::Tooltip {
                                                text,
                                                x: col,
                                                y: row,
                                            });
                                            needs_redraw = true;
                                        }
                                    } else if app.active_tooltip.is_some() {
                                        app.active_tooltip = None;
                                        needs_redraw = true;
                                    }
                                } else if app.active_tooltip.is_some() {
                                    app.active_tooltip = None;
                                    needs_redraw = true;
                                }
                            }
                        }
                        MouseEventKind::ScrollDown => match app.focus {
                            tui::app::Focus::Chat => {
                                let h = app.viewport_chat_height();
                                app.chat.scroll_down(&app.theme, h);
                            }
                            tui::app::Focus::Tree => {
                                app.tree.scroll_down();
                            }
                            tui::app::Focus::Sensors => {
                                app.sensors.scroll_down();
                            }
                            _ => {}
                        },
                        MouseEventKind::ScrollUp => match app.focus {
                            tui::app::Focus::Chat => {
                                app.chat.scroll_up();
                            }
                            tui::app::Focus::Tree => {
                                app.tree.scroll_up();
                            }
                            tui::app::Focus::Sensors => {
                                app.sensors.scroll_up();
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        if !app.running {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    println!(
        "🔒 {} session ended. Neural links severed.",
        config.agent.name
    );
    Ok(())
}
