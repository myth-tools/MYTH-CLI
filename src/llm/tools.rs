//! Rig tool definitions — bridges MCP tools to Rig's tool-calling interface.

use crate::mcp::discover::ToolDiscovery;
use crate::mcp::execute::ToolExecutor;
use owo_colors::OwoColorize;
use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::Write;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{Mutex, RwLock};

use crate::tui::app::TuiEvent;
use tokio::sync::mpsc;
use url::Url;

#[derive(Debug, Error)]
#[error("{0}")]
pub struct ToolError(pub String);

// ─── Execute Tool Definition ───

/// Rig tool: Execute a security tool inside the sandbox.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ExecuteToolArgs {
    /// The binary name to execute (e.g., "nmap")
    pub binary: String,
    /// Command-line arguments for local binaries (e.g., ["-sV", "-sC", "target.com"])
    pub args: Vec<String>,
    /// Structured arguments for external MCP tools (e.g., {"query": "SELECT..."})
    pub structured_args: Option<serde_json::Value>,
}

/// Holds the executor and discovery registry for the Rig tool bridge.
pub struct ExecuteToolBridge {
    executor: Arc<ToolExecutor>,
    discovery: Arc<RwLock<ToolDiscovery>>,
    clients: Arc<RwLock<crate::mcp::client::McpClientManager>>,
    executed_tools: Arc<Mutex<Vec<(String, String)>>>,
    missing_tools: Arc<Mutex<HashSet<String>>>,
    workspace_path: std::path::PathBuf,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    builtin_registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
    rate_limit_ms: u64,
    memory: Option<Arc<crate::memory::qdrant::InMemoryStore>>,
    generator: Option<Arc<dyn crate::memory::embeddings::EmbeddingGenerator>>,
}

/// Consolidated context for tool bridges to resolve clippy::too_many_arguments
pub struct BridgeContext {
    pub executor: Arc<ToolExecutor>,
    pub discovery: Arc<RwLock<ToolDiscovery>>,
    pub clients: Arc<RwLock<crate::mcp::client::McpClientManager>>,
    pub executed_tools: Arc<Mutex<Vec<(String, String)>>>,
    pub missing_tools: Arc<Mutex<HashSet<String>>>,
    pub workspace_path: std::path::PathBuf,
    pub event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    pub builtin_registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
    pub rate_limit_ms: u64,
}

impl ExecuteToolBridge {
    pub fn new(ctx: BridgeContext) -> Self {
        Self {
            executor: ctx.executor,
            discovery: ctx.discovery,
            clients: ctx.clients,
            executed_tools: ctx.executed_tools,
            missing_tools: ctx.missing_tools,
            workspace_path: ctx.workspace_path,
            event_tx: ctx.event_tx,
            builtin_registry: ctx.builtin_registry,
            rate_limit_ms: ctx.rate_limit_ms,
            memory: None,
            generator: None,
        }
    }

    pub fn with_memory(
        mut self,
        memory: Arc<crate::memory::qdrant::InMemoryStore>,
        generator: Arc<dyn crate::memory::embeddings::EmbeddingGenerator>,
    ) -> Self {
        self.memory = Some(memory);
        self.generator = Some(generator);
        self
    }

    async fn emit_web_sources(&self, res: &serde_json::Value) {
        let mut sources = Vec::new();
        self.extract_urls(res, &mut sources);

        let mut seen_domains = std::collections::HashSet::new();
        for url_str in sources {
            if let Ok(parsed) = Url::parse(&url_str) {
                if let Some(domain) = parsed.domain() {
                    if seen_domains.insert(domain.to_string()) {
                        if let Some(ref tx) = self.event_tx {
                            let _ = tx.send(TuiEvent::WebSourceFound {
                                source: domain.to_string(),
                            });
                        } else {
                            println!(
                                "    {} Source identified: {}",
                                "📍".bright_blue(),
                                domain.bright_black().italic()
                            );
                        }
                    }
                }
            }
        }
    }

    fn extract_urls(&self, value: &serde_json::Value, urls: &mut Vec<String>) {
        match value {
            serde_json::Value::Object(map) => {
                for (k, v) in map {
                    if k == "url" || k == "link" || k == "href" || k == "source" {
                        if let Some(s) = v.as_str() {
                            urls.push(s.to_string());
                        }
                    }
                    self.extract_urls(v, urls);
                }
            }
            serde_json::Value::Array(arr) => {
                for v in arr {
                    self.extract_urls(v, urls);
                }
            }
            _ => {
                if let Some(s) = value.as_str() {
                    if s.starts_with("http") {
                        urls.push(s.to_string());
                    }
                }
            }
        }
    }
}

impl Tool for ExecuteToolBridge {
    const NAME: &'static str = "execute_tool";
    type Error = ToolError;
    type Args = ExecuteToolArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Execute a tool inside the sandbox. MUST use discovering tools first to find paths. CRITICAL: Always use verbose flags (e.g., -v, -vv, --verbose, -d) when executing tools. The live UI depends on verbose streaming to show progress to the human operator. Never run a tool silently if verbose is available.".to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(ExecuteToolArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // H-01: Global LLM Tool Call Rate Limiting
        if self.rate_limit_ms > 0 {
            tokio::time::sleep(std::time::Duration::from_millis(self.rate_limit_ms)).await;
        }

        let args_joined = args.args.join(" ");
        let parsed_args: Vec<&str> = args.args.iter().map(|s| s.as_str()).collect();

        // Detect tool type for enhanced telemetry
        let (tool_type_tag, tool_type_color) = if args.binary.contains('/') {
            let srv = args.binary.split('/').next().unwrap_or("");
            (format!("MCP:{}", srv), "magenta")
        } else if self
            .builtin_registry
            .list_tools()
            .iter()
            .any(|t| t.name == args.binary)
        {
            ("NATIVE".to_string(), "cyan")
        } else {
            ("EXEC".to_string(), "yellow")
        };

        // Intelligence Detection (Web Search)
        let srv_name = if args.binary.contains('/') {
            args.binary.split('/').next().unwrap_or("")
        } else {
            ""
        };
        let is_search = args.binary.contains("search")
            || args.binary.contains("research")
            || args.binary.contains("query")
            || args.binary.contains("fetch")
            || args.binary.contains("web")
            || srv_name.contains("search")
            || srv_name.contains("exa")
            || srv_name.contains("tavily");

        // Smart MCP Argument Resolution
        let resolve_mcp_args = |bin: &str,
                                raw_args: &[String],
                                structured: Option<serde_json::Value>|
         -> serde_json::Value {
            if let Some(s) = structured {
                return s;
            }
            if raw_args.is_empty() {
                return serde_json::Value::Object(serde_json::Map::new());
            }

            let name = if bin.contains('/') {
                bin.split('/').next_back().unwrap_or(bin)
            } else {
                bin
            };
            let val = raw_args.first().cloned().unwrap_or_default();

            if name.contains("search") || name.contains("research") || name.contains("query") {
                serde_json::json!({"query": val})
            } else if name.contains("fetch") || name.contains("browse") || name.contains("read") {
                serde_json::json!({"url": val})
            } else {
                serde_json::to_value(raw_args).unwrap_or(serde_json::Value::Array(vec![]))
            }
        };

        let search_query = if is_search {
            if let Some(structured) = &args.structured_args {
                structured
                    .get("query")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .or_else(|| structured.as_str().map(|s| s.to_string()))
            } else {
                args.args.first().cloned()
            }
        } else {
            None
        };

        let (server_emit, tool_emit) = if args.binary.contains('/') {
            let mut parts = args.binary.splitn(2, '/');
            (
                parts.next().unwrap_or("UNKNOWN").to_string(),
                parts.next().unwrap_or("").to_string(),
            )
        } else if self
            .builtin_registry
            .list_tools()
            .iter()
            .any(|t| t.name == args.binary)
        {
            ("NATIVE".to_string(), args.binary.clone())
        } else {
            ("LOCAL".to_string(), args.binary.clone())
        };

        // Dual-Mode Telemetry
        if let Some(ref tx) = self.event_tx {
            if is_search {
                if let Some(ref q) = search_query {
                    let _ = tx.send(TuiEvent::WebSearchStarted {
                        server: if srv_name.is_empty() {
                            args.binary.clone()
                        } else {
                            srv_name.to_string()
                        },
                        query: q.clone(),
                    });
                }
            }

            let _ = tx.send(TuiEvent::ToolStarted {
                server: server_emit,
                tool: tool_emit,
                args: args_joined.clone(),
            });
        } else {
            // CLI Premium Display with tool type indicator
            if is_search {
                if let Some(query) = search_query.as_ref() {
                    println!(
                        "\r\x1b[K  {} {} {} {}",
                        "🌐".bright_blue(),
                        "INTEL".on_bright_blue().white().bold(),
                        "Searched web for =>".bright_black(),
                        query.bright_cyan().bold().italic()
                    );
                }
            } else {
                let tag_display = match tool_type_color {
                    "magenta" => tool_type_tag.magenta().bold().to_string(),
                    "cyan" => tool_type_tag.cyan().bold().to_string(),
                    _ => tool_type_tag.bright_yellow().bold().to_string(),
                };
                println!(
                    "\r\x1b[K  {} {} {} {}",
                    "⚡".bright_yellow(),
                    format!("[{}]", tag_display)
                        .on_bright_black()
                        .white()
                        .bold(),
                    "Running =>".bright_black(),
                    format!("{} {}", args.binary, args_joined)
                        .bright_cyan()
                        .bold()
                );
            }
        }

