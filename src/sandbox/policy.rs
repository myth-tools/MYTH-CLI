//! Security policy — argument validation, audit logging, and optional command filtering.
//!
//! MYTH is an unrestricted red-team agent by design. The policy layer provides:
//! - Configurable per-user command filtering (via `blocked_commands` in config)
//! - Argument validation (shell injection, path traversal)
//! - Audit logging for all policy decisions (ring-buffered to prevent OOM)
//!
//! No hardcoded blocklists — the operator has full control.

use crate::config::AppConfig;
use std::collections::VecDeque;
use tracing::{info, warn};

/// Maximum number of audit entries to retain (C-03 fix: prevents unbounded memory growth).
const MAX_AUDIT_LOG_ENTRIES: usize = 10_000;

/// Validates commands against the security policy before execution.
pub struct SecurityPolicy {
    blocked_commands: Vec<String>,
    max_output_bytes: usize,
    audit_log: VecDeque<AuditEntry>,
}

/// An audit log entry for every policy check.
#[derive(Debug, Clone, serde::Serialize)]
pub struct AuditEntry {
    pub timestamp: String,
    pub binary: String,
    pub args: Vec<String>,
    pub verdict: Verdict,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub enum Verdict {
    Allowed,
    Blocked,
}

/// Dangerous shell metacharacters that indicate injection attempts.
const DANGEROUS_CHARS: &[char] = &[
    '|', '&', ';', '`', '$', '(', ')', '{', '}', '<', '>', '\n', '\r', '\0',
];

/// Returns the list of sensitive host paths that should never be referenced in arguments.
/// This list is dynamically augmented based on the operating environment (e.g. Termux).
pub fn get_sensitive_paths() -> Vec<String> {
    let ctx = crate::config::SystemContext::sense();

    let mut paths = if ctx.is_termux {
        vec![
            ctx.join_config("shadow").to_string_lossy().to_string(),
            ctx.join_config("gshadow").to_string_lossy().to_string(),
            ctx.join_config("sudoers").to_string_lossy().to_string(),
            ctx.join_config("passwd").to_string_lossy().to_string(),
            ctx.join_config("ssh").to_string_lossy().to_string(),
            ctx.join_config("tls").to_string_lossy().to_string(),
            ctx.join_config("tor/torrc").to_string_lossy().to_string(),
            ctx.bin_root().join("login").to_string_lossy().to_string(),
            ctx.bin_root().join("passwd").to_string_lossy().to_string(),
        ]
    } else {
        vec![
            "/etc/shadow".to_string(),
            "/etc/gshadow".to_string(),
            "/etc/sudoers".to_string(),
            "/etc/passwd".to_string(),
            "/etc/ssh".to_string(),
            "/root".to_string(),
            "/boot".to_string(),
        ]
    };

    // Generic system-critical neural conduits
    paths.extend(vec![
        "/sys".to_string(),
        "/proc/sysrq-trigger".to_string(),
        "/dev/sda".to_string(),
        "/dev/vda".to_string(),
        "/dev/nvme".to_string(),
        "/mnt".to_string(),
        "/media".to_string(),
        "/.ssh".to_string(),
    ]);

    paths
}

impl SecurityPolicy {
    /// Create from config. Uses the operator-defined blocklist from config.
    /// By default, `blocked_commands` is empty for unrestricted red-team operation.
    pub fn from_config(config: &AppConfig) -> Self {
        let blocked = config.mcp.blocked_commands.clone();
        info!(blocked_count = blocked.len(), "Security policy initialized");

        Self {
            blocked_commands: blocked,
            max_output_bytes: config.mcp.max_output_bytes,
            audit_log: VecDeque::with_capacity(MAX_AUDIT_LOG_ENTRIES.min(1024)),
        }
    }

