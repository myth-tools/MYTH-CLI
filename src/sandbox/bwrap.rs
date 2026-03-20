//! Bubblewrap (bwrap) sandbox — per-command OS isolation.
//!
//! Wraps every tool invocation in a bubblewrap namespace with:
//! - Read-only host filesystem
//! - Shared network (required for recon)
//! - tmpfs writable dirs
//! - PID/IPC isolation
//! - TIOCSTI escape prevention

use crate::config::AppConfig;
use std::process::Stdio;
use thiserror::Error;
use tokio::io::AsyncBufReadExt;
use tokio::process::Command;

#[derive(Debug, Error)]
pub enum SandboxError {
    #[error("Bubblewrap binary not found at: {0}")]
    BwrapNotFound(String),

    #[error("Command blocked by security policy: {0}")]
    CommandBlocked(String),

    #[error("Tool binary not found: {0}")]
    BinaryNotFound(String),

    #[error("Sandbox execution failed: {0}")]
    ExecutionError(String),

    #[error("Command timed out after {0}s")]
    Timeout(u64),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Callback for live output
pub type OutputCallback = Box<dyn Fn(String) + Send + Sync>;

/// Result of a sandboxed command execution.
#[derive(Debug, Clone)]
pub struct SandboxResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub duration_ms: u64,
    pub command: String,
}

pub struct BubblewrapSandbox {
    bwrap_path: String,
    share_network: bool,
    new_session: bool,
    die_with_parent: bool,
    read_only_paths: Vec<String>,
    tool_paths: Vec<String>,
    writable_tmpfs: Vec<String>,
    hostname: String,
    timeout_seconds: u64,
    proxy_config: crate::config::ProxyConfig,
}

impl BubblewrapSandbox {
    /// Create a new sandbox from config.
    pub fn from_config(config: &AppConfig) -> Self {
        Self {
            bwrap_path: config.sandbox.bwrap_path.clone(),
            share_network: config.sandbox.share_network,
            new_session: config.sandbox.new_session,
            die_with_parent: config.sandbox.die_with_parent,
            read_only_paths: config.sandbox.read_only_paths.clone(),
            tool_paths: config.mcp.tool_paths.clone(),
            writable_tmpfs: config.sandbox.writable_tmpfs.clone(),
            hostname: config.sandbox.hostname.clone(),
            timeout_seconds: config.agent.timeout_seconds,
            proxy_config: config.proxy.clone(),
        }
    }

    /// Parse a proxy URL robustly, handling socks4/5, http/https, auth, IPv6.
    fn parse_proxy_url(proxy_url: &str) -> (String, String, String, String, String) {
        // Returns (proxy_type, host, port, user, pass)
        // Try url::Url first for robust parsing
        if let Ok(parsed) = url::Url::parse(proxy_url) {
            let proxy_type = match parsed.scheme() {
                "socks5" | "socks5h" => "socks5",
                "socks4" | "socks4a" => "socks4",
                _ => "http",
            }
            .to_string();

            let host = parsed.host_str().unwrap_or("127.0.0.1").to_string();
            let port = parsed.port().map(|p| p.to_string()).unwrap_or_else(|| {
                if proxy_type == "socks5" || proxy_type == "socks4" {
                    "9050".to_string()
                } else {
                    "8080".to_string()
                }
            });
            let user = if parsed.username().is_empty() {
                String::new()
            } else {
                urlencoding::decode(parsed.username())
                    .unwrap_or_default()
                    .to_string()
            };
            let pass = parsed
                .password()
                .map(|p| urlencoding::decode(p).unwrap_or_default().to_string())
                .unwrap_or_default();

            (proxy_type, host, port, user, pass)
        } else {
            // Fallback: manual parsing for edge cases
            let (proxy_type, remainder) = if proxy_url.starts_with("socks5") {
                (
                    "socks5".to_string(),
                    proxy_url.replace("socks5://", "").replace("socks5h://", ""),
                )
            } else if proxy_url.starts_with("socks4") {
                (
                    "socks4".to_string(),
                    proxy_url.replace("socks4://", "").replace("socks4a://", ""),
                )
            } else {
                (
                    "http".to_string(),
                    proxy_url.replace("http://", "").replace("https://", ""),
                )
            };

            let mut host_part = remainder;
            let default_port = if proxy_type.starts_with("socks") {
                "9050"
            } else {
                "8080"
            };
            let mut port = default_port.to_string();

            if let Some(pos) = host_part.rfind(':') {
                port = host_part[pos + 1..].to_string();
                host_part.truncate(pos);
            }

            let mut user = String::new();
            let mut pass = String::new();
            if let Some(pos) = host_part.find('@') {
                let auth = host_part[..pos].to_string();
                host_part = host_part[pos + 1..].to_string();
                if let Some(p) = auth.find(':') {
                    user = auth[..p].to_string();
                    pass = auth[p + 1..].to_string();
                }
            }

            (proxy_type, host_part, port, user, pass)
        }
    }