        // Track execution
        self.executed_tools
            .lock()
            .await
            .push((args.binary.clone(), args_joined.clone()));

        let bin_name_log = args.binary.clone();
        let tool_tag_log = tool_type_tag.clone();
        let event_tx_clone = self.event_tx.clone();
        let callback = Box::new(move |line: String| {
            if let Some(ref tx) = event_tx_clone {
                let _ = tx.send(TuiEvent::ToolStream {
                    tool: bin_name_log.clone(),
                    line: line.clone(),
                });
            } else {
                println!(
                    "    {} {} {}",
                    "│".bright_black(),
                    format!("[{}]", tool_tag_log).dimmed(),
                    line.bright_black().italic()
                );
                let _ = std::io::stdout().flush();
            }
        });

        // Resolve binary name
        if let Some((srv_name, tool_name)) = args.binary.split_once('/') {
            let cm = self.clients.read().await;
            let args_val = resolve_mcp_args(&args.binary, &args.args, args.structured_args.clone());

            match cm.call_external_tool(srv_name, tool_name, args_val).await {
                Ok(res) => {
                    if is_search {
                        self.emit_web_sources(&res).await;
                    }

                    if let Some(ref tx) = self.event_tx {
                        let _ = tx.send(TuiEvent::ToolFinished {
                            tool: format!("[{}] {}", tool_type_tag, args.binary),
                            success: true,
                        });
                    } else {
                        if !is_search {
                            println!(
                                "\r\x1b[K  {} {} {} {}",
                                "✓".green(),
                                format!("[{}]", tool_type_tag).magenta().bold(),
                                "Complete =>".bright_black(),
                                args.binary.bright_green().bold()
                            );
                        }
                    }
                    return Ok(serde_json::to_string_pretty(&res).unwrap_or_else(|_| "null".into()));
                }
                Err(e) => {
                    if let Some(ref tx) = self.event_tx {
                        let _ = tx.send(TuiEvent::ToolFinished {
                            tool: format!("[{}] {}", tool_type_tag, args.binary),
                            success: false,
                        });
                    } else {
                        println!(
                            "\r\x1b[K  {} {} {} {}",
                            "✗".red(),
                            format!("[{}]", tool_type_tag).magenta().bold(),
                            "Failed =>".bright_black(),
                            format!("{}: {}", args.binary, e).red()
                        );
                    }
                    return Err(ToolError(format!("External MCP execute failed: {}", e)));
                }
            }
        }

        // --- Phoenix Phase 4: Native Tool Parity ---
        // Check if it's a native utility tool from the registry
        if self
            .builtin_registry
            .list_tools()
            .iter()
            .any(|t| t.name == args.binary)
        {
            match self
                .builtin_registry
                .execute(&args.binary, args.structured_args.clone())
                .await
            {
                Ok(res) => {
                    if is_search {
                        self.emit_web_sources(&res).await;
                    }

                    if let Some(ref tx) = self.event_tx {
                        let _ = tx.send(TuiEvent::ToolFinished {
                            tool: format!("[{}] {}", tool_type_tag, args.binary),
                            success: true,
                        });
                    } else {
                        if !is_search {
                            println!(
                                "\r\x1b[K  {} {} {} {}",
                                "✓".green(),
                                format!("[{}]", tool_type_tag).cyan().bold(),
                                "Complete =>".bright_black(),
                                args.binary.bright_green().bold()
                            );
                        }
                    }
                    return Ok(serde_json::to_string_pretty(&res).unwrap_or_else(|_| "null".into()));
                }
                Err(e) => {
                    if let Some(ref tx) = self.event_tx {
                        let _ = tx.send(TuiEvent::ToolFinished {
                            tool: format!("[{}] {}", tool_type_tag, args.binary),
                            success: false,
                        });
                    } else {
                        println!(
                            "\r\x1b[K  {} {} {} {}",
                            "✗".red(),
                            format!("[{}]", tool_type_tag).cyan().bold(),
                            "Failed =>".bright_black(),
                            format!("{}: {}", args.binary, e).red()
                        );
                    }
                    return Err(ToolError(format!("Native tool execution failed: {}", e)));
                }
            }
        }

        // Resolve local binary name to absolute path using discovery registry
        let binary_path = {
            let discovery = self.discovery.read().await;
            discovery
                .get(&args.binary)
                .map(|t| t.path.clone())
                .unwrap_or(args.binary.clone())
        };

        match self
            .executor
            .execute(
                &binary_path,
                &parsed_args,
                &self.workspace_path,
                Some(callback),
            )
            .await
        {
            Ok(result) => {
                if is_search {
                    let val = serde_json::from_str(&result.stdout)
                        .unwrap_or_else(|_| serde_json::Value::String(result.stdout.clone()));
                    self.emit_web_sources(&val).await;
                }

                if let Some(ref tx) = self.event_tx {
                    let _ = tx.send(TuiEvent::ToolFinished {
                        tool: format!("[{}] {}", tool_type_tag, args.binary),
                        success: result.exit_code == 0,
                    });
                } else {
                    if !is_search {
                        println!(
                            "\r\x1b[K  {} {} {} {}",
                            "✓".green(),
                            format!("[{}]", tool_type_tag)
                                .bright_white()
                                .on_yellow()
                                .bold(),
                            "Complete =>".bright_black(),
                            args.binary.bright_green().bold()
                        );
                    }
                }
                let mut output = serde_json::to_string_pretty(&result).unwrap_or_else(|_| {
                    format!("stdout: {}\nstderr: {}", result.stdout, result.stderr)
                });

                // Context Window Protection (Memory-First Strategy)
                const MAX_OUTPUT_CHARS: usize = 8000;
                if output.len() > MAX_OUTPUT_CHARS {
                    let truncated = &output[..MAX_OUTPUT_CHARS];
                    output = format!(
                        "{}\n\n... [OUTPUT TRUNCATED: {} chars total. Full output stored in Memory Database with Tool Name '{}'. Use `search_memory` to query specific details.]",
                        truncated,
                        output.len(),
                        args.binary
                    );
                }

                // Store in memory automatically (Zero-Latency Background Offloading)
                if let (Some(mem), Some(gen)) = (self.memory.clone(), self.generator.clone()) {
                    let content = serde_json::to_string_pretty(&result).unwrap_or_else(|_| {
                        format!("stdout: {}\nstderr: {}", result.stdout, result.stderr)
                    });
                    let tool_name = args.binary.clone();
                    let metadata = serde_json::json!({"binary": args.binary, "args": args.args, "exit_code": result.exit_code});
                    let timestamp = chrono::Utc::now().to_rfc3339();

                    tokio::spawn(async move {
                        let vector = gen.generate(&content).await;
                        let m = mem.clone();
                        let entry = crate::memory::qdrant::MemoryEntry {
                            id: uuid::Uuid::new_v4().to_string(),
                            content,
                            entry_type: crate::memory::qdrant::MemoryEntryType::ToolOutput,
                            tool_name: Some(tool_name),
                            target: None,
                            timestamp,
                            metadata,
                            vector: Some(vector),
                        };
                        let _ = m.store(entry).await;
                    });
                }

                Ok(output)
            }
            Err(e) => {
                if let Some(ref tx) = self.event_tx {
                    let _ = tx.send(TuiEvent::ToolFinished {
                        tool: format!("[{}] {}", tool_type_tag, args.binary),
                        success: false,
                    });
                }

                // Track missing tools for end-of-task notification for Industry-Grade reporting
                if matches!(e, crate::mcp::execute::ExecuteError::MissingBinary(_)) {
                    self.missing_tools.lock().await.insert(args.binary.clone());
                }

                Err(ToolError(e.to_string()))
            }
        }
    }
}

// ─── Discover Tools Definition ───

/// Rig tool: Discover available security tools.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct DiscoverToolsArgs {
    /// Optional category filter
    #[serde(default)]
    pub category: Option<String>,
    /// Optional search query
    #[serde(default)]
    pub query: Option<String>,
}

/// Holds the discovery system for the Rig tool bridge.
pub struct DiscoverToolsBridge {
    discovery: Arc<RwLock<ToolDiscovery>>,
    clients: Arc<RwLock<crate::mcp::client::McpClientManager>>,
    builtin_registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl DiscoverToolsBridge {
    pub fn new(
        discovery: Arc<RwLock<ToolDiscovery>>,
        clients: Arc<RwLock<crate::mcp::client::McpClientManager>>,
        builtin_registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self {
            discovery,
            clients,
            builtin_registry,
            event_tx,
        }
    }
}

impl Tool for DiscoverToolsBridge {
    const NAME: &'static str = "discover_tools";
    type Error = ToolError;
    type Args = DiscoverToolsArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Discover available tools by search query or category filter.".to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(DiscoverToolsArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let discovery = self.discovery.read().await;

        let scope = match (&args.category, &args.query) {
            (Some(c), Some(q)) => format!("Category='{}' Query='{}'", c, q),
            (Some(c), None) => format!("Category='{}'", c),
            (None, Some(q)) => format!("Query='{}'", q),
            (None, None) => "All".to_string(),
        };