    /// Check if a command binary is allowed to execute.
    pub fn is_allowed(&self, binary: &str) -> Result<(), String> {
        // Extract just the binary name (strip path)
        let name = std::path::Path::new(binary)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(binary);

        // 1. Check blocklist (user + hardcoded)
        if self.blocked_commands.iter().any(|b| b == name) {
            warn!(binary = name, "BLOCKED by policy");
            let alternative = match name {
                "curl" | "wget" => " (USE `httpx`, `whatweb`, or `nikto` for web probing instead)",
                "ping" | "ping6" => " (USE `nmap -sn` or `fping` for host discovery instead)",
                "rm" | "mv" => " (File destructive operations are not available in this context)",
                "apt" | "pip" | "npm" => " (Package management is unavailable in sandbox)",
                _ => "",
            };
            return Err(format!(
                "Command '{}' execution failed: access denied by local configuration layer.{}",
                name, alternative
            ));
        }

        // 2. Reject absolute paths to blocked commands
        for blocked in &self.blocked_commands {
            if binary.ends_with(&format!("/{}", blocked)) {
                warn!(
                    binary,
                    blocked_match = blocked.as_str(),
                    "BLOCKED by path match"
                );
                return Err(format!(
                    "🚫 Command path '{}' matches blocked command '{}'.",
                    binary, blocked
                ));
            }
        }

        // 3. Reject shell injection in binary name
        if binary.chars().any(|c| DANGEROUS_CHARS.contains(&c)) {
            warn!(binary, "BLOCKED: shell injection detected");
            return Err(format!(
                "🚫 Shell injection detected in command: '{}'",
                binary
            ));
        }

        // 4. Reject empty or whitespace-only commands
        if binary.trim().is_empty() {
            return Err("🚫 Command must be non-empty.".to_string());
        }

        Ok(())
    }

    /// Validate command arguments for safety.
    pub fn validate_args(&self, args: &[&str]) -> Result<(), String> {
        /// Flags that indicate the NEXT argument is an output path.
        const OUTPUT_FLAGS: &[&str] = &[
            "-o",
            "--output",
            "-w",
            "--write",
            "-oN",
            "-oX",
            "-oG",
            "-oA",
            "-output",
            "--output-dir",
            "-of",
        ];

        let tmp_dir = std::env::temp_dir();
        let tmp_path = tmp_dir.to_string_lossy();

        for (i, arg) in args.iter().enumerate() {
            // 1. Dangerous Shell Characters
            // Unconditionally block '$' (Finding 7) to prevent env var exfiltration since bwrap inherits env
            if arg.chars().any(|c| DANGEROUS_CHARS.contains(&c)) {
                warn!(arg, index = i, "BLOCKED: dangerous chars in argument");
                return Err(format!(
                    "🚫 Argument {} contains dangerous characters: '{}'",
                    i, arg
                ));
            }

            // 2. Block sensitive host path references
            // Only exempt if the arg is specifically an output flag (not all -- flags)
            let sensitive_paths = get_sensitive_paths();
            for path in &sensitive_paths {
                if arg.contains(path) {
                    // Check if this is *specifically* an output flag with an embedded value
                    let is_output_flag_value = OUTPUT_FLAGS
                        .iter()
                        .any(|f| arg.starts_with(&format!("{}=", f)));
                    if !is_output_flag_value {
                        warn!(
                            arg,
                            sensitive_path = path,
                            "BLOCKED: sensitive path reference"
                        );
                        return Err(format!(
                            "🚫 Argument references sensitive host path: '{}'",
                            arg
                        ));
                    }
                }
            }

            // 3. Block path traversal attempts
            if arg.contains("../")
                || arg.contains("..%2F")
                || arg.contains("..%2f")
                || arg.contains("..\\")
            {
                warn!(arg, "BLOCKED: path traversal attempt");
                return Err(format!("🚫 Path traversal detected in argument: '{}'", arg));
            }

            // 4. Block writing to system directories
            // Handle both "--output /path" (two-arg) and "--output=/path" (single-arg) forms
            if OUTPUT_FLAGS.contains(arg) {
                // Two-arg form: the next arg is the write target
                if let Some(next_arg) = args.get(i + 1) {
                    if !next_arg.starts_with("/workspace")
                        && !next_arg.starts_with(tmp_path.as_ref())
                        && !next_arg.starts_with("./")
                    {
                        warn!(
                            flag = arg,
                            target = next_arg,
                            "BLOCKED: write to system path"
                        );
                        return Err(format!(
                            "🚫 Cannot write to '{}'. Only /workspace and {} are writable.",
                            next_arg, tmp_path
                        ));
                    }
                }
            } else if let Some(value) = OUTPUT_FLAGS
                .iter()
                .find_map(|f| arg.strip_prefix(&format!("{}=", f)))
            {
                // Single-arg form: --output=/path
                if !value.starts_with("/workspace")
                    && !value.starts_with(tmp_path.as_ref())
                    && !value.starts_with("./")
                {
                    warn!(
                        flag = arg,
                        "BLOCKED: write to system path (single-arg form)"
                    );
                    return Err(format!(
                        "🚫 Cannot write to '{}'. Only /workspace and {} are writable.",
                        value, tmp_path
                    ));
                }
            }
        }

        Ok(())
    }

