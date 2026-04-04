//! MCP tool schemas — JSON schema definitions for dynamic tools.

use serde::{Deserialize, Serialize};

/// Schema for the `discover_tools` MCP tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoverToolsInput {
    /// Optional category filter (e.g., "network", "web", "dns")
    #[serde(default)]
    pub category: Option<String>,

    /// Optional name search query
    #[serde(default)]
    pub query: Option<String>,
}

/// Schema for the `execute_tool` MCP tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteToolInput {
    /// The binary name or path to execute (e.g., "nmap")
    pub binary: String,

    /// Command-line arguments for local binaries (e.g., ["-sV", "-sC", "target.com"])
    #[serde(default)]
    pub args: Vec<String>,

    /// Structured arguments for external MCP tools (e.g., {"query": "SELECT..."})
    #[serde(default)]
    pub structured_args: Option<serde_json::Value>,

    /// Working directory inside sandbox (default: /workspace)
    #[serde(default)]
    pub working_dir: Option<String>,
}

/// Schema for the `generate_file` utility tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateFileInput {
    /// Relative path within the workspace (e.g., "reports/scan.md")
    pub path: String,
    /// Content to write to the file
    pub content: String,
}

/// Schema for the `append_to_file` utility tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendToFileInput {
    /// Relative path within the workspace
    pub path: String,
    /// Content to append
    pub content: String,
}

/// Schema for the `read_output` MCP tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadOutputInput {
    /// Path to the file to read (within the workspace)
    pub path: String,

    /// Maximum bytes to read (default: 1MB)
    #[serde(default = "default_max_read")]
    pub max_bytes: usize,
}

fn default_max_read() -> usize {
    1024 * 1024
}

/// Schema for the `get_tool_help` MCP tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetToolHelpInput {
    /// Name of the tool to get help for
    pub tool_name: String,
}

/// Schema for listing MCP resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResourcesInput {
    /// Optional server filter (e.g., "sqlite")
    #[serde(default)]
    pub server: Option<String>,
}

/// Schema for reading an MCP resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadResourceInput {
    /// The name of the server providing the resource (e.g., "sqlite")
    pub server: String,
    /// The URI of the resource to read (e.g., "file:///db.sqlite")
    pub uri: String,
}

/// Schema for listing MCP prompts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPromptsInput {
    /// Optional server filter
    #[serde(default)]
    pub server: Option<String>,
}

/// Schema for getting/executing an MCP prompt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPromptInput {
    /// The name of the server providing the prompt
    pub server: String,
    /// The name of the prompt to execute
    pub name: String,
    /// Arguments for the prompt template
    #[serde(default)]
    pub arguments: serde_json::Value,
}

/// Schema for the `search_memory` tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMemoryInput {
    /// The query text to search for
    pub query: String,
    /// Optional limit on results (default: 5)
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    5
}

/// Schema for the `report_phase_completion` tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportPhaseInput {
    /// The phase that was completed
    pub phase: String,
    /// Summary of what was accomplished
    pub summary: String,
    /// Next recommended steps
    pub next_steps: Vec<String>,
}

/// Schema for the `browse` tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowseInput {
    /// URL to visit
    pub url: String,
    /// Optional session name for authenticated interaction
    #[serde(default)]
    pub session_name: Option<String>,
}

/// Schema for the `web_action` tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebActionInput {
    /// Action type: click, type, screenshot, wait_for
    pub action: String,
    /// Target CSS selector (required for click/type/wait_for)
    #[serde(default)]
    pub selector: Option<String>,
    /// Text to type (required for type)
    #[serde(default)]
    pub text: Option<String>,
    /// URL for the action (required for screenshot)
    #[serde(default)]
    pub url: Option<String>,
    /// Output path for screenshot
    #[serde(default)]
    pub output_path: Option<String>,
    /// Timeout in seconds (for wait_for)
    #[serde(default)]
    pub timeout: Option<u64>,
    /// Optional session name
    #[serde(default)]
    pub session_name: Option<String>,
}

/// Schema for the `web_login` tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebLoginInput {
    /// Login page URL
    pub url: String,
    /// Username field CSS selector
    pub user_selector: String,
    /// Password field CSS selector
    pub pass_selector: String,
    /// Username value
    pub user_value: String,
    /// Password value
    pub pass_value: String,
    /// Optional session name
    #[serde(default)]
    pub session_name: Option<String>,
}

/// Schema for the `generate_secure_asset` tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateSecureAssetInput {
    pub path: String,
    #[serde(default)]
    pub content: Option<String>,
    pub key: String, // Hex-encoded 32-byte key
}

