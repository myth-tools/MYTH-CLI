//! Built-in Tool Registry — decoupled management of native Rust tools.
//!
//! Provides a single source of truth for tool discovery and execution logic.

use crate::builtin_tools::utilities::file_generation::{FileGenerationConfig, FileGenerator};
use crate::builtin_tools::utilities::web::{LoginRequest, WebAutomation, WebConfig};
use crate::core::recon_graph::ReconGraph;
use crate::mcp::schemas::{
    AppendToFileInput, BrowseInput, GenerateFileInput, ReportPhaseInput, SearchMemoryInput,
    ToolEntry, WebActionInput, WebLoginInput,
};
use crate::memory::qdrant::InMemoryStore;
use color_eyre::eyre::{eyre, Result};
use serde_json::Value;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Metadata for a built-in tool.
pub struct BuiltinToolInfo {
    pub name: String,
    pub category: String,
    pub description: String,
}

pub struct BuiltinRegistry {
    workspace_root: PathBuf,
    recon_graph: Arc<Mutex<ReconGraph>>,
    memory: Arc<InMemoryStore>,
    generator: Arc<dyn crate::memory::embeddings::EmbeddingGenerator>,
    all_report_path: PathBuf,
    // Finding M-09/M-10: Cache instances for performance
    web_automation: Mutex<Option<Arc<WebAutomation>>>,
    file_generator: Mutex<Option<Arc<FileGenerator>>>,
}

impl BuiltinRegistry {
    pub fn new(
        workspace_root: PathBuf,
        recon_graph: Arc<Mutex<ReconGraph>>,
        memory: Arc<InMemoryStore>,
        generator: Arc<dyn crate::memory::embeddings::EmbeddingGenerator>,
        all_report_path: PathBuf,
    ) -> Self {
        Self {
            workspace_root,
            recon_graph,
            memory,
            generator,
            all_report_path,
            web_automation: Mutex::new(None),
            file_generator: Mutex::new(None),
        }
    }

    /// List all built-in tools for discovery.
    pub fn list_tools(&self) -> Vec<ToolEntry> {
        vec![
            ToolEntry {
                name: "generate_file".to_string(),
                category: "Utility".to_string(),
                path: "native://file_generation".to_string(),
                description: "Generate a new file (or overwrite) with precise content control. Scoped to mission workspace.".to_string(),
            },
            ToolEntry {
                name: "append_to_file".to_string(),
                category: "Utility".to_string(),
                path: "native://file_generation".to_string(),
                description: "Append content to an existing mission asset. Scoped to mission workspace.".to_string(),
            },
            ToolEntry {
                name: "search_memory".to_string(),
                category: "Memory".to_string(),
                path: "native://memory".to_string(),
                description: "Search session memory using semantic retrieval. Recall past findings and tool outputs.".to_string(),
            },
            ToolEntry {
                name: "report_phase_completion".to_string(),
                category: "Mission".to_string(),
                path: "native://mission".to_string(),
                description: "Advance to the next mission phase. Formally status findings.".to_string(),
            },
            ToolEntry {
                name: "browse".to_string(),
                category: "Web".to_string(),
                path: "native://web".to_string(),
                description: "Navigate to a URL and extract content. Supports sessions.".to_string(),
            },
            ToolEntry {
                name: "web_action".to_string(),
                category: "Web".to_string(),
                path: "native://web".to_string(),
                description: "Perform browser actions: click, type, screenshot. Supports headless automation.".to_string(),
            },
            ToolEntry {
                name: "web_login".to_string(),
                category: "Web".to_string(),
                path: "native://web".to_string(),
                description: "Automate form-based authentication to establish a session.".to_string(),
            },
            ToolEntry {
                name: "generate_secure_asset".to_string(),
                category: "Security".to_string(),
                path: "native://file_generation".to_string(),
                description: "Generate an AES-256-GCM-SIV encrypted mission asset with forensic metadata.".to_string(),
            },
            ToolEntry {
                name: "generate_batch".to_string(),
                category: "Utility".to_string(),
                path: "native://file_generation".to_string(),
                description: "Generate multiple assets in parallel using multi-core hybrid orchestration.".to_string(),
            },
            ToolEntry {
                name: "generate_secure_batch".to_string(),
                category: "Security".to_string(),
                path: "native://file_generation".to_string(),
                description: "Generate multiple encrypted assets in parallel (Sovereign Tier Scale).".to_string(),
            },
            ToolEntry {
                name: "generate_compressed_batch".to_string(),
                category: "Utility".to_string(),
                path: "native://file_generation".to_string(),
                description: "Parallel multi-threaded compression (Zstd) of mission archives.".to_string(),
            },
            ToolEntry {
                name: "patch_json".to_string(),
                category: "Utility".to_string(),
                path: "native://file_generation".to_string(),
                description: "Apply atomic RFC 6902 structural patches to JSON datasets.".to_string(),
            },
            ToolEntry {
                name: "read_mmap".to_string(),
                category: "Utility".to_string(),
                path: "native://file_generation".to_string(),
                description: "Zero-copy Memory-Mapped reading of massive (1GB+) mission assets.".to_string(),
            },
            ToolEntry {
                name: "generate_payload".to_string(),
                category: "Security".to_string(),
                path: "native://file_generation".to_string(),
                description: "Generate specialized security payloads (webshells, reverseshells) for specific targets.".to_string(),
            },
            ToolEntry {
                name: "generate_payload_file".to_string(),
                category: "Security".to_string(),
                path: "native://file_generation".to_string(),
                description: "Generate a standalone payload file with targeted formatting and headers.".to_string(),
            },
            ToolEntry {
                name: "generate_with_metadata".to_string(),
                category: "Utility".to_string(),
                path: "native://file_generation".to_string(),
                description: "Generate a file with custom mission metadata and forensic tracking.".to_string(),
            },
            ToolEntry {
                name: "generate_compressed".to_string(),
                category: "Utility".to_string(),
                path: "native://file_generation".to_string(),
                description: "High-speed Zstd compression of a single mission asset.".to_string(),
            },
            ToolEntry {
                name: "get_statistics".to_string(),
                category: "Mission".to_string(),
                path: "native://file_generation".to_string(),
                description: "Retrieve real-time telemetry on file generation performance and asset cataloging.".to_string(),
            },
            ToolEntry {
                name: "subdomain_fetch".to_string(),
                category: "Recon".to_string(),
                path: "native://subdomain_fetch".to_string(),
                description: "Elite, 18-phase subdomain discovery engine. Combines 77+ passive sources with high-speed active brute-forcing, permutations, TLS SAN extraction, and web crawling. Support for proxies, Tor, and stealth profiles.".to_string(),
            },
        ]
    }

