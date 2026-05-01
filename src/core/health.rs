use crate::config::AppConfig;
use async_trait::async_trait;
use owo_colors::OwoColorize;
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Pass,
    Warn,
    Fail,
}

#[derive(Debug, Clone)]
pub struct HealthResult {
    pub name: String,
    pub status: HealthStatus,
    pub message: String,
    pub details: Vec<String>,
    pub latency_ms: Option<u128>,
}

#[async_trait]
pub trait HealthCheck: Send + Sync {
    async fn run(&self, config: &AppConfig) -> HealthResult;
}

// ─── Environment Checker ───
pub struct EnvironmentChecker;

#[async_trait]
impl HealthCheck for EnvironmentChecker {
    async fn run(&self, _config: &AppConfig) -> HealthResult {
        let mut details = Vec::new();
        let mut status = HealthStatus::Pass;
        let mut message = "Environment is stable".to_string();

        // 1. Check OS / Kernel
        if let Ok(kernel) = std::fs::read_to_string("/proc/version") {
            let kernel_ver = kernel.split_whitespace().nth(2).unwrap_or("Unknown");
            details.push(format!("Kernel: {}", kernel_ver));
        }

        // 2. Check Memory
        if let Ok(meminfo) = std::fs::read_to_string("/proc/meminfo") {
            let total_mem_kb = meminfo
                .lines()
                .next()
                .and_then(|l| l.split_whitespace().nth(1))
                .and_then(|v| v.parse::<u64>().ok());

            if let Some(kb) = total_mem_kb {
                let mb = kb / 1024;
                details.push(format!("Total Memory: {} MB", mb));
                if mb < 4096 {
                    status = HealthStatus::Warn;
                    message = "Sub-optimal memory detected (< 4GB)".to_string();
                }
            }
        }

        // 3. Check temp space
        let tmp_dir = std::env::temp_dir();
        let tmp_path = tmp_dir.to_string_lossy();
        use std::process::Command;
        let df = Command::new("df")
            .arg("-B1") // Bytes
            .arg(tmp_dir.as_os_str())
            .output();

        if let Ok(output) = df {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = stdout.lines().nth(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let avail_bytes = parts[3].parse::<u64>().unwrap_or(u64::MAX);
                    let avail_mb = avail_bytes / (1024 * 1024);
                    details.push(format!("Storage ({}): {} MB available", tmp_path, avail_mb));

                    if avail_mb < 512 {
                        status = HealthStatus::Warn;
                        message = format!("Low storage space on {} (< 512MB)", tmp_path);
                    }
                    if avail_mb < 50 {
                        status = HealthStatus::Fail;
                        message = format!("Critical storage exhaustion on {}", tmp_path);
                    }
                }
            }
        }

        HealthResult {
            name: "Environment".to_string(),
            status,
            message,
            details,
            latency_ms: None,
        }
    }
}

// ─── Sandbox Checker ───
pub struct SandboxChecker;