        if self.event_tx.is_none() {
            println!(
                "\r\x1b[K  {} {} {} {}",
                "🔍".bright_blue(),
                "DSC".on_bright_black().white().bold(),
                "Searching =>".bright_black(),
                scope.bright_cyan().bold()
            );
        }

        let tools = if let Some(ref query) = args.query {
            discovery
                .search(query)
                .into_iter()
                .cloned()
                .collect::<Vec<_>>()
        } else {
            discovery
                .list_all()
                .into_iter()
                .cloned()
                .collect::<Vec<_>>()
        };

        let filtered = if let Some(ref cat) = args.category {
            let cat_lower = cat.to_lowercase();
            tools
                .into_iter()
                .filter(|t| {
                    format!("{:?}", t.category)
                        .to_lowercase()
                        .contains(&cat_lower)
                })
                .collect::<Vec<_>>()
        } else {
            tools
        };

        let entries: Vec<serde_json::Value> = filtered
            .iter()
            .take(200) // Truncate to protect LLM context window
            .map(|t| {
                serde_json::json!({
                    "name": t.name,
                    "category": format!("{:?}", t.category),
                    "path": t.path,
                    "source": "local_binary"
                })
            })
            .collect();

        // ── External MCP Server Tools ──
        let mut mcp_entries: Vec<serde_json::Value> = Vec::new();
        {
            let cm = self.clients.read().await;
            let all_external = cm.list_all_tools().await;
            let query_lower = args.query.as_ref().map(|q| q.to_lowercase());
            let cat_lower = args.category.as_ref().map(|c| c.to_lowercase());

            // Skip MCP tools if user is filtering by a non-MCP category
            let include_mcp = cat_lower.as_ref().map_or(true, |c| {
                c.contains("mcp") || c.contains("external") || c.contains("all")
            });

            if include_mcp {
                for (srv_name, tools) in all_external {
                    for tool in tools {
                        if let Some(name) = tool["name"].as_str() {
                            let desc = tool["description"].as_str().unwrap_or("");
                            let full_name = format!("{}/{}", srv_name, name);

                            // Apply query filter if present
                            if let Some(ref q) = query_lower {
                                if !full_name.to_lowercase().contains(q)
                                    && !desc.to_lowercase().contains(q)
                                {
                                    continue;
                                }
                            }

                            mcp_entries.push(serde_json::json!({
                                "name": full_name,
                                "category": "ExternalMCP",
                                "description": desc,
                                "server": srv_name,
                                "source": "mcp_server"
                            }));
                        }
                    }
                }
            }
        }

        // ── Built-in Native Utility Tools ──
        let mut native_entries: Vec<serde_json::Value> = Vec::new();
        {
            let query_lower = args.query.as_ref().map(|q| q.to_lowercase());
            let cat_lower = args.category.as_ref().map(|c| c.to_lowercase());

            let include_native = cat_lower.as_ref().map_or(true, |c| {
                c.contains("native")
                    || c.contains("utility")
                    || c.contains("builtin")
                    || c.contains("all")
            });

            if include_native {
                for tool in self.builtin_registry.list_tools() {
                    if let Some(ref q) = query_lower {
                        if !tool.name.to_lowercase().contains(q)
                            && !tool.description.to_lowercase().contains(q)
                        {
                            continue;
                        }
                    }
                    native_entries.push(serde_json::json!({
                        "name": tool.name,
                        "category": "NativeUtility",
                        "description": tool.description,
                        "source": "builtin_registry"
                    }));
                }
            }
        }

        let total_local = filtered.len();
        let total_mcp = mcp_entries.len();
        let total_native = native_entries.len();

        let mut all_entries = entries;
        all_entries.extend(mcp_entries);
        all_entries.extend(native_entries);

        let mut response = serde_json::json!({
            "total_found": total_local + total_mcp + total_native,
            "returned": all_entries.len(),
            "breakdown": {
                "local_binaries": total_local,
                "mcp_server_tools": total_mcp,
                "native_utilities": total_native
            },
            "tools": all_entries,
        });

        if total_local > 200 {
            response["warning"] = serde_json::json!("Local binary results truncated to 200 items to protect context window. Please use a more specific 'query' or 'category' to narrow your search. Your environment has 3000+ available tools.");
        }

        serde_json::to_string_pretty(&response).map_err(|e| ToolError(e.to_string()))
    }
}

// ─── Get Tool Help Definition ───

/// Rig tool: Get --help output for a tool.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetToolHelpArgs {
    /// Name of the tool
    pub tool_name: String,
}

pub struct GetToolHelpBridge {
    discovery: Arc<RwLock<ToolDiscovery>>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl GetToolHelpBridge {
    pub fn new(
        discovery: Arc<RwLock<ToolDiscovery>>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self {
            discovery,
            event_tx,
        }
    }
}

impl Tool for GetToolHelpBridge {
    const NAME: &'static str = "get_tool_help";
    type Error = ToolError;
    type Args = GetToolHelpArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Get the --help output for a specific tool binary.".to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(GetToolHelpArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let discovery = self.discovery.read().await;

        // Dual-Mode Telemetry
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!("📖 Searching help archives for: '{}'...", args.tool_name),
            });
        } else {
            // CLI Premium Display
            println!(
                "\r\x1b[K  {} {} Reading => {}",
                "📖".bright_magenta(),
                "HLP".on_bright_black().white().bold(),
                args.tool_name.bright_cyan().bold()
            );
        }

        match discovery.get_help(&args.tool_name).await {
            Some(help) => Ok(format!("Help for '{}':\n\n{}", args.tool_name, help)),
            None => Err(ToolError(format!(
                "Could not get help for '{}'",
                args.tool_name
            ))),
        }
    }
}

// ─── Phase Completion Tool Definition ───

/// Rig tool: Report the completion of a reconnaissance phase.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ReportPhaseCompletionArgs {
    /// The phase number that was just completed (0-12)
    pub completed_phase: u8,
    /// A concise summary of the key findings from this phase
    pub summary: String,
    /// Mandatory: Answers to the specific mindmap questions for this phase (e.g. for Phase 4: How does the site reference a user? Are there multiple user roles?)
    pub answers_to_mandatory_questions: Option<String>,
}

pub struct ReportPhaseCompletionBridge {
    recon_graph: Arc<Mutex<crate::core::recon_graph::ReconGraph>>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl ReportPhaseCompletionBridge {
    pub fn new(
        recon_graph: Arc<Mutex<crate::core::recon_graph::ReconGraph>>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self {
            recon_graph,
            event_tx,
        }
    }
}

impl Tool for ReportPhaseCompletionBridge {
    const NAME: &'static str = "report_phase_completion";
    type Error = ToolError;
    type Args = ReportPhaseCompletionArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "MUST BE CALLED at the end of every reconnaissance phase to formally advance to the next phase. Summarize findings and answer any mandatory phase questions.".to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(ReportPhaseCompletionArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let milestone_msg = format!("PHASE {} COMPLETE: {}", args.completed_phase, args.summary);

        // Dual-Mode Telemetry
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!(
                    "░▒▓█ MILESTONE █▓▒░\n{}\nAnswers: {}",
                    milestone_msg,
                    args.answers_to_mandatory_questions
                        .as_deref()
                        .unwrap_or("N/A")
                ),
            });
        } else {
            // CLI Premium Display
            println!(
                "\r\x1b[K\n  {} {} {} Phase {} Completed",
                "✅".bright_green(),
                "PHS".on_bright_green().white().bold(),
                "->".bright_black(),
                args.completed_phase
            );
            println!(
                "    {} {}",
                "│".bright_black(),
                args.summary.bright_black().italic()
            );

            if let Some(answers) = &args.answers_to_mandatory_questions {
                println!(
                    "    {} {}",
                    "│".bright_black(),
                    "Mandatory Answers:".bright_yellow()
                );
                for line in answers.lines() {
                    println!(
                        "    {} {}",
                        "│".bright_black(),
                        line.bright_black().italic()
                    );
                }
            }
            println!();
        }

        let mut graph = self.recon_graph.lock().await;
        graph.advance_phase(args.completed_phase, args.summary.clone());

        Ok(format!(
            "Phase {} successfully marked as complete. Proceeding to Phase {}.",
            args.completed_phase,
            args.completed_phase + 1
        ))
    }
}

// ─── Execute Batch Definition ───

/// Rig tool: Execute multiple security tools in parallel (Swarm Mode).
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ExecuteBatchArgs {
    /// A list of tool commands to run in parallel.
    pub commands: Vec<ExecuteToolArgs>,
}

/// Holds the executor and discovery registry for the Rig batch tool bridge.
pub struct ExecuteBatchBridge {
    executor: Arc<ToolExecutor>,
    discovery: Arc<RwLock<ToolDiscovery>>,
    clients: Arc<RwLock<crate::mcp::client::McpClientManager>>,
    executed_tools: Arc<Mutex<Vec<(String, String)>>>,
    missing_tools: Arc<Mutex<HashSet<String>>>,
    workspace_path: std::path::PathBuf,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    builtin_registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
    rate_limit_ms: u64,
    memory: Option<Arc<crate::memory::qdrant::InMemoryStore>>,
    generator: Option<Arc<dyn crate::memory::embeddings::EmbeddingGenerator>>,
}

