use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use color_eyre::eyre::Result;
use owo_colors::OwoColorize;

use crate::config;
use crate::core;
use crate::core::commands::{handle_command, CommandAction};
use crate::interactive;
use crate::mcp;
use crate::stream;
use crate::ui;

/// MYTH — AI Reconnaissance Agent
#[derive(Parser, Debug)]
#[command(
    name = "myth",
    version = env!("CARGO_PKG_VERSION"),

    about = "⚡ MYTH — AI-Powered Reconnaissance Agent for Kali Linux",
    long_about = "An ultra-fast, sandboxed, volatile CLI agent that leverages \
                  3000+ Kali security tools via MCP, powered by NVIDIA NIM and Rig.rs.\n\n\
                  All operations run inside a Bubblewrap sandbox.\n\
                  All data is stored in RAM and destroyed on exit."
)]
pub struct Cli {
    /// Path to user config file (default: ~/.config/myth/user.yaml)
    #[arg(short, long)]
    pub config: Option<String>,

    /// Override log level (trace, debug, info, warn, error)
    #[arg(short, long)]
    pub log_level: Option<String>,

    /// Disable TUI (use simple interactive CLI)
    #[arg(long)]
    pub no_tui: bool,

    /// Disable sandbox (NOT RECOMMENDED)
    #[arg(long)]
    pub no_sandbox: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize full-spectrum target acquisition and reconnaissance
    #[command(alias = "recon")]
    Scan {
        /// Target domain, IP, or URL for mission focus
        target: String,

        /// Reconnaissance methodology profile
        #[arg(short, long, default_value = "full")]
        profile: String,
    },

    /// Launch immediate low-signature, passive-only reconnaissance
    Stealth {
        /// Target domain, IP, or URL
        target: String,
    },

    /// Launch specialized Open Source Intelligence (OSINT) ops
    Osint {
        /// Target domain, IP, or URL
        target: String,
    },

    /// Perform a deep, multi-vector vulnerability assessment
    Vuln {
        /// Target domain, IP, or URL
        target: String,
    },

    /// Catalog and display all synchronized mission assets/tools
    Tools {
        /// Filter mission assets by technical category
        #[arg(short, long)]
        category: Option<String>,

        /// Search mission assets by name or keyword
        #[arg(short, long)]
        search: Option<String>,
    },

    /// Force rotate the mission focus to a new target/CIDR
    Target {
        /// Target domain, IP, or URL
        target: String,

        /// Reconnaissance methodology profile
        #[arg(short, long, default_value = "quick")]
        profile: String,
    },

    /// Launch an interactive tactical chat session with the agent
    Chat,

    /// Retrieve the current mission configuration metadata
    Config,

    /// View or modulate tactical reconnaissance profiles/phases
    Profile {
        /// Name of the profile
        name: Option<String>,
        /// Action to perform (enable/disable)
        action: Option<String>,
        /// Phase indices (comma-separated, e.g., 0,1,2)
        index: Option<String>,
    },

    /// Verify system health, sandbox status, and tool availability
    #[command(alias = "health")]
    Check,

    /// Manage Custom User MCP Servers (Local & Remote)
    #[command(subcommand)]
    Mcp(McpCommands),

    /// Analyze neural pulses and session lifecycle metadata
    #[command(alias = "status")]
    Vitals,

    /// Aggregate and display all discovered tactical intelligence
    Findings,

    /// Render the infrastructure relationship graph of target assets
    Graph,

    /// Aggregate tactical event logs and mission history
    History,

    /// Generate a comprehensive executive intelligence summary
    Report,

    /// Force a re-synchronization with local tool registries
    Sync,

    /// EMERGENCY: Immediate shred of all data and system shutdown
    Burn,

    /// Wipe the current session memory and tactical context
    Wipe,

    /// Purge visual buffers (Memory remains)
    Clear,

    /// Modulate the maximum neural iteration depth for the agent
    Depth {
        /// Iteration count
        depth: u32,
    },

    /// Retrieve deep technical documentation for a specific asset
    Inspect {
        /// Tool or topic name
        name: String,
    },

