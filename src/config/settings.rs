//! MYTH Configuration — Two-tier YAML-based settings with validation.
//!
//! Tier 1: `config/agent.yaml` — embedded at compile time (internal defaults)
//! Tier 2: `config/user.yaml`   — user-facing overrides (API keys, profiles, tuning)

use crate::builtin_mcp;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[macro_export]
macro_rules! mcp_env {
    ($($key:expr => $val:expr),* $(,)?) => {{
        let mut map = std::collections::HashMap::new();
        $(
            let effective_val = std::env::var($key).unwrap_or_else(|_| $val.to_string());
            map.insert($key.to_string(), effective_val);
        )*
        map
    }};
}

// ─── Error Types ───

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Config file not found: {0}")]
    NotFound(PathBuf),

    #[error("Failed to read config: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("Invalid YAML: {0}")]
    ParseError(#[from] serde_yaml::Error),

    #[error("Missing required env var: {0}")]
    MissingEnvVar(String),

    #[error("Validation failed: {0}")]
    ValidationError(String),
}

// ═══════════════════════════════════════════════════════════
//  TIER 1: Internal AppConfig (embedded defaults)
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub agent: AgentConfig,
    pub creator: CreatorConfig,
    pub llm: LlmConfig,
    pub mcp: McpConfig,
    pub sandbox: SandboxConfig,
    pub memory: MemoryConfig,
    pub tui: TuiConfig,
    pub proxy: ProxyConfig,
    #[serde(default)]
    pub profiles: std::collections::HashMap<String, ReconProfile>,
}

impl AppConfig {
    /// Returns the directory for the system-wide or user configuration.
    /// FIX (I-05): Support MYTH_CONFIG_DIR environment variable.
    pub fn config_dir() -> PathBuf {
        if let Ok(env_dir) = std::env::var("MYTH_CONFIG_DIR") {
            let path = PathBuf::from(env_dir);
            if path.is_absolute() {
                return path;
            }
        }

        // System-wide path if installed globally
        let system_dir = PathBuf::from("/etc/myth");
        if system_dir.exists() {
            return system_dir;
        }

        // Default to user config dir
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("/etc/myth"));
        path.push("myth");
        path
    }
}

// ─── Tactical MCP Storage (mcp.json) ───

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpStorage {
    pub mcp_servers: std::collections::HashMap<String, CustomMcpServer>,
}

impl McpStorage {
    pub fn load() -> Result<Self, ConfigError> {
        let path = AppConfig::mcp_config_path();
        if !path.exists() {
            return Ok(McpStorage::default());
        }
        let content = std::fs::read_to_string(&path)?;

        // Advanced: Strip comments and trailing commas for industry-grade robustness
        let sanitized = sanitize_json(&content);

        let storage: McpStorage = serde_json::from_str(&sanitized)
            .map_err(|e| {
                tracing::warn!(error = %e, "MCP Neural link registry syntax error. Check mcp.json for accidental typos.");
                ConfigError::ValidationError(format!("Neural link registry corrupted at {}:{}", e.line(), e.column()))
            })?;
        Ok(storage)
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let path = AppConfig::mcp_config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| ConfigError::ValidationError(format!("Serialization failed: {}", e)))?;

        // Atomic write: Write to temp file first, then rename
        let tmp_path = path.with_extension("tmp");
        std::fs::write(&tmp_path, content)?;
        std::fs::rename(tmp_path, path)?;

        Ok(())
    }

    /// Sync factory defaults into the storage.
    ///
    /// - Adds any new builtin servers that don't exist yet
    /// - Updates existing builtin servers' command/args/description to match source code
    /// - Preserves user settings: enabled state, env vars, allowed_tools
    ///
    /// Returns true if any changes were made (and saved).
    pub fn sync_factory_defaults(&mut self) -> bool {
        let factory_defaults = crate::builtin_mcp::get_factory_defaults();
        let mut changed = false;

        for (name, def_srv) in &factory_defaults {
            if let Some(cur_srv) = self.mcp_servers.get_mut(name) {
                if cur_srv.merge_with_default(def_srv) {
                    tracing::debug!(asset = %name, "Factory asset blueprint updated to source parity");
                    changed = true;
                }
            } else {
                self.mcp_servers.insert(name.clone(), def_srv.clone());
                tracing::info!(asset = %name, "New factory asset synced to registry");
                changed = true;
            }
        }

        if changed {
            if let Err(e) = self.save() {
                tracing::error!("Failed to save factory default sync: {}", e);
                return false;
            }
            tracing::info!("Factory defaults synchronized to mcp.json [Parity: OK]");
        }
        changed
    }
}

