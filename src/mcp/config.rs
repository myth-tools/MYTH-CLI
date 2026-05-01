use crate::config;
use crate::ui::CyberTheme;
use color_eyre::eyre::Result;
use owo_colors::OwoColorize;
use std::collections::HashMap;

/// Management operations for MCP servers.
#[derive(Debug, Clone)]
pub enum McpManagementCmd {
    List,
    Toggle {
        name: String,
        state: String,
    },
    AddLocal {
        name: String,
        command: String,
        args: Vec<String>,
        env: HashMap<String, String>,
        dir: Option<String>,
        transport: Option<String>,
        description: Option<String>,
    },
    AddRemote {
        name: String,
        url: String,
        headers: HashMap<String, String>,
        transport: Option<String>,
        description: Option<String>,
    },
    AllowTool {
        server: String,
        tool: String,
    },
    BlockTool {
        server: String,
        tool: String,
    },
    /// List all available tools for an MCP server
    Tools {
        name: String,
    },
    /// Remove an MCP server from the registry
    Remove {
        name: String,
    },
    Sync,
}

/// Handle an MCP management command and return a formatted response string.
pub async fn handle_mcp_management_cmd(
    _app_config: &config::AppConfig,
    cmd: McpManagementCmd,
    manager: Option<&crate::mcp::client::McpClientManager>,
) -> Result<String> {
    // We use the dedicated MCP JSON storage for all management operations
    let mut storage = config::settings::McpStorage::load()?;
    let mut response = String::new();

    match cmd {
        McpManagementCmd::List => {
            response.push_str(&format!(
                "\n{}\n",
                CyberTheme::primary(" ⚡ MYTH STRATEGIC ASSET REGISTRY ")
                    .bold()
                    .reversed()
            ));
            response.push_str(&format!("{}\n", CyberTheme::dim("━".repeat(105))));

            let mut online_count = 0;
            let mut offline_count = 0;
            let mut local_count = 0;
            let mut remote_count = 0;

            // Header for Categories
            response.push_str(&format!(
                "  {:<18}  {:<30}  {:<11}  {:<10}  {:<26}  {:<25}\n",
                "IDENTIFIER", "LOCATION", "TRANSPORT", "STATE", "HEALTH / TELEMETRY", "DESCRIPTION"
            ));
            response.push_str(&format!("  {}\n", CyberTheme::dim("─".repeat(135))));

            let factory_defaults = crate::builtin_mcp::get_factory_defaults();
            let mut all_names: Vec<String> = storage.mcp_servers.keys().cloned().collect();
            all_names.sort();

            // Fetch all live diagnostics in PARALLEL
            let live_diagnostics = if let Some(mgr) = manager {
                mgr.get_all_statuses_parallel().await
            } else {
                HashMap::new()
            };

            for name in all_names {
                let srv = storage.mcp_servers.get(&name).unwrap();
                let is_core = factory_defaults.contains_key(&name);
                let (is_enabled, desc, transport, location) = match srv {
                    config::CustomMcpServer::Local(l) => {
                        local_count += 1;
                        (
                            l.enabled,
                            l.description.as_ref(),
                            &l.transport,
                            l.command.clone(),
                        )
                    }
                    config::CustomMcpServer::Remote(r) => {
                        remote_count += 1;
                        (
                            r.enabled,
                            r.description.as_ref(),
                            &r.transport,
                            r.url.clone(),
                        )
                    }
                };

                // Resolve "Real Live Location" (Absolute Path) for local assets
                let real_location = match srv {
                    config::CustomMcpServer::Local(_) => {
                        let output = tokio::process::Command::new("which")
                            .arg(&location)
                            .output()
                            .await
                            .ok();

                        if let Some(out) = output {
                            if out.status.success() {
                                String::from_utf8_lossy(&out.stdout).trim().to_string()
                            } else {
                                location.clone()
                            }
                        } else {
                            location.clone()
                        }
                    }
                    config::CustomMcpServer::Remote(_) => location.clone(),
                };

                let state_label = if is_enabled {
                    online_count += 1;
                    "ENABLED"
                } else {
                    offline_count += 1;
                    "DISABLED"
                };

                // Health logic with ANSI-aware padding resolution
                let (health_info, visible_health_len) = if let Some((alive, status, pid, healthy)) =
                    live_diagnostics.get(&name)
                {
                    let pid_str = if let Some(p) = pid {
                        format!("(PID: {})", p)
                    } else {
                        "".to_string()
                    };
                    let pulse = if *healthy || *alive { "●" } else { "○" };
                    let pulse_colored = if *healthy {
                        pulse.green().to_string()
                    } else if *alive {
                        pulse.yellow().to_string()
                    } else {
                        pulse.dimmed().to_string()
                    };
                    let status_colored: String = if *healthy {
                        status.green().to_string()
                    } else if *alive {
                        status.yellow().to_string()
                    } else {
                        status.dimmed().to_string()
                    };

                    // Calculation: icon(1) + space(1) + status(len) + (space(1) + pid_str(len))?
                    let visible_len = 1
                        + 1
                        + status.chars().count()
                        + (if pid_str.is_empty() {
                            0
                        } else {
                            1 + pid_str.chars().count()
                        });
                    (
                        format!("{} {} {}", pulse_colored, status_colored, pid_str.dimmed()),
                        visible_len,
                    )
                } else {
                    let (icon, label_text) = match srv {
                        config::CustomMcpServer::Local(l) => {
                            let exists = tokio::process::Command::new("which")
                                .arg(&l.command)
                                .stdout(std::process::Stdio::null())
                                .stderr(std::process::Stdio::null())
                                .status()
                                .await
                                .map(|s| s.success())
                                .unwrap_or(false);
                            if exists {
                                ("○".dimmed().to_string(), "READY")
                            } else {
                                ("×".red().to_string(), "MISSING")
                            }
                        }
                        config::CustomMcpServer::Remote(_) => ("○".dimmed().to_string(), "READY"),
                    };
                    let visible_len = 1 + 1 + label_text.chars().count();
                    (format!("{} {}", icon, label_text.dimmed()), visible_len)
                };

                let transport_str = match transport {
                    config::McpTransport::Stdio => "STDIO",
                    config::McpTransport::Sse => "SSE",
                    config::McpTransport::Http => "HTTP",
                };

                // Truncate location for table
                let display_location = if real_location.len() > 28 {
                    format!("{}...", &real_location[..25])
                } else {
                    real_location.clone()
                };

                let name_colored = if is_core {
                    name.bright_yellow().bold().to_string()
                } else {
                    name.bright_cyan().to_string()
                };
                let loc_colored = display_location.dimmed().to_string();
                let transport_colored = match transport {
                    config::McpTransport::Stdio => transport_str.magenta().to_string(),
                    config::McpTransport::Sse => transport_str.blue().to_string(),
                    config::McpTransport::Http => transport_str.yellow().to_string(),
                };
                let state_colored = if is_enabled {
                    state_label.green().bold().to_string()
                } else {
                    state_label.bright_red().dimmed().to_string()
                };

                let desc_str = desc.cloned().unwrap_or_else(|| {
                    if is_core {
                        "Core system asset".into()
                    } else {
                        "Tactical augmentation".into()
                    }
                });

                // ELITE ALIGNMENT: Manual padding based on visible length to handle ANSI codes correctly
                response.push_str("  ");
                response.push_str(&format!(
                    "{}{:<width$}  ",
                    name_colored,
                    "",
                    width = 18usize.saturating_sub(name.len())
                ));
                response.push_str(&format!(
                    "{}{:<width$}  ",
                    loc_colored,
                    "",
                    width = 30usize.saturating_sub(display_location.len())
                ));
                response.push_str(&format!(
                    "{}{:<width$}  ",
                    transport_colored,
                    "",
                    width = 11usize.saturating_sub(transport_str.len())
                ));
                response.push_str(&format!(
                    "{}{:<width$}  ",
                    state_colored,
                    "",
                    width = 10usize.saturating_sub(state_label.len())
                ));

                // For Health Info, manually pad based on visible characters
                response.push_str(&health_info);
                response.push_str(&" ".repeat(26usize.saturating_sub(visible_health_len)));
                response.push_str("  ");

                response.push_str(&CyberTheme::dim(&desc_str).to_string());
                response.push('\n');
            }

            response.push_str(&format!("\n{}\n", CyberTheme::dim("━".repeat(135))));
            let status_msg = if online_count > 0 {
                "OPERATIONAL".green().bold().to_string()
            } else {
                "DEGRADED".yellow().bold().to_string()
            };

            response.push_str(&format!(
                "  {}  DEPLOYMENTS: {} LOCAL | {} REMOTE LINK\n",
                "📡".dimmed(),
                local_count,
                remote_count
            ));
            response.push_str(&format!(
                "  {}  READINESS:   {} ACTIVE / {} STANDBY\n",
                "🛡️".dimmed(),
                online_count.green().bold(),
                offline_count.yellow()
            ));
            response.push_str(&format!(
                "  {}  STATUS:      {}\n",
                CyberTheme::primary("⚡").dimmed(),
                status_msg
            ));
            response.push_str(&format!(
                "  {}  MODULATE:    {} OR {}\n",
                "💡".dimmed(),
                "/mcp toggle <name>".cyan(),
                "/mcp tools <name>".cyan()
            ));
        }
        McpManagementCmd::Toggle { name, state } => {
            let enable = matches!(
                state.to_lowercase().as_str(),
                "on" | "enable" | "true" | "1"
            );
            if let Some(srv) = storage.mcp_servers.get_mut(&name) {
                match srv {
                    config::CustomMcpServer::Local(l) => l.enabled = enable,
                    config::CustomMcpServer::Remote(r) => r.enabled = enable,
                }
                storage.save()?;
                let status_str = if enable {
                    "ENABLED".green().to_string()
                } else {
                    "DISABLED".red().to_string()
                };
                response.push_str(&format!(
                    "{} Server '{}' status updated to: {}\n",
                    "✓".green(),
                    name.bold(),
                    status_str
                ));
            } else {
                return Err(color_eyre::eyre::eyre!(
                    "Tactical asset '{}' not found in registry",
                    name
                ));
            }
        }
        McpManagementCmd::AddLocal {
            name,
            command,
            args,
            env,
            dir,
            transport,
            description,
        } => {
            if storage.mcp_servers.contains_key(&name) {
                return Err(color_eyre::eyre::eyre!(
                    "Tactical asset '{}' already exists. Use '/mcp toggle' or edit registry.",
                    name
                ));
            }

            // Hardening: Verify command exists (Silently)
            let status = std::process::Command::new("which")
                .arg(&command)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status();
            if status.is_err() || !status.unwrap().success() {
                response.push_str(&format!("{} {} Warning: Command '{}' not found in system $PATH. Asset may fail to initialize.\n",
                    "⚠️".yellow(), "LOGISTICS:".bold(), command.cyan()));
            }

            let transport_type = match transport.as_deref() {
                Some("sse") => config::McpTransport::Sse,
                Some("http") => config::McpTransport::Http,
                _ => config::McpTransport::Stdio,
            };

            let new_srv = config::CustomMcpServer::Local(config::LocalMcpConfig {
                enabled: true,
                command,
                args,
                env,
                description,
                working_dir: dir,
                timeout: 60,
                allowed_tools: vec![],
                transport: transport_type,
                install_script: None,
            });
            storage.mcp_servers.insert(name.clone(), new_srv);
            storage.save()?;
            response.push_str(&format!(
                "{} Tactical asset '{}' successfully registered and deployed.\n",
                "✓".green(),
                name.bold()
            ));
        }
        McpManagementCmd::AddRemote {
            name,
            url,
            headers,
            transport,
            description,
        } => {
            if storage.mcp_servers.contains_key(&name) {
                return Err(color_eyre::eyre::eyre!(
                    "Tactical asset '{}' already exists.",
                    name
                ));
            }

            // Hardening: Basic URL validation
            if !url.starts_with("http") {
                return Err(color_eyre::eyre::eyre!(
                    "Invalid remote protocol. SSE must use HTTP/HTTPS."
                ));
            }

            let transport_type = match transport.as_deref() {
                Some("http") => config::McpTransport::Http,
                Some("stdio") => config::McpTransport::Stdio,
                _ => config::McpTransport::Sse,
            };

            let new_srv = config::CustomMcpServer::Remote(config::RemoteMcpConfig {
                enabled: true,
                url,
                description,
                headers,
                timeout: 30,
                allowed_tools: vec![],
                transport: transport_type,
            });
            storage.mcp_servers.insert(name.clone(), new_srv);
            storage.save()?;
            response.push_str(&format!(
                "{} Secure remote link to '{}' established and registered.\n",
                "✓".green(),
                name.bold()
            ));
        }
        McpManagementCmd::AllowTool {
            server: server_name,
            tool,
        } => {
            if let Some(srv) = storage.mcp_servers.get_mut(&server_name) {
                let allowed = match srv {
                    config::CustomMcpServer::Local(l) => &mut l.allowed_tools,
                    config::CustomMcpServer::Remote(r) => &mut r.allowed_tools,
                };
                if !allowed.contains(&tool) {
                    allowed.push(tool.clone());
                    storage.save()?;
                    response.push_str(&format!(
                        "{} Tool '{}' is now allowed for MCP server '{}'.\n",
                        CyberTheme::primary("✓"),
                        tool.bright_white(),
                        server_name.bright_yellow()
                    ));
                } else {
                    response.push_str(&format!(
                        "{} Tool '{}' is already allowed for MCP server '{}'.\n",
                        "⚠".yellow(),
                        tool.bright_white(),
                        server_name.bright_yellow()
                    ));
                }
            } else {
                return Err(color_eyre::eyre::eyre!(
                    "Tactical asset '{}' not found in registry",
                    server_name
                ));
            }
        }
        McpManagementCmd::BlockTool {
            server: server_name,
            tool,
        } => {
            if let Some(srv) = storage.mcp_servers.get_mut(&server_name) {
                let allowed = match srv {
                    config::CustomMcpServer::Local(l) => &mut l.allowed_tools,
                    config::CustomMcpServer::Remote(r) => &mut r.allowed_tools,
                };
                if allowed.contains(&tool) {
                    allowed.retain(|x| x != &tool);
                    storage.save()?;
                    response.push_str(&format!(
                        "{} Tool '{}' is now blocked for MCP server '{}'.\n",
                        CyberTheme::primary("✓"),
                        tool.bright_white(),
                        server_name.bright_yellow()
                    ));
                } else {
                    response.push_str(&format!(
                        "{} Tool '{}' was not found in allowed list for MCP server '{}'.\n",
                        "⚠".yellow(),
                        tool.bright_white(),
                        server_name.bright_yellow()
                    ));
                }
            } else {
                return Err(color_eyre::eyre::eyre!(
                    "Tactical asset '{}' not found in registry",
                    server_name
                ));
            }
        }
        McpManagementCmd::Tools { name } => {
            if let Some(manager) = manager {
                match manager.list_external_tools_raw(&name).await {
                    Ok(tools) => {
                        response.push_str(&format!(
                            "\n{}\n",
                            CyberTheme::primary(format!(
                                "🔍 MCP TOOL AUDIT: {}",
                                name.to_uppercase()
                            ))
                            .bold()
                        ));
                        response.push_str(&format!("{:-<65}\n", ""));
                        use owo_colors::OwoColorize;
                        response.push_str(&format!(
                            "  {} TOTAL ASSETS DISCOVERED: {}\n\n",
                            "┃".bright_black(),
                            tools.len().to_string().bright_white()
                        ));

                        let srv_opt = storage.mcp_servers.get(&name);
                        let allowed_tools = if let Some(srv) = srv_opt {
                            match srv {
                                config::CustomMcpServer::Local(l) => Some(&l.allowed_tools),
                                config::CustomMcpServer::Remote(r) => Some(&r.allowed_tools),
                            }
                        } else {
                            None
                        };

                        if allowed_tools.is_none() {
                            return Err(color_eyre::eyre::eyre!(
                                "MCP Server '{}' not found in active configuration.",
                                name
                            ));
                        }
                        let allowed_tools = allowed_tools.unwrap();

                        for tool in tools {
                            let tool_name = tool["name"].as_str().unwrap_or("unknown");
                            let desc = tool["description"]
                                .as_str()
                                .unwrap_or("No description provided.");

                            let (status_icon, status_label) = if allowed_tools.is_empty()
                                || allowed_tools.contains(&tool_name.to_string())
                            {
                                (
                                    "✓".green().to_string(),
                                    "ALLOWED".green().bold().to_string(),
                                )
                            } else {
                                ("🔒".red().to_string(), "BLOCKED".red().bold().to_string())
                            };

                            response.push_str(&format!(
                                "  {} {:.<25} [{}] — {}\n",
                                status_icon,
                                tool_name.bright_white().bold(),
                                status_label,
                                desc.italic().bright_black()
                            ));
                        }

                        response.push_str(&format!("{:-<65}\n", ""));
                        response.push_str(&format!(
                            "{} Modulation: {} to allow, {} to block.\n",
                            "🛠".dimmed(),
                            "/mcp allow-tool".cyan(),
                            "/mcp block-tool".cyan()
                        ));
                    }
                    Err(e) => response.push_str(&format!(
                        "\n{} Failed to fetch tools from {}: {}\n",
                        "✗".bright_red(),
                        name.bright_yellow(),
                        e
                    )),
                }
            } else {
                response.push_str(&format!(
                    "{} Error: Registry synchronization required for discovery.\n",
                    "✗".red()
                ));
            }
        }
        McpManagementCmd::Remove { name } => {
            if !storage.mcp_servers.contains_key(&name) {
                return Ok(format!(
                    "\n{} Asset '{}' not found in registry.\n",
                    "⚠".bright_yellow(),
                    name.bright_white()
                ));
            }

            let factory_defaults = crate::builtin_mcp::get_factory_defaults();
            if factory_defaults.contains_key(&name) {
                return Ok(format!("\n{} Access Denied: '{}' is a core system asset and cannot be decommissioned.\n", "✗".bright_red(), name.bright_white()));
            }

            storage.mcp_servers.remove(&name);
            storage.save()?;

            response.push_str(&format!("\n{} Tactical asset '{}' has been successfully decommissioned and removed from registry.\n", "✓".bright_green(), name.bright_white()));
        }
        McpManagementCmd::Sync => {
            let synced = storage.sync_factory_defaults();
            if synced {
                response.push_str(&format!(
                    "{} Factory defaults force-synced to mcp.json. New/updated assets deployed.\n",
                    "✓".green()
                ));
            } else {
                response.push_str(&format!(
                    "{} All factory defaults already in sync. No changes needed.\n",
                    "✓".green()
                ));
            }

            let total = storage.mcp_servers.len();
            let factory_count = crate::builtin_mcp::get_factory_defaults().len();
            let user_count = total.saturating_sub(factory_count);
            response.push_str(&format!(
                "  {} {} factory + {} user = {} total assets\n",
                "📡".dimmed(),
                factory_count,
                user_count,
                total
            ));
        }
    }

    Ok(response)
}