    /// Generate high-performance shell autocompletion tactical scripts
    Completions {
        /// Target shell environment (bash, zsh, fish, powershell, elvish)
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Display the tactical usage documentation
    Usage,

    /// Display the current neural core version version
    Version,

    /// High-speed multi-source subdomain discovery
    Subdomains {
        /// Target domain
        domain: String,
        /// Enable active brute-force and permutations
        #[arg(long)]
        active: bool,
        /// Enable recursive discovery
        #[arg(long)]
        recursive: bool,
        /// Filter results to only show live subdomains (Default: true)
        #[arg(long, default_value_t = true)]
        only_alive: bool,
        /// ULTRA-ROBUST MODE: Tor + Proxies + Mega Wordlist + Deep Recursion
        #[arg(long, alias = "ultra")]
        master: bool,
    },

    /// ULTRA-ROBUST DISCOVERY: Tor + Proxies + Mega Wordlist + Deep Recursion
    #[command(alias = "ultra")]
    Master {
        /// Target domain
        domain: String,
    },

    /// Manage tactical typography and terminal font synchronization
    #[command(alias = "fonts", subcommand)]
    Typography(TypographyCommands),

    /// \[HIDDEN\] Internal tactical bridge for dynamic shell completions
    #[command(hide = true)]
    CompleteBridge {
        /// The full operational command line being completed
        line: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum TypographyCommands {
    /// List all 10 available tactical font assets
    List,
    /// Set the terminal font to a specific asset ID
    Set {
        /// Font ID from the mission registry (e.g. jet-brains-mono)
        id: String,
    },
    /// Revert the terminal to the system default font
    Revert,
}

#[derive(Subcommand, Debug)]
pub enum McpCommands {
    /// List all configured MCP servers
    List,
    /// Toggle an MCP server (enable/disable)
    Toggle {
        /// Name of the server
        name: String,
        /// State to set (on/off, enable/disable, true/false)
        state: String,
    },
    /// Add a new Local MCP Server
    AddLocal {
        /// Name of the server
        name: String,
        /// Command to run (e.g., npx, python)
        command: String,
        /// Arguments for the command (comma-separated or multiple flags)
        #[arg(short, long, value_delimiter = ',')]
        args: Vec<String>,
        /// Environment variables (KEY=VALUE, comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        env: Vec<String>,
        /// Working directory
        #[arg(short, long)]
        dir: Option<String>,
        /// Operational description
        #[arg(long)]
        description: Option<String>,
        /// Transport protocol (stdio, sse, http). Default: stdio
        #[arg(short, long)]
        transport: Option<String>,
    },
    /// Add a new Remote MCP Server via SSE
    AddRemote {
        /// Name of the server
        name: String,
        /// SSE URL
        url: String,
        /// Custom headers (Header:Value, comma-separated)
        #[arg(long, value_delimiter = ',')]
        headers: Vec<String>,
        /// Operational description
        #[arg(long)]
        description: Option<String>,
        /// Transport protocol (sse, http, stdio). Default: sse
        #[arg(short, long)]
        transport: Option<String>,
    },
    /// List all available tools for an MCP server
    Tools {
        /// Name of the server
        name: String,
    },
    /// Force re-sync factory defaults to mcp.json
    Sync,
    /// Remove an MCP server from the registry
    #[command(alias = "rm", alias = "delete")]
    Remove {
        /// Name of the server
        name: String,
    },
    /// Allow a specific tool from an MCP server
    AllowTool {
        /// Name of the server
        server: String,
        /// Name of the tool
        tool: String,
    },
    /// Block a specific tool from an MCP server
    BlockTool {
        /// Name of the server
        server: String,
        /// Name of the tool
        tool: String,
    },
}

pub async fn dispatch(cli: Cli, agent_config: &config::AppConfig) -> Result<()> {
    match cli.command {
        Some(Commands::Scan { target, profile }) => {
            run_scan(agent_config, &target, &profile, cli.no_tui).await?;
        }
        Some(Commands::Tools { category, search }) => {
            run_tools_list(agent_config, category, search).await?;
        }
        Some(Commands::Target { target, profile }) => {
            run_scan(agent_config, &target, &profile, cli.no_tui).await?;
        }
        Some(Commands::Config) => {
            let mut display_config = agent_config.clone();
            display_config.llm.mask_api_keys();
            println!("{}", serde_yaml::to_string(&display_config)?);
        }
        Some(Commands::Profile {
            name,
            action,
            index,
        }) => {
            run_profile_cmd(agent_config, name, action, index).await?;
        }
        Some(Commands::Mcp(mcp_cmd)) => {
            run_mcp_cmd(agent_config, mcp_cmd).await?;
        }
        Some(Commands::Typography(typo_cmd)) => {
            run_typography_cmd(agent_config, typo_cmd).await?;
        }
        Some(Commands::Check) => {
            run_check(agent_config).await?;
        }
        Some(Commands::Stealth { target }) => {
            run_scan_with_prompt(
                agent_config,
                &target,
                "stealth",
                "Begin immediate low-signature, passive-only reconnaissance.",
                cli.no_tui,
            )
            .await?;
        }
        Some(Commands::Osint { target }) => {
            run_scan_with_prompt(
                agent_config,
                &target,
                "full",
                "Launch specialized Open Source Intelligence (OSINT) ops.",
                cli.no_tui,
            )
            .await?;
        }
        Some(Commands::Vuln { target }) => {
            run_scan_with_prompt(
                agent_config,
                &target,
                "full",
                "Perform a deep, multi-vector vulnerability assessment.",
                cli.no_tui,
            )
            .await?;
        }
        Some(Commands::Vitals) => {
            run_session_cmd(agent_config, "/vitals").await?;
        }
        Some(Commands::Findings) => {
            run_session_cmd(agent_config, "/findings").await?;
        }
        Some(Commands::Graph) => {
            run_session_cmd(agent_config, "/graph").await?;
        }
        Some(Commands::History) => {
            run_session_cmd(agent_config, "/history").await?;
        }
        Some(Commands::Report) => {
            run_session_cmd(agent_config, "/report").await?;
        }
        Some(Commands::Sync) => {
            run_session_cmd(agent_config, "/sync").await?;
        }
        Some(Commands::Burn) => {
            run_session_cmd(agent_config, "/burn").await?;
        }
        Some(Commands::Wipe) => {
            run_session_cmd(agent_config, "/wipe").await?;
        }
        Some(Commands::Clear) => {
            run_session_cmd(agent_config, "/clear").await?;
        }
        Some(Commands::Depth { depth }) => {
            run_session_cmd(agent_config, &format!("/depth {}", depth)).await?;
        }
        Some(Commands::Inspect { name }) => {
            run_session_cmd(agent_config, &format!("/inspect {}", name)).await?;
        }
        Some(Commands::Completions { shell }) => {
            run_completions_cmd(shell).await?;
        }
        Some(Commands::Usage) => {
            run_session_cmd(agent_config, "/usage").await?;
        }
        Some(Commands::Version) => {
            run_session_cmd(agent_config, "/version").await?;
        }
        Some(Commands::Subdomains {
            domain,
            active,
            recursive,
            only_alive,
            master,
        }) => {
            let mut prompt = format!("/subdomains {}", domain);
            if master {
                prompt.push_str(" --master");
            } else {
                if active {
                    prompt.push_str(" --active");
                }
                if recursive {
                    prompt.push_str(" --recursive");
                }
                if !only_alive {
                    prompt.push_str(" --only-alive false");
                }
            }
            run_session_cmd(agent_config, &prompt).await?;
        }
        Some(Commands::Master { domain }) => {
            run_session_cmd(agent_config, &format!("/master {}", domain)).await?;
        }
        Some(Commands::Chat) | None => {
            if agent_config.tui.enabled {
                interactive::run_tui(agent_config, "(no target)").await?;
            } else if cli.no_tui {
                interactive::run_interactive(agent_config).await?;
            } else {
                ui::print_tui_disabled_message();
            }
        }
        Some(Commands::CompleteBridge { line }) => {
            run_internal_complete(agent_config, &line).await?;
        }
    }
    Ok(())
}

/// Run a targeted scan session with a starting prompt.
pub async fn run_scan_with_prompt(
    config: &config::AppConfig,
    target: &str,
    profile: &str,
    prompt: &str,
    no_tui: bool,
) -> Result<()> {
    if !config.tui.enabled && !no_tui {
        ui::print_tui_disabled_message();
        return Ok(());
    }

    ui::print_status("PRE-FLIGHT", "Initializing precision reconnaissance");
    ui::print_boxed("MISSION TARGET", target);

    let mut agent = core::agent::ReconAgent::new(config.clone())
        .await
        .map_err(|e| color_eyre::eyre::eyre!("Agent init failed: {}", e))?;

    agent.start_session(target, profile).await;

    if config.tui.enabled {
        interactive::run_tui(config, target).await?;
    } else {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        agent.set_event_tx(Some(tx));
        let (s, _) = agent.chat_stream(prompt).await?;
        let executed_tools = std::sync::Arc::new(tokio::sync::Mutex::new(Vec::new()));
        let start_time = std::time::Instant::now();
        stream::consume_agent_stream(
            Box::pin(s),
            executed_tools,
            &mut agent,
            rx,
            start_time,
            None,
        )
        .await?;
    }

    agent.cleanup().await;
    Ok(())
}

/// Run a one-off command session for tactical metadata.
pub async fn run_session_cmd(config: &config::AppConfig, cmd: &str) -> Result<()> {
    let mut agent = core::agent::ReconAgent::new(config.clone())
        .await
        .map_err(|e| color_eyre::eyre::eyre!("Agent init failed: {}", e))?;

    // Attempt to restore mission context for standalone CLI state commands
    if let Some(meta) = core::session::MissionMetadata::load() {
        agent.restore_session(meta).await;
    }

    let mission_events = std::collections::VecDeque::new();
    match handle_command(cmd, &mut agent, &mission_events).await {
        CommandAction::Response(resp) => println!("{}", resp),
        CommandAction::SetDepth(d) => println!("Iteration depth modulated to: {}", d),
        CommandAction::WipeSession => {
            use std::io::Write;
            agent.reset_session().await;
            print!("\x1B[2J\x1B[1;1H\x1B[3J");
            let _ = std::io::stdout().flush();
            crate::ui::print_banner();
            println!(
                "{} Session wiped. Tactical memory purged.\n",
                ui::CyberTheme::primary("✓").bold()
            );
        }
        CommandAction::Clear => {
            use std::io::Write;
            print!("\x1B[2J\x1B[1;1H\x1B[3J");
            let _ = std::io::stdout().flush();
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
                }
                Err(e) => println!("\n{} Direct execution failed: {}\n", "✗".bright_red(), e),
            }
        }
        _ => println!("Command executed (no direct CLI output)."),
    }

    agent.cleanup().await;
    Ok(())
}

/// Run a targeted scan session.
pub async fn run_scan(
    config: &config::AppConfig,
    target: &str,
    profile: &str,
    no_tui: bool,
) -> Result<()> {
    if !config.tui.enabled && !no_tui {
        ui::print_tui_disabled_message();
        return Ok(());
    }

    ui::print_status("PRE-FLIGHT", "Synchronizing mission parameters");
    ui::print_boxed("TACTICAL OBJECTIVE", target);

    ui::print_status(
        "SANDBOX",
        if config.sandbox.enabled {
            "ACTIVE (BUBBLEWRAP)"
        } else {
            "DISABLED ⚠ (HIGH RISK)"
        },
    );

    // Initialize agent
    let mut agent = core::agent::ReconAgent::new(config.clone())
        .await
        .map_err(|e| color_eyre::eyre::eyre!("Agent init failed: {}", e))?;

    // Start session
    agent.start_session(target, profile).await;

    ui::print_status(
        "CORE",
        "Neural links synchronized. Reconnaissance in progress...",
    );

    // If TUI is enabled, launch TUI mode
    if config.tui.enabled {
        interactive::run_tui(config, target).await?;
    } else {
        println!(
            "{}",
            ui::CyberTheme::primary(
                "✅ Agent initialized. Precision reconnaissance stream active."
            )
            .bold()
        );
        println!("  Type commands or press Ctrl+C to disconnect.");
        println!();

        let mission_targets = std::sync::Arc::new(std::sync::Mutex::new(vec![target.to_string()]));
        let discovered_tools = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

        // Populate tools for interactive mode
        {
            let mcp = agent.mcp_server();
            let disc_lock = mcp.discovery();
            let disc = disc_lock.read().await;
            let mut tools = discovered_tools.lock().unwrap();
            for tool in disc.list_all() {
                tools.push(tool.name.clone());
            }
        }

        interactive::run_interactive_with_context(config, agent, mission_targets, discovered_tools)
            .await?;
    }

    Ok(())
}

/// Manage Custom User MCP Servers
pub async fn run_mcp_cmd(app_config: &config::AppConfig, mcp_cmd: McpCommands) -> Result<()> {
    let internal_cmd = match mcp_cmd {
        McpCommands::List => mcp::config::McpManagementCmd::List,
        McpCommands::Toggle { name, state } => {
            mcp::config::McpManagementCmd::Toggle { name, state }
        }
        McpCommands::AddLocal {
            name,
            command,
            args,
            env,
            dir,
            transport,
            description,
        } => {
            let mut env_map = std::collections::HashMap::new();
            for e in env {
                if let Some((k, v)) = e.split_once('=') {
                    env_map.insert(k.to_string(), v.to_string());
                }
            }
            mcp::config::McpManagementCmd::AddLocal {
                name,
                command,
                args,
                env: env_map,
                dir,
                transport,
                description,
            }
        }
        McpCommands::AddRemote {
            name,
            url,
            headers,
            transport,
            description,
        } => {
            let mut header_map = std::collections::HashMap::new();
            for h in headers {
                if let Some((k, v)) = h.split_once('=') {
                    header_map.insert(k.to_string(), v.to_string());
                }
            }
            mcp::config::McpManagementCmd::AddRemote {
                name,
                url,
                headers: header_map,
                transport,
                description,
            }
        }
        McpCommands::Tools { name } => mcp::config::McpManagementCmd::Tools { name },
        McpCommands::Sync => mcp::config::McpManagementCmd::Sync,
        McpCommands::Remove { name } => mcp::config::McpManagementCmd::Remove { name },
        McpCommands::AllowTool { server, tool } => {
            mcp::config::McpManagementCmd::AllowTool { server, tool }
        }
        McpCommands::BlockTool { server, tool } => {
            mcp::config::McpManagementCmd::BlockTool { server, tool }
        }
    };

    let manager = if matches!(
        internal_cmd,
        mcp::config::McpManagementCmd::List | mcp::config::McpManagementCmd::Tools { .. }
    ) {
        let mut cm = mcp::client::McpClientManager::new();
        let _ = cm.sync_with_config(&app_config.mcp.mcp_servers).await;
        Some(cm)
    } else {
        None
    };

    match mcp::config::handle_mcp_management_cmd(app_config, internal_cmd, manager.as_ref()).await {
        Ok(resp) => {
            println!("{}", resp);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

/// List available security tools.
pub async fn run_tools_list(
    config: &config::AppConfig,
    category: Option<String>,
    search: Option<String>,
) -> Result<()> {
    ui::print_status("REGISTRY", "Synchronizing mission assets");

    // Initialize lite agent for discovery
    let agent = core::agent::ReconAgent::new(config.clone())
        .await
        .map_err(|e| color_eyre::eyre::eyre!("Agent init failed: {}", e))?;

    let input = mcp::schemas::DiscoverToolsInput {
        category: category.clone(),
        query: search.clone(),
    };

    let result = agent.mcp_server().handle_discover(input).await;
    let tools: Vec<mcp::schemas::ToolEntry> =
        serde_json::from_value(result["tools"].clone()).unwrap_or_default();

    println!("Found {} tactical assets:\n", tools.len());
    println!("{:<28} {:<15} SOURCE", "IDENTIFIER", "CATEGORY");
    println!("{}", ui::CyberTheme::dim("━".repeat(75)));

    for tool in tools {
        let name_styled = if tool.category == "ExternalMCP" {
            tool.name.bright_cyan().to_string()
        } else if tool.category.starts_with("Native/") {
            tool.name.bright_yellow().bold().to_string()
        } else {
            tool.name.white().to_string()
        };

        println!(
            "{:<45} {:<15} {}",
            name_styled,
            tool.category.dimmed(),
            tool.path.bright_black().italic()
        );
    }

    Ok(())
}

/// View and modulate reconnaissance profiles.
pub async fn run_profile_cmd(
    config: &config::AppConfig,
    name: Option<String>,
    action: Option<String>,
    index: Option<String>,
) -> Result<()> {
    let mut agent = core::agent::ReconAgent::new(config.clone())
        .await
        .map_err(|e| color_eyre::eyre::eyre!("Agent init failed: {}", e))?;

    let cmd = match (name, action, index) {
        (Some(n), Some(a), Some(i)) => format!("/profile {} {} {}", n, a, i),
        (Some(n), _, _) => format!("/profile {}", n),
        _ => "/profile".to_string(),
    };

    let mission_events = std::collections::VecDeque::new();
    match handle_command(&cmd, &mut agent, &mission_events).await {
        CommandAction::Response(resp) => {
            println!("{}", resp);
        }
        _ => {
            println!(
                "\n{} Unexpected response from profile command.\n",
                "⚠".bright_yellow()
            );
        }
    }

    Ok(())
}

/// Verify system health, sandbox status, and tool availability with an advanced diagnostic engine.
pub async fn run_check(config: &config::AppConfig) -> Result<()> {
    use crate::core::health::{render_results, HealthEngine};

    // Initialize and run the advanced health engine
    let engine = HealthEngine::new(None);
    let results = engine.run_all(config).await;

    // Render results with premium professional UI
    render_results(&results);

    Ok(())
}

/// Generate high-performance shell autocompletion tactical scripts with dynamic bridge.
pub async fn run_completions_cmd(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();

    match shell {
        Shell::Zsh => {
            // Write standard clap completions to a buffer
            let mut buf = Vec::new();
            generate(Shell::Zsh, &mut cmd, &name, &mut buf);
            let base = String::from_utf8_lossy(&buf);

            // Inject dynamic bridge
            println!("{}", base);
            println!("\n# MYTH Dynamic Intelligence Bridge");
            println!("_myth_dynamic_completions() {{");
            println!("  local -a completions");
            println!("  completions=($({} complete-bridge \"$words\"))", name);
            println!("  compadd -a completions");
            println!("}}");
            println!("compdef _myth_dynamic_completions {}", name);
        }
        Shell::Bash => {
            let mut buf = Vec::new();
            generate(Shell::Bash, &mut cmd, &name, &mut buf);
            let base = String::from_utf8_lossy(&buf);
            println!("{}", base);
            println!("\n# MYTH Dynamic Intelligence Bridge");
            println!("_myth_dynamic_completions() {{");
            println!("  local cur=\"${{COMP_WORDS[COMP_CWORD]}}\"");
            println!("  local line=\"${{COMP_LINE}}\"");
            println!("  local suggestions=$({} complete-bridge \"$line\")", name);
            println!("  COMPREPLY=( $(compgen -W \"$suggestions\" -- \"$cur\") )");
            println!("}}");
            println!("complete -F _myth_dynamic_completions {}", name);
        }
        _ => {
            // Fallback to static for other shells for now
            generate(shell, &mut cmd, name, &mut std::io::stdout());
        }
    }
    Ok(())
}

/// Run internal dynamic completion bridge.
pub async fn run_internal_complete(config: &config::AppConfig, line: &str) -> Result<()> {
    let mission_targets = if let Some(meta) = core::session::MissionMetadata::load() {
        vec![meta.target]
    } else {
        vec![]
    };

    let empty_history = vec![];
    let empty_tools = vec![];
    let ctx = core::commands::CommandContext {
        config,
        mission_targets: &mission_targets,
        history: &empty_history,
        discovered_tools: &empty_tools,
    };

    let suggestions = core::commands::get_argument_suggestions(line, &ctx);
    for s in suggestions {
        println!("{}", s);
    }
    Ok(())
}

pub async fn run_typography_cmd(config: &config::AppConfig, cmd: TypographyCommands) -> Result<()> {
    use crate::core::commands::CommandAction;
    use std::io::Write;

    match cmd {
        TypographyCommands::List => {
            ui::render_font_list();
        }
        TypographyCommands::Set { id } => {
            let mut agent = crate::core::agent::ReconAgent::new(config.clone()).await?;
            let mission_events = std::collections::VecDeque::new();
            let prompt = format!("/typography set {}", id);
            let action =
                crate::core::commands::handle_command(&prompt, &mut agent, &mission_events).await;

            match action {
                CommandAction::Response(resp) => {
                    if !resp.is_empty() {
                        println!("{}", resp);
                    }
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
                            "\n  {} Provisioning aborted. Fidelity remain degraded.\n",
                            ui::CyberTheme::accent("✗")
                        );
                    }
                }
                _ => {}
            }
        }
        TypographyCommands::Revert => {
            ui::revert_terminal_font();
            println!(
                "  {} Terminal typography reverted to OS default.",
                ui::CyberTheme::primary("✓")
            );
        }
    }
    Ok(())
}