    /// Build the full bwrap command for a given tool invocation.
    fn build_command(
        &self,
        binary: &str,
        args: &[&str],
        workspace_path: &std::path::Path,
    ) -> Command {
        let mut cmd = Command::new(&self.bwrap_path);

        // Unshare all namespaces
        cmd.arg("--unshare-all");

        // Re-share network (required for recon tools)
        if self.share_network {
            cmd.arg("--share-net");
        }

        // Read-only host paths FIRST (base layer)
        for path in &self.read_only_paths {
            cmd.arg("--ro-bind").arg(path).arg(path);
        }

        // Writable tmpfs directories SECOND (overlays ro-bind, ensuring writability wins)
        for path in &self.writable_tmpfs {
            cmd.arg("--tmpfs").arg(path);
        }

        // Bind workspace
        cmd.arg("--bind").arg(workspace_path).arg("/workspace");

        // /proc and /dev
        cmd.arg("--proc").arg("/proc");
        cmd.arg("--dev").arg("/dev");

        // Security hardening
        if self.new_session {
            cmd.arg("--new-session");
        }
        if self.die_with_parent {
            cmd.arg("--die-with-parent");
        }

        // Set internal PATH to include all discovered tool directories
        let path_dirs: Vec<String> = self
            .tool_paths
            .iter()
            .filter(|p| !p.is_empty())
            .cloned()
            .collect();
        let internal_path = path_dirs.join(":");
        cmd.arg("--setenv").arg("PATH").arg(internal_path);

        // Custom hostname
        cmd.arg("--hostname").arg(&self.hostname);

        // Set working directory
        cmd.arg("--chdir").arg("/workspace");

        // Create proxychains config if enabled
        let mut final_binary = binary.to_string();
        let mut final_args = args.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        let use_proxychains = self.proxy_config.enabled && self.proxy_config.use_for_tools;
        let proxy_url_opt = if self.proxy_config.auto_rotate && use_proxychains {
            Some(
                self.proxy_config
                    .url
                    .clone()
                    .unwrap_or_else(|| "socks5://127.0.0.1:9050".to_string()),
            )
        } else if use_proxychains {
            self.proxy_config.url.clone()
        } else {
            None
        };

        if let Some(proxy_url) = proxy_url_opt {
            let proxy_config_path = workspace_path.join(".proxychains.conf");

            let (proxy_type, host_part, port, user, pass) = Self::parse_proxy_url(&proxy_url);

            let mut config_content = format!(
                "strict_chain\nproxy_dns\nremote_dns_subnet 224\ntcp_read_time_out 15000\ntcp_connect_time_out 10000\n[ProxyList]\n{} {} {}",
                proxy_type, host_part, port
            );
            if !user.is_empty() {
                config_content.push_str(&format!(" {} {}", user, pass));
            }

            let _ = std::fs::write(&proxy_config_path, config_content);

            // Overlay config into the sandbox
            cmd.arg("--ro-bind")
                .arg(&proxy_config_path)
                .arg("/etc/proxychains.conf");

            // Inject proxy env vars for tools that respect them
            cmd.arg("--setenv").arg("http_proxy").arg(&proxy_url);
            cmd.arg("--setenv").arg("https_proxy").arg(&proxy_url);
            cmd.arg("--setenv").arg("ALL_PROXY").arg(&proxy_url);

            // Wrap command through proxychains4
            final_binary = "proxychains4".to_string();
            let mut new_args = vec![
                "-f".to_string(),
                "/etc/proxychains.conf".to_string(),
                "-q".to_string(),
                binary.to_string(),
            ];
            new_args.extend(args.iter().map(|s| s.to_string()));
            final_args = new_args;
        }

        // --- Tactical Anti-Bot Evasion ---
        // Inject a random, pristine modern User-Agent to evade WAFs (2026 versions)
        let user_agents = [
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_5) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/19.4 Safari/605.1.15",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:147.0) Gecko/20100101 Firefox/147.0",
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36",
            "Mozilla/5.0 (iPhone; CPU iPhone OS 19_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/19.3 Mobile/15E148 Safari/604.1",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36 Edg/144.0.0.0",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36",
        ];
        // Weak pseudo-random using time to avoid pulling rand dependency just for this
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros();
        let selected_ua = user_agents[(ts % user_agents.len() as u128) as usize];

        cmd.arg("--setenv").arg("USER_AGENT").arg(selected_ua);
        cmd.arg("--setenv").arg("HTTP_USER_AGENT").arg(selected_ua);

        // The actual command to run
        cmd.arg("--").arg(final_binary);
        for arg in final_args {
            cmd.arg(arg);
        }