#[async_trait]
impl HealthCheck for SandboxChecker {
    async fn run(&self, config: &AppConfig) -> HealthResult {
        let start = Instant::now();
        let bwrap_path = &config.sandbox.bwrap_path;

        // Check if bwrap exists
        let status = std::process::Command::new(bwrap_path)
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();

        if let Ok(s) = status {
            if s.success() {
                // Perform a robust isolation test: Try to read a host file that should NOT be accessible
                // We use a restricted set of args similar to BubblewrapSandbox::build_command
                let ctx = crate::config::SystemContext::sense();
                let sentinel = if ctx.is_termux {
                    ctx.join_config("tor/torrc").to_string_lossy().to_string()
                } else {
                    // Dynamic host sentinel discovery: prioritize common sensitive anchors
                    let mut found = "/etc/shadow".to_string();
                    for candidate in &["/etc/shadow", "/etc/gshadow", "/etc/sudoers"] {
                        if std::path::Path::new(candidate).exists() {
                            found = candidate.to_string();
                            break;
                        }
                    }
                    found
                };

                let mut test_args = vec![
                    "--unshare-all".to_string(),
                    "--proc".to_string(),
                    "/proc".to_string(),
                    "--dev".to_string(),
                    "/dev".to_string(),
                ];

                if ctx.is_termux {
                    if let Some(prefix) = &ctx.prefix {
                        test_args.push("--ro-bind".to_string());
                        test_args.push(prefix.to_string());
                        test_args.push(prefix.to_string());
                    }
                } else {
                    // Standard Linux neural conduits
                    for path in &["/usr", "/lib", "/lib64", "/bin", "/sbin"] {
                        if std::path::Path::new(path).exists() {
                            test_args.push("--ro-bind".into());
                            test_args.push((*path).into());
                            test_args.push((*path).into());
                        }
                    }
                }

                test_args.push("--".into());
                test_args.push("cat".into());
                test_args.push(sentinel.clone());

                let test_exec = std::process::Command::new(bwrap_path)
                    .args(&test_args)
                    .output();

                match test_exec {
                    Ok(output) => {
                        // We expect failure (exit code 1 or similar) because /etc/shadow is NOT bound
                        if !output.status.success() {
                            HealthResult {
                                name: "Sandbox".to_string(),
                                status: HealthStatus::Pass,
                                message: "Bubblewrap verified and isolation confirmed".to_string(),
                                details: vec![
                                    format!("Path: {}", bwrap_path),
                                    "Isolation Test: PASSED (Isolated)".to_string(),
                                ],
                                latency_ms: Some(start.elapsed().as_millis()),
                            }
                        } else {
                            HealthResult {
                                name: "Sandbox".to_string(),
                                status: HealthStatus::Fail,
                                message: "Sandbox isolation breach detected".to_string(),
                                details: vec![
                                    format!("Path: {}", bwrap_path),
                                    format!(
                                        "Isolation Test: FAILED (Isolation breach detected: {}!)",
                                        sentinel
                                    )
                                    .to_string(),
                                ],
                                latency_ms: Some(start.elapsed().as_millis()),
                            }
                        }
                    }
                    Err(_) => HealthResult {
                        name: "Sandbox".to_string(),
                        status: HealthStatus::Warn,
                        message: "Bubblewrap present but test execution failed".to_string(),
                        details: vec![
                            format!("Path: {}", bwrap_path),
                            "Isolation Test: UNKNOWN (Exec failed)".to_string(),
                        ],
                        latency_ms: Some(start.elapsed().as_millis()),
                    },
                }
            } else {
                HealthResult {
                    name: "Sandbox".to_string(),
                    status: HealthStatus::Fail,
                    message: "Bubblewrap returned error code".to_string(),
                    details: vec![format!("Path: {}", bwrap_path)],
                    latency_ms: None,
                }
            }
        } else {
            HealthResult {
                name: "Sandbox".to_string(),
                status: HealthStatus::Fail,
                message: "Bubblewrap binary not found".to_string(),
                details: vec![format!("Expected at: {}", bwrap_path), {
                    let is_termux = std::env::var("PREFIX")
                        .map(|p| p.contains("com.termux"))
                        .unwrap_or(false);
                    if is_termux {
                        "Install with: pkg install bubblewrap".to_string()
                    } else if std::process::Command::new("which")
                        .arg("dnf")
                        .output()
                        .map(|o| o.status.success())
                        .unwrap_or(false)
                    {
                        "Install with: sudo dnf install bubblewrap".to_string()
                    } else if std::process::Command::new("which")
                        .arg("pacman")
                        .output()
                        .map(|o| o.status.success())
                        .unwrap_or(false)
                    {
                        "Install with: sudo pacman -S bubblewrap".to_string()
                    } else {
                        "Install with: sudo apt install bubblewrap".to_string()
                    }
                }],
                latency_ms: None,
            }
        }
    }
}

// ─── AI Connectivity Checker ───
pub struct AiChecker;