impl ExecuteBatchBridge {
    pub fn new(ctx: BridgeContext) -> Self {
        Self {
            executor: ctx.executor,
            discovery: ctx.discovery,
            clients: ctx.clients,
            executed_tools: ctx.executed_tools,
            missing_tools: ctx.missing_tools,
            workspace_path: ctx.workspace_path,
            event_tx: ctx.event_tx,
            builtin_registry: ctx.builtin_registry,
            rate_limit_ms: ctx.rate_limit_ms,
            memory: None,
            generator: None,
        }
    }

    pub fn with_memory(
        mut self,
        memory: Arc<crate::memory::qdrant::InMemoryStore>,
        generator: Arc<dyn crate::memory::embeddings::EmbeddingGenerator>,
    ) -> Self {
        self.memory = Some(memory);
        self.generator = Some(generator);
        self
    }
}

impl Tool for ExecuteBatchBridge {
    const NAME: &'static str = "execute_batch";
    type Error = ToolError;
    type Args = ExecuteBatchArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Execute multiple tools in PARALLEL. Great for surface mapping, port scanning multiple targets, or simultaneous DNS/WHOIS/CT enumeration. Use only when commands are independent. CRITICAL: Always use verbose flags (e.g., -v, -vv, --verbose, -d) for all executing tools. The live UI depends on verbose streaming to show progress to the human operator.".to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(ExecuteBatchArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // H-01: Global LLM Tool Call Rate Limiting
        if self.rate_limit_ms > 0 {
            tokio::time::sleep(std::time::Duration::from_millis(self.rate_limit_ms)).await;
        }

        let cmd_count = args.commands.len();

        // CLI Premium Display
        if self.event_tx.is_none() {
            println!(
                "\r\x1b[K  {} {} {} Launching {} tools in parallel",
                "🐝".bright_yellow(),
                "SWM".on_bright_yellow().black().bold(),
                "Swarming =>".bright_black(),
                cmd_count.bright_cyan().bold()
            );
        }

        let mut futures = Vec::new();
        let mut executed_batch_info = Vec::new();