/// Advanced JSON pre-processor to enable industry-grade robustness.
/// Strips // and /* */ comments while respecting strings, and removes trailing commas.
fn sanitize_json(json: &str) -> String {
    let mut result = String::with_capacity(json.len());
    let mut in_string = false;
    let mut chars = json.chars().peekable();

    while let Some(c) = chars.next() {
        if in_string {
            result.push(c);
            if c == '\\' {
                if let Some(next) = chars.next() {
                    result.push(next);
                    // Finding 30 Fix: Handle unicode escapes in strings to prevent premature termination
                    if next == 'u' {
                        for _ in 0..4 {
                            if let Some(h) = chars.next() {
                                result.push(h);
                            }
                        }
                    }
                }
            } else if c == '"' {
                in_string = false;
            }
        } else {
            if c == '"' {
                in_string = true;
                result.push(c);
            } else if c == '/' {
                match chars.peek() {
                    Some(&'/') => {
                        chars.next(); // consume second /
                        for next in chars.by_ref() {
                            if next == '\n' {
                                result.push('\n');
                                break;
                            }
                        }
                    }
                    Some(&'*') => {
                        chars.next(); // consume *
                        while let Some(next) = chars.next() {
                            if next == '*' && chars.peek() == Some(&'/') {
                                chars.next(); // consume /
                                break;
                            }
                        }
                    }
                    _ => result.push(c),
                }
            } else {
                result.push(c);
            }
        }
    }

    // Secondary pass: remove trailing commas before ] or }
    let mut final_result = String::with_capacity(result.len());
    let mut temp = result.chars().peekable();
    while let Some(c) = temp.next() {
        if c == ',' {
            // Peek past whitespace/comments to see if we're followed by a closer
            let mut ahead = temp.clone();
            let mut is_trailing = false;
            while let Some(next) = ahead.next() {
                if next == ']' || next == '}' {
                    is_trailing = true;
                    break;
                } else if next.is_whitespace() {
                    continue;
                } else if next == '/' {
                    // Skip potential comments while looking ahead
                    if let Some('/') = ahead.next() {
                        for n in ahead.by_ref() {
                            if n == '\n' {
                                break;
                            }
                        }
                    } else if let Some('*') = ahead.next() {
                        while let Some(n) = ahead.next() {
                            if n == '*' {
                                if let Some('/') = ahead.next() {
                                    break;
                                }
                            }
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            if !is_trailing {
                final_result.push(c);
            }
        } else {
            final_result.push(c);
        }
    }

    final_result
}

// ─── Agent ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub name: String,
    pub version: String,
    pub author: String,
    pub repository_url: String,
    #[serde(default = "default_max_iterations")]
    pub max_iterations: u32,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
    #[serde(default = "default_user_name")]
    pub user_name: String,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_all_report_path")]
    pub all_report_path: String,
    #[serde(default = "default_max_history_turns")]
    pub max_history_turns: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorConfig {
    pub name: String,
    pub role: String,
    pub contact: String,
    pub organization: String,
    #[serde(default = "default_clearance")]
    pub clearance_level: String,
    #[serde(default = "default_license")]
    pub system_license: String,
}

impl Default for CreatorConfig {
    fn default() -> Self {
        Self {
            name: "Shesher Hasan".to_string(),
            role: "Chief Architect".to_string(),
            contact: "shesher@myth-ops.internal".to_string(),
            organization: "MYTH Offensive Operations".to_string(),
            clearance_level: "OPERATIVE-LEVEL-4".to_string(),
            system_license: "MYTH-EULA-2026-BETA".to_string(),
        }
    }
}

fn default_clearance() -> String {
    "OPERATIVE-LEVEL-4".to_string()
}
fn default_license() -> String {
    "MYTH-EULA-2026-BETA".to_string()
}

fn default_max_iterations() -> u32 {
    100
}
fn default_timeout() -> u64 {
    600
}
fn default_user_name() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "Chief".to_string())
}
fn default_log_level() -> String {
    "info".to_string()
}
fn default_all_report_path() -> String {
    "mission_report.md".to_string()
}
fn default_max_history_turns() -> usize {
    40
}

// ─── Proxy ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    #[serde(default)]
    pub enabled: bool,
    /// [OPTIONAL] Proxy URL (e.g., "socks5://127.0.0.1:9050" or "http://user:pass@proxy.com:8080")
    pub url: Option<String>,
    #[serde(default = "default_true")]
    pub use_for_llm: bool,
    #[serde(default = "default_true")]
    pub use_for_tools: bool,
    /// [OPTIONAL] Automatically rotate IP using Tor Control Port before every request
    #[serde(default)]
    pub auto_rotate: bool,
    /// [OPTIONAL] Tor Control Port (default 9051)
    pub tor_control_port: Option<u16>,
    /// [OPTIONAL] Tor Control Password (if authentication is required)
    pub tor_control_password: Option<String>,
    /// [OPTIONAL] Custom proxychains configuration template path
    pub proxychains_template: Option<String>,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            url: None,
            use_for_llm: true,
            use_for_tools: true,
            auto_rotate: false,
            tor_control_port: None,
            tor_control_password: None,
            proxychains_template: None,
        }
    }
}

// ─── LLM / NVIDIA NIM ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub provider: String,
    pub base_url: String,
    #[serde(default = "default_api_keys")]
    pub nvidia_nim_api_key: Vec<String>,
    pub model: String,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    #[serde(default = "default_top_p")]
    pub top_p: f32,
    pub fallback_model: Option<String>,
    #[serde(default = "default_rate_limit")]
    pub rate_limit_ms: u64,
}

fn default_temperature() -> f32 {
    0.1
}
fn default_max_tokens() -> u32 {
    8192
}
fn default_top_p() -> f32 {
    0.9
}
fn default_api_keys() -> Vec<String> {
    vec![]
}
fn default_rate_limit() -> u64 {
    0
}

impl LlmConfig {
    /// Resolves the API keys from the configuration.
    /// If a key matches an environment variable, it uses that value.
    /// Otherwise, it assumes the string is the literal API key.
    pub fn resolve_api_keys(&self) -> Result<Vec<String>, ConfigError> {
        if self.nvidia_nim_api_key.is_empty() {
            // Try environment variable as last resort
            if let Ok(env_key) = std::env::var("NVIDIA_API_KEY") {
                if !env_key.is_empty() {
                    return Ok(vec![env_key]);
                }
            }
            return Err(ConfigError::ValidationError(
                "No API key configured. Set 'api_keys' in ~/.config/myth/user.yaml or export NVIDIA_API_KEY".into()
            ));
        }

        let keys: Vec<String> = self
            .nvidia_nim_api_key
            .iter()
            .map(|k| std::env::var(k).unwrap_or_else(|_| k.clone()))
            .filter(|k| {
                !k.is_empty()
                    && !k.starts_with("YOUR_")
                    && !k.contains("_YOUR_")
                    && k != "nvapi-PLACEHOLDER"
            })
            .collect();

        if keys.is_empty() {
            // Try environment variable as last resort
            if let Ok(env_key) = std::env::var("NVIDIA_API_KEY") {
                if !env_key.is_empty() {
                    return Ok(vec![env_key]);
                }
            }
            return Err(ConfigError::ValidationError(
                "No valid API key found. Configure 'api_keys' in ~/.config/myth/user.yaml or export NVIDIA_API_KEY".into()
            ));
        }

        Ok(keys)
    }

    /// Masks the API keys for safe display in the terminal.
    pub fn mask_api_keys(&mut self) {
        for key in &mut self.nvidia_nim_api_key {
            if key.len() > 8 {
                let prefix = &key[0..4];
                let suffix = &key[key.len() - 4..];
                *key = format!("{}****{}", prefix, suffix);
            } else if !key.is_empty() {
                *key = "****HIDDEN****".to_string();
            }
        }
    }
}

