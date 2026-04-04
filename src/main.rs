#![recursion_limit = "256"]
//! MYTH — AI-Powered Reconnaissance Agent for Kali Linux
//!
//! Ultra-fast, sandboxed, volatile CLI agent that leverages
//! 3000+ Kali tools via MCP, powered by NVIDIA NIM and Rig.rs.

use clap::Parser;
use color_eyre::eyre::Result;
use owo_colors::OwoColorize;

use myth::cli::{dispatch, Cli};
use myth::config;
use myth::ui;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    // ─── Load Config (Two-Tier: embedded defaults + user.yaml) ───
    let mut agent_config = config::AppConfig::load_merged().unwrap_or_else(|e| {
        eprintln!("⚠ Config error: {}. Using embedded defaults.", e);
        serde_yaml::from_str(include_str!("../config/agent.yaml"))
            .expect("Embedded default config should be valid")
    });

    // ─── Proxy Global Setup ───
    if agent_config.proxy.enabled && agent_config.proxy.use_for_llm {
        let proxy_url_opt = if agent_config.proxy.auto_rotate {
            Some(
                agent_config
                    .proxy
                    .url
                    .clone()
                    .unwrap_or_else(|| "socks5://127.0.0.1:9050".to_string()),
            )
        } else {
            agent_config.proxy.url.clone()
        };

        if let Some(proxy_url) = proxy_url_opt {
            std::env::set_var("http_proxy", &proxy_url);
            std::env::set_var("https_proxy", &proxy_url);
            std::env::set_var("ALL_PROXY", &proxy_url);
            // Some libraries look for uppercase versions
            std::env::set_var("HTTP_PROXY", &proxy_url);
            std::env::set_var("HTTPS_PROXY", &proxy_url);
            tracing::info!(proxy = %proxy_url, "Neural uplink proxied (IP Rotation Active)");
        }
    }

    // Apply CLI overrides
    if let Some(ref level) = cli.log_level {
        agent_config.agent.log_level = level.clone();
    }
    if cli.no_sandbox {
        println!(
            "{}",
            "\n======================================================="
                .red()
                .bold()
        );
        println!(
            "{}",
            "                 CRITICAL WARNING                      "
                .red()
                .bold()
                .on_black()
        );
        println!(
            "{}",
            "======================================================="
                .red()
                .bold()
        );
        println!("You are starting MYTH CLI with the sandbox DISABLED.");
        println!("All agent commands will run directly on your host OS.");
        println!("This includes any malicious payloads the LLM might hallucinate");
        println!("or execute due to prompt injection vulnerabilities.");
        println!(
            "{}",
            "\nDo you understand the risks and wish to proceed? [y/N]: "
                .yellow()
                .bold()
        );

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read stdin");
        if input.trim().eq_ignore_ascii_case("y") || input.trim().eq_ignore_ascii_case("yes") {
            agent_config.sandbox.enabled = false;
            println!(
                "{}",
                "Sandbox disabled. Proceeding at your own risk.\n"
                    .red()
                    .italic()
            );
        } else {
            println!("{}", "Mission aborted.".green());
            std::process::exit(1);
        }
    }
    if cli.no_tui {
        agent_config.tui.enabled = false;
    }

    let is_completion = matches!(cli.command, Some(myth::cli::Commands::Completions { .. }));

    // ─── Init Logging (Dual-Sink: Stdout + Forensic File) ───
    let mut _guard = None; // Keep the worker alive
    if !is_completion {
        use tracing_subscriber::prelude::*;

        // 1. Log File Sink: Forensic-grade telemetry
        // Tries /var/log/myth (system install) first; falls back to ~/.myth/logs
        // for user-mode installs and Termux (Android) where /var/log is not writable.
        let ctx = myth::config::SystemContext::sense();
        let system_log_dir = ctx.join_log("myth");

        let (file_appender, guard) = if system_log_dir.is_dir()
            && std::fs::metadata(&system_log_dir)
                .map(|m| !m.permissions().readonly())
                .unwrap_or(false)
        {
            tracing_appender::non_blocking(tracing_appender::rolling::daily(
                &system_log_dir,
                "agent.log",
            ))
        } else {
            // User-space fallback — works on Termux, rootless installs, any Linux
            let home_log = dirs::home_dir()
                .unwrap_or_else(std::env::temp_dir)
                .join(".myth/logs");
            std::fs::create_dir_all(&home_log).ok();
            tracing_appender::non_blocking(tracing_appender::rolling::daily(home_log, "agent.log"))
        };
        _guard = Some(guard);

        let log_filter = agent_config.agent.log_level.to_lowercase();
        let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&log_filter));

        // Registry construction
        tracing_subscriber::registry()
            .with(env_filter)
            // Layer 1: Premium Compact Stdout
            .with(
                tracing_subscriber::fmt::layer()
                    .with_target(false)
                    .compact(),
            )
            // Layer 2: Forensic JSON File (Detailed)
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file_appender)
                    .json(),
            )
            .init();

        // Premium Initialization Sequence
        ui::print_banner();
        ui::boot_sequence(&agent_config).await;

        tracing::info!(
            name = %agent_config.agent.name,
            version = %agent_config.agent.version,
            "MISSION START: Neural Core Active"
        );
    }

    // Medical-Grade Panic Recovery: Ensure font restoration even on fatal crashes.
    std::panic::set_hook(Box::new(|_info| {
        ui::revert_terminal_font();
    }));

    // ─── Handle Commands with Protocol Zero (SIGINT, SIGTERM, SIGHUP) ───
    use tokio::signal::unix::{signal, SignalKind};
    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sighup = signal(SignalKind::hangup())?;

    tokio::select! {
        res = dispatch(cli, &agent_config) => {
            res?;
        }
        _ = tokio::signal::ctrl_c() => {
            println!("\n\n{}", " [!] PROTOCOL ZERO INITIATED (SIGINT) ".on_red().white().bold());
            terminal_cleanup();
        }
        _ = sigterm.recv() => {
            println!("\n\n{}", " [!] EXTERNAL ABORT DETECTED (SIGTERM) ".on_red().white().bold());
            terminal_cleanup();
        }
        _ = sighup.recv() => {
            // SIGHUP is often sent when the terminal tab is closed.
            ui::revert_terminal_font();
            std::process::exit(129);
        }
    }

    ui::revert_terminal_font(); // Restore terminal typography on normal exit
    Ok(())
}

fn terminal_cleanup() -> ! {
    println!("{}", "  >> Scrubbing Neural DRAM...".dimmed());
    println!("{}", "  >> Liquidating Tactical Artifacts...".dimmed());

    // DRAM Wipe simulation (Clearing sensitive env vars)
    std::env::remove_var("NVIDIA_API_KEY");
    std::env::remove_var("ANTHROPIC_API_KEY");
    std::env::remove_var("OPENAI_API_KEY");

    println!(
        "{}",
        "  >> Mission Aborted. Footprint Neutralized.".red().bold()
    );
    ui::revert_terminal_font(); // Restore terminal typography
    std::process::exit(130);
}