    /// Transform command arguments for safety (e.g., adding -c to ping).
    pub fn transform_arguments(&self, binary: &str, args: &[String]) -> Vec<String> {
        let name = std::path::Path::new(binary)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(binary);

        let mut transformed = args.to_vec();

        match name {
            "ping" | "ping6" => {
                if !args.iter().any(|a| a == "-c" || a.starts_with("-c")) {
                    transformed.insert(0, "-c".to_string());
                    transformed.insert(1, "4".to_string());
                }
            }
            "fping" | "hping3" => {
                if !args.iter().any(|a| a == "-c" || a.starts_with("-c")) {
                    transformed.insert(0, "-c".to_string());
                    transformed.insert(1, "4".to_string());
                }
            }
            "traceroute" | "tcptraceroute" => {
                if !args.iter().any(|a| a == "-m" || a.starts_with("-m")) {
                    transformed.insert(0, "-m".to_string());
                    transformed.insert(1, "15".to_string());
                }
            }
            "nmap" => {
                // ELITE: nmap requires raw sockets for default SYN scans, which are blocked in unshared namespaces.
                // We automatically inject -sT (Connect Scan) and -Pn (No Ping) for industrial-grade stability.
                if !args.iter().any(|a| a == "-sT" || a == "-sS" || a == "-sU") {
                    transformed.insert(0, "-sT".to_string());
                }
                if !args.iter().any(|a| a == "-Pn") {
                    transformed.insert(0, "-Pn".to_string());
                }
                if !args.iter().any(|a| a == "-n") {
                    transformed.insert(0, "-n".to_string());
                }
                // Latency optimization
                if !args.iter().any(|a| a.contains("timeout")) {
                    transformed.push("--host-timeout".to_string());
                    transformed.push("5m".to_string());
                }
            }

            "masscan" => {
                if !args.iter().any(|a| a.contains("--wait")) {
                    transformed.push("--wait".to_string());
                    transformed.push("3".to_string());
                }
            }
            "nikto" => {
                if !args.iter().any(|a| a.contains("-maxtime")) {
                    transformed.push("-maxtime".to_string());
                    transformed.push("300".to_string());
                }
            }
            "gobuster" | "ffuf" => {
                if !args.iter().any(|a| a == "-t") {
                    transformed.push("-t".to_string());
                    transformed.push("20".to_string());
                }
            }
            "sqlmap" => {
                if !args.iter().any(|a| a.contains("--batch")) {
                    transformed.push("--batch".to_string());
                }
                if !args.iter().any(|a| a.contains("--level")) {
                    transformed.push("--level".to_string());
                    transformed.push("1".to_string());
                }
            }
            "trufflehog" => {
                if !args.iter().any(|a| a == "--no-update") {
                    transformed.insert(0, "--no-update".to_string());
                }
            }
            "gitleaks" => {
                if !args.iter().any(|a| a == "detect" || a == "protect") {
                    transformed.insert(0, "detect".to_string());
                }
                if !args.iter().any(|a| a == "--no-git") {
                    transformed.push("--no-git".to_string());
                }
            }
            "cloudfox" => {
                // Ensure cloudfox operates in non-interactive mode for automated recon
                if !args.iter().any(|a| a == "--out") {
                    transformed.push("--out".to_string());
                    transformed.push("json".to_string());
                }
            }
            _ => {}
        }

        transformed
    }

    /// Log an audit entry (ring-buffered: oldest evicted when cap exceeded).
    pub fn audit(&mut self, binary: &str, args: &[&str], verdict: Verdict, reason: Option<String>) {
        if self.audit_log.len() >= MAX_AUDIT_LOG_ENTRIES {
            self.audit_log.pop_front();
        }
        self.audit_log.push_back(AuditEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            binary: binary.to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
            verdict,
            reason,
        });
    }

    /// Get the audit log as a contiguous slice pair.
    /// Use `make_contiguous()` on a mutable reference if you need a single slice.
    pub fn audit_log(&self) -> &VecDeque<AuditEntry> {
        &self.audit_log
    }

    /// Maximum output bytes to capture.
    pub fn max_output_bytes(&self) -> usize {
        self.max_output_bytes
    }

    /// Get the total number of blocked commands.
    pub fn blocked_count(&self) -> usize {
        self.blocked_commands.len()
    }

    /// Get the full blocklist (for debugging/display).
    pub fn blocked_commands(&self) -> &[String] {
        &self.blocked_commands
    }
}