/// Schema for the `generate_batch` tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateBatchInput {
    #[serde(default)]
    pub files: Vec<(String, Option<String>)>,
}

/// Schema for the `generate_secure_batch` tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateSecureBatchInput {
    #[serde(default)]
    pub assets: Vec<(String, Option<String>, String)>, // (path, content, hex_key)
}

/// Schema for the `generate_compressed_batch` tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateCompressedBatchInput {
    #[serde(default)]
    pub files: Vec<(String, Option<String>, i32)>, // (path, content, level)
}

/// Schema for the `generate_compressed` tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateCompressedInput {
    pub path: String,
    #[serde(default)]
    pub content: Option<String>,
    pub level: i32,
}

/// Schema for the `generate_payload` tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratePayloadInput {
    pub path: String,
    pub payload_type: String,
}

/// Schema for the `generate_payload_file` tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratePayloadFileInput {
    pub format: String,
    pub payload_type: String,
}

/// Schema for the `generate_with_metadata` tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateWithMetadataInput {
    pub path: String,
    pub format: String,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,
}

/// Schema for the `get_statistics` tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStatisticsInput {}

/// Schema for the `patch_json` tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchJsonInput {
    pub path: String,
    pub patch: serde_json::Value,
}

/// Schema for the `read_mmap` tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadMmapInput {
    pub path: String,
}

/// Schema for the `subdomain_fetch` tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubdomainFetchInput {
    pub domains: Vec<String>,
    #[serde(default)]
    pub output_path: Option<String>,
    #[serde(default)]
    pub stdin: bool,
    #[serde(default)]
    pub custom_wordlists: Vec<String>,
    #[serde(default)]
    pub json: bool,
    #[serde(default)]
    pub quiet: bool,
    #[serde(default)]
    pub recursive: bool,
    #[serde(default)]
    pub active: bool,
    #[serde(default = "default_only_alive")]
    pub only_alive: bool,
    #[serde(default)]
    pub stealth: bool,
    #[serde(default)]
    pub verbose: bool,
    #[serde(default = "default_concurrency")]
    pub concurrency: u32,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
    #[serde(default = "default_retries")]
    pub retries: u32,
    #[serde(default = "default_min_delay")]
    pub min_delay_ms: u64,
    #[serde(default = "default_max_delay")]
    pub max_delay_ms: u64,
    #[serde(default)]
    pub use_proxies: bool,
    #[serde(default)]
    pub proxies_file: Option<String>,
    #[serde(default)]
    pub use_tor: bool,
    #[serde(default = "default_tor_address")]
    pub tor_address: String,
    #[serde(default)]
    pub disable_ua_rotation: bool,
    #[serde(default = "default_true")]
    pub respect_robots: bool,
    #[serde(default)]
    pub disable_captcha_avoidance: bool,
    #[serde(default)]
    pub custom_resolvers: Vec<String>,
    #[serde(default)]
    pub resolvers_file: Option<String>,
    #[serde(default)]
    pub disable_resolver_rotation: bool,
    #[serde(default)]
    pub disable_wildcard_filter: bool,
    #[serde(default = "default_depth")]
    pub depth: u8,
    #[serde(default = "default_recursive_depth")]
    pub recursive_depth: u8,
    #[serde(default = "default_max_pages")]
    pub max_pages: u32,
    #[serde(default = "default_max_crawl_depth")]
    pub max_crawl_depth: u8,
    #[serde(default)]
    pub disable_checkpoints: bool,
    #[serde(default)]
    pub checkpoint_dir: Option<String>,
    #[serde(default = "default_wordlist_type")]
    pub wordlist_type: String,
    #[serde(default)]
    pub disable_proxy_test: bool,
    #[serde(default)]
    pub master: bool,
    #[serde(default)]
    pub tor_fallback: bool,
}

fn default_only_alive() -> bool {
    true
}
fn default_concurrency() -> u32 {
    250
}
fn default_timeout() -> u64 {
    15
}
fn default_depth() -> u8 {
    3
}
fn default_recursive_depth() -> u8 {
    4
}
fn default_wordlist_type() -> String {
    "medium".to_string()
}
fn default_retries() -> u32 {
    5
}
fn default_min_delay() -> u64 {
    50
}
fn default_max_delay() -> u64 {
    2000
}
fn default_tor_address() -> String {
    "127.0.0.1:9050".to_string()
}
fn default_true() -> bool {
    true
}
fn default_max_pages() -> u32 {
    50000
}
fn default_max_crawl_depth() -> u8 {
    3
}

/// Tool listing response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolListResponse {
    pub tools: Vec<ToolEntry>,
    pub total_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolEntry {
    pub name: String,
    pub category: String,
    pub path: String,
    pub description: String,
}