// ─── MCP Server ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    #[serde(default = "default_tool_paths")]
    pub tool_paths: Vec<String>,
    #[serde(default)]
    pub blocked_commands: Vec<String>,
    #[serde(default = "default_max_output")]
    pub max_output_bytes: usize,
    #[serde(default)]
    pub mcp_servers: std::collections::HashMap<String, CustomMcpServer>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum CustomMcpServer {
    Local(LocalMcpConfig),
    Remote(RemoteMcpConfig),
}

impl CustomMcpServer {
    /// Deep-merges factory defaults into the current configuration.
    /// Preserves user-defined 'enabled' state and 'env' variables (sensitive keys),
    /// but updates core tactical blueprints (command, args, description, transport).
    pub fn merge_with_default(&mut self, default: &CustomMcpServer) -> bool {
        let mut changed = false;
        match (self, default) {
            (CustomMcpServer::Local(ref mut cur), CustomMcpServer::Local(def)) => {
                if cur.command != def.command {
                    cur.command = def.command.clone();
                    changed = true;
                }
                if cur.args != def.args {
                    cur.args = def.args.clone();
                    changed = true;
                }
                if cur.description != def.description {
                    cur.description = def.description.clone();
                    changed = true;
                }
                if cur.transport != def.transport {
                    cur.transport = def.transport.clone();
                    changed = true;
                }
                // Robust Sync: Add missing keys OR upgrade empty placeholders if blueprint has a significant value
                for (k, v) in &def.env {
                    let should_update = match cur.env.get(k) {
                        Some(cur_v) => cur_v.is_empty() && !v.is_empty(),
                        None => true,
                    };
                    if should_update {
                        cur.env.insert(k.clone(), v.clone());
                        changed = true;
                    }
                }
            }
            (CustomMcpServer::Remote(ref mut cur), CustomMcpServer::Remote(def)) => {
                if cur.url != def.url {
                    cur.url = def.url.clone();
                    changed = true;
                }
                if cur.description != def.description {
                    cur.description = def.description.clone();
                    changed = true;
                }
                if cur.transport != def.transport {
                    cur.transport = def.transport.clone();
                    changed = true;
                }
                // Robust Sync: Add missing headers OR upgrade empty placeholders
                for (k, v) in &def.headers {
                    let should_update = match cur.headers.get(k) {
                        Some(cur_v) => cur_v.is_empty() && !v.is_empty(),
                        None => true,
                    };
                    if should_update {
                        cur.headers.insert(k.clone(), v.clone());
                        changed = true;
                    }
                }
            }
            _ => {
                // Type mismatch? Force default for tactical integrity
                tracing::warn!(
                    "MCP Server type mismatch in registry. Reverting to tactical default."
                );
                return false; // Caller should handle full replacement
            }
        }
        changed
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum McpTransport {
    /// Standard Input/Output communication
    Stdio,
    /// Server-Sent Events (over HTTP)
    Sse,
    /// Standard HTTP JSON-RPC
    Http,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocalMcpConfig {
    /// [OPTIONAL] Whether this server is active. Default: true.
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// [MANDATORY] Binary or command to execute (e.g., "npx", "python3").
    pub command: String,

    /// [OPTIONAL] Arguments to pass to the command.
    #[serde(default)]
    pub args: Vec<String>,

    /// [OPTIONAL] Map of environment variables required by the server.
    #[serde(default)]
    pub env: std::collections::HashMap<String, String>,

    /// [OPTIONAL] Human-readable description of the asset.
    pub description: Option<String>,

    /// [OPTIONAL] Custom working directory for the server process.
    pub working_dir: Option<String>,

    /// [OPTIONAL] Connection/Operation timeout in seconds. Default: 60.
    #[serde(default = "default_mcp_timeout")]
    pub timeout: u64,

    /// [OPTIONAL] Whitelist of allowed tool names. Empty = Allow All.
    #[serde(default)]
    pub allowed_tools: Vec<String>,

    /// [OPTIONAL] Transport layer (e.g., "stdio", "http"). Default: "stdio".
    #[serde(default = "default_transport_stdio")]
    pub transport: McpTransport,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RemoteMcpConfig {
    /// [OPTIONAL] Whether this link is active. Default: true.
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// [MANDATORY] SSE Endpoint URL for the remote MCP server.
    pub url: String,

    /// [OPTIONAL] Human-readable description of the remote link.
    pub description: Option<String>,

    /// [OPTIONAL] Custom HTTP headers for the connection (e.g., Auth).
    #[serde(default)]
    pub headers: std::collections::HashMap<String, String>,

    /// [OPTIONAL] Connection/Operation timeout in seconds. Default: 60.
    #[serde(default = "default_mcp_timeout")]
    pub timeout: u64,

    /// [OPTIONAL] Whitelist of allowed tool names. Empty = Allow All.
    #[serde(default)]
    pub allowed_tools: Vec<String>,

    /// [OPTIONAL] Transport layer (e.g., "sse", "https"). Default: "sse".
    #[serde(default = "default_transport_sse")]
    pub transport: McpTransport,
}

fn default_mcp_timeout() -> u64 {
    180
}

fn default_transport_stdio() -> McpTransport {
    McpTransport::Stdio
}
fn default_transport_sse() -> McpTransport {
    McpTransport::Sse
}

fn default_tool_paths() -> Vec<String> {
    vec![
        "/usr/bin".into(),
        "/usr/sbin".into(),
        "/usr/local/bin".into(),
    ]
}
fn default_max_output() -> usize {
    10 * 1024 * 1024
} // 10 MB

// ─── Sandbox ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_bwrap_path")]
    pub bwrap_path: String,
    #[serde(default = "default_true")]
    pub share_network: bool,
    #[serde(default = "default_true")]
    pub new_session: bool,
    #[serde(default = "default_true")]
    pub die_with_parent: bool,
    #[serde(default)]
    pub read_only_paths: Vec<String>,
    #[serde(default)]
    pub writable_tmpfs: Vec<String>,
    #[serde(default = "default_workspace_size")]
    pub workspace_size_mb: u32,
    #[serde(default = "default_hostname")]
    pub hostname: String,
}

fn default_true() -> bool {
    true
}
fn default_bwrap_path() -> String {
    "/usr/bin/bwrap".to_string()
}
fn default_workspace_size() -> u32 {
    512
}
fn default_hostname() -> String {
    "myth-sandbox".to_string()
}

// ─── Memory ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_backend")]
    pub backend: String,
    #[serde(default = "default_memory_mode")]
    pub mode: String,
    #[serde(default = "default_grpc_port")]
    pub grpc_port: u16,
    #[serde(default = "default_http_port")]
    pub http_port: u16,
    #[serde(default = "default_collection")]
    pub collection_name: String,
    #[serde(default = "default_vector_size")]
    pub vector_size: u32,
    #[serde(default = "default_true")]
    pub auto_start: bool,
    #[serde(default = "default_qdrant_path")]
    pub qdrant_path: String,
    #[serde(default = "default_max_memory_entries")]
    pub max_entries: usize,
}

fn default_backend() -> String {
    "qdrant".to_string()
}
fn default_memory_mode() -> String {
    "in-memory".to_string()
}
fn default_grpc_port() -> u16 {
    6334
}
fn default_http_port() -> u16 {
    6333
}
fn default_collection() -> String {
    "agent_session".to_string()
}
fn default_vector_size() -> u32 {
    1024
}
fn default_qdrant_path() -> String {
    "qdrant".to_string()
}
fn default_max_memory_entries() -> usize {
    100_000
}

// ─── TUI ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_true")]
    pub show_tree_panel: bool,
    #[serde(default = "default_true")]
    pub show_status_bar: bool,
    #[serde(default = "default_max_output_lines")]
    pub max_output_lines: usize,
    #[serde(default = "default_scroll_speed")]
    pub scroll_speed: u16,
    #[serde(default)]
    pub colors: TuiColors,
}

fn default_theme() -> String {
    "dark".to_string()
}
fn default_max_output_lines() -> usize {
    5000
}
fn default_scroll_speed() -> u16 {
    3
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiColors {
    #[serde(default = "default_primary")]
    pub primary: String,
    #[serde(default = "default_secondary")]
    pub secondary: String,
    #[serde(default = "default_accent")]
    pub accent: String,
    #[serde(default = "default_bg")]
    pub background: String,
    #[serde(default = "default_surface")]
    pub surface: String,
    #[serde(default = "default_text")]
    pub text: String,
    #[serde(default = "default_dim")]
    pub dim: String,
}

impl Default for TuiColors {
    fn default() -> Self {
        Self {
            primary: default_primary(),
            secondary: default_secondary(),
            accent: default_accent(),
            background: default_bg(),
            surface: default_surface(),
            text: default_text(),
            dim: default_dim(),
        }
    }
}

fn default_primary() -> String {
    "#00ff88".to_string()
}
fn default_secondary() -> String {
    "#0088ff".to_string()
}
fn default_accent() -> String {
    "#ff0055".to_string()
}
fn default_bg() -> String {
    "#0a0a0f".to_string()
}
fn default_surface() -> String {
    "#1a1a2e".to_string()
}
fn default_text() -> String {
    "#e0e0e0".to_string()
}
fn default_dim() -> String {
    "#666680".to_string()
}

// ─── Recon Profile (Dual-Mode: Agent-Auto / User-Manual) ───

/// Execution mode: who decides tool selection at each step.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProfileMode {
    /// Agent autonomously selects tools per step (default behavior)
    #[default]
    Agent,
    /// User defines exactly which tools to use at every step of every phase
    User,
}

/// A single step within a reconnaissance phase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseStep {
    /// Human-readable step name (e.g., "Root Domain Enumeration")
    pub name: String,
    /// Tools the user wants to use for this step (User Mode only).
    /// Empty in Agent Mode (agent decides).
    #[serde(default)]
    pub tools: Vec<String>,
    /// Exact raw commands the user wants the agent to execute (User Mode only).
    #[serde(default)]
    pub commands: Vec<String>,
}