#[cfg(test)]
impl SecurityPolicy {
    pub fn new(blocked_commands: Vec<String>) -> Self {
        Self {
            blocked_commands,
            max_output_bytes: 1_048_576,
            audit_log: std::collections::VecDeque::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_ring_buffer() {
        let mut policy = SecurityPolicy::new(vec![]);
        for i in 0..MAX_AUDIT_LOG_ENTRIES + 5 {
            policy.audit(
                "test",
                &["arg"],
                Verdict::Allowed,
                Some(format!("test {}", i)),
            );
        }
        assert_eq!(policy.audit_log().len(), MAX_AUDIT_LOG_ENTRIES);
        assert_eq!(
            policy.audit_log().back().unwrap().reason.as_ref().unwrap(),
            &format!("test {}", MAX_AUDIT_LOG_ENTRIES + 4)
        );
    }

    #[test]
    fn test_transformation_cloudfox() {
        let policy = SecurityPolicy::new(vec![]);
        let transformed =
            policy.transform_arguments("cloudfox", &["aws".to_string(), "inventory".to_string()]);
        assert!(transformed.contains(&"--out".to_string()));
        assert!(transformed.contains(&"json".to_string()));
    }

    #[test]
    fn test_blocklist() {
        let policy = SecurityPolicy::new(vec!["rm".to_string(), "sudo".to_string()]);
        assert!(policy.is_allowed("rm").is_err());
        assert!(policy.is_allowed("ls").is_ok());
    }

    fn make_test_config() -> AppConfig {
        let yaml = include_str!("../../config/agent.yaml");
        let mut config: AppConfig = serde_yaml::from_str(yaml).expect("Test config should parse");
        // Merge factory defaults for tests
        for (name, srv) in crate::builtin_mcp::get_factory_defaults() {
            config.mcp.mcp_servers.insert(name, srv);
        }
        config
    }

    #[test]
    fn test_blocks_shell_injection() {
        let config = make_test_config();
        let policy = SecurityPolicy::from_config(&config);
        assert!(policy.is_allowed("nmap; rm -rf /").is_err());
        assert!(policy.is_allowed("$(whoami)").is_err());
        assert!(policy.is_allowed("nmap | tee").is_err());
    }

    #[test]
    fn test_allows_recon_tools() {
        let config = make_test_config();
        let policy = SecurityPolicy::from_config(&config);
        assert!(policy.is_allowed("nmap").is_ok());
        assert!(policy.is_allowed("gobuster").is_ok());
        assert!(policy.is_allowed("dig").is_ok());
        assert!(policy.is_allowed("rustscan").is_ok());
        assert!(policy.is_allowed("curl").is_ok());
    }

    #[test]
    fn test_blocks_sensitive_paths_in_args() {
        let config = make_test_config();
        let policy = SecurityPolicy::from_config(&config);
        assert!(policy.validate_args(&["/etc/shadow"]).is_err());
        assert!(policy.validate_args(&["/root/.ssh"]).is_err());
    }

    #[test]
    fn test_blocks_path_traversal() {
        let config = make_test_config();
        let policy = SecurityPolicy::from_config(&config);
        assert!(policy.validate_args(&["../../etc/passwd"]).is_err());
    }

    #[test]
    fn test_allows_normal_args() {
        let config = make_test_config();
        let policy = SecurityPolicy::from_config(&config);
        assert!(policy.validate_args(&["-sV", "-sC", "target.com"]).is_ok());
        assert!(policy
            .validate_args(&["--top-ports", "1000", "10.0.0.1"])
            .is_ok());
    }

    #[test]
    fn test_empty_command_blocked() {
        let config = make_test_config();
        let policy = SecurityPolicy::from_config(&config);
        assert!(policy.is_allowed("").is_err());
        assert!(policy.is_allowed("   ").is_err());
    }

    #[test]
    fn test_blocks_sensitive_paths_in_flags() {
        let config = make_test_config();
        let policy = SecurityPolicy::from_config(&config);
        // Previously bypassed with -- prefix
        assert!(policy.validate_args(&["--input=/etc/shadow"]).is_err());
        assert!(policy.validate_args(&["-v", "/root/.ssh"]).is_err());
    }

    #[test]
    fn test_blocks_write_to_system_single_arg() {
        let config = make_test_config();
        let policy = SecurityPolicy::from_config(&config);
        assert!(policy.validate_args(&["--output=/etc/malicious"]).is_err());
        assert!(policy.validate_args(&["--output=/workspace/ok"]).is_ok());
    }
}