    /// Get detailed help for a built-in tool, including formal JSON Schema for parameters.
    pub fn get_help(&self, name: &str) -> Option<String> {
        let schema = match name {
            "generate_file" => serde_json::json!({
                "tool": "generate_file",
                "description": "Generate a new file with precise content control.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path in workspace" },
                        "content": { "type": "string", "description": "Raw content to write" }
                    },
                    "required": ["path", "content"]
                }
            }),
            "append_to_file" => serde_json::json!({
                "tool": "append_to_file",
                "description": "Append content to an existing asset.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path in workspace" },
                        "content": { "type": "string", "description": "Content to append" }
                    },
                    "required": ["path", "content"]
                }
            }),
            "search_memory" => serde_json::json!({
                "tool": "search_memory",
                "description": "Semantic search across session history.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "Search query" },
                        "limit": { "type": "integer", "description": "Max results", "default": 5 }
                    },
                    "required": ["query"]
                }
            }),
            "report_phase_completion" => serde_json::json!({
                "tool": "report_phase_completion",
                "description": "Transition to the next mission phase.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "phase": { "type": "string", "description": "Completed phase number" },
                        "summary": { "type": "string", "description": "Accomplishment summary" },
                        "next_steps": { "type": "array", "items": { "type": "string" }, "description": "Strategic moves" }
                    },
                    "required": ["phase", "summary"]
                }
            }),
            "browse" => serde_json::json!({
                "tool": "browse",
                "description": "Authenticated web content extraction.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "url": { "type": "string", "description": "Target URL" },
                        "session_name": { "type": "string", "description": "Optional auth session" }
                    },
                    "required": ["url"]
                }
            }),
            "web_action" => serde_json::json!({
                "tool": "web_action",
                "description": "Advanced browser automation (click, type, screenshot, wait_for).",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "action": { "type": "string", "enum": ["click", "type", "screenshot", "wait_for"] },
                        "selector": { "type": "string", "description": "CSS selector" },
                        "text": { "type": "string", "description": "Text to type" },
                        "url": { "type": "string", "description": "URL for screenshot" },
                        "output_path": { "type": "string", "description": "Local screenshot path" },
                        "timeout": { "type": "integer", "description": "Wait timeout (secs)", "default": 30 },
                        "session_name": { "type": "string", "description": "Auth session" }
                    },
                    "required": ["action"]
                }
            }),
            "web_login" => serde_json::json!({
                "tool": "web_login",
                "description": "Automated form-based authentication.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "url": { "type": "string", "description": "Login page URL" },
                        "user_selector": { "type": "string", "description": "Username selector" },
                        "pass_selector": { "type": "string", "description": "Password selector" },
                        "user_value": { "type": "string", "description": "Username" },
                        "pass_value": { "type": "string", "description": "Password" },
                        "session_name": { "type": "string", "description": "Target session ID" }
                    },
                    "required": ["url", "user_selector", "pass_selector", "user_value", "pass_value"]
                }
            }),
            "generate_payload" => serde_json::json!({
                "tool": "generate_payload",
                "description": "Generate specialized security payloads.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path in workspace" },
                        "payload_type": { "type": "string", "description": "Type of payload (e.g., webshell, reverseshell)" }
                    },
                    "required": ["path", "payload_type"]
                }
            }),
            "generate_payload_file" => serde_json::json!({
                "tool": "generate_payload_file",
                "description": "Generate a standalone payload file.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "format": { "type": "string", "description": "File format (e.g., php, py, exe)" },
                        "payload_type": { "type": "string", "description": "Type of payload" }
                    },
                    "required": ["format", "payload_type"]
                }
            }),
            "generate_with_metadata" => serde_json::json!({
                "tool": "generate_with_metadata",
                "description": "Generate a file with custom mission metadata.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path in workspace" },
                        "format": { "type": "string", "description": "Target format" },
                        "content": { "type": "string", "description": "Optional raw content" },
                        "metadata": { "type": "object", "description": "Custom key-value pairs" }
                    },
                    "required": ["path", "format", "metadata"]
                }
            }),
            "generate_compressed" => serde_json::json!({
                "tool": "generate_compressed",
                "description": "High-speed Zstd compression of a single mission asset.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Target path" },
                        "content": { "type": "string", "description": "Raw content to compress" },
                        "level": { "type": "integer", "description": "Compression level (1-22)", "default": 3 }
                    },
                    "required": ["path", "content"]
                }
            }),
            "generate_compressed_batch" => serde_json::json!({
                "tool": "generate_compressed_batch",
                "description": "Parallel multi-threaded compression (Zstd) of mission archives.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "files": {
                            "type": "array",
                            "items": {
                                "type": "array",
                                "items": [
                                    { "type": "string", "description": "Path" },
                                    { "type": "string", "description": "Optional Content" },
                                    { "type": "integer", "description": "Level" }
                                ]
                            }
                        }
                    },
                    "required": ["files"]
                }
            }),
            "get_statistics" => serde_json::json!({
                "tool": "get_statistics",
                "description": "Retrieve real-time telemetry.",
                "parameters": { "type": "object", "properties": {} }
            }),
            "generate_secure_asset" => serde_json::json!({
                "tool": "generate_secure_asset",
                "description": "Generate an AES-256-GCM-SIV encrypted mission asset.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": { "type": "string" },
                        "content": { "type": "string" },
                        "key": { "type": "string", "description": "Hex-encoded 32-byte key" }
                    },
                    "required": ["path", "key"]
                }
            }),
            "generate_batch" => serde_json::json!({
                "tool": "generate_batch",
                "description": "Generate multiple assets in parallel.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "files": { "type": "array", "items": { "type": "array", "items": [{ "type": "string" }, { "type": "string" }] } }
                    },
                    "required": ["files"]
                }
            }),
            "generate_secure_batch" => serde_json::json!({
                "tool": "generate_secure_batch",
                "description": "Generate multiple encrypted assets in parallel.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "assets": { "type": "array", "items": { "type": "array", "items": [{ "type": "string" }, { "type": "string" }, { "type": "string" }] } }
                    },
                    "required": ["assets"]
                }
            }),
            "patch_json" => serde_json::json!({
                "tool": "patch_json",
                "description": "Apply atomic RFC 6902 structural patches.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": { "type": "string" },
                        "patch": { "type": "object" }
                    },
                    "required": ["path", "patch"]
                }
            }),
            "read_mmap" => serde_json::json!({
                "tool": "read_mmap",
                "description": "Zero-copy Memory-Mapped reading of massive assets.",
                "parameters": {
                    "type": "object",
                    "properties": { "path": { "type": "string" } },
                    "required": ["path"]
                }
            }),
            "subdomain_fetch" => serde_json::json!({
                "tool": "subdomain_fetch",
                "description": "Discover all subdomains for a given target domain using an elite, multi-phase pipeline.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "domain": { "type": "string", "description": "Target domain (e.g., example.com)" },
                        "custom_wordlists": { "type": "array", "items": { "type": "string" }, "description": "Custom wordlist files" },
                        "json": { "type": "boolean", "description": "Output results in JSONL format", "default": false },
                        "quiet": { "type": "boolean", "description": "Suppress all progress/stats output", "default": false },
                        "recursive": { "type": "boolean", "description": "Enable recursive discovery (scans sub-subdomains)", "default": false },
                        "active": { "type": "boolean", "description": "Enable active brute-force and permutation scanning", "default": false },
                        "only_alive": { "type": "boolean", "description": "Only return subdomains that resolve to IPs", "default": true },
                        "stealth": { "type": "boolean", "description": "Stealth mode: reduces concurrency and adds randomized delays to evade detection", "default": false },
                        "concurrency": { "type": "integer", "description": "Maximum concurrent discovery tasks", "default": 50 },
                        "timeout": { "type": "integer", "description": "Global timeout in seconds", "default": 3600 },
                        "retries": { "type": "integer", "description": "Number of retries on failure", "default": 5 },
                        "min_delay_ms": { "type": "integer", "description": "Minimum delay between requests", "default": 50 },
                        "max_delay_ms": { "type": "integer", "description": "Maximum delay between requests", "default": 2000 },
                        "use_proxies": { "type": "boolean", "description": "Use rotating proxies for all discovery phases", "default": false },
                        "proxies_file": { "type": "string", "description": "Use a specific list of proxies from file" },
                        "use_tor": { "type": "boolean", "description": "Route all discovery traffic through the Tor network", "default": false },
                        "tor_address": { "type": "string", "description": "Custom Tor SOCKS5 address", "default": "127.0.0.1:9050" },
                        "disable_ua_rotation": { "type": "boolean", "description": "Disable User-Agent rotation (120+ agents)", "default": false },
                        "respect_robots": { "type": "boolean", "description": "Follow robots.txt exclusion rules", "default": true },
                        "disable_captcha_avoidance": { "type": "boolean", "description": "Disable built-in CAPTCHA detection/bypass", "default": false },
                        "custom_resolvers": { "type": "array", "items": { "type": "string" }, "description": "Comma-separated list of DNS servers" },
                        "resolvers_file": { "type": "string", "description": "Load DNS servers from a specific file" },
                        "disable_resolver_rotation": { "type": "boolean", "description": "Disable rotation of DNS servers", "default": false },
                        "disable_wildcard_filter": { "type": "boolean", "description": "Disable intelligent wildcard DNS detection", "default": false },
                        "depth": { "type": "integer", "description": "Initial depth of subdomain gathering (1-2 recommended)", "default": 1 },
                        "recursive_depth": { "type": "integer", "description": "Maximum depth for recursive discovery (up to 5)", "default": 3 },
                        "max_pages": { "type": "integer", "description": "Max pages to crawl during web scraping", "default": 50000 },
                        "max_crawl_depth": { "type": "integer", "description": "Maximum depth for web crawler", "default": 3 },
                        "disable_checkpoints": { "type": "boolean", "description": "Disable session saving and resumption", "default": false },
                        "checkpoint_dir": { "type": "string", "description": "Directory to store session checkpoints", "default": ".subdomain_fetch_checkpoints" },
                        "wordlist_type": { "type": "string", "enum": ["none", "small", "medium", "large", "quick", "deep", "mega"], "description": "Built-in wordlist size for brute-forcing", "default": "medium" },
                        "disable_proxy_test": { "type": "boolean", "description": "Disables testing proxies for connectivity before use", "default": false }
                    },
                    "required": ["domain"]
                }
            }),
            _ => return None,
        };

        Some(serde_json::to_string_pretty(&schema).unwrap_or_default())
    }

    /// Execute a built-in tool by name.
    pub async fn execute(&self, name: &str, structured_args: Option<Value>) -> Result<Value> {
        match name {
            "generate_file" => {
                let input: GenerateFileInput = serde_json::from_value(
                    structured_args.clone().unwrap_or_default(),
                )
                .map_err(|e| {
                    eyre!(
                        "Invalid arguments for generate_file. Received: {:?}, Error: {}",
                        structured_args,
                        e
                    )
                })?;

                match self
                    .get_file_generator()
                    .await
                    .generate_file(&input.path, Some(input.content.as_bytes()))
                    .await
                {
                    Ok(meta) => Ok(serde_json::json!({
                        "success": true,
                        "message": format!("Generated {} (SHA256: {}...)", input.path, &meta.hash[..8]),
                        "metadata": meta
                    })),
                    Err(e) => Ok(serde_json::json!({ "success": false, "error": e.to_string() })),
                }
            }
            "append_to_file" => {
                let input: AppendToFileInput = serde_json::from_value(
                    structured_args.clone().unwrap_or_default(),
                )
                .map_err(|e| {
                    eyre!(
                        "Invalid arguments for append_to_file. Received: {:?}, Error: {}",
                        structured_args,
                        e
                    )
                })?;

                match self
                    .get_file_generator()
                    .await
                    .append_to_file(&input.path, &input.content)
                    .await
                {
                    Ok(msg) => Ok(serde_json::json!({ "success": true, "message": msg })),
                    Err(e) => Ok(serde_json::json!({ "success": false, "error": e.to_string() })),
                }
            }
            "search_memory" => {
                let input: SearchMemoryInput = match serde_json::from_value(
                    structured_args.clone().unwrap_or_default(),
                ) {
                    Ok(i) => i,
                    Err(e) => {
                        return Ok(
                            serde_json::json!({ "success": false, "error": format!("Invalid arguments for search_memory: {}", e) }),
                        )
                    }
                };

                let vector = self.generator.generate(&input.query).await;
                let mem = self.memory.clone();
                match mem
                    .search(Some(&vector), Some(&input.query), input.limit)
                    .await
                {
                    Ok(results) => Ok(serde_json::json!({ "success": true, "results": results })),
                    Err(e) => Ok(serde_json::json!({ "success": false, "error": e.to_string() })),
                }
            }
            "report_phase_completion" => {
                let input: ReportPhaseInput = serde_json::from_value(
                    structured_args.clone().unwrap_or_default(),
                )
                .map_err(|e| {
                    eyre!(
                        "Invalid arguments for report_phase_completion. Received: {:?}, Error: {}",
                        structured_args,
                        e
                    )
                })?;

                let mut graph = self.recon_graph.lock().await;
                // Parse phase as u8 if it was passed as string in JSON
                let phase_num = input.phase.parse::<u8>().unwrap_or(0);
                graph.advance_phase(phase_num, input.summary);

                Ok(serde_json::json!({ "success": true, "new_phase": phase_num + 1 }))
            }
            "browse" => {
                let input: BrowseInput = serde_json::from_value(
                    structured_args.clone().unwrap_or_default(),
                )
                .map_err(|e| {
                    eyre!(
                        "Invalid arguments for browse. Received: {:?}, Error: {}",
                        structured_args,
                        e
                    )
                })?;

                let web = {
                    let mut lock = self.web_automation.lock().await;
                    if lock.is_none() {
                        *lock = Some(Arc::new(WebAutomation::new(WebConfig::default()).await?));
                    }
                    lock.as_ref().unwrap().clone()
                };
                match web.get(&input.url, input.session_name.as_deref()).await {
                    Ok(content) => Ok(serde_json::json!({ "success": true, "content": content })),
                    Err(e) => Ok(serde_json::json!({ "success": false, "error": e.to_string() })),
                }
            }
            "web_action" => {
                let input: WebActionInput =
                    serde_json::from_value(structured_args.unwrap_or_default())
                        .map_err(|e| eyre!("Invalid arguments for web_action: {}", e))?;

                let web = {
                    let mut lock = self.web_automation.lock().await;
                    if lock.is_none() {
                        *lock = Some(Arc::new(WebAutomation::new(WebConfig::default()).await?));
                    }
                    lock.as_ref().unwrap().clone()
                };
                match input.action.as_str() {
                    "click" => {
                        let selector = input
                            .selector
                            .ok_or_else(|| eyre!("click action requires selector"))?;
                        web.click_element(&selector, input.session_name.as_deref())
                            .await?;
                        Ok(serde_json::json!({ "success": true, "message": "Element clicked" }))
                    }
                    "type" => {
                        let selector = input
                            .selector
                            .ok_or_else(|| eyre!("type action requires selector"))?;
                        let text = input
                            .text
                            .ok_or_else(|| eyre!("type action requires text"))?;
                        web.type_text(&selector, &text, input.session_name.as_deref())
                            .await?;
                        Ok(serde_json::json!({ "success": true, "message": "Text typed" }))
                    }
                    "screenshot" => {
                        let url = input
                            .url
                            .ok_or_else(|| eyre!("screenshot action requires url"))?;
                        let path = input
                            .output_path
                            .ok_or_else(|| eyre!("screenshot action requires output_path"))?;
                        web.screenshot(&url, &path, true).await?;
                        Ok(
                            serde_json::json!({ "success": true, "message": format!("Screenshot saved to {}", path) }),
                        )
                    }
                    "wait_for" => {
                        let selector = input
                            .selector
                            .ok_or_else(|| eyre!("wait_for action requires selector"))?;
                        let timeout = input.timeout.unwrap_or(30);
                        web.wait_for_selector(&selector, timeout).await?;
                        Ok(
                            serde_json::json!({ "success": true, "message": format!("Element '{}' appeared", selector) }),
                        )
                    }
                    _ => Err(eyre!("Unsupported web action: {}", input.action)),
                }
            }
            "web_login" => {
                let input: WebLoginInput =
                    serde_json::from_value(structured_args.unwrap_or_default())
                        .map_err(|e| eyre!("Invalid arguments for web_login: {}", e))?;

                let web = {
                    let mut lock = self.web_automation.lock().await;
                    if lock.is_none() {
                        *lock = Some(Arc::new(WebAutomation::new(WebConfig::default()).await?));
                    }
                    lock.as_ref().unwrap().clone()
                };
                match web
                    .login_form(LoginRequest {
                        url: &input.url,
                        user_field: &input.user_selector,
                        pass_field: &input.pass_selector,
                        username: &input.user_value,
                        password: &input.pass_value,
                        session_name: input.session_name.as_deref(),
                        additional_data: None,
                    })
                    .await
                {
                    Ok(_) => {
                        Ok(serde_json::json!({ "success": true, "message": "Login successful" }))
                    }
                    Err(e) => Ok(serde_json::json!({ "success": false, "error": e.to_string() })),
                }
            }
            "generate_secure_asset" => {
                let input: crate::mcp::schemas::GenerateSecureAssetInput =
                    serde_json::from_value(structured_args.unwrap_or_default())?;
                let key_bytes =
                    hex::decode(&input.key).map_err(|e| eyre!("Invalid hex key: {}", e))?;
                if key_bytes.len() != 32 {
                    return Ok(serde_json::json!({
                        "success": false,
                        "error": format!("Key must be exactly 32 bytes (64 hex chars), got {} bytes", key_bytes.len())
                    }));
                }
                let mut key = [0u8; 32];
                key.copy_from_slice(&key_bytes);

                match self
                    .get_file_generator()
                    .await
                    .generate_secure_asset(
                        &input.path,
                        input.content.as_ref().map(|c| c.as_bytes()),
                        key,
                    )
                    .await
                {
                    Ok(meta) => Ok(serde_json::json!({ "success": true, "metadata": meta })),
                    Err(e) => Ok(serde_json::json!({ "success": false, "error": e.to_string() })),
                }
            }
            "generate_batch" => {
                let input: crate::mcp::schemas::GenerateBatchInput = match serde_json::from_value(
                    structured_args.unwrap_or_default(),
                ) {
                    Ok(i) => i,
                    Err(e) => {
                        return Ok(
                            serde_json::json!({ "success": false, "error": format!("Invalid arguments for generate_batch: {}", e) }),
                        )
                    }
                };
                let files: Vec<(String, Option<Vec<u8>>)> = input
                    .files
                    .into_iter()
                    .map(|(p, c)| (p, c.map(|s| s.into_bytes())))
                    .collect();
                let results = self.get_file_generator().await.generate_batch(files).await;
                Ok(serde_json::json!({ "success": true, "results": results }))
            }
            "generate_secure_batch" => {
                let input: crate::mcp::schemas::GenerateSecureBatchInput =
                    match serde_json::from_value(structured_args.unwrap_or_default()) {
                        Ok(i) => i,
                        Err(e) => {
                            return Ok(
                                serde_json::json!({ "success": false, "error": format!("Invalid arguments for generate_secure_batch: {}", e) }),
                            )
                        }
                    };
                let mut assets: Vec<(String, Option<Vec<u8>>, [u8; 32])> =
                    Vec::with_capacity(input.assets.len());
                for (p, c, k) in input.assets {
                    let k_bytes = match hex::decode(&k) {
                        Ok(b) => b,
                        Err(e) => {
                            return Ok(
                                serde_json::json!({ "success": false, "error": format!("Invalid hex key for '{}': {}", p, e) }),
                            )
                        }
                    };
                    if k_bytes.len() != 32 {
                        return Ok(serde_json::json!({
                            "success": false,
                            "error": format!("Key for '{}' must be exactly 32 bytes (64 hex chars), got {} bytes", p, k_bytes.len())
                        }));
                    }
                    let mut key = [0u8; 32];
                    key.copy_from_slice(&k_bytes);
                    assets.push((p, c.map(|s| s.into_bytes()), key));
                }
                let results = self
                    .get_file_generator()
                    .await
                    .generate_secure_batch(assets)
                    .await;
                Ok(serde_json::json!({ "success": true, "results": results }))
            }
            "patch_json" => {
                let input: crate::mcp::schemas::PatchJsonInput = match serde_json::from_value(
                    structured_args.unwrap_or_default(),
                ) {
                    Ok(i) => i,
                    Err(e) => {
                        return Ok(
                            serde_json::json!({ "success": false, "error": format!("Invalid arguments for patch_json: {}", e) }),
                        )
                    }
                };
                match self
                    .get_file_generator()
                    .await
                    .patch_json(&input.path, input.patch)
                    .await
                {
                    Ok(meta) => Ok(serde_json::json!({ "success": true, "metadata": meta })),
                    Err(e) => Ok(serde_json::json!({ "success": false, "error": e.to_string() })),
                }
            }
            "generate_payload" => {
                let input: crate::mcp::schemas::GeneratePayloadInput = match serde_json::from_value(
                    structured_args.unwrap_or_default(),
                ) {
                    Ok(i) => i,
                    Err(e) => {
                        return Ok(
                            serde_json::json!({ "success": false, "error": format!("Invalid arguments for generate_payload: {}", e) }),
                        )
                    }
                };
                match self
                    .get_file_generator()
                    .await
                    .generate_payload(&input.path, &input.payload_type)
                    .await
                {
                    Ok(meta) => Ok(serde_json::json!({ "success": true, "metadata": meta })),
                    Err(e) => Ok(serde_json::json!({ "success": false, "error": e.to_string() })),
                }
            }
            "generate_payload_file" => {
                let input: crate::mcp::schemas::GeneratePayloadFileInput =
                    match serde_json::from_value(structured_args.unwrap_or_default()) {
                        Ok(i) => i,
                        Err(e) => {
                            return Ok(
                                serde_json::json!({ "success": false, "error": format!("Invalid arguments for generate_payload_file: {}", e) }),
                            )
                        }
                    };
                match self
                    .get_file_generator()
                    .await
                    .generate_payload_file(&input.format, &input.payload_type)
                    .await
                {
                    Ok(meta) => Ok(serde_json::json!({ "success": true, "metadata": meta })),
                    Err(e) => Ok(serde_json::json!({ "success": false, "error": e.to_string() })),
                }
            }
            "generate_with_metadata" => {
                let input: crate::mcp::schemas::GenerateWithMetadataInput =
                    match serde_json::from_value(structured_args.unwrap_or_default()) {
                        Ok(i) => i,
                        Err(e) => {
                            return Ok(
                                serde_json::json!({ "success": false, "error": format!("Invalid arguments for generate_with_metadata: {}", e) }),
                            )
                        }
                    };
                match self
                    .get_file_generator()
                    .await
                    .generate_with_metadata(
                        &input.path,
                        &input.format,
                        input.content.as_ref().map(|c| c.as_bytes()),
                        input.metadata,
                    )
                    .await
                {
                    Ok(meta) => Ok(serde_json::json!({ "success": true, "metadata": meta })),
                    Err(e) => Ok(serde_json::json!({ "success": false, "error": e.to_string() })),
                }
            }
            "generate_compressed" => {
                let input: crate::mcp::schemas::GenerateCompressedInput =
                    match serde_json::from_value(structured_args.unwrap_or_default()) {
                        Ok(i) => i,
                        Err(e) => {
                            return Ok(
                                serde_json::json!({ "success": false, "error": format!("Invalid arguments for generate_compressed: {}", e) }),
                            )
                        }
                    };
                match self
                    .get_file_generator()
                    .await
                    .generate_compressed(
                        &input.path,
                        input.content.as_ref().map(|c| c.as_bytes()),
                        input.level,
                    )
                    .await
                {
                    Ok(meta) => Ok(serde_json::json!({ "success": true, "metadata": meta })),
                    Err(e) => Ok(serde_json::json!({ "success": false, "error": e.to_string() })),
                }
            }
            "generate_compressed_batch" => {
                let input: crate::mcp::schemas::GenerateCompressedBatchInput =
                    match serde_json::from_value(structured_args.unwrap_or_default()) {
                        Ok(i) => i,
                        Err(e) => {
                            return Ok(
                                serde_json::json!({ "success": false, "error": format!("Invalid arguments for generate_compressed_batch: {}", e) }),
                            )
                        }
                    };
                let files: Vec<(String, Option<Vec<u8>>, i32)> = input
                    .files
                    .into_iter()
                    .map(|(p, c, l)| (p, c.map(|s| s.into_bytes()), l))
                    .collect();
                let results = self
                    .get_file_generator()
                    .await
                    .generate_compressed_batch(files)
                    .await;
                Ok(serde_json::json!({ "success": true, "results": results }))
            }
            "get_statistics" => {
                let stats = self.get_file_generator().await.get_statistics().await;
                Ok(serde_json::json!({ "success": true, "statistics": stats }))
            }
            "read_mmap" => {
                let input: crate::mcp::schemas::ReadMmapInput =
                    serde_json::from_value(structured_args.unwrap_or_default())?;
                match self.get_file_generator().await.read_mmap(&input.path).await {
                    Ok(data) => {
                        let len = data.len();
                        let preview_len = len.min(100);
                        Ok(serde_json::json!({
                            "success": true,
                            "size": len,
                            "preview": String::from_utf8_lossy(&data[..preview_len]).to_string()
                        }))
                    }
                    Err(e) => Ok(serde_json::json!({ "success": false, "error": e.to_string() })),
                }
            }
            "subdomain_fetch" => {
                let input: crate::mcp::schemas::SubdomainFetchInput =
                    serde_json::from_value(structured_args.unwrap_or_default())
                        .map_err(|e| eyre!("Invalid arguments for subdomain_fetch: {}", e))?;

                // Import and execute the tool
                use crate::builtin_tools::recon::subdomain_fetch;
                // Handle the `active` flag by explicitly adding the builtin wordlist if requested
                // and if no other specific wordlist type is selected. (medium is the default if active).
                let mut wordlist_sources = input.custom_wordlists.clone();
                if input.active
                    && input.wordlist_type != "quick"
                    && input.wordlist_type != "deep"
                    && input.wordlist_type != "mega"
                    && input.wordlist_type != "none"
                    && !wordlist_sources.contains(&"builtin".to_string())
                {
                    wordlist_sources.push("builtin".to_string());
                }

                let config = subdomain_fetch::Config {
                    domains: input.domains.clone(),
                    wordlist_sources,
                    json_output: input.json,
                    quiet: input.quiet,
                    recursive: input.recursive,
                    only_alive: input.only_alive,
                    stealth_mode: input.stealth,
                    concurrency: input.concurrency as usize,
                    timeout: std::time::Duration::from_secs(input.timeout),
                    retries: input.retries,
                    min_delay_ms: input.min_delay_ms,
                    max_delay_ms: input.max_delay_ms,
                    use_proxies: input.use_proxies,
                    proxies_file: input.proxies_file,
                    use_tor: input.use_tor,
                    tor_address: input.tor_address.clone(),
                    rotate_user_agents: !input.disable_ua_rotation,
                    respect_robots: input.respect_robots,
                    captcha_avoidance: !input.disable_captcha_avoidance,
                    resolvers: if input.custom_resolvers.is_empty() {
                        subdomain_fetch::Config::default().resolvers
                    } else {
                        input.custom_resolvers.clone()
                    },
                    resolvers_file: input.resolvers_file.clone(),
                    rotate_resolvers: !input.disable_resolver_rotation,
                    no_wildcard_filter: input.disable_wildcard_filter,
                    depth: input.depth as usize,
                    recursive_depth: input.recursive_depth as usize,
                    max_pages_per_domain: input.max_pages as usize,
                    max_depth: input.max_crawl_depth as u32,
                    checkpoints: !input.disable_checkpoints,
                    checkpoint_dir: input.checkpoint_dir.clone().map(std::path::PathBuf::from),
                    use_quick_list: input.wordlist_type == "quick",
                    use_deep_list: input.wordlist_type == "deep",
                    use_mega_list: input.wordlist_type == "mega",
                    proxy_test: !input.disable_proxy_test,
                    base_report_dir: Some(self.all_report_path.clone()),
                    stdin: input.stdin,
                    output_path: input.output_path.clone(),
                    verbose: input.verbose,
                    master_mode: input.master,
                    tor_fallback: input.tor_fallback,
                    ..subdomain_fetch::Config::default()
                };

                match subdomain_fetch::run_fetch(config).await {
                    Ok(results) => Ok(serde_json::json!({ "success": true, "results": results })),
                    Err(e) => {
                        let err_msg = e.to_string();
                        Ok(serde_json::json!({ "success": false, "error": err_msg }))
                    }
                }
            }
            _ => Ok(
                serde_json::json!({ "success": false, "error": format!("Tool '{}' not found in built-in registry", name) }),
            ),
        }
    }

    async fn get_file_generator(&self) -> Arc<FileGenerator> {
        let mut lock = self.file_generator.lock().await;
        if lock.is_none() {
            *lock = Some(Arc::new(FileGenerator::new(
                self.workspace_root.clone(),
                Some(self.all_report_path.clone()),
                FileGenerationConfig::default(),
            )));
        }
        lock.as_ref().unwrap().clone()
    }
}