/// One of the Eleven Phases of the reconnaissance methodology.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseConfig {
    /// Phase name (e.g., "Phase 0: Organizational Mapping")
    pub name: String,
    /// Whether this phase is enabled. Disabled phases are skipped entirely.
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Steps within this phase, each with optional tool assignments.
    #[serde(default)]
    pub steps: Vec<PhaseStep>,
}

/// Recon profile defining execution mode, phase selection, and per-step tool mapping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconProfile {
    pub description: String,
    /// Execution mode: `agent` (LLM decides tools) or `user` (user defines tools).
    #[serde(default)]
    pub mode: ProfileMode,
    /// Legacy flat tool list — used as fallback/display in Agent Mode.
    #[serde(default)]
    pub tools: Vec<String>,
    #[serde(default = "default_max_iterations")]
    pub max_iterations: u32,
    /// Enforce execution of ONLY the user-defined tools and commands.
    #[serde(default)]
    pub strict_custom_commands: bool,
    /// Per-phase configuration with step-level tool assignments.
    /// If empty, `default_phases()` populates all 13 phases (Phase 0 through Phase 12).
    #[serde(default = "default_phases")]
    pub phases: Vec<PhaseConfig>,
}

impl ReconProfile {
    /// In User Mode, returns the list of allowed tools for a given phase number (0-12).
    /// In Agent Mode, returns None (no restriction — agent decides).
    pub fn allowed_tools_for_phase(&self, phase: u8) -> Option<Vec<String>> {
        if self.mode != ProfileMode::User {
            return None;
        }
        self.phases
            .get(phase as usize)
            .map(|p| p.steps.iter().flat_map(|s| s.tools.clone()).collect())
    }

    /// Returns the phase numbers (0-12) that are enabled.
    pub fn enabled_phases(&self) -> Vec<u8> {
        self.phases
            .iter()
            .enumerate()
            .filter(|(_, p)| p.enabled)
            .map(|(i, _)| i as u8)
            .collect()
    }

    /// Returns the phase config for a given phase number.
    pub fn phase(&self, phase: u8) -> Option<&PhaseConfig> {
        self.phases.get(phase as usize)
    }
}

