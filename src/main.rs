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

    // ─── Init Logging ───
    if !is_completion {
        let log_filter = agent_config.agent.log_level.to_lowercase();
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&log_filter)),
            )
            .with_target(false)
            .compact()
            .init();

        // Premium Initialization Sequence
        ui::print_banner(&agent_config.agent.name, &agent_config.agent.version);
        ui::boot_sequence(&agent_config).await;

        tracing::info!(
            name = %agent_config.agent.name,
            version = %agent_config.agent.version,
            "MISSION START: Neural Core Active"
        );
    }

    // ─── Handle Commands ───
    dispatch(cli, &agent_config).await?;

    Ok(())
}
