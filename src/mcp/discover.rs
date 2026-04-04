//! Tool Discovery — scans PATH directories, extracts --help, and builds tool registry.

use std::collections::HashMap;
use std::path::Path;
use tokio::process::Command;

/// Information about a discovered tool.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolInfo {
    /// Binary name (e.g., "nmap")
    pub name: String,
    /// Full path to the binary
    pub path: String,
    /// Short description (extracted from --help or man)
    pub description: String,
    /// Detected category (network, web, dns, etc.)
    pub category: ToolCategory,
    /// Whether the tool is available/executable
    pub available: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ToolCategory {
    NetworkScanner,
    WebScanner,
    DnsRecon,
    Fuzzer,
    Vulnerability,
    Osint,
    Exploitation,
    PasswordAttack,
    WirelessAttack,
    ForensicTool,
    CryptoTool,
    Utility,
    Unknown,
}

/// Discovers and catalogs security tools on the system.
pub struct ToolDiscovery {
    tool_paths: Vec<String>,
    cache: HashMap<String, ToolInfo>,
}

impl ToolDiscovery {
    pub fn new(tool_paths: Vec<String>) -> Self {
        Self {
            tool_paths,
            cache: HashMap::new(),
        }
    }

    /// Clear cache and re-scan all paths.
    pub async fn reload(&mut self) -> Vec<ToolInfo> {
        self.cache.clear();
        self.scan().await
    }

    /// Scan all configured paths and build the tool registry.
    /// Scans directories concurrently for maximum speed.
    /// Filters to known security/recon tools and deduplicates across directories.
    pub async fn scan(&mut self) -> Vec<ToolInfo> {
        use tokio::task::JoinSet;

        let mut join_set = JoinSet::new();

        for dir in self.tool_paths.clone() {
            join_set.spawn(async move {
                let mut dir_tools = Vec::new();
                let path = Path::new(&dir);
                if !path.exists() {
                    return dir_tools;
                }

                if let Ok(mut entries) = tokio::fs::read_dir(path).await {
                    while let Ok(Some(entry)) = entries.next_entry().await {
                        let entry_path = entry.path();
                        if !entry_path.is_file() {
                            continue;
                        }

                        // Check if executable
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::PermissionsExt;
                            if let Ok(meta) = entry.metadata().await {
                                if meta.permissions().mode() & 0o111 == 0 {
                                    continue;
                                }
                            }
                        }
                        // Note (Finding 19): We currently rely on the bwrap sandbox to isolate execution,
                        // rather than cryptographically signing or hashing discovered binaries.
                        // Future enterprise releases may enforce strict checksum allowlisting.

                        let name = entry_path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("")
                            .to_string();

                        // M-03 fix: Filter to known security tools
                        if !Self::is_security_tool(&name) {
                            continue;
                        }

                        let info = ToolInfo {
                            name,
                            path: entry_path.to_string_lossy().to_string(),
                            description: String::new(),
                            category: ToolCategory::Unknown, // Categorize after merge
                            available: true,
                        };
                        dir_tools.push(info);
                    }
                }
                dir_tools
            });
        }

        let mut tools = Vec::new();
        while let Some(result) = join_set.join_next().await {
            if let Ok(dir_tools) = result {
                for mut tool in dir_tools {
                    // L-04 fix: Deduplicate tools across directories (first path wins)
                    if self.cache.contains_key(&tool.name) {
                        continue;
                    }
                    tool.category = Self::categorize_tool(&tool.name);
                    self.cache.insert(tool.name.clone(), tool.clone());
                    tools.push(tool);
                }
            }
        }