#[async_trait]
impl HealthCheck for AiChecker {
    async fn run(&self, config: &AppConfig) -> HealthResult {
        let start = Instant::now();
        let api_key = match config.llm.resolve_api_keys() {
            Ok(keys) if !keys.is_empty() => Some(keys[0].clone()),
            _ => None,
        };

        if let Some(_key) = api_key {
            // Verify endpoint connectivity (NVIDIA NIM)
            let connect_fut = tokio::net::TcpStream::connect("integrate.api.nvidia.com:443");
            match tokio::time::timeout(std::time::Duration::from_secs(5), connect_fut).await {
                Ok(Ok(_)) => {
                    let latency = start.elapsed().as_millis();
                    HealthResult {
                        name: "AI Engine".to_string(),
                        status: HealthStatus::Pass,
                        message: "NVIDIA NIM connectivity verified".to_string(),
                        details: vec![
                            "API Key: Configured".to_string(),
                            "Network: Connected".to_string(),
                        ],
                        latency_ms: Some(latency),
                    }
                }
                Ok(Err(e)) => HealthResult {
                    name: "AI Engine".to_string(),
                    status: HealthStatus::Warn,
                    message: "NIM endpoint unreachable".to_string(),
                    details: vec![
                        "API Key: Configured".to_string(),
                        format!("Network Error: {}", e),
                    ],
                    latency_ms: None,
                },
                Err(_) => HealthResult {
                    name: "AI Engine".to_string(),
                    status: HealthStatus::Warn,
                    message: "NIM endpoint connection timeout".to_string(),
                    details: vec![
                        "API Key: Configured".to_string(),
                        "Timeout: 5s exceeded".to_string(),
                    ],
                    latency_ms: None,
                },
            }
        } else {
            HealthResult {
                name: "AI Engine".to_string(),
                status: HealthStatus::Fail,
                message: "NVIDIA_API_KEY missing".to_string(),
                details: vec![
                    "Critical: Agent brain will not function without a valid API key.".to_string(),
                ],
                latency_ms: None,
            }
        }
    }
}

// ─── Tool Inventory Checker ───
pub struct ToolInventoryChecker;

#[async_trait]
impl HealthCheck for ToolInventoryChecker {
    async fn run(&self, config: &AppConfig) -> HealthResult {
        let mut discovery = crate::mcp::discover::ToolDiscovery::new(config.mcp.tool_paths.clone());
        let tools = discovery.scan().await;

        let core_tools = [
            "nmap",
            "subfinder",
            "httpx",
            "nuclei",
            "amass",
            "whois",
            "curl",
            "dig",
            "ffuf",
            "sqlmap",
        ];
        let mut missing = Vec::new();
        let mut found_count = 0;

        for &t in &core_tools {
            if tools.iter().any(|found| found.name == t) {
                found_count += 1;
            } else {
                missing.push(t.to_string());
            }
        }

        let status = if missing.is_empty() {
            HealthStatus::Pass
        } else if found_count > 0 {
            HealthStatus::Warn
        } else {
            HealthStatus::Fail
        };

        let message = format!(
            "{}/{} core security tools discovered",
            found_count,
            core_tools.len()
        );
        let mut details = Vec::new();
        if !missing.is_empty() {
            details.push(format!("Missing: {}", missing.join(", ")));
        }

        HealthResult {
            name: "Toolbox".to_string(),
            status,
            message,
            details,
            latency_ms: None,
        }
    }
}

// ─── MCP Server Status Checker ───
pub struct McpChecker {
    pub manager: Option<Arc<tokio::sync::RwLock<crate::mcp::client::McpClientManager>>>,
}

#[async_trait]
impl HealthCheck for McpChecker {
    async fn run(&self, config: &AppConfig) -> HealthResult {
        let mut details = Vec::new();
        let mut status = HealthStatus::Pass;
        let mut message = "All enabled MCP servers are functional".to_string();
        let mut enabled_count = 0;

        // 1. Check if npx is available
        if std::process::Command::new("npx")
            .arg("--version")
            .output()
            .is_err()
        {
            return HealthResult {
                name: "MCP Registry".to_string(),
                status: HealthStatus::Fail,
                message: "npx runtime not found".to_string(),
                details: vec!["Error: 'npx' is required for local MCP servers.".to_string()],
                latency_ms: None,
            };
        }

        // 2. Deep diagnostics if manager is available
        if let Some(ref mg_lock) = self.manager {
            let manager = mg_lock.read().await;
            for (name, srv_cfg) in &config.mcp.mcp_servers {
                let is_enabled = match srv_cfg {
                    crate::config::CustomMcpServer::Local(l) => l.enabled,
                    crate::config::CustomMcpServer::Remote(r) => r.enabled,
                };

                if is_enabled {
                    enabled_count += 1;
                    if let Some(client) = manager.get_client(name) {
                        let proc_info = client
                            .process_info()
                            .map(|(p, s): (u32, String)| {
                                format!(" [PID: {}] [{}]", p.cyan(), s.bold())
                            })
                            .unwrap_or_default();
                        details.push(format!(
                            "Server [{}]: {}{}",
                            name.bright_yellow(),
                            "CONNECTED".green().bold(),
                            proc_info
                        ));
                    } else {
                        status = HealthStatus::Warn;
                        message = "Some enabled servers are disconnected".to_string();
                        details.push(format!(
                            "Server [{}]: {}",
                            name.bright_yellow(),
                            "DISCONNECTED".red().bold()
                        ));
                    }
                }
            }
        } else {
            // Fallback to basic config check
            for (name, srv_cfg) in &config.mcp.mcp_servers {
                if match srv_cfg {
                    crate::config::CustomMcpServer::Local(l) => l.enabled,
                    crate::config::CustomMcpServer::Remote(r) => r.enabled,
                } {
                    enabled_count += 1;
                    details.push(format!(
                        "Server [{}]: {} (Configured)",
                        name.bright_yellow(),
                        "OK".green()
                    ));
                }
            }
        }

        if enabled_count == 0 {
            message = "No MCP servers enabled".to_string();
            status = HealthStatus::Warn;
        }

        HealthResult {
            name: "MCP Registry".to_string(),
            status,
            message,
            details,
            latency_ms: None,
        }
    }
}
// ─── Browser Engine Checker ───
pub struct BrowserChecker;