/// Generates the canonical Thirteen Phases (0–12) with all steps (empty tool lists = agent decides).
fn default_phases() -> Vec<PhaseConfig> {
    vec![
        PhaseConfig {
            name: "Phase 0: Organizational Mapping".into(),
            enabled: true,
            steps: vec![
                PhaseStep {
                    name: "Root Domain Enumeration".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "ASN Discovery".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Acquisition & Subsidiary Research".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Reverse WHOIS".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Linked Discovery".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Seed Target List".into(),
                    tools: vec![],
                    commands: vec![],
                },
            ],
        },
        PhaseConfig {
            name: "Phase 1: Identity & Credential Intelligence".into(),
            enabled: true,
            steps: vec![
                PhaseStep {
                    name: "Employee & Hierarchy Mapping".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Email Format Discovery & Validation".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Historical Credential Leak Analysis".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Identity-to-Service Mapping".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Password Policy Inference".into(),
                    tools: vec![],
                    commands: vec![],
                },
            ],
        },
        PhaseConfig {
            name: "Phase 2: Asset Discovery & Enumeration".into(),
            enabled: true,
            steps: vec![
                PhaseStep {
                    name: "WHOIS Deep Dive".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "DNS Full Enumeration".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Subdomain Discovery".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Reverse DNS".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Certificate Transparency Logs".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "OSINT Gathering".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Technology Fingerprinting".into(),
                    tools: vec![],
                    commands: vec![],
                },
            ],
        },
        PhaseConfig {
            name: "Phase 3: Active Reconnaissance".into(),
            enabled: true,
            steps: vec![
                PhaseStep {
                    name: "Port Scanning".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Service Version Detection".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Version-to-CVSS Investigation".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Stack Identification".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "CDN/WAF Fingerprinting".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Cloud Infrastructure Detection".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "OS Fingerprinting".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Interesting Endpoint Discovery".into(),
                    tools: vec![],
                    commands: vec![],
                },
            ],
        },
        PhaseConfig {
            name: "Phase 4: Content & Application Discovery".into(),
            enabled: true,
            steps: vec![
                PhaseStep {
                    name: "Directory & File Bruteforce".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Exposed Source Code & Git Extraction".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "JavaScript Analysis".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Virtual Host Discovery".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "CMS Identification & Version Detection".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "API Endpoint Discovery".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Authentication Mechanism Analysis".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "WebSocket Endpoint Discovery".into(),
                    tools: vec![],
                    commands: vec![],
                },
            ],
        },
        PhaseConfig {
            name: "Phase 5: Supply Chain & Dependency Analysis".into(),
            enabled: true,
            steps: vec![
                PhaseStep {
                    name: "JavaScript Library Analysis".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Open Source Dependency Scanning".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "CDN & Third-Party Service Mapping".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Subresource Integrity (SRI) Checking".into(),
                    tools: vec![],
                    commands: vec![],
                },
            ],
        },
        PhaseConfig {
            name: "Phase 6: AI/ML Attack Surface".into(),
            enabled: true,
            steps: vec![
                PhaseStep {
                    name: "AI Endpoint Discovery".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Model File Detection".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Training Data Exposure".into(),
                    tools: vec![],
                    commands: vec![],
                },
            ],
        },
        PhaseConfig {
            name: "Phase 7: Dynamic Input & Interaction Analysis".into(),
            enabled: true,
            steps: vec![
                PhaseStep {
                    name: "Hidden Parameter Discovery".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "File Upload Testing".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Special Character & Injection Testing".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "User Reference Analysis (IDOR Prep)".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "User Role & Privilege Analysis".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Session Management Deep Dive".into(),
                    tools: vec![],
                    commands: vec![],
                },
            ],
        },
        PhaseConfig {
            name: "Phase 8: Vulnerability Assessment".into(),
            enabled: true,
            steps: vec![
                PhaseStep {
                    name: "Known CVE Exploitation Check".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Default/Weak Credential Testing".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "SSL/TLS Analysis".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Security Header Analysis".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Misconfiguration Detection".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Subdomain Takeover Checks".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "IDOR Testing".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "CORS Misconfiguration Testing".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "XSS Detection".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Open Redirect Detection".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "SSRF Entry Point Identification".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Race Condition Checks".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Template Injection (SSTI)".into(),
                    tools: vec![],
                    commands: vec![],
                },
            ],
        },
        PhaseConfig {
            name: "Phase 9: Secrets & Exposure Analysis".into(),
            enabled: true,
            steps: vec![
                PhaseStep {
                    name: "Git & Source Code Leak Mining".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Cloud Storage Exposure".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Document Metadata Extraction".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Paste Site & Dark Web Monitoring".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "GitHub Organization Monitoring".into(),
                    tools: vec![],
                    commands: vec![],
                },
            ],
        },
        PhaseConfig {
            name: "Phase 10: Social Engineering & OSINT Exposure".into(),
            enabled: true,
            steps: vec![
                PhaseStep {
                    name: "Public Document & Metadata Mining".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Social Media & Tech Talk Analysis".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Helpdesk & Support Channel Discovery".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Executive OSINT Profile".into(),
                    tools: vec![],
                    commands: vec![],
                },
            ],
        },
        PhaseConfig {
            name: "Phase 11: Continuous Monitoring & Delta Discovery".into(),
            enabled: true,
            steps: vec![
                PhaseStep {
                    name: "DNS Change Detection".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Port & Service Change Alerts".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "SSL Certificate Monitoring".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Content Hashing (File Change Detection)".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "GitHub Org & New Repo Monitoring".into(),
                    tools: vec![],
                    commands: vec![],
                },
            ],
        },
        PhaseConfig {
            name: "Phase 12: Attack Surface Synthesis & Reporting".into(),
            enabled: true,
            steps: vec![
                PhaseStep {
                    name: "Critical Asset Identification".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Attack Path Visualization".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Findings Summary with Severity".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "OWASP/CWE Mapping".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Remediation Recommendations".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Complete Asset Inventory".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Attack Surface Map".into(),
                    tools: vec![],
                    commands: vec![],
                },
                PhaseStep {
                    name: "Methodology & Tools Documentation".into(),
                    tools: vec![],
                    commands: vec![],
                },
            ],
        },
    ]
}

// ═══════════════════════════════════════════════════════════
//  TIER 2: User Config (user.yaml overlay)
// ═══════════════════════════════════════════════════════════

/// User-facing configuration that overlays the embedded defaults.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserConfig {
    #[serde(default)]
    pub provider: Option<UserProviderConfig>,
    #[serde(default)]
    pub profiles: Option<std::collections::HashMap<String, ReconProfile>>,
    #[serde(default)]
    pub agent: Option<UserAgentConfig>,
    #[serde(default)]
    pub mcp: Option<UserMcpConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserMcpConfig {
    #[serde(default)]
    pub tool_paths: Option<Vec<String>>,
    #[serde(default)]
    pub blocked_commands: Option<Vec<String>>,
    #[serde(default)]
    pub max_output_bytes: Option<usize>,
    #[serde(default)]
    pub mcp_servers: Option<std::collections::HashMap<String, CustomMcpServer>>,
}