        for cmd in args.commands {
            let args_joined = cmd.args.join(" ");
            let binary_name = cmd.binary.clone();

            // Track execution
            self.executed_tools
                .lock()
                .await
                .push((binary_name.clone(), args_joined.clone()));

            let executor = self.executor.clone();
            let clients = self.clients.clone();
            let discovery = self.discovery.clone();
            let event_tx = self.event_tx.clone();
            let workspace = self.workspace_path.clone();
            let structured_args = cmd.structured_args.clone();
            let args_vec = cmd.args.clone();
            let memory = self.memory.clone();
            let generator = self.generator.clone();
            let missing_tools = self.missing_tools.clone();

            executed_batch_info.push(binary_name.clone());
            let srv_name_batch = if binary_name.contains('/') {
                binary_name.split('/').next().unwrap_or("")
            } else {
                ""
            };
            let is_search_batch = binary_name.contains("search")
                || binary_name.contains("research")
                || binary_name.contains("query")
                || binary_name.contains("fetch")
                || binary_name.contains("web")
                || srv_name_batch.contains("search")
                || srv_name_batch.contains("exa")
                || srv_name_batch.contains("tavily");

            futures.push(async move {
                if let Some((srv_name, tool_name)) = binary_name.split_once('/') {
                    // Start external tool telemetry
                    if let Some(ref tx) = event_tx {
                        if is_search_batch {
                            let q = structured_args.as_ref()
                                .and_then(|v| v.get("query").and_then(|vq| vq.as_str()).map(|s| s.to_string())
                                    .or_else(|| v.as_str().map(|s| s.to_string())))
                                .or_else(|| args_vec.first().cloned());

                            if let Some(ref search_q) = q {
                                let _ = tx.send(TuiEvent::WebSearchStarted {
                                    server: srv_name.to_string(),
                                    query: search_q.clone(),
                                });
                            }
                        }

                        let _ = tx.send(TuiEvent::ToolStarted {
                            server: srv_name.to_string(),
                            tool: tool_name.to_string(),
                            args: args_vec.join(" "),
                        });
                    }

                    let resolve_mcp_args_batch = |bin: &str, raw_args: &[String], structured: Option<serde_json::Value>| -> serde_json::Value {
                        if let Some(s) = structured { return s; }
                        if raw_args.is_empty() { return serde_json::Value::Object(serde_json::Map::new()); }
                        let name = if bin.contains('/') { bin.split('/').next_back().unwrap_or(bin) } else { bin };
                        let val = raw_args.first().cloned().unwrap_or_default();
                        if name.contains("search") || name.contains("research") || name.contains("query") {
                            serde_json::json!({"query": val})
                        } else if name.contains("fetch") || name.contains("browse") || name.contains("read") {
                            serde_json::json!({"url": val})
                        } else {
                            serde_json::to_value(raw_args).unwrap_or(serde_json::Value::Array(vec![]))
                        }
                    };
                    let args_val = resolve_mcp_args_batch(&binary_name, &args_vec, structured_args.clone());

                    let cm = clients.read().await;
                    let res = cm.call_external_tool(srv_name, tool_name, args_val).await;

                    if let Some(ref tx) = event_tx {
                        let _ = tx.send(TuiEvent::ToolFinished { tool: binary_name.clone(), success: res.is_ok() });
                    }

                    match res {
                        Ok(val) => {
                            if is_search_batch {
                                // Manual extraction for batch results - simpler since we don't have &self in the future
                                let mut urls = Vec::new();
                                fn extract_simple(v: &serde_json::Value, u: &mut Vec<String>) {
                                    match v {
                                        serde_json::Value::Object(m) => {
                                            for (k, val) in m {
                                                if k == "url" || k == "link" { if let Some(s) = val.as_str() { u.push(s.to_string()); } }
                                                extract_simple(val, u);
                                            }
                                        }
                                        serde_json::Value::Array(a) => { for val in a { extract_simple(val, u); } }
                                        _ => {}
                                    }
                                }
                                extract_simple(&val, &mut urls);
                                let mut seen = std::collections::HashSet::new();
                                for u_str in urls {
                                    if let Ok(p) = Url::parse(&u_str) {
                                        if let Some(d) = p.domain() {
                                            if seen.insert(d.to_string()) {
                                                if let Some(ref tx) = event_tx {
                                                    let _ = tx.send(TuiEvent::WebSourceFound { source: d.to_string() });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            let out = serde_json::to_string_pretty(&val).unwrap_or_else(|_| "null".into());
                            // Store in memory
                            if let (Some(ref mem), Some(ref gen)) = (&memory, &generator) {
                                let vector = gen.generate(&out).await;
                                let m = mem.clone();
                                let entry = crate::memory::qdrant::MemoryEntry {
                                    id: uuid::Uuid::new_v4().to_string(),
                                    content: out.clone(),
                                    entry_type: crate::memory::qdrant::MemoryEntryType::ToolOutput,
                                    tool_name: Some(binary_name.clone()),
                                    target: None,
                                    timestamp: chrono::Utc::now().to_rfc3339(),
                                    metadata: serde_json::json!({"binary": binary_name, "is_external": true, "success": true}),
                                    vector: Some(vector),
                                };
                                let _ = m.store(entry).await;
                            }
                            Ok(out)
                        }
                        Err(e) => Err(ToolError(format!("External tool {} failed: {}", binary_name, e))),
                    }
                } else if self.builtin_registry.list_tools().iter().any(|t| t.name == binary_name) {
                    // Start native tool telemetry
                    if let Some(ref tx) = event_tx {
                        let _ = tx.send(TuiEvent::ToolStarted {
                            server: "NATIVE".into(),
                            tool: binary_name.clone(),
                            args: structured_args.as_ref().map(|v| v.to_string()).unwrap_or_default(),
                        });
                    }

                    let res = self.builtin_registry.execute(&binary_name, structured_args.clone()).await;

                    if let Some(ref tx) = event_tx {
                        let _ = tx.send(TuiEvent::ToolFinished { tool: binary_name.clone(), success: res.is_ok() });
                    }

                    match res {
                        Ok(val) => {
                            if is_search_batch {
                                let mut urls = Vec::new();
                                fn extract_simple_native(v: &serde_json::Value, u: &mut Vec<String>) {
                                    match v {
                                        serde_json::Value::Object(m) => {
                                            for (k, val) in m {
                                                if k == "url" || k == "link" { if let Some(s) = val.as_str() { u.push(s.to_string()); } }
                                                extract_simple_native(val, u);
                                            }
                                        }
                                        serde_json::Value::Array(a) => { for val in a { extract_simple_native(val, u); } }
                                        _ => {}
                                    }
                                }
                                extract_simple_native(&val, &mut urls);
                                let mut seen = std::collections::HashSet::new();
                                for u_str in urls {
                                    if let Ok(p) = Url::parse(&u_str) {
                                        if let Some(d) = p.domain() {
                                            if seen.insert(d.to_string()) {
                                                if let Some(ref tx) = event_tx {
                                                    let _ = tx.send(TuiEvent::WebSourceFound { source: d.to_string() });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Ok(serde_json::to_string_pretty(&val).unwrap_or_else(|_| "null".into()))
                        }
                        Err(e) => Err(ToolError(format!("Native tool {} failed: {}", binary_name, e))),
                    }
                } else {
                    // Resolve local binary
                    let binary_path = {
                        let d = discovery.read().await;
                        d.get(&binary_name).map(|t| t.path.clone()).unwrap_or(binary_name.clone())
                    };

                    if let Some(ref tx) = event_tx {
                        let _ = tx.send(TuiEvent::ToolStarted {
                            server: "LOCAL".into(),
                            tool: binary_name.clone(),
                            args: args_vec.join(" "),
                        });
                    }

                    let bin_name_log = binary_name.clone();
                    let tx_clone = event_tx.clone();
                    let callback = Box::new(move |line: String| {
                        if let Some(ref tx) = tx_clone {
                            let _ = tx.send(TuiEvent::ToolStream { tool: bin_name_log.clone(), line });
                        }
                    });

                    let args_refs: Vec<&str> = args_vec.iter().map(|s| s.as_str()).collect();
                    match executor.execute(&binary_path, &args_refs, &workspace, Some(callback)).await {
                         Ok(res) => {
                            if is_search_batch {
                                let val_json = serde_json::from_str(&res.stdout).unwrap_or_else(|_| serde_json::Value::String(res.stdout.clone()));
                                let mut urls = Vec::new();
                                fn extract_simple_local(v: &serde_json::Value, u: &mut Vec<String>) {
                                    match v {
                                        serde_json::Value::Object(m) => {
                                            for (k, val) in m {
                                                if k == "url" || k == "link" { if let Some(s) = val.as_str() { u.push(s.to_string()); } }
                                                extract_simple_local(val, u);
                                            }
                                        }
                                        serde_json::Value::Array(a) => { for val in a { extract_simple_local(val, u); } }
                                        serde_json::Value::String(s) if s.starts_with("http") => { u.push(s.clone()); }
                                        _ => {}
                                    }
                                }
                                extract_simple_local(&val_json, &mut urls);
                                let mut seen = std::collections::HashSet::new();
                                for u_str in urls {
                                    if let Ok(p) = Url::parse(&u_str) {
                                        if let Some(d) = p.domain() {
                                            if seen.insert(d.to_string()) {
                                                if let Some(ref tx) = event_tx {
                                                    let _ = tx.send(TuiEvent::WebSourceFound { source: d.to_string() });
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                             if let Some(ref tx) = event_tx {
                                 let _ = tx.send(TuiEvent::ToolFinished { tool: binary_name.clone(), success: res.exit_code == 0 });
                             }
                            let mut out = serde_json::to_string_pretty(&res).unwrap_or_else(|_| format!("stdout: {}", res.stdout));

                            // Context Window Protection (Memory-First Strategy)
                            const MAX_OUTPUT_CHARS: usize = 8000;
                            if out.len() > MAX_OUTPUT_CHARS {
                                let truncated = &out[..MAX_OUTPUT_CHARS];
                                out = format!(
                                    "{}\n\n... [OUTPUT TRUNCATED: {} chars total. Full output stored in Memory Database with Tool Name '{}'. Use `search_memory` to query specific details.]",
                                    truncated,
                                    out.len(),
                                    binary_name
                                );
                            }

                            // Store in memory (Zero-Latency Background Swarm Offloading)
                            if let (Some(mem), Some(gen)) = (memory.clone(), generator.clone()) {
                                let content = serde_json::to_string_pretty(&res).unwrap_or_else(|_| format!("stdout: {}", res.stdout));
                                let tool_name = binary_name.clone();
                                let metadata = serde_json::json!({"binary": binary_name, "exit_code": res.exit_code});
                                let timestamp = chrono::Utc::now().to_rfc3339();

                                tokio::spawn(async move {
                                    let vector = gen.generate(&content).await;
                                    let m = mem.clone();
                                    let entry = crate::memory::qdrant::MemoryEntry {
                                        id: uuid::Uuid::new_v4().to_string(),
                                        content,
                                        entry_type: crate::memory::qdrant::MemoryEntryType::ToolOutput,
                                        tool_name: Some(tool_name),
                                        target: None,
                                        timestamp,
                                        metadata,
                                        vector: Some(vector),
                                    };
                                    let _ = m.store(entry).await;
                                });
                            }
                            Ok(out)
                        }
                        Err(e) => {
                            if let Some(ref tx) = event_tx {
                                let _ = tx.send(TuiEvent::ToolFinished { tool: binary_name.clone(), success: false });
                            }
                            if matches!(e, crate::mcp::execute::ExecuteError::MissingBinary(_)) {
                                missing_tools.lock().await.insert(binary_name.clone());
                            }
                            Err(ToolError(e.to_string()))
                        }
                    }
                }
            });
        }

        let results = futures::future::join_all(futures).await;
        let mut combined_output = String::new();

        for (i, res) in results.into_iter().enumerate() {
            let tool_name: &String = &executed_batch_info[i];
            combined_output.push_str(&format!(
                "\n┳━ [ SWARM NODE {} // {} ]\n",
                i + 1,
                tool_name.to_uppercase()
            ));
            match res {
                Ok(out) => combined_output.push_str(&out),
                Err(e) => combined_output.push_str(&format!("Error: {}", e)),
            }
            combined_output.push('\n');
        }

        if self.event_tx.is_none() {
            println!(
                "  {} {} {} Parallel batch complete",
                "🏁".bright_green(),
                "FIN".on_bright_black().white().bold(),
                "->".bright_black()
            );
        }

        Ok(combined_output)
    }
}

// ─── Report Finding Tool Definition ───

/// Rig tool: Report a critical finding discovered during reconnaissance.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ReportFindingArgs {
    /// Concise title of the finding (e.g., "Open SSH Port", "Admin Panel Exposed")
    pub title: String,
    /// Detailed description of the discovery
    pub description: String,
    /// Technical evidence (e.g., tool output snippet, URL, or data leak)
    pub evidence: String,
    /// Severity level
    pub severity: String, // "Critical", "High", "Medium", "Low", "Informational"
}

pub struct ReportFindingBridge {
    recon_graph: Arc<Mutex<crate::core::recon_graph::ReconGraph>>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl ReportFindingBridge {
    pub fn new(
        recon_graph: Arc<Mutex<crate::core::recon_graph::ReconGraph>>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self {
            recon_graph,
            event_tx,
        }
    }
}

impl Tool for ReportFindingBridge {
    const NAME: &'static str = "report_finding";
    type Error = ToolError;
    type Args = ReportFindingArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Register a critical finding (vulnerability, leak, or asset) into the session graph.".to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(ReportFindingArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        use crate::core::recon_graph::{Finding, Severity};

        let severity = match args.severity.to_lowercase().as_str() {
            "critical" => Severity::Critical,
            "high" => Severity::High,
            "medium" => Severity::Medium,
            "low" => Severity::Low,
            _ => Severity::Informational,
        };

        let finding = Finding {
            id: uuid::Uuid::new_v4().to_string(),
            title: args.title.clone(),
            severity: severity.clone(),
            description: args.description.clone(),
            evidence: args.evidence.clone(),
            tool_used: "LLM_ANALYSIS".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        let mut graph = self.recon_graph.lock().await;
        graph.add_finding(finding);

        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!(
                    "{} NEW FINDING: [{}] {}",
                    "🔴".bright_red(),
                    severity,
                    args.title
                ),
            });
        } else {
            println!(
                "\r\x1b[K  {} {} [{}] {}",
                "🚩".bright_red(),
                "FND".on_bright_red().white().bold(),
                severity,
                args.title.bright_white().bold()
            );
        }

        Ok(format!(
            "Finding '{}' successfully registered in the mission graph.",
            args.title
        ))
    }
}

// ─── MCP Prompt Bridges ───

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ListPromptsArgs {
    /// Optional server name filter
    pub server: Option<String>,
}

pub struct ListPromptsBridge {
    clients: Arc<RwLock<crate::mcp::client::McpClientManager>>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl ListPromptsBridge {
    pub fn new(
        clients: Arc<RwLock<crate::mcp::client::McpClientManager>>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self { clients, event_tx }
    }
}

impl Tool for ListPromptsBridge {
    const NAME: &'static str = "list_prompts";
    type Error = ToolError;
    type Args = ListPromptsArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "List available prompt templates from MCP servers.".to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(ListPromptsArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let scope = args.server.as_deref().unwrap_or("ALL servers");

        // Dual-Mode Telemetry
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!("📑 Discovery: Listing prompt templates from {}...", scope),
            });
        } else {
            // CLI Premium Display
            println!(
                "\r\x1b[K  {} {} Listing prompts from => {}",
                "📑".bright_blue(),
                "PRM".on_bright_black().white().bold(),
                scope.bright_cyan().bold()
            );
        }

        let cm = self.clients.read().await;
        if let Some(srv) = args.server {
            match cm.list_external_prompts(&srv).await {
                Ok(res) => Ok(serde_json::to_string_pretty(&res).unwrap_or_default()),
                Err(e) => Err(ToolError(format!(
                    "Failed to list prompts from {}: {}",
                    srv, e
                ))),
            }
        } else {
            let mut all = serde_json::Map::new();
            for name in cm.get_server_names() {
                if let Ok(res) = cm.list_external_prompts(&name).await {
                    all.insert(name, serde_json::Value::Array(res));
                }
            }
            Ok(serde_json::to_string_pretty(&all).unwrap_or_default())
        }
    }
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetPromptArgs {
    /// The server name
    pub server: String,
    /// The prompt name
    pub name: String,
    /// Arguments for the prompt template
    #[serde(default)]
    pub arguments: Option<std::collections::HashMap<String, String>>,
}

pub struct GetPromptBridge {
    clients: Arc<RwLock<crate::mcp::client::McpClientManager>>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl GetPromptBridge {
    pub fn new(
        clients: Arc<RwLock<crate::mcp::client::McpClientManager>>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self { clients, event_tx }
    }
}

impl Tool for GetPromptBridge {
    const NAME: &'static str = "get_prompt";
    type Error = ToolError;
    type Args = GetPromptArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Retrieve a specific prompt template with arguments from an MCP server."
                .to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(GetPromptArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Dual-Mode Telemetry
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!(
                    "📝 Intelligence Retrieval: Fetching prompt '{}' from {}...",
                    args.name, args.server
                ),
            });
        } else {
            // CLI Premium Display
            println!(
                "\r\x1b[K  {} {} Fetching prompt => {}/{}",
                "📝".bright_magenta(),
                "GET".on_bright_black().white().bold(),
                args.server.bright_black(),
                args.name.bright_cyan().bold()
            );
        }

        let cm = self.clients.read().await;
        let args_val = serde_json::to_value(args.arguments.unwrap_or_default())
            .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));
        match cm
            .get_external_prompt(&args.server, &args.name, args_val)
            .await
        {
            Ok(res) => Ok(serde_json::to_string_pretty(&res).unwrap_or_default()),
            Err(e) => Err(ToolError(format!(
                "Failed to get prompt {} from {}: {}",
                args.name, args.server, e
            ))),
        }
    }
}

// ─── MCP Resources Bridges ───

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ListResourcesArgs {
    /// Optional server name filter (e.g., "sqlite")
    pub server: Option<String>,
}

pub struct ListResourcesBridge {
    clients: Arc<RwLock<crate::mcp::client::McpClientManager>>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl ListResourcesBridge {
    pub fn new(
        clients: Arc<RwLock<crate::mcp::client::McpClientManager>>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self { clients, event_tx }
    }
}

impl Tool for ListResourcesBridge {
    const NAME: &'static str = "list_resources";
    type Error = ToolError;
    type Args = ListResourcesArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "List available resources (data files, schemas, logs) from MCP servers."
                .to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(ListResourcesArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let scope = args.server.as_deref().unwrap_or("ALL nodes");

        // Dual-Mode Telemetry
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!("📂 Inventory: Listing resources from {}...", scope),
            });
        } else {
            // CLI Premium Display
            println!(
                "\r\x1b[K  {} {} Listing resources from => {}",
                "📂".bright_blue(),
                "RES".on_bright_black().white().bold(),
                scope.bright_cyan().bold()
            );
        }

        let cm = self.clients.read().await;
        if let Some(srv) = args.server {
            match cm.list_external_resources(&srv).await {
                Ok(res) => Ok(serde_json::to_string_pretty(&res).unwrap_or_default()),
                Err(e) => Err(ToolError(format!(
                    "Failed to list resources from {}: {}",
                    srv, e
                ))),
            }
        } else {
            // List from all connected servers
            let mut all = serde_json::Map::new();
            for name in cm.get_server_names() {
                if let Ok(res) = cm.list_external_resources(&name).await {
                    all.insert(name, serde_json::Value::Array(res));
                }
            }
            Ok(serde_json::to_string_pretty(&all).unwrap_or_default())
        }
    }
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ReadResourceArgs {
    /// The server name (e.g., "sqlite")
    pub server: String,
    /// The URI of the resource to read (found via list_resources)
    pub uri: String,
}

pub struct ReadResourceBridge {
    clients: Arc<RwLock<crate::mcp::client::McpClientManager>>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl ReadResourceBridge {
    pub fn new(
        clients: Arc<RwLock<crate::mcp::client::McpClientManager>>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self { clients, event_tx }
    }
}

impl Tool for ReadResourceBridge {
    const NAME: &'static str = "read_resource";
    type Error = ToolError;
    type Args = ReadResourceArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Read the content of an MCP resource (e.g., schema, config file)."
                .to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(ReadResourceArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Dual-Mode Telemetry
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!(
                    "📖 Intelligence Extraction: Reading resource '{}' from {}...",
                    args.uri, args.server
                ),
            });
        } else {
            // CLI Premium Display
            println!(
                "\r\x1b[K  {} {} Reading resource => {}/{}",
                "📖".bright_magenta(),
                "RD".on_bright_black().white().bold(),
                args.server.bright_black(),
                args.uri.bright_cyan().bold()
            );
        }

        let cm = self.clients.read().await;
        match cm.read_external_resource(&args.server, &args.uri).await {
            Ok(res) => Ok(serde_json::to_string_pretty(&res).unwrap_or_default()),
            Err(e) => Err(ToolError(format!(
                "Failed to read resource {} from {}: {}",
                args.uri, args.server, e
            ))),
        }
    }
}

// ─── Generate File Tool Definition ───

/// Rig tool: Generate a new file with precise content control.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GenerateFileArgs {
    /// Relative path in workspace
    pub path: String,
    /// Raw content to write
    pub content: String,
}

pub struct GenerateFileBridge {
    registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl GenerateFileBridge {
    pub fn new(
        registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self { registry, event_tx }
    }
}

impl Tool for GenerateFileBridge {
    const NAME: &'static str = "generate_file";
    type Error = ToolError;
    type Args = GenerateFileArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Generate a new file (or overwrite) with precise content control. Scoped to mission workspace.".to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(GenerateFileArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Dual-Mode Telemetry
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!("💾 File Generation: Creating path '{}'...", args.path),
            });
        } else {
            // CLI Premium Display
            println!(
                "\r\x1b[K  {} {} Generating Asset => {}",
                "💾".bright_green(),
                "GEN".on_bright_black().white().bold(),
                args.path.bright_cyan().bold()
            );
        }

        let args_val = serde_json::to_value(&args).unwrap_or_default();
        match self.registry.execute("generate_file", Some(args_val)).await {
            Ok(res) => Ok(serde_json::to_string_pretty(&res).unwrap_or_default()),
            Err(e) => Err(ToolError(format!("Generate file failed: {}", e))),
        }
    }
}

// ─── Append To File Tool Definition ───

/// Rig tool: Append content to an existing asset.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct AppendToFileArgs {
    /// Relative path in workspace
    pub path: String,
    /// Content to append
    pub content: String,
}

pub struct AppendToFileBridge {
    registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl AppendToFileBridge {
    pub fn new(
        registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self { registry, event_tx }
    }
}

impl Tool for AppendToFileBridge {
    const NAME: &'static str = "append_to_file";
    type Error = ToolError;
    type Args = AppendToFileArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description:
                "Append content to an existing mission asset. Scoped to mission workspace."
                    .to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(AppendToFileArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Dual-Mode Telemetry
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!("📝 File Append: Updating path '{}'...", args.path),
            });
        } else {
            // CLI Premium Display
            println!(
                "\r\x1b[K  {} {} Appending Asset => {}",
                "📝".bright_yellow(),
                "APP".on_bright_black().white().bold(),
                args.path.bright_cyan().bold()
            );
        }