#[async_trait]
impl HealthCheck for BrowserChecker {
    async fn run(&self, _config: &AppConfig) -> HealthResult {
        let start = Instant::now();

        // Build ordered candidate list
        let mut candidate_dirs = Vec::new();

        // 1. $PREFIX/bin — Termux (Android)
        if let Ok(prefix) = std::env::var("PREFIX") {
            if prefix.contains("com.termux") {
                candidate_dirs.push(format!("{}/bin", prefix));
            }
        }

        // 2. ~/.local/bin — user install (Linux)
        if let Some(home) = dirs::home_dir() {
            candidate_dirs.push(home.join(".local/bin").to_string_lossy().into_owned());
        }

        // 3. System-wide bin directories (Generic POSIX)
        let standard_bins = vec!["/usr/local/bin", "/usr/bin", "/bin", "/usr/sbin", "/sbin"];
        for path in standard_bins {
            if std::path::Path::new(path).exists() {
                candidate_dirs.push(path.to_string());
            }
        }

        let current_path = std::env::var("PATH").unwrap_or_default();
        let extended_path = format!("{}:{}", candidate_dirs.join(":"), current_path);

        // Check direct file existence in candidate dirs (fastest)
        let mut bin_path = None;
        for dir in &candidate_dirs {
            let path = std::path::Path::new(dir).join("lightpanda");
            if path.exists() {
                bin_path = Some(path.to_string_lossy().into_owned());
                break;
            }
        }

        if bin_path.is_none() {
            // Try all candidate dirs directly
            let found = candidate_dirs.iter().find_map(|dir| {
                let p = format!("{}/lightpanda", dir);
                if std::path::Path::new(&p).exists() {
                    Some(p)
                } else {
                    None
                }
            });
            bin_path = if found.is_some() {
                found
            } else {
                // Fall back to PATH probe
                std::process::Command::new("which")
                    .env("PATH", &extended_path)
                    .arg("lightpanda")
                    .output()
                    .ok()
                    .and_then(|o| {
                        if o.status.success() {
                            Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
                        } else {
                            None
                        }
                    })
            };
        }

        if let Some(path) = bin_path {
            // 2. Version & Arch Audit
            // Lightpanda uses 'version' subcommand, most others use '--version'
            let mut output = std::process::Command::new(&path).arg("version").output();

            if output.as_ref().map(|o| !o.status.success()).unwrap_or(true) {
                // Fallback to --version for standard compliant binaries
                output = std::process::Command::new(&path).arg("--version").output();
            }

            match output {
                Ok(out) if out.status.success() => {
                    let version_info = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    let arch = std::env::consts::ARCH;
                    let latency = start.elapsed().as_millis();

                    HealthResult {
                        name: "Browser Engine".to_string(),
                        status: HealthStatus::Pass,
                        message: "Lightpanda engine is operational".to_string(),
                        details: vec![
                            format!("Path: {}", path),
                            format!("Version: {}", version_info),
                            format!("Arch: {} (Verified)", arch),
                        ],
                        latency_ms: Some(latency),
                    }
                }
                _ => HealthResult {
                    name: "Browser Engine".to_string(),
                    status: HealthStatus::Warn,
                    message: "Engine present but unresponsive".to_string(),
                    details: vec![
                        format!("Path: {}", path),
                        "Error: Failed to execute --version".to_string(),
                    ],
                    latency_ms: None,
                },
            }
        } else {
            HealthResult {
                name: "Browser Engine".to_string(),
                status: HealthStatus::Fail,
                message: "Lightpanda engine not found".to_string(),
                details: vec![
                    "Status: Level-1 Asset Missing".to_string(),
                    "Action: Will be autonomously provisioned on next reconnaissance.".to_string(),
                ],
                latency_ms: None,
            }
        }
    }
}