impl UserConfig {
    /// Load the user config directly from the standard path.
    pub fn load() -> Result<Self, ConfigError> {
        let path = AppConfig::user_config_path();
        if !path.exists() {
            return Ok(UserConfig::default());
        }
        let content = std::fs::read_to_string(&path)?;
        let user_config: UserConfig = serde_yaml::from_str(&content)?;
        Ok(user_config)
    }

    /// Save the user config directly to the standard path.
    pub fn save(&self) -> Result<(), ConfigError> {
        let path = AppConfig::user_config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        // Save using serde_yaml.
        let content = serde_yaml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProviderConfig {
    #[serde(default)]
    pub api_keys: Vec<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub fallback_model: Option<String>,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAgentConfig {
    #[serde(default)]
    pub max_iterations: Option<u32>,
    #[serde(default)]
    pub timeout_seconds: Option<u64>,
    pub user_name: Option<String>,
    #[serde(default)]
    pub log_level: Option<String>,
}

// ═══════════════════════════════════════════════════════════
//  Loading & Merging
// ═══════════════════════════════════════════════════════════

/// The embedded default config (compiled into the binary).
const EMBEDDED_DEFAULTS: &str = include_str!("../../config/agent.yaml");

/// The embedded user config template (for auto-generation).
const USER_CONFIG_TEMPLATE: &str = include_str!("../../config/user.yaml");

impl AppConfig {
    /// Load from a specific path (legacy support).
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        if !path.exists() {
            return Err(ConfigError::NotFound(path.to_path_buf()));
        }
        let content = std::fs::read_to_string(path)?;
        let mut config: AppConfig = serde_yaml::from_str(&content)?;
        config.validate()?;
        config.expand_tool_paths();
        Ok(config)
    }

    /// Primary loading method: embedded defaults + user config overlay.
    ///
    /// 1. Parse the compiled-in `agent.yaml` as the base
    /// 2. Search for `user.yaml` and overlay user settings
    /// 3. Auto-generate `~/.config/myth/user.yaml` if none found
    pub fn load_merged() -> Result<Self, ConfigError> {
        // Step 1: Load embedded defaults
        let mut config: AppConfig = serde_yaml::from_str(EMBEDDED_DEFAULTS)
            .map_err(|e| ConfigError::ValidationError(format!("Embedded config invalid: {}", e)))?;

        // Step 2: Load or Initialize Dedicated MCP Storage (mcp.json)
        // Guarded load: if corrupted, fall back to factory defaults
        let factory_defaults = builtin_mcp::get_factory_defaults();
        let mut mcp_storage = match McpStorage::load() {
            Ok(s) => s,
            Err(e) => {
                tracing::error!(error = %e, "Failed to load mcp.json - neural link registry corrupted. Using system emergency defaults.");
                McpStorage {
                    mcp_servers: factory_defaults,
                }
            }
        };

        // Single source of truth: sync factory defaults via the dedicated method
        mcp_storage.sync_factory_defaults();

        // Overlay JSON servers onto base config (JSON is source of truth)
        for (name, srv) in mcp_storage.mcp_servers {
            config.mcp.mcp_servers.insert(name, srv);
        }

        // Step 3: Find and load user config (for non-MCP keys)
        let user_config_path = Self::find_user_config();

        if let Some(ref path) = user_config_path {
            tracing::info!(path = %path.display(), "Loading user config");
            let user_content = std::fs::read_to_string(path)?;
            let user_config: UserConfig = serde_yaml::from_str(&user_content)?;
            config.apply_user_config(&user_config);
        } else {
            // Auto-generate user config on first run
            let generated_path = Self::auto_generate_user_config();
            if let Some(ref p) = generated_path {
                tracing::info!(path = %p.display(), "Auto-generated user config");
            }
        }

        config.validate()?;
        config.expand_tool_paths();
        Ok(config)
    }

    /// Search for user.yaml in standard locations.
    fn find_user_config() -> Option<PathBuf> {
        let candidates = vec![
            // Local project config (for development)
            PathBuf::from("config/user.yaml"),
            // XDG standard: ~/.config/myth/user.yaml
            dirs::config_dir()
                .unwrap_or_default()
                .join("myth/user.yaml"),
            // Legacy fallback
            PathBuf::from("/etc/myth/user.yaml"),
        ];

        for path in &candidates {
            if path.exists() {
                return Some(path.clone());
            }
        }
        None
    }

    /// Search for mcp.json in standard locations.
    fn find_mcp_config() -> Option<PathBuf> {
        let candidates = vec![
            // Local project config (for development)
            PathBuf::from("config/mcp.json"),
            // XDG standard: ~/.config/myth/mcp.json
            dirs::config_dir().unwrap_or_default().join("myth/mcp.json"),
        ];

        for path in &candidates {
            if path.exists() {
                return Some(path.clone());
            }
        }
        None
    }

    /// Auto-generate user.yaml at ~/.config/myth/user.yaml.
    fn auto_generate_user_config() -> Option<PathBuf> {
        let config_dir = dirs::config_dir()?.join("myth");
        let config_path = config_dir.join("user.yaml");

        if config_path.exists() {
            return Some(config_path);
        }

        // Create directory
        if let Err(e) = std::fs::create_dir_all(&config_dir) {
            tracing::warn!(error = %e, "Failed to create config directory");
            return None;
        }

        // Write template
        if let Err(e) = std::fs::write(&config_path, USER_CONFIG_TEMPLATE) {
            tracing::warn!(error = %e, "Failed to write user config template");
            return None;
        }

        Some(config_path)
    }

    /// Returns the path where user config was found or will be generated.
    pub fn user_config_path() -> PathBuf {
        Self::find_user_config().unwrap_or_else(|| {
            dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("~/.config"))
                .join("myth/user.yaml")
        })
    }

    pub fn mcp_config_path() -> PathBuf {
        Self::find_mcp_config().unwrap_or_else(|| {
            dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("~/.config"))
                .join("myth/mcp.json")
        })
    }