        let args_val = serde_json::to_value(&args).unwrap_or_default();
        match self
            .registry
            .execute("append_to_file", Some(args_val))
            .await
        {
            Ok(res) => Ok(serde_json::to_string_pretty(&res).unwrap_or_default()),
            Err(e) => Err(ToolError(format!("Append to file failed: {}", e))),
        }
    }
}
// ─── Sovereign Tier: Secure Asset Bridge ───

/// Rig tool: Generate an encrypted mission asset.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GenerateSecureAssetArgs {
    /// Relative path in workspace
    pub path: String,
    /// Raw content (optional)
    pub content: Option<String>,
    /// Hex-encoded 32-byte key
    pub key: String,
}

pub struct GenerateSecureAssetBridge {
    registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl GenerateSecureAssetBridge {
    pub fn new(
        registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self { registry, event_tx }
    }
}

impl Tool for GenerateSecureAssetBridge {
    const NAME: &'static str = "generate_secure_asset";
    type Error = ToolError;
    type Args = GenerateSecureAssetArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description:
                "Generate an AES-256-GCM-SIV encrypted mission asset with forensic metadata."
                    .to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(GenerateSecureAssetArgs))
                .unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!("🔐 Secure Delivery: Encrypting asset '{}'...", args.path),
            });
        }
        let args_val = serde_json::to_value(&args).unwrap_or_default();
        match self
            .registry
            .execute("generate_secure_asset", Some(args_val))
            .await
        {
            Ok(res) => Ok(serde_json::to_string_pretty(&res).unwrap_or_default()),
            Err(e) => Err(ToolError(format!("Secure generation failed: {}", e))),
        }
    }
}

// ─── Sovereign Tier: Batch Bridge ───

/// Rig tool: Generate multiple files in parallel.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GenerateBatchArgs {
    /// List of (path, content) tuples
    pub files: Vec<(String, Option<String>)>,
}