        // Capture output
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        cmd
    }

    /// Execute a command inside the sandbox.
    pub async fn execute(
        &self,
        binary: &str,
        args: &[&str],
        workspace_path: &std::path::Path,
        callback: Option<OutputCallback>,
    ) -> Result<SandboxResult, SandboxError> {
        let full_cmd = format!("{} {}", binary, args.join(" "));
        tracing::info!(cmd = %full_cmd, "Executing in sandbox");

        let mut cmd = self.build_command(binary, args, workspace_path);
        Self::run_child_process(&mut cmd, binary, &full_cmd, self.timeout_seconds, callback).await
    }

    /// Execute without sandbox (for when sandbox is disabled).
    pub async fn execute_unsandboxed(
        binary: &str,
        args: &[&str],
        timeout_seconds: u64,
        callback: Option<OutputCallback>,
    ) -> Result<SandboxResult, SandboxError> {
        let full_cmd = format!("{} {}", binary, args.join(" "));
        tracing::warn!(cmd = %full_cmd, "Executing WITHOUT sandbox");

        let mut cmd = Command::new(binary);
        cmd.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());

        Self::run_child_process(&mut cmd, binary, &full_cmd, timeout_seconds, callback).await
    }

    /// Shared execution loop — spawns the child process and captures output with
    /// timeout, memory guards, and optional streaming callback.
    ///
    /// FIX (C-01): Tracks EOF on stdout/stderr to prevent busy-loop when
    /// streams close before the child process exits.
    async fn run_child_process(
        cmd: &mut Command,
        binary: &str,
        full_cmd: &str,
        timeout_seconds: u64,
        callback: Option<OutputCallback>,
    ) -> Result<SandboxResult, SandboxError> {
        let start = std::time::Instant::now();

        let mut child = cmd.spawn().map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                SandboxError::BinaryNotFound(binary.to_string())
            } else {
                SandboxError::ExecutionError(e.to_string())
            }
        })?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| SandboxError::ExecutionError("Failed to capture stdout".into()))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| SandboxError::ExecutionError("Failed to capture stderr".into()))?;

        let mut stdout_reader = tokio::io::BufReader::new(stdout).lines();
        let mut stderr_reader = tokio::io::BufReader::new(stderr).lines();

        let mut final_stdout = String::new();
        let mut final_stderr = String::new();

        const STDOUT_LIMIT: usize = 2 * 1024 * 1024; // 2MB
        const STDERR_LIMIT: usize = 512 * 1024; // 512KB

        let timeout = tokio::time::sleep(std::time::Duration::from_secs(timeout_seconds));
        tokio::pin!(timeout);

        let full_cmd_owned = full_cmd.to_string();

        // Track stream EOF to prevent CPU-burning busy-loop (C-01 fix)
        let mut stdout_done = false;
        let mut stderr_done = false;

        loop {
            tokio::select! {
                line = stdout_reader.next_line(), if !stdout_done => {
                    match line {
                        Ok(Some(l)) => {
                            if let Some(ref cb) = callback {
                                cb(l.clone());
                            }
                            if final_stdout.len() < STDOUT_LIMIT {
                                final_stdout.push_str(&l);
                                final_stdout.push('\n');
                            }
                        }
                        _ => {
                            // Ok(None) = EOF, Err(_) = broken pipe — mark as done
                            stdout_done = true;
                        }
                    }
                }
                line = stderr_reader.next_line(), if !stderr_done => {
                    match line {
                        Ok(Some(l)) => {
                            if let Some(ref cb) = callback {
                                cb(format!("stderr: {}", l));
                            }
                            if final_stderr.len() < STDERR_LIMIT {
                                final_stderr.push_str(&l);
                                final_stderr.push('\n');
                            }
                        }
                        _ => {
                            stderr_done = true;
                        }
                    }
                }
                status = child.wait() => {
                    let exit_status = status.map_err(|e| SandboxError::ExecutionError(e.to_string()))?;
                    let duration_ms = start.elapsed().as_millis() as u64;

                    // Consume remaining output with memory guards
                    if !stdout_done {
                        while let Ok(Some(l)) = stdout_reader.next_line().await {
                            if let Some(ref cb) = callback { cb(l.clone()); }
                            if final_stdout.len() < STDOUT_LIMIT {
                                final_stdout.push_str(&l);
                                final_stdout.push('\n');
                            }
                        }
                    }
                    if !stderr_done {
                        while let Ok(Some(l)) = stderr_reader.next_line().await {
                            if let Some(ref cb) = callback { cb(format!("stderr: {}", l)); }
                            if final_stderr.len() < STDERR_LIMIT {
                                final_stderr.push_str(&l);
                                final_stderr.push('\n');
                            }
                        }
                    }

                    return Ok(SandboxResult {
                        stdout: final_stdout,
                        stderr: final_stderr,
                        exit_code: exit_status.code().unwrap_or(-1),
                        duration_ms,
                        command: full_cmd_owned,
                    });
                }
                _ = &mut timeout => {
                    let _ = child.kill().await;
                    return Err(SandboxError::Timeout(timeout_seconds));
                }
            }
        }
    }
}