        tracing::info!(count = tools.len(), "Discovered security tools");
        tools
    }

    /// Get the --help output for a specific tool (lazy loading).
    /// Runs inside a hardened bwrap sandbox with a short timeout to prevent hangs.
    pub async fn get_help(&self, tool_name: &str) -> Option<String> {
        let tool = self.cache.get(tool_name)?;

        // Try --help first, then -h, with a safety timeout
        for flag in &["--help", "-h"] {
            let result = tokio::time::timeout(
                std::time::Duration::from_secs(5),
                Command::new("bwrap")
                    .args([
                        "--unshare-all",
                        "--ro-bind",
                        "/",
                        "/",
                        "--dev",
                        "/dev",
                        "--proc",
                        "/proc",
                        "--new-session",     // C-04 fix: TIOCSTI prevention
                        "--die-with-parent", // C-04 fix: kill child if parent dies
                        "--hostname",
                        "myth-sandbox",
                        "--",
                        &tool.path,
                        flag,
                    ])
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped())
                    .output(),
            )
            .await;

            match result {
                Ok(Ok(output)) => {
                    let text = if output.stdout.is_empty() {
                        String::from_utf8_lossy(&output.stderr).to_string()
                    } else {
                        String::from_utf8_lossy(&output.stdout).to_string()
                    };

                    if !text.is_empty() {
                        // Truncate to first 2000 chars for LLM context
                        return Some(text.chars().take(2000).collect());
                    }
                }
                Ok(Err(_)) => continue, // IO error, try next flag
                Err(_) => {
                    tracing::warn!(tool = tool_name, "get_help timed out after 5s");
                    continue; // Timeout — try next flag
                }
            }
        }

        None
    }

    /// Search tools by name pattern.
    pub fn search(&self, query: &str) -> Vec<&ToolInfo> {
        let query_lower = query.to_lowercase();
        self.cache
            .values()
            .filter(|t| {
                t.name.to_lowercase().contains(&query_lower)
                    || format!("{:?}", t.category)
                        .to_lowercase()
                        .contains(&query_lower)
            })
            .collect()
    }

    /// List all discovered tools.
    pub fn list_all(&self) -> Vec<&ToolInfo> {
        self.cache.values().collect()
    }

    /// Get a tool by name.
    pub fn get(&self, name: &str) -> Option<&ToolInfo> {
        self.cache.get(name)
    }

    /// Check if a binary is likely a security/recon tool.
    /// FIX (I-07): Optimized with HashSet for O(1) lookup.
    fn is_security_tool(name: &str) -> bool {
        use once_cell::sync::Lazy;
        use std::collections::HashSet;

        static KNOWN_TOOLS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
            [
                // Network
                "nmap",
                "masscan",
                "rustscan",
                "netcat",
                "nc",
                "ncat",
                "hping3",
                "arping",
                "fping",
                "traceroute",
                "tcpdump",
                "wireshark",
                "tshark",
                // Web
                "nikto",
                "gobuster",
                "dirb",
                "dirbuster",
                "ffuf",
                "wfuzz",
                "feroxbuster",
                "httpx",
                "httprobe",
                "whatweb",
                "wpscan",
                "sqlmap",
                "nuclei",
                "dalfox",
                "xsstrike",
                "burpsuite",
                // DNS
                "dig",
                "host",
                "nslookup",
                "dnsenum",
                "dnsrecon",
                "fierce",
                "subfinder",
                "sublist3r",
                "amass",
                "massdns",
                "knockpy",
                // OSINT
                "theHarvester",
                "maltego",
                "recon-ng",
                "sherlock",
                "holehe",
                "spiderfoot",
                "shodan",
                // Vulnerability
                "searchsploit",
                "msfconsole",
                "msfvenom",
                "openvas",
                // Password
                "john",
                "hashcat",
                "hydra",
                "medusa",
                "crunch",
                "cewl",
                // Wireless
                "airmon-ng",
                "airodump-ng",
                "aircrack-ng",
                "wifite",
                "kismet",
                // Exploit
                "metasploit",
                "beef-xss",
                "setoolkit",
                // Utility
                "curl",
                "wget",
                "whois",
                "ping",
                "ssh",
                "scp",
                "netstat",
                "ss",
                "ip",
                "arp",
                "openssl",
                "base64",
                "xxd",
                "strings",
                "file",
                "grep",
                "awk",
                "sed",
                "jq",
                "python3",
                "perl",
                "ruby",
            ]
            .iter()
            .cloned()
            .collect()
        });

        KNOWN_TOOLS.contains(name)
    }

    /// Categorize a tool based on its name.
    fn categorize_tool(name: &str) -> ToolCategory {
        match name {
            n if [
                "naabu",
                "nmap",
                "masscan",
                "rustscan",
                "hping3",
                "fping",
                "netcat",
                "nc",
                "ncat",
                "tcpdump",
                "tshark",
                "arping",
                "traceroute",
            ]
            .contains(&n) =>
            {
                ToolCategory::NetworkScanner
            }

            n if [
                "nikto", "wpscan", "nuclei", "dalfox", "xsstrike", "whatweb", "httpx", "httprobe",
                "cmseek",
            ]
            .contains(&n) =>
            {
                ToolCategory::WebScanner
            }

            n if [
                "dig",
                "host",
                "nslookup",
                "dnsenum",
                "dnsrecon",
                "fierce",
                "subfinder",
                "amass",
                "massdns",
                "knockpy",
                "sublist3r",
            ]
            .contains(&n) =>
            {
                ToolCategory::DnsRecon
            }

            n if [
                "ffuf",
                "wfuzz",
                "gobuster",
                "dirb",
                "dirbuster",
                "feroxbuster",
            ]
            .contains(&n) =>
            {
                ToolCategory::Fuzzer
            }

            n if ["searchsploit", "openvas"].contains(&n) => ToolCategory::Vulnerability,

            n if [
                "theHarvester",
                "recon-ng",
                "sherlock",
                "holehe",
                "shodan",
                "spiderfoot",
                "whois",
            ]
            .contains(&n) =>
            {
                ToolCategory::Osint
            }

            n if [
                "sqlmap",
                "msfconsole",
                "msfvenom",
                "metasploit",
                "kiterunner",
                "arjun",
                "commix",
            ]
            .contains(&n) =>
            {
                ToolCategory::Exploitation
            }

            n if ["john", "hashcat", "hydra", "medusa", "crunch", "cewl"].contains(&n) => {
                ToolCategory::PasswordAttack
            }

            n if [
                "airmon-ng",
                "airodump-ng",
                "aircrack-ng",
                "wifite",
                "kismet",
            ]
            .contains(&n) =>
            {
                ToolCategory::WirelessAttack
            }

            n if ["strings", "file", "xxd"].contains(&n) => ToolCategory::ForensicTool,

            n if ["openssl", "base64"].contains(&n) => ToolCategory::CryptoTool,

            n if ["curl", "wget", "ping", "ssh", "python3", "grep", "jq"].contains(&n) => {
                ToolCategory::Utility
            }

            _ => ToolCategory::Unknown,
        }
    }
}
