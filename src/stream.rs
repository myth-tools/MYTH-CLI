use crate::core;
use crate::markdown_renderer::{render_char_pureviz, RenderVizState};
use crate::tui;
use crate::ui;
use color_eyre::eyre::Result;
use futures::StreamExt;
use owo_colors::OwoColorize;
use std::io::Write;

/// Consumes the agent stream for TUI mode, filtering JSON outputs natively at ultra-high speed
/// ensuring zero-latency chat rendering with no UI blocking.
pub async fn stream_to_tui(
    mut stream: impl futures::Stream<Item = Result<String, core::agent::AgentError>> + Unpin,
    tui_tx: &tokio::sync::mpsc::UnboundedSender<tui::app::TuiEvent>,
) {
    let mut buffer = String::new();
    let mut in_json = false;
    let mut brace_count = 0;

    while let Some(chunk_result) = stream.next().await {
        if let Ok(chunk) = chunk_result {
            for c in chunk.chars() {
                buffer.push(c);

                if in_json {
                    if c == '{' || c == '[' {
                        brace_count += 1;
                    } else if c == '}' || c == ']' {
                        brace_count -= 1;
                        if brace_count <= 0 {
                            in_json = false;
                            buffer.clear();
                        }
                    }
                    continue;
                }

                if (c == '{' || c == '[') && buffer.trim().is_empty() {
                    in_json = true;
                    brace_count = 1;
                    continue;
                }

                let to_send = if buffer.contains('\n') {
                    let idx = buffer.find('\n').unwrap();
                    let s = buffer[..=idx].to_string();
                    buffer.drain(..=idx);
                    s
                } else if buffer.len() > 12 {
                    let safe_len = buffer.len() - 12;
                    let split_pos = buffer
                        .char_indices()
                        .map(|(idx, _)| idx)
                        .take_while(|&idx| idx <= safe_len)
                        .last()
                        .unwrap_or(0);

                    if split_pos > 0 {
                        let s = buffer[..split_pos].to_string();
                        buffer.drain(..split_pos);
                        s
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };

                if !to_send.is_empty() {
                    let _ = tui_tx.send(tui::app::TuiEvent::MessageChunk { chunk: to_send });
                }
            }
        }
    }

    if !buffer.is_empty() && !in_json {
        let _ = tui_tx.send(tui::app::TuiEvent::MessageChunk { chunk: buffer });
    }
}

/// Consume a streaming response from the agent and print it live, suppressing technical JSON blocks.
pub async fn consume_agent_stream(
    mut stream: impl futures::Stream<Item = Result<String, core::agent::AgentError>> + Unpin,
    executed_tools: std::sync::Arc<tokio::sync::Mutex<Vec<(String, String)>>>,
    agent: &mut core::agent::ReconAgent,
    mut event_rx: tokio::sync::mpsc::UnboundedReceiver<crate::tui::app::TuiEvent>,
    start: std::time::Instant,
) -> Result<()> {
    let mut rv_state = RenderVizState::default();

    let target = agent
        .session()
        .map(|s| s.target.clone())
        .unwrap_or_else(|| "INDEPENDENT".to_string());
    let model = agent.config().llm.model.clone();

    let sync_msgs = [
        format!("Synthesizing tactical landscape for {}...", target),
        format!("Fingerprinting {} offensive surface...", target),
        format!("Modeling payload delivery via {}...", model),
        format!("Optimizing AST sub-structures for {}...", target),
        "Refining neural weights for tactical analysis...".to_string(),
        "Indexing semantic memory shards into QDRANT...".to_string(),
        "Validating MCP schema integrity and tool-gates...".to_string(),
        "Calibrating side-channel jitter on mission threads...".to_string(),
        "Injecting entropy markers into tactical stream...".to_string(),
        "Encrypting volatile telemetry blocks (AES-GCM-256)...".to_string(),
        format!("Mapping vulnerable paths on node {}...", target),
        "Negotiating zero-knowledge proof for tool-access...".to_string(),
        "Probing entropy residual in target RNG pool...".to_string(),
        "Scanning for volatile memory artifacts...".to_string(),
        "Calculating sub-millisecond response latency...".to_string(),
    ];
    let msg_idx = (chrono::Utc::now().timestamp_subsec_millis() as usize) % sync_msgs.len();

    ui::print_agent_prefix(&agent.config().agent.name);

    let mut interrupted = false;
    let mut ctrl_c = Box::pin(tokio::signal::ctrl_c());

    let stdout = std::io::stdout();
    let mut writer = std::io::BufWriter::with_capacity(4096, stdout.lock());

    let mut first_chunk = true;
    let spinner = indicatif::ProgressBar::new_spinner();

    spinner.set_style(
        indicatif::ProgressStyle::default_spinner()
            .tick_strings(&[
                " ", "▂", "▃", "▄", "▅", "▆", "▇", "█", "▇", "▆", "▅", "▄", "▃", " ",
            ])
            .template(&format!("{{spinner:.{}}} {{msg}}", "magenta"))
            .unwrap(),
    );
    spinner.set_message(ui::CyberTheme::dim(&sync_msgs[msg_idx]).to_string());
    spinner.enable_steady_tick(std::time::Duration::from_millis(60));

    let mut full_response = String::new();
    let mut buffer = String::new();
    let mut in_json_block = false;
    let mut in_string = false;
    let mut escaped = false;
    let mut brace_count: i32 = 0;
    let mut char_counter: u64 = 0;
    let mut in_thought = false;

    loop {
        let chunk_result = tokio::select! {
            res = stream.next() => res,
            Some(event) = event_rx.recv() => {
                match event {
                    crate::tui::app::TuiEvent::ToolStarted { tool: _, command } => {

                        if !buffer.is_empty() {
                            let to_flush = buffer.clone();
                            buffer.clear();
                            for bc in to_flush.chars() {
                                render_char_pureviz(bc, &mut rv_state, &mut writer);
                            }
                            writer.flush().ok();
                        }
                        writeln!(writer, "\n  {} {} {}",
                            ui::CyberTheme::secondary("⚙").bold(),
                            ui::CyberTheme::dim("SYNCHRONIZING TACTICAL LOGIC:").italic(),
                            command.bright_cyan().bold()
                        ).ok();
                        writer.flush().ok();
                    }
                    crate::tui::app::TuiEvent::ToolFinished { tool: _, success: _ } => {
                        writeln!(writer, "  {} {}", "✓".bright_green(), "Technical logic synchronized.".dimmed().italic()).ok();
                        writer.flush().ok();
                    }
                    crate::tui::app::TuiEvent::ToolStream { tool, line } => {
                        let line_lc = line.to_lowercase();
                        let mut style = owo_colors::Style::new().italic();

                        if line_lc.contains("error") || line_lc.contains("fail") || line_lc.contains("panic") || line_lc.contains("denied") {
                            style = style.bright_red().bold();
                        } else if line_lc.contains("warn") || line_lc.contains("suspicious") || line_lc.contains("risk") {
                            style = style.yellow();
                        } else if line_lc.contains("success") || line_lc.contains("found") || line_lc.contains("complete") || line_lc.contains("allowed") {
                            style = style.bright_green();
                        } else if line_lc.contains("searching") || line_lc.contains("query") || line_lc.contains("scanning") {
                            style = style.bright_blue();
                        } else if line_lc.contains("node") || line_lc.contains("addr") || line_lc.contains("ip") {
                            style = style.bright_magenta();
                        } else {
                            style = style.bright_black();
                        };

                        writeln!(writer, "    {} {} {}",
                            "│".bright_black(),
                            tool.dimmed(),
                            line.style(style)
                        ).ok();
                        writer.flush().ok();
                    }
                    _ => {}
                }
                continue;
            }
            _ = &mut ctrl_c => {
                println!("\n\n{} {}", "⚠".yellow().bold(), "MISSION INTERRUPT: Stream terminated by operative.".bright_red().italic());
                interrupted = true;
                None
            }
        };

        let chunk_result = match chunk_result {
            Some(res) => res,
            None => break,
        };

        match chunk_result {
            Ok(chunk) => {
                if chunk.is_empty() {
                    continue;
                }

                if first_chunk {
                    spinner.finish_and_clear();
                    write!(writer, "\r{} ", ui::CyberTheme::accent("└─❯")).ok();
                    writer.flush().ok();
                    first_chunk = false;
                }

                full_response.push_str(&chunk);

                for c in chunk.chars() {
                    buffer.push(c);

                    if !in_json_block && !in_thought {
                        // Smooth streaming: sleep every 3rd char for ~3x faster throughput
                        char_counter += 1;
                        if char_counter.is_multiple_of(3) {
                            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
                        }
                    }

                    if !in_thought && !in_json_block && buffer.contains("<thought>") {
                        if let Some(pos) = buffer.find("<thought>") {
                            let pre_thought = buffer[..pos].to_string();
                            for bc in pre_thought.chars() {
                                render_char_pureviz(bc, &mut rv_state, &mut writer);
                            }
                            in_thought = true;
                            write!(
                                writer,
                                "\n  {} {}\n",
                                ui::CyberTheme::primary("🧠").bold(),
                                ui::CyberTheme::dim("NEURAL THOUGHT PROCESS").italic()
                            )
                            .ok();
                            write!(writer, "\x1b[2;3m").ok(); // Dim + Italic
                            buffer.drain(..pos + "<thought>".len());
                        }
                    }

                    if in_thought {
                        if buffer.contains("</thought>") {
                            if let Some(pos) = buffer.find("</thought>") {
                                let thought_content = buffer[..pos].to_string();
                                write!(writer, "{}", thought_content).ok();
                                write!(writer, "\x1b[0m").ok(); // Reset thought style
                                in_thought = false;
                                buffer.drain(..pos + "</thought>".len());
                                continue;
                            }
                        }

                        if buffer.len() > 24 {
                            let safe_to_print = buffer.len() - 24;
                            let split_pos = buffer
                                .char_indices()
                                .map(|(idx, _)| idx)
                                .take_while(|&idx| idx <= safe_to_print)
                                .last()
                                .unwrap_or(0);

                            if split_pos > 0 {
                                let to_print: String = buffer.drain(..split_pos).collect();
                                write!(writer, "{}", to_print).ok();
                            }
                        }
                        continue;
                    }

                    if in_json_block {
                        if escaped {
                            escaped = false;
                        } else if c == '\\' {
                            escaped = true;
                        } else if c == '"' {
                            in_string = !in_string;
                        } else if !in_string {
                            if c == '{' || c == '[' {
                                brace_count += 1;
                            } else if c == '}' || c == ']' {
                                brace_count -= 1;
                                if brace_count <= 0 {
                                    in_json_block = false;
                                    if buffer.contains("\"stdout\":")
                                        || buffer.contains("\"tools\":")
                                        || buffer.contains("\"call\":")
                                    {
                                        writeln!(
                                            writer,
                                            "  {} {}",
                                            "✓".bright_green(),
                                            "Technical logic synchronized.".dimmed().italic()
                                        )
                                        .ok();
                                    }
                                    buffer.clear();
                                }
                            }
                        }
                        continue;
                    }

                    if !in_thought && (c == '{' || c == '[') {
                        // Detect JSON blocks at content boundaries:
                        // 1. At start of buffer (only whitespace before the brace)
                        // 2. At start of a new line
                        let pre_brace = buffer[..buffer.len() - 1].trim();
                        let at_boundary = pre_brace.is_empty() || pre_brace.ends_with('\n');
                        if at_boundary {
                            in_json_block = true;
                            in_string = false;
                            escaped = false;
                            brace_count = 1;
                            write!(
                                writer,
                                "\n  {} {}\n",
                                ui::CyberTheme::secondary("⚙").bold(),
                                ui::CyberTheme::dim("SYNCHRONIZING TACTICAL LOGIC...").italic()
                            )
                            .ok();
                            continue;
                        }
                    }

                    if buffer.len() > 32
                        || (buffer.contains('\n')
                            && !buffer.starts_with('<')
                            && !buffer.starts_with('{'))
                    {
                        let to_process = if buffer.contains('\n')
                            && !buffer.starts_with('<')
                            && !buffer.starts_with('{')
                        {
                            let idx = buffer.find('\n').unwrap();
                            let s = buffer[..=idx].to_string();
                            buffer.drain(..=idx);
                            s
                        } else if buffer.len() > 32 {
                            let safe_len = buffer.len() - 32;
                            let split_pos = buffer
                                .char_indices()
                                .map(|(idx, _)| idx)
                                .take_while(|&idx| idx <= safe_len)
                                .last()
                                .unwrap_or(0);

                            if split_pos > 0 {
                                buffer.drain(..split_pos).collect()
                            } else {
                                String::new()
                            }
                        } else {
                            String::new()
                        };

                        for bc in to_process.chars() {
                            render_char_pureviz(bc, &mut rv_state, &mut writer);
                        }
                    }
                }
                writer.flush().ok();
            }

            Err(e) => {
                writeln!(writer, "\n{} Link degraded: {}", "✗".bright_red(), e).ok();
                writer.flush().ok();
                break;
            }
        }
    }

    if first_chunk {
        spinner.finish_and_clear();
    }

    {
        let stdout_final = std::io::stdout();
        let mut writer_final = std::io::BufWriter::new(stdout_final.lock());

        if !buffer.is_empty() {
            let final_dump = buffer.replace("<thought>", "").replace("</thought>", "");
            if !final_dump.trim().is_empty() {
                for bc in final_dump.chars() {
                    render_char_pureviz(bc, &mut rv_state, &mut writer_final);
                }
            }
        }

        write!(writer_final, "\x1b[0m").ok();
        writeln!(writer_final).ok();
        writer_final.flush().ok();
    }

    if !interrupted {
        let duration = start.elapsed();
        agent.print_tool_summary(executed_tools).await;
        let secs = duration.as_secs_f32();
        println!(
            "  {} {}s {}",
            "└→".dimmed(),
            format!("{:.2}", secs).bright_white(),
            "Response latency synchronized.".dimmed().italic()
        );

        agent.add_assistant_message(&full_response);
    }

    Ok(())
}