pub struct GenerateBatchBridge {
    registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl GenerateBatchBridge {
    pub fn new(
        registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self { registry, event_tx }
    }
}

impl Tool for GenerateBatchBridge {
    const NAME: &'static str = "generate_batch";
    type Error = ToolError;
    type Args = GenerateBatchArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Generate multiple mission assets in parallel using multi-core hybrid orchestration.".to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(GenerateBatchArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!(
                    "⚡ Lightning Batch: Staging {} parallel assets...",
                    args.files.len()
                ),
            });
        }
        let args_val = serde_json::to_value(&args).unwrap_or_default();
        match self
            .registry
            .execute("generate_batch", Some(args_val))
            .await
        {
            Ok(res) => Ok(serde_json::to_string_pretty(&res).unwrap_or_default()),
            Err(e) => Err(ToolError(format!("Batch generation failed: {}", e))),
        }
    }
}

// ─── Sovereign Tier: Secure Batch Bridge ───

/// Rig tool: Generate multiple encrypted files.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GenerateSecureBatchArgs {
    /// List of (path, content, hex_key)
    pub assets: Vec<(String, Option<String>, String)>,
}

pub struct GenerateSecureBatchBridge {
    registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl GenerateSecureBatchBridge {
    pub fn new(
        registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self { registry, event_tx }
    }
}

impl Tool for GenerateSecureBatchBridge {
    const NAME: &'static str = "generate_secure_batch";
    type Error = ToolError;
    type Args = GenerateSecureBatchArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description:
                "Generate multiple encrypted mission assets in parallel (Sovereign Tier Scale)."
                    .to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(GenerateSecureBatchArgs))
                .unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!(
                    "🔐 Secure Parallel Delivery: Encrypting {} assets...",
                    args.assets.len()
                ),
            });
        }
        let args_val = serde_json::to_value(&args).unwrap_or_default();
        match self
            .registry
            .execute("generate_secure_batch", Some(args_val))
            .await
        {
            Ok(res) => Ok(serde_json::to_string_pretty(&res).unwrap_or_default()),
            Err(e) => Err(ToolError(format!("Secure batch failed: {}", e))),
        }
    }
}

// ─── Sovereign Tier: JSON Patch Bridge ───

/// Rig tool: Apply RFC 6902 patches.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct PatchJsonArgs {
    pub path: String,
    pub patch: serde_json::Value,
}

pub struct PatchJsonBridge {
    registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl PatchJsonBridge {
    pub fn new(
        registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self { registry, event_tx }
    }
}

impl Tool for PatchJsonBridge {
    const NAME: &'static str = "patch_json";
    type Error = ToolError;
    type Args = PatchJsonArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Apply atomic RFC 6902 structural patches to JSON datasets.".to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(PatchJsonArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!("🔧 Patching JSON dataset: {}", args.path),
            });
        }
        let args_val = serde_json::to_value(&args).unwrap_or_default();
        match self.registry.execute("patch_json", Some(args_val)).await {
            Ok(res) => Ok(serde_json::to_string_pretty(&res).unwrap_or_default()),
            Err(e) => Err(ToolError(format!("JSON patch failed: {}", e))),
        }
    }
}

/// Rig tool: Extract content from a URL.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct BrowseArgs {
    /// Target URL
    pub url: String,
    /// Optional auth session
    pub session_name: Option<String>,
}

pub struct BrowseBridge {
    registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl BrowseBridge {
    pub fn new(
        registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self { registry, event_tx }
    }
}

impl Tool for BrowseBridge {
    const NAME: &'static str = "browse";
    type Error = ToolError;
    type Args = BrowseArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Navigate to a URL and extract content. Supports sessions.".to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(BrowseArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Dual-Mode Telemetry
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!("🌐 Web Recon: Navigating to '{}'...", args.url),
            });
        } else {
            // CLI Premium Display
            println!(
                "\r\x1b[K  {} {} Navigating URL => {}",
                "🌐".bright_blue(),
                "WEB".on_bright_black().white().bold(),
                args.url.bright_cyan().bold()
            );
        }

        let args_val = serde_json::to_value(&args).unwrap_or_default();
        match self.registry.execute("browse", Some(args_val)).await {
            Ok(res) => Ok(serde_json::to_string_pretty(&res).unwrap_or_default()),
            Err(e) => Err(ToolError(format!("Browse failed: {}", e))),
        }
    }
}

// ─── Web Action Tool Definition ───

/// Rig tool: Advanced browser automation.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct WebActionArgs {
    /// Action to perform
    pub action: String,
    /// CSS selector
    pub selector: Option<String>,
    /// Text to type
    pub text: Option<String>,
    /// URL for screenshot
    pub url: Option<String>,
    /// Local screenshot path
    pub output_path: Option<String>,
    /// Wait timeout (secs)
    pub timeout: Option<i32>,
    /// Auth session
    pub session_name: Option<String>,
}

pub struct WebActionBridge {
    registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl WebActionBridge {
    pub fn new(
        registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self { registry, event_tx }
    }
}

impl Tool for WebActionBridge {
    const NAME: &'static str = "web_action";
    type Error = ToolError;
    type Args = WebActionArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description:
                "Perform browser actions: click, type, screenshot. Supports headless automation."
                    .to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(WebActionArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Dual-Mode Telemetry
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!(
                    "🕹️ Web Action: Executing '{}' on selector '{}'...",
                    args.action,
                    args.selector.as_deref().unwrap_or("N/A")
                ),
            });
        } else {
            // CLI Premium Display
            println!(
                "\r\x1b[K  {} {} Automating Action => {}",
                "🕹️".bright_magenta(),
                "ACT".on_bright_black().white().bold(),
                args.action.bright_cyan().bold()
            );
        }

        let args_val = serde_json::to_value(&args).unwrap_or_default();
        match self.registry.execute("web_action", Some(args_val)).await {
            Ok(res) => Ok(serde_json::to_string_pretty(&res).unwrap_or_default()),
            Err(e) => Err(ToolError(format!("Web action failed: {}", e))),
        }
    }
}

// ─── Web Login Tool Definition ───

/// Rig tool: Automated form-based authentication.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct WebLoginArgs {
    /// Login page URL
    pub url: String,
    /// Username selector
    pub user_selector: String,
    /// Password selector
    pub pass_selector: String,
    /// Username
    pub user_value: String,
    /// Password
    pub pass_value: String,
    /// Target session ID
    pub session_name: Option<String>,
}

pub struct WebLoginBridge {
    registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl WebLoginBridge {
    pub fn new(
        registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self { registry, event_tx }
    }
}

impl Tool for WebLoginBridge {
    const NAME: &'static str = "web_login";
    type Error = ToolError;
    type Args = WebLoginArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Automate form-based authentication to establish a session.".to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(WebLoginArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Dual-Mode Telemetry
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!("🔐 Web Auth: Authenticating session for '{}'...", args.url),
            });
        } else {
            // CLI Premium Display
            println!(
                "\r\x1b[K  {} {} Establishing Session => {}",
                "🔐".bright_red(),
                "ATH".on_bright_black().white().bold(),
                args.url.bright_cyan().bold()
            );
        }

        let args_val = serde_json::to_value(&args).unwrap_or_default();
        match self.registry.execute("web_login", Some(args_val)).await {
            Ok(res) => Ok(serde_json::to_string_pretty(&res).unwrap_or_default()),
            Err(e) => Err(ToolError(format!("Web login failed: {}", e))),
        }
    }
}

// ─── Specialized Generation: Payload Bridge ───

/// Rig tool: Generate specialized security payloads.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GeneratePayloadArgs {
    /// Relative path in workspace
    pub path: String,
    /// Type of payload (e.g., webshell, reverseshell)
    pub payload_type: String,
}

pub struct GeneratePayloadBridge {
    registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl GeneratePayloadBridge {
    pub fn new(
        registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self { registry, event_tx }
    }
}

impl Tool for GeneratePayloadBridge {
    const NAME: &'static str = "generate_payload";
    type Error = ToolError;
    type Args = GeneratePayloadArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Generate specialized security payloads (webshells, reverseshells) for specific targets.".to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(GeneratePayloadArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!(
                    "💀 Payload Generation: Arming vector '{}' type '{}'...",
                    args.path, args.payload_type
                ),
            });
        }
        let args_val = serde_json::to_value(&args).unwrap_or_default();
        match self
            .registry
            .execute("generate_payload", Some(args_val))
            .await
        {
            Ok(res) => Ok(serde_json::to_string_pretty(&res).unwrap_or_default()),
            Err(e) => Err(ToolError(format!("Payload generation failed: {}", e))),
        }
    }
}

// ─── Specialized Generation: Payload File Bridge ───

/// Rig tool: Generate a standalone payload file.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GeneratePayloadFileArgs {
    /// File format (e.g., php, py, exe)
    pub format: String,
    /// Type of payload
    pub payload_type: String,
}

