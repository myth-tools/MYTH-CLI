use crate::core;
use crate::markdown_renderer::{flush_final, render_char_pureviz, RenderVizState};
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
                } else if buffer.len() > 6 {
                    let safe_len = buffer.len() - 6;
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
    existing_spinner: Option<indicatif::ProgressBar>,
) -> Result<()> {
    let mut rv_state = RenderVizState::default();

    let mut first_chunk = true;
    let spinner_frames = ui::get_premium_loading_frames("flux");
    let spinner = if let Some(s) = existing_spinner {
        s
    } else {
        let (prompt, target) = {
            let target = agent
                .session()
                .map(|s| s.target.clone())
                .unwrap_or_else(|| "INDEPENDENT".to_string());
            let prompt = agent
                .messages()
                .iter()
                .rev()
                .find(|m| matches!(m.role, crate::core::agent::MessageRole::User))
                .map(|m| m.content.clone())
                .unwrap_or_default();
            (prompt, target)
        };
        let dynamic_msg = nlp::synthesize_topic_message(&prompt, &target);

        ui::print_agent_header(
            &agent.config().agent.name,
            agent.config().tui.simulated_mode,
        );

        let s = indicatif::ProgressBar::new_spinner();
        s.set_style(
            indicatif::ProgressStyle::default_spinner()
                .tick_strings(&spinner_frames)
                .template(&format!("{{prefix}} {{spinner:.{}}} {{msg}}", "magenta"))
                .unwrap(),
        );
        s.set_prefix(ui::CyberTheme::accent("└─❯").to_string());
        s.set_message(ui::CyberTheme::dim(&dynamic_msg).to_string());
        s.enable_steady_tick(std::time::Duration::from_millis(80));
        s
    };

    // Ensure the spinner uses the new premium frames even if passed from caller
    spinner.set_style(
        indicatif::ProgressStyle::default_spinner()
            .tick_strings(&spinner_frames)
            .template(&format!("{{prefix}} {{spinner:.{}}} {{msg}}", "magenta"))
            .unwrap(),
    );

    let mut interrupted = false;
    let mut ctrl_c = Box::pin(tokio::signal::ctrl_c());

    let stdout = std::io::stdout();
    let mut writer = std::io::BufWriter::with_capacity(4096, stdout.lock());

    let mut full_response = String::new();
    let mut buffer = String::new();
    let mut in_json_block = false;
    let mut in_string = false;
    let mut escaped = false;
    let mut brace_count: i32 = 0;
    let mut in_thought = false;

    loop {
        let chunk_result = tokio::select! {
            res = stream.next() => res,
            Some(event) = event_rx.recv() => {
                match event {
                    crate::tui::app::TuiEvent::WebSearchStarted { server, query } => {
                        if !buffer.is_empty() {
                            let to_flush = buffer.clone();
                            buffer.clear();
                            for bc in to_flush.chars() {
                                render_char_pureviz(bc, &mut rv_state, &mut writer);
                            }
                            writer.flush().ok();
                        }
                        writeln!(writer, "\n  {} {} {} {}",
                            ui::CyberTheme::accent("🌐"),
                            ui::CyberTheme::primary("INTEL").on_bright_blue().white().bold(),
                            ui::CyberTheme::dim(format!("[ {} ] SEARCHING =>", server.to_uppercase())).italic(),
                            query.bright_cyan().bold().italic()
                        ).ok();
                        writer.flush().ok();
                    }
                    crate::tui::app::TuiEvent::WebSourceFound { source } => {
                        writeln!(writer, "    {} {} {} {}",
                            "│".bright_black(),
                            "📍".bright_blue(),
                            ui::CyberTheme::secondary("SOURCE").on_bright_black().white().bold(),
                            ui::CyberTheme::dim(format!("[ {} ] INDEXED", source.to_uppercase())).italic()
                        ).ok();
                        writer.flush().ok();
                    }
                    crate::tui::app::TuiEvent::ToolStarted { server, tool, args } => {
                        if !buffer.is_empty() {
                            let to_flush = buffer.clone();
                            buffer.clear();
                            for bc in to_flush.chars() {
                                render_char_pureviz(bc, &mut rv_state, &mut writer);
                            }
                            writer.flush().ok();
                        }
                        writeln!(writer, "\n  {} {} {} {}",
                            ui::CyberTheme::accent("⚡"),
                            ui::CyberTheme::primary(format!("[ {} ]", server.to_uppercase())).on_bright_black().white().bold(),
                            ui::CyberTheme::dim("EXECUTING =>").italic(),
                            format!("{} {}", tool, args).bright_cyan().bold()
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
                        // Ultra-fast streaming: no artificial sleep.
                    }

                    if !in_thought && !in_json_block && buffer.contains("<thought>") {
                        if let Some(pos) = buffer.find("<thought>") {
                            let pre_thought = buffer[..pos].to_string();
                            let mapped_thought = ui::apply_font(
                                &pre_thought,
                                &agent.config().tui.font,
                                agent.config().tui.simulated_mode,
                            );
                            for bc in mapped_thought.chars() {
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
                                let mapped_print = ui::apply_font(
                                    &to_print,
                                    &agent.config().tui.font,
                                    agent.config().tui.simulated_mode,
                                );
                                write!(writer, "{}", mapped_print).ok();
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

                        let mapped_process = ui::apply_font(
                            &to_process,
                            &agent.config().tui.font,
                            agent.config().tui.simulated_mode,
                        );
                        for bc in mapped_process.chars() {
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
                let mapped_final = ui::apply_font(
                    &final_dump,
                    &agent.config().tui.font,
                    agent.config().tui.simulated_mode,
                );
                for bc in mapped_final.chars() {
                    render_char_pureviz(bc, &mut rv_state, &mut writer_final);
                }
            }
        }

        if rv_state.in_table
            || !rv_state.pending_markdown.is_empty()
            || !rv_state.line_buffer.is_empty()
        {
            flush_final(&mut rv_state, &mut writer_final);
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

/// Advanced NLP/Entropy engine for generating highly contextual loading states
pub mod nlp {
    use once_cell::sync::Lazy;
    use std::collections::HashSet;

    static STOPWORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
        vec![
            "the", "is", "at", "which", "on", "in", "for", "a", "an", "and", "or", "to", "of",
            "you", "me", "can", "could", "would", "should", "please", "make", "this", "do", "it",
            "that", "how", "what", "where", "why", "who", "when", "with", "from", "about", "my",
            "your", "are", "be", "been", "was", "were", "have", "has", "had", "not", "no", "yes",
            "i", "we", "they", "he", "she", "so", "but", "as", "if",
        ]
        .into_iter()
        .collect()
    });

    pub fn synthesize_topic_message(prompt: &str, target: &str) -> String {
        let clean_prompt = prompt.trim();
        if clean_prompt.is_empty() {
            return "Synchronizing cognitive streams...".to_string();
        }

        let words: Vec<&str> = clean_prompt.split_whitespace().collect();
        if words.is_empty() {
            return "Establishing neural handshake protocol...".to_string();
        }

        let build_verbs = [
            "create",
            "build",
            "make",
            "generate",
            "write",
            "code",
            "develop",
            "implement",
            "construct",
        ];
        let analyze_verbs = [
            "analyze",
            "check",
            "scan",
            "investigate",
            "explore",
            "find",
            "search",
            "nmap",
            "discover",
            "recon",
            "audit",
        ];
        let fix_verbs = [
            "fix", "debug", "resolve", "patch", "repair", "correct", "update",
        ];
        let explain_verbs = [
            "explain",
            "tell",
            "summarize",
            "describe",
            "understand",
            "read",
            "show",
            "what",
            "how",
        ];

        let mut intent = "synthesize";
        let mut best_topic = if target == "INDEPENDENT" {
            String::new()
        } else {
            target.to_string()
        };
        let mut max_gravity = -1.0;

        for w in words.iter().take(4) {
            let lw = w.to_lowercase();
            let clean_lw: String = lw.chars().filter(|c| c.is_alphanumeric()).collect();
            if build_verbs.contains(&clean_lw.as_str()) {
                intent = "build";
                break;
            }
            if analyze_verbs.contains(&clean_lw.as_str()) {
                intent = "analyze";
                break;
            }
            if fix_verbs.contains(&clean_lw.as_str()) {
                intent = "fix";
                break;
            }
            if explain_verbs.contains(&clean_lw.as_str()) {
                intent = "explain";
                break;
            }
        }

        for i in 0..words.len() {
            let w1 = words[i];
            let cw1: String = w1
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '-' || *c == '/')
                .collect();
            if cw1.is_empty() || STOPWORDS.contains(cw1.as_str()) {
                continue;
            }

            let mut gravity = cw1.len() as f32;
            if cw1.contains('.') || cw1.contains('/') || cw1.contains('-') || cw1.contains('_') {
                gravity += 12.0;
            }
            if cw1.ends_with("ing")
                || cw1.ends_with("ion")
                || cw1.ends_with("ity")
                || cw1.ends_with("ment")
            {
                gravity += 4.0;
            }
            if w1.chars().next().unwrap_or('a').is_uppercase() && i > 0 {
                gravity += 6.0;
            }

            let mut candidate_phrase = w1.to_string();
            // N-Gram synergy logic for multi-word precision topics
            if i + 1 < words.len() {
                let w2 = words[i + 1];
                let cw2: String = w2
                    .to_lowercase()
                    .chars()
                    .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '-' || *c == '/')
                    .collect();
                if !cw2.is_empty() && !STOPWORDS.contains(cw2.as_str()) {
                    let mut gravity2 = cw2.len() as f32;
                    if cw2.contains('.')
                        || cw2.contains('/')
                        || cw2.contains('-')
                        || cw2.contains('_')
                    {
                        gravity2 += 12.0;
                    }
                    if w2.chars().next().unwrap_or('a').is_uppercase() {
                        gravity2 += 6.0;
                    }
                    gravity += gravity2 + 5.0; // Amplification bonus for adjacent high-entropy words
                    candidate_phrase = format!("{} {}", w1, w2);
                }
            }

            if gravity > max_gravity {
                max_gravity = gravity;
                best_topic = candidate_phrase;
            }
        }

        let best_topic = best_topic
            .trim_end_matches(|c: char| !c.is_alphanumeric() && c != '.')
            .to_string();

        if best_topic.is_empty() || max_gravity < 4.0 {
            if intent == "explain" && target != "INDEPENDENT" && !target.is_empty() {
                return format!("Querying logic subsystems for {}...", target);
            }
            return "Calibrating linguistic models for interaction...".to_string();
        }

        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_micros() as usize)
            .unwrap_or_else(|_| {
                (chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) / 1000) as usize
            });

        // Add thread-local pseudo-random jitter factor for selection variety
        let jitter = (ts % 7) + (ts >> 4);
        let idx = (ts + jitter) % 3;

        match intent {
            "build" => {
                let msgs = [
                    format!("Compiling structural execution graph for {}...", best_topic),
                    format!(
                        "Initializing architectural scaffolding for {}...",
                        best_topic
                    ),
                    format!("Assembling abstract components for {}...", best_topic),
                ];
                msgs[idx % msgs.len()].clone()
            }
            "analyze" => {
                let msgs = [
                    format!("Deploying deep-scan heuristics against {}...", best_topic),
                    format!("Triangulating threat vectors surrounding {}...", best_topic),
                    format!("Executing recursive analysis on {}...", best_topic),
                ];
                msgs[idx % msgs.len()].clone()
            }
            "fix" => {
                let msgs = [
                    format!(
                        "Isolating logical constraints and faults in {}...",
                        best_topic
                    ),
                    format!("Correlating error pathways for {}...", best_topic),
                    format!("Synthesizing defensive patches for {}...", best_topic),
                ];
                msgs[idx % msgs.len()].clone()
            }
            "explain" => {
                let msgs = [
                    format!("Synthesizing epistemological shards on {}...", best_topic),
                    format!("Querying semantic memory banks for {}...", best_topic),
                    format!("Constructing theoretical models of {}...", best_topic),
                ];
                msgs[idx % msgs.len()].clone()
            }
            _ => {
                // synthesize
                let msgs = [
                    format!("Calibrating neural weights to process {}...", best_topic),
                    format!("Refining vector embeddings for {}...", best_topic),
                    format!("Aligning topological space with {}...", best_topic),
                ];
                msgs[idx % msgs.len()].clone()
            }
        }
    }
}