    /// Returns the path for the lightweight mission context persistence.
    pub fn mission_context_path() -> PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("~/.local/share"))
            .join("myth/session.json")
    }

    /// Apply user config overrides onto the base config.
    fn apply_user_config(&mut self, user: &UserConfig) {
        // Provider overrides
        if let Some(ref provider) = user.provider {
            if !provider.api_keys.is_empty() {
                self.llm.nvidia_nim_api_key = provider.api_keys.clone();
            }
            if let Some(ref model) = provider.model {
                self.llm.model = model.clone();
            }
            if let Some(ref fallback) = provider.fallback_model {
                self.llm.fallback_model = Some(fallback.clone());
            }
            if let Some(ref url) = provider.base_url {
                self.llm.base_url = url.clone();
            }
            if let Some(temp) = provider.temperature {
                self.llm.temperature = temp;
            }
            if let Some(tokens) = provider.max_tokens {
                self.llm.max_tokens = tokens;
            }
        }

        // Profile overrides (merge, don't replace)
        if let Some(ref profiles) = user.profiles {
            for (name, profile) in profiles {
                self.profiles.insert(name.clone(), profile.clone());
            }
        }

        // Agent tuning overrides
        if let Some(ref agent) = user.agent {
            if let Some(max_iter) = agent.max_iterations {
                self.agent.max_iterations = max_iter;
            }
            if let Some(timeout) = agent.timeout_seconds {
                self.agent.timeout_seconds = timeout;
            }
            if let Some(ref user_name) = agent.user_name {
                self.agent.user_name = user_name.clone();
            }
            if let Some(ref log_level) = agent.log_level {
                self.agent.log_level = log_level.clone();
            }
        }

        // MCP overrides
        if let Some(ref mcp) = user.mcp {
            if let Some(ref tool_paths) = mcp.tool_paths {
                for path in tool_paths {
                    if !self.mcp.tool_paths.contains(path) {
                        self.mcp.tool_paths.push(path.clone());
                    }
                }
            }
            if let Some(ref blocked) = mcp.blocked_commands {
                for cmd in blocked {
                    if !self.mcp.blocked_commands.contains(cmd) {
                        self.mcp.blocked_commands.push(cmd.clone());
                    }
                }
            }
            if let Some(max_output) = mcp.max_output_bytes {
                self.mcp.max_output_bytes = max_output;
            }
            if let Some(ref mcp_servers) = mcp.mcp_servers {
                for (name, srv) in mcp_servers {
                    self.mcp.mcp_servers.insert(name.clone(), srv.clone());
                }
            }
        }
    }

    /// Load from default locations (legacy — falls back to load_merged).
    pub fn load_default() -> Result<Self, ConfigError> {
        Self::load_merged()
    }

    /// Validate the loaded config for logical correctness.
    fn validate(&self) -> Result<(), ConfigError> {
        if self.llm.base_url.is_empty() {
            return Err(ConfigError::ValidationError(
                "llm.base_url cannot be empty".into(),
            ));
        }
        if self.llm.model.is_empty() {
            return Err(ConfigError::ValidationError(
                "llm.model cannot be empty".into(),
            ));
        }
        if self.sandbox.workspace_size_mb == 0 {
            return Err(ConfigError::ValidationError(
                "sandbox.workspace_size_mb must be > 0".into(),
            ));
        }
        Ok(())
    }

    /// Dynamically fetch all paths from the system `$PATH` environment variable,
    /// merge them into `tool_paths`, and ensure the sandbox mounts them.
    pub fn expand_tool_paths(&mut self) {
        if let Ok(path_var) = std::env::var("PATH") {
            for path in path_var.split(':') {
                if !path.is_empty() && !self.mcp.tool_paths.contains(&path.to_string()) {
                    let p = Path::new(path);
                    if p.exists() && p.is_dir() {
                        self.mcp.tool_paths.push(path.to_string());
                    }
                }
            }
        }

        // Ensure all discovered tool paths are mounted read-only in the sandbox
        for path in &self.mcp.tool_paths {
            if !self.sandbox.read_only_paths.contains(path) {
                self.sandbox.read_only_paths.push(path.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn make_test_config() -> AppConfig {
        let yaml = r#"
agent:
  name: "MYTH-TEST"
  version: "1.0.0"
  author: "test"
  repository_url: "http"
  max_iterations: 10
  timeout_seconds: 30
  user_name: "test"
  log_level: "info"
llm:
  provider: "nvidia-nim"
  base_url: "url"
  nvidia_nim_api_key: []
  model: "model"
  temperature: 0.1
  max_tokens: 100
  top_p: 1.0
  fallback_model: "fallback"
creator:
  name: "Shesher Hasan"
  role: "Chief Architect"
  organization: "MYTH Org"
  contact: "shesher0007@gmail.com"
  clearance_level: "OPERATIVE-LEVEL-4"
  system_license: "MYTH-PRO-UNLIMITED-2026"
mcp:
  mcp_servers: {}
  tool_paths: []
  blocked_commands: []
  max_output_bytes: 1000
sandbox:
  enabled: false
  bwrap_path: ""
  share_network: true
  new_session: true
  die_with_parent: true
  read_only_paths: []
  writable_tmpfs: []
  workspace_size_mb: 10
  hostname: ""
memory:
  enabled: false
  backend: ""
  mode: ""
  grpc_port: 0
  http_port: 0
  collection_name: ""
  vector_size: 0
  auto_start: false
  qdrant_path: ""
tui:
  enabled: false
  theme: ""
  show_tree_panel: false
  show_status_bar: false
  max_output_lines: 0
  scroll_speed: 0
  colors:
    primary: ""
    secondary: ""
    accent: ""
    background: ""
    surface: ""
    text: ""
    dim: ""
proxy:
  enabled: false
  url: null
  use_for_llm: false
  use_for_tools: false
  auto_rotate: false
profiles:
  quick:
    description: ""
    mode: "agent"
    tools: []
    max_iterations: 10
  full:
    description: ""
    mode: "agent"
    tools: []
    max_iterations: 10
  stealth:
    description: ""
    mode: "agent"
    tools: []
    max_iterations: 10
  webapp:
    description: ""
    mode: "agent"
    tools: []
    max_iterations: 10
  deep:
    description: ""
    mode: "agent"
    tools: []
    max_iterations: 10
  custom:
    description: ""
    mode: "user"
    tools: []
    max_iterations: 10
"#;
        let mut config: AppConfig = serde_yaml::from_str(yaml).expect("agent.yaml should parse");
        // Merge factory defaults for tests
        for (name, srv) in crate::builtin_mcp::get_factory_defaults() {
            config.mcp.mcp_servers.insert(name, srv);
        }
        config
    }

    #[test]
    fn test_full_config_parse_with_new_profiles() {
        let config = make_test_config();
        assert!(config.profiles.contains_key("quick"));
        assert!(config.profiles.contains_key("full"));
        assert!(config.profiles.contains_key("stealth"));
        assert!(config.profiles.contains_key("webapp"));
        assert!(config.profiles.contains_key("deep"));
        assert!(config.profiles.contains_key("custom"));
    }

    #[test]
    fn test_new_mcp_servers_found() {
        let config = make_test_config();
        assert!(config.mcp.mcp_servers.contains_key("webfetch"));
        assert!(config.mcp.mcp_servers.contains_key("llm_researcher"));
    }

    #[test]
    fn test_backward_compat_no_mode_defaults_agent() {
        let yaml = r#"
description: "Test profile"
tools: ["nmap", "dig"]
max_iterations: 50
"#;
        let profile: ReconProfile = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(profile.mode, ProfileMode::Agent);
        assert_eq!(profile.tools, vec!["nmap", "dig"]);
        assert_eq!(profile.max_iterations, 50);
    }

    #[test]
    fn test_backward_compat_no_phases_gets_defaults() {
        let yaml = r#"
description: "No phases specified"
tools: ["nmap"]
"#;
        let profile: ReconProfile = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(profile.phases.len(), 13);
        assert_eq!(profile.phases[0].name, "Phase 0: Organizational Mapping");
        assert_eq!(
            profile.phases[12].name,
            "Phase 12: Attack Surface Synthesis & Reporting"
        );
    }

    #[test]
    fn test_parse_agent_mode_profile() {
        let yaml = r#"
description: "Agent mode test"
mode: agent
tools: ["nmap", "gobuster"]
max_iterations: 100
"#;
        let profile: ReconProfile = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(profile.mode, ProfileMode::Agent);
        assert_eq!(profile.tools, vec!["nmap", "gobuster"]);
    }

    #[test]
    fn test_parse_user_mode_profile() {
        let yaml = r#"
description: "User mode test"
mode: user
max_iterations: 80
phases:
  - name: "Phase 0: Organizational Mapping"
    enabled: true
    steps:
      - name: "Root Domain Enumeration"
        tools: ["whois", "amass"]
      - name: "ASN Discovery"
        tools: ["whois"]
  - name: "Phase 1: Asset Discovery"
    enabled: false
    steps:
      - name: "DNS Enum"
        tools: ["dig"]
"#;
        let profile: ReconProfile = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(profile.mode, ProfileMode::User);
        assert_eq!(profile.phases.len(), 2);
        assert!(profile.phases[0].enabled);
        assert!(!profile.phases[1].enabled);
        assert_eq!(profile.phases[0].steps.len(), 2);
        assert_eq!(profile.phases[0].steps[0].tools, vec!["whois", "amass"]);
    }

    #[test]
    fn test_enabled_phases() {
        let yaml = r#"
description: "Phase test"
mode: user
phases:
  - name: "Phase 0"
    enabled: true
    steps: []
  - name: "Phase 1"
    enabled: false
    steps: []
  - name: "Phase 2"
    enabled: true
    steps: []
"#;
        let profile: ReconProfile = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(profile.enabled_phases(), vec![0, 2]);
    }

    #[test]
    fn test_allowed_tools_agent_mode_returns_none() {
        let yaml = r#"
description: "Agent mode"
mode: agent
tools: ["nmap"]
"#;
        let profile: ReconProfile = serde_yaml::from_str(yaml).unwrap();
        assert!(profile.allowed_tools_for_phase(0).is_none());
        assert!(profile.allowed_tools_for_phase(3).is_none());
    }

    #[test]
    fn test_allowed_tools_user_mode() {
        let yaml = r#"
description: "User mode"
mode: user
phases:
  - name: "Phase 0"
    enabled: true
    steps:
      - name: "Step A"
        tools: ["whois", "dig"]
      - name: "Step B"
        tools: ["amass"]
"#;
        let profile: ReconProfile = serde_yaml::from_str(yaml).unwrap();
        let allowed = profile.allowed_tools_for_phase(0).unwrap();
        assert!(allowed.contains(&"whois".to_string()));
        assert!(allowed.contains(&"dig".to_string()));
        assert!(allowed.contains(&"amass".to_string()));
        assert_eq!(allowed.len(), 3);
    }

    #[test]
    fn test_custom_profile_parses_correctly() {
        let config = make_test_config();
        let custom = config
            .profiles
            .get("custom")
            .expect("custom profile should exist");
        assert_eq!(custom.mode, ProfileMode::User);
        assert_eq!(custom.phases.len(), 13);
        // All phases should be enabled in the default custom template
        for phase in &custom.phases {
            assert!(phase.enabled);
        }
    }

    #[test]
    fn test_user_config_mcp_override() {
        let mut config = make_test_config();
        // Populate with factory defaults for the test
        config.mcp.mcp_servers = crate::builtin_mcp::get_factory_defaults();

        // Initial state
        assert!(config.mcp.mcp_servers.contains_key("filesystem"));

        let yaml = r#"
mcp:
  mcp_servers:
    filesystem:
      type: local
      enabled: false
      command: "custom-fs"
      args: ["--root", "/tmp"]
    new_asset:
      type: local
      enabled: true
      command: "python3"
      args: ["server.py"]
"#;
        let user: UserConfig = serde_yaml::from_str(yaml).unwrap();
        config.apply_user_config(&user);

        // Check override
        let fs = match config.mcp.mcp_servers.get("filesystem").unwrap() {
            CustomMcpServer::Local(l) => l,
            _ => panic!("Expected local server"),
        };
        assert!(!fs.enabled);
        assert_eq!(fs.command, "custom-fs");
        assert_eq!(fs.args, vec!["--root", "/tmp"]);

        // Check new asset
        let new = match config.mcp.mcp_servers.get("new_asset").unwrap() {
            CustomMcpServer::Local(l) => l,
            _ => panic!("Expected local server"),
        };
        assert!(new.enabled);
        assert_eq!(new.command, "python3");
        assert_eq!(new.args, vec!["server.py"]);
    }
}