pub struct HealthEngine {
    checkers: Vec<Box<dyn HealthCheck>>,
}

impl HealthEngine {
    pub fn new(
        mcp_manager: Option<Arc<tokio::sync::RwLock<crate::mcp::client::McpClientManager>>>,
    ) -> Self {
        Self {
            checkers: vec![
                Box::new(EnvironmentChecker),
                Box::new(SandboxChecker),
                Box::new(AiChecker),
                Box::new(ToolInventoryChecker),
                Box::new(BrowserChecker),
                Box::new(McpChecker {
                    manager: mcp_manager,
                }),
            ],
        }
    }

    pub async fn run_all(&self, config: &AppConfig) -> Vec<HealthResult> {
        let futures = self.checkers.iter().map(|checker| checker.run(config));
        futures::future::join_all(futures).await
    }
}

pub fn render_results(results: &[HealthResult]) {
    print!("{}", format_results(results));
}

pub fn format_results(results: &[HealthResult]) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "\n  {}\n",
        "┌───────────────────────────────────────────────────────────┐".bright_black()
    ));
    out.push_str(&format!(
        "  {}     {} {}\n",
        "│".bright_black(),
        "🛡️".bright_green(),
        "MYTH COMMANDER: SYSTEM HEALTH REPORT".bold().bright_white()
    ));
    out.push_str(&format!(
        "  {}\n",
        "├───────────────────────────────────────────────────────────┤".bright_black()
    ));

    let mut all_pass = true;
    let mut has_warn = false;

    for res in results {
        let (icon, color_status) = match res.status {
            HealthStatus::Pass => ("✓".green().to_string(), "PASS".green().bold().to_string()),
            HealthStatus::Warn => {
                has_warn = true;
                ("!".yellow().to_string(), "WARN".yellow().bold().to_string())
            }
            HealthStatus::Fail => {
                all_pass = false;
                ("✗".red().to_string(), "FAIL".red().bold().to_string())
            }
        };

        let latency = res
            .latency_ms
            .map(|l| format!(" [{}ms]", l).dimmed().to_string())
            .unwrap_or_default();

        out.push_str(&format!(
            "  {}  {} {:<12} {:>31} {}\n",
            "│".bright_black(),
            icon,
            res.name.bold(),
            color_status,
            latency
        ));
        out.push_str(&format!(
            "  {}     {} {}\n",
            "│".bright_black(),
            "→".dimmed(),
            res.message.bright_black().italic()
        ));

        for detail in &res.details {
            out.push_str(&format!(
                "  {}       {}\n",
                "│".bright_black(),
                detail.dimmed()
            ));
        }
        out.push_str(&format!("  {} \n", "│".bright_black()));
    }

    out.push_str(&format!(
        "  {}\n",
        "├───────────────────────────────────────────────────────────┤".bright_black()
    ));

    if all_pass && !has_warn {
        out.push_str(&format!(
            "  {}  {} SYSTEM STATUS: {}\n",
            "│".bright_black(),
            "🟢".green(),
            "OPTIMAL".bold().green()
        ));
    } else if all_pass && has_warn {
        out.push_str(&format!(
            "  {}  {} SYSTEM STATUS: {}\n",
            "│".bright_black(),
            "🟡".yellow(),
            "DEGRADED - Review warnings above".bold().yellow()
        ));
    } else {
        out.push_str(&format!(
            "  {}  {} SYSTEM STATUS: {}\n",
            "│".bright_black(),
            "🔴".red(),
            "CRITICAL - System requires attention".bold().red()
        ));
    }

    out.push_str(&format!(
        "  {}\n\n",
        "└───────────────────────────────────────────────────────────┘".bright_black()
    ));
    out
}