pub struct GeneratePayloadFileBridge {
    registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl GeneratePayloadFileBridge {
    pub fn new(
        registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self { registry, event_tx }
    }
}

impl Tool for GeneratePayloadFileBridge {
    const NAME: &'static str = "generate_payload_file";
    type Error = ToolError;
    type Args = GeneratePayloadFileArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Generate a standalone payload file with targeted formatting and headers."
                .to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(GeneratePayloadFileArgs))
                .unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!(
                    "💉 Injection Prep: Crafting {} payload for {} format...",
                    args.payload_type, args.format
                ),
            });
        }
        let args_val = serde_json::to_value(&args).unwrap_or_default();
        match self
            .registry
            .execute("generate_payload_file", Some(args_val))
            .await
        {
            Ok(res) => Ok(serde_json::to_string_pretty(&res).unwrap_or_default()),
            Err(e) => Err(ToolError(format!("Payload file generation failed: {}", e))),
        }
    }
}

// ─── Specialized Generation: Metadata Bridge ───

/// Rig tool: Generate a file with custom mission metadata.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GenerateWithMetadataArgs {
    /// Relative path in workspace
    pub path: String,
    /// Target format
    pub format: String,
    /// Optional raw content
    pub content: Option<String>,
    /// Custom key-value pairs
    pub metadata: std::collections::HashMap<String, String>,
}

pub struct GenerateWithMetadataBridge {
    registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl GenerateWithMetadataBridge {
    pub fn new(
        registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self { registry, event_tx }
    }
}

impl Tool for GenerateWithMetadataBridge {
    const NAME: &'static str = "generate_with_metadata";
    type Error = ToolError;
    type Args = GenerateWithMetadataArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Generate a file with custom mission metadata and forensic tracking."
                .to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(GenerateWithMetadataArgs))
                .unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!(
                    "🏷️ Metadata Tagging: Stamping asset '{}' with mission tags...",
                    args.path
                ),
            });
        }
        let args_val = serde_json::to_value(&args).unwrap_or_default();
        match self
            .registry
            .execute("generate_with_metadata", Some(args_val))
            .await
        {
            Ok(res) => Ok(serde_json::to_string_pretty(&res).unwrap_or_default()),
            Err(e) => Err(ToolError(format!("Metadata generation failed: {}", e))),
        }
    }
}

// ─── Specialized Generation: Compression Bridge ───

/// Rig tool: High-speed Zstd compression.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GenerateCompressedArgs {
    /// Target path
    pub path: String,
    /// Raw content to compress
    pub content: String,
    /// Compression level (1-22)
    #[serde(default = "default_compression_level")]
    pub level: i32,
}

fn default_compression_level() -> i32 {
    3
}

pub struct GenerateCompressedBridge {
    registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl GenerateCompressedBridge {
    pub fn new(
        registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self { registry, event_tx }
    }
}

impl Tool for GenerateCompressedBridge {
    const NAME: &'static str = "generate_compressed";
    type Error = ToolError;
    type Args = GenerateCompressedArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "High-speed Zstd compression of a single mission asset.".to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(GenerateCompressedArgs))
                .unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!(
                    "🗜️ Archive Command: Compressing asset '{}' (Level {})...",
                    args.path, args.level
                ),
            });
        }
        let args_val = serde_json::to_value(&args).unwrap_or_default();
        match self
            .registry
            .execute("generate_compressed", Some(args_val))
            .await
        {
            Ok(res) => Ok(serde_json::to_string_pretty(&res).unwrap_or_default()),
            Err(e) => Err(ToolError(format!("Compression failed: {}", e))),
        }
    }
}

// ─── Specialized Generation: Archive Batch Bridge ───

/// Rig tool: Parallel multi-threaded compression (Zstd) of mission archives.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GenerateCompressedBatchArgs {
    /// List of (path, content, level)
    pub files: Vec<(String, Option<String>, i32)>,
}

pub struct GenerateCompressedBatchBridge {
    registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl GenerateCompressedBatchBridge {
    pub fn new(
        registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self { registry, event_tx }
    }
}

impl Tool for GenerateCompressedBatchBridge {
    const NAME: &'static str = "generate_compressed_batch";
    type Error = ToolError;
    type Args = GenerateCompressedBatchArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Parallel multi-threaded compression (Zstd) of mission archives."
                .to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(GenerateCompressedBatchArgs))
                .unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!(
                    "🗜️ Batch Archive: Compressing {} assets in parallel...",
                    args.files.len()
                ),
            });
        }
        let args_val = serde_json::to_value(&args).unwrap_or_default();
        match self
            .registry
            .execute("generate_compressed_batch", Some(args_val))
            .await
        {
            Ok(res) => Ok(serde_json::to_string_pretty(&res).unwrap_or_default()),
            Err(e) => Err(ToolError(format!("Compressed batch failed: {}", e))),
        }
    }
}

// ─── Specialized Generation: Mission Stats Bridge ───

/// Rig tool: Retrieve mission statistics.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetStatisticsArgs {}

pub struct GetStatisticsBridge {
    registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl GetStatisticsBridge {
    pub fn new(
        registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self { registry, event_tx }
    }
}

impl Tool for GetStatisticsBridge {
    const NAME: &'static str = "get_statistics";
    type Error = ToolError;
    type Args = GetStatisticsArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description:
                "Retrieve real-time telemetry on file generation performance and asset cataloging."
                    .to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(GetStatisticsArgs)).unwrap(),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: "📊 Fetching mission telemetry and I/O statistics...".to_string(),
            });
        }
        match self.registry.execute("get_statistics", None).await {
            Ok(res) => Ok(serde_json::to_string_pretty(&res).unwrap_or_default()),
            Err(e) => Err(ToolError(format!("Statistics retrieval failed: {}", e))),
        }
    }
}

// ─── Search Memory Tool Definition ───

/// Rig tool: Search the agent's long-term session memory for past findings or tool outputs.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SearchMemoryArgs {
    /// The search query (natural language or keywords)
    pub query: String,
    /// Optional: limit the number of results (default 5)
    pub limit: Option<usize>,
    /// Optional: filter by entry type (ScanResult, Finding, ToolOutput, Analysis, Note)
    pub entry_type_filter: Option<String>,
}

pub struct SearchMemoryBridge {
    memory: Arc<crate::memory::qdrant::InMemoryStore>,
    generator: Arc<dyn crate::memory::embeddings::EmbeddingGenerator>,
    event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
}

impl SearchMemoryBridge {
    pub fn new(
        memory: Arc<crate::memory::qdrant::InMemoryStore>,
        generator: Arc<dyn crate::memory::embeddings::EmbeddingGenerator>,
        event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    ) -> Self {
        Self {
            memory,
            generator,
            event_tx,
        }
    }
}

impl Tool for SearchMemoryBridge {
    const NAME: &'static str = "search_memory";
    type Error = ToolError;
    type Args = SearchMemoryArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Search session memory using semantic retrieval. Use this to recall past scan results, findings, or technical notes before repeating work.".to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(SearchMemoryArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // CLI/TUI Telemetry
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::Message {
                role: "system".to_string(),
                content: format!("🧠 Querying memory: '{}'...", args.query),
            });
        } else {
            println!(
                "\r\x1b[K  {} {} Searching memory for => {}",
                "🧠".bright_magenta(),
                "MEM".on_bright_black().white().bold(),
                args.query.bright_cyan().bold()
            );
        }

        let vector = self.generator.generate(&args.query).await;
        let memory = self.memory.clone();

        let mut results = memory
            .search(Some(&vector), Some(&args.query), args.limit.unwrap_or(5))
            .await
            .map_err(|e| ToolError(format!("Memory search failed: {}", e)))?;

        // Apply type filter if provided
        if let Some(target_type) = args.entry_type_filter {
            results.retain(|e| {
                format!("{:?}", e.entry_type).to_lowercase() == target_type.to_lowercase()
            });
        }

        if results.is_empty() {
            return Ok("No relevant memories found for this query.".to_string());
        }

        let output = results
            .iter()
            .map(|e| {
                format!(
                    "[{:?}] ({}) @ {}: {}\nMetadata: {}\n",
                    e.entry_type,
                    e.tool_name.as_deref().unwrap_or("N/A"),
                    e.timestamp,
                    e.content,
                    e.metadata
                )
            })
            .collect::<Vec<_>>()
            .join("\n---\n");

        Ok(format!(
            "Search Results (ordered by semantic similarity):\n\n{}",
            output
        ))
    }
}

// ─── Sovereign Tier: Memory-Mapped Read Bridge ───

/// Rig tool: Zero-copy read for massive assets.
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ReadMmapArgs {
    /// Path to the massive file
    pub path: String,
}

pub struct ReadMmapBridge {
    registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
}

impl ReadMmapBridge {
    pub fn new(registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>) -> Self {
        Self { registry }
    }
}

impl Tool for ReadMmapBridge {
    const NAME: &'static str = "read_mmap";
    type Error = ToolError;
    type Args = ReadMmapArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Zero-copy Memory-Mapped reading of massive (1GB+) mission assets."
                .to_string(),
            parameters: serde_json::to_value(schemars::schema_for!(ReadMmapArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let args_val = serde_json::to_value(&args).unwrap_or_default();
        match self.registry.execute("read_mmap", Some(args_val)).await {
            Ok(res) => Ok(serde_json::to_string_pretty(&res).unwrap_or_default()),
            Err(e) => Err(ToolError(format!("Mmap read failed: {}", e))),
        }
    }
}
