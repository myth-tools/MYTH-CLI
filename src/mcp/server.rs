//! MCP Server — stdio-based Model Context Protocol server.
//!
//! Exposes 3 dynamic tools to the LLM:
//! 1. `discover_tools` — Lists available security tools on the system
//! 2. `execute_tool` — Runs a tool inside the bubblewrap sandbox
//! 3. `get_tool_help` — Gets --help output for a specific tool

use crate::config::AppConfig;
use crate::mcp::client::McpClientError;
use crate::mcp::discover::ToolDiscovery;
use crate::mcp::execute::ToolExecutor;
use crate::mcp::schemas::*;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

/// The MCP server that bridges the LLM to system tools.
pub struct McpServer {
    discovery: Arc<RwLock<ToolDiscovery>>,
    executor: Arc<ToolExecutor>,
    clients: Arc<RwLock<crate::mcp::client::McpClientManager>>,
    builtin_registry: Arc<crate::builtin_tools::registry::BuiltinRegistry>,
    _recon_graph: Arc<Mutex<crate::core::recon_graph::ReconGraph>>,
    _memory: Arc<crate::memory::qdrant::InMemoryStore>,
    _generator: Arc<dyn crate::memory::embeddings::EmbeddingGenerator>,
    _config: Arc<RwLock<AppConfig>>,
    _watcher: Option<crate::config::watcher::ConfigWatcher>,
    background_tasks: Arc<Mutex<Vec<tokio::task::JoinHandle<()>>>>,
}

impl McpServer {
    /// Create a new MCP server from config.
    /// Tool discovery runs in the background — returns instantly.
    /// Optionally accepts a TUI event sender for hot-reload propagation.
    pub async fn new(
        config: &AppConfig,
        recon_graph: Arc<Mutex<crate::core::recon_graph::ReconGraph>>,
        memory: Arc<crate::memory::qdrant::InMemoryStore>,
        generator: Arc<dyn crate::memory::embeddings::EmbeddingGenerator>,
        tui_tx: Option<tokio::sync::mpsc::UnboundedSender<crate::tui::app::TuiEvent>>,
    ) -> Self {
        let discovery = Arc::new(RwLock::new(ToolDiscovery::new(
            config.mcp.tool_paths.clone(),
        )));
        let background_tasks = Arc::new(Mutex::new(Vec::new()));

        // Spawn background tool scan (non-blocking)
        let bg_discovery = Arc::clone(&discovery);
        let discovery_handle = tokio::spawn(async move {
            let mut disc = bg_discovery.write().await;
            let tools = disc.scan().await;
            tracing::info!(count = tools.len(), "Background tool scan complete");
        });
        {
            let mut tasks = background_tasks.lock().await;
            tasks.push(discovery_handle);
        }

        // Initialize external client manager
        let clients = Arc::new(RwLock::new(crate::mcp::client::McpClientManager::new()));
        let config_arc = Arc::new(RwLock::new(config.clone()));

        // --- Phoenix Phase 3: Tactical Hot-Reload ---
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let mcp_path = crate::config::AppConfig::mcp_config_path();
        // Watch the parent directory (not the file itself) so both user.yaml and mcp.json changes are caught
        let watch_dir = mcp_path
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .to_path_buf();

        let watcher = match crate::config::watcher::ConfigWatcher::new(&watch_dir, tx) {
            Ok(w) => {
                let clients_for_bg = Arc::clone(&clients);
                let config_for_bg = Arc::clone(&config_arc);
                let tui_tx_bg = tui_tx;

                let reload_handle = tokio::spawn(async move {
                    tracing::info!("Phoenix Hot-Reload active for mcp.json");
                    while rx.recv().await.is_some() {
                        tracing::info!("HOT-RELOAD: Registry modification detected");

                        // Step 1: Load storage and re-merge factory defaults
                        match crate::config::settings::McpStorage::load() {
                            Ok(mut storage) => {
                                // Re-merge factory defaults: adds new builtins, updates existing ones
                                let factory_synced = storage.sync_factory_defaults();
                                if factory_synced {
                                    tracing::info!(
                                        "HOT-RELOAD: Factory defaults re-synced to mcp.json"
                                    );
                                }

                                // Step 2: Update config_arc servers
                                {
                                    let mut conf = config_for_bg.write().await;
                                    conf.mcp.mcp_servers = storage.mcp_servers.clone();
                                }

                                // Step 3: Sync clients (Non-blocking Refactor)
                                Self::sync_clients_arc(
                                    clients_for_bg.clone(),
                                    &storage.mcp_servers,
                                )
                                .await;

                                // Step 4: Emit TUI event so widgets refresh instantly
                                if let Some(ref tx) = tui_tx_bg {
                                    let conf = config_for_bg.read().await;
                                    let _ = tx.send(crate::tui::app::TuiEvent::ConfigReloaded(
                                        Box::new(conf.clone()),
                                    ));
                                }

                                tracing::info!("HOT-RELOAD: Registry synchronization complete");
                            }
                            Err(e) => tracing::error!("HOT-RELOAD: Registry reload failed: {}", e),
                        }
                    }
                });
                {
                    let mut tasks = background_tasks.lock().await;
                    tasks.push(reload_handle);
                }
                Some(w)
            }
            Err(e) => {
                tracing::warn!(
                    "Hot-Reload initialization failed: {}. Manual edits will require restart.",
                    e
                );
                None
            }
        };

        let workspace_root =
            std::env::temp_dir().join(format!("{}-workspace", config.agent.name.to_lowercase()));
        let server = Self {
            discovery,
            executor: Arc::new(ToolExecutor::from_config(config)),
            clients,
            builtin_registry: Arc::new(crate::builtin_tools::registry::BuiltinRegistry::new(
                workspace_root,
                recon_graph.clone(),
                memory.clone(),
                generator.clone(),
                PathBuf::from(&config.agent.all_report_path),
            )),
            _recon_graph: recon_graph,
            _memory: memory,
            _generator: generator,
            _config: config_arc,
            _watcher: watcher,
            background_tasks: background_tasks.clone(),
        };

        // Phoenix Init: Background synchronization using the shared sync method
        let clients_init = Arc::clone(&server.clients);
        let config_init = config.clone();
        let init_handle = tokio::spawn(async move {
            tracing::info!("PHOENIX: Background MCP synchronization active");
            Self::sync_clients_arc(clients_init.clone(), &config_init.mcp.mcp_servers).await;

            // Startup Diagnostics: Log connection summary
            let cm = clients_init.read().await;
            let connected: Vec<String> = cm.list_client_names();
            let enabled_count = config_init
                .mcp
                .mcp_servers
                .iter()
                .filter(|(_, srv)| match srv {
                    crate::config::CustomMcpServer::Local(c) => c.enabled,
                    crate::config::CustomMcpServer::Remote(c) => c.enabled,
                })
                .count();
            let failed_count = enabled_count.saturating_sub(connected.len());

            if connected.is_empty() {
                tracing::warn!("PHOENIX: No MCP servers connected ({} enabled, {} failed). Web search and research tools unavailable.", enabled_count, failed_count);
            } else {
                tracing::info!(
                    "PHOENIX: MCP sync complete — {}/{} servers online: [{}]{}",
                    connected.len(),
                    enabled_count,
                    connected.join(", "),
                    if failed_count > 0 {
                        format!(" ({} failed)", failed_count)
                    } else {
                        String::new()
                    }
                );
            }
        });
        {
            let mut tasks = background_tasks.lock().await;
            tasks.push(init_handle);
        }

        server
    }

    pub async fn tool_count(&self) -> usize {
        self.clients.read().await.tool_count().await
    }

    /// Synchronize MCP clients (Differential & Non-blocking)
    /// Synchronize MCP clients using the shared manager reference
    pub async fn sync_clients(&self, config: &AppConfig) {
        Self::sync_clients_arc(self.clients.clone(), &config.mcp.mcp_servers).await;
    }

    /// Differential and non-blocking synchronization of MCP clients
    pub async fn sync_clients_arc(
        clients: Arc<RwLock<crate::mcp::client::McpClientManager>>,
        mcp_servers: &std::collections::HashMap<String, crate::config::CustomMcpServer>,
    ) {
        use futures::stream::{FuturesUnordered, StreamExt};

        // 1. Get delta under READ lock
        let (to_remove, to_add) = {
            let cm = clients.read().await;
            cm.get_sync_delta(mcp_servers)
        };

        // 2. Remove old clients under WRITE lock
        for name in to_remove {
            let mut cm = clients.write().await;
            cm.remove_cached_tools(&name);
            match cm.remove_client(&name).await {
                Ok(_) => {
                    tracing::info!(server = %name, "PHOENIX: Successfully decommissioned server and purged tool cache")
                }
                Err(e) => {
                    tracing::error!(server = %name, error = %e, "PHOENIX: Server decommissioning failed during sync")
                }
            }
        }

        // 3. Connect new clients OUTSIDE of any lock
        let mut add_futures = FuturesUnordered::new();
        for (name, cfg) in to_add {
            tracing::info!(server = %name, "Initializing MCP server (parallel)");
            add_futures.push(async move {
                let result =
                    crate::mcp::client::McpClientManager::connect_client_static(name.clone(), cfg)
                        .await;
                (name, result)
            });
        }

        // 4. Insert new clients and warm the tool cache under WRITE lock
        while let Some((name, result)) = add_futures.next().await {
            match result {
                Ok(client) => {
                    let mut cm = clients.write().await;
                    cm.insert_client(name.clone(), client);
                    // Pre-warm the tool cache so discovery is near-instant
                    let _ = cm.refresh_tools_for_client(&name).await;
                    tracing::info!(server = %name, "PHOENIX: Server synchronized and tool cache warmed");
                }
                Err(e) => {
                    tracing::error!(server = %name, error = %e, "PHOENIX: Tactical sync failure for MCP server")
                }
            }
        }
    }

    /// Re-scan local tool discovery.
    pub async fn reload_tools(&self) {
        let mut discovery = self.discovery.write().await;
        discovery.reload().await;
    }

    /// Handle: discover_tools
    pub async fn handle_discover(&self, input: DiscoverToolsInput) -> serde_json::Value {
        let discovery = self.discovery.read().await;

        let tools = if let Some(ref query) = input.query {
            discovery.search(query).into_iter().cloned().collect()
        } else {
            discovery
                .list_all()
                .into_iter()
                .cloned()
                .collect::<Vec<_>>()
        };

        // Filter by category if specified
        let tools: Vec<_> = if let Some(ref cat) = input.category {
            let cat_lower = cat.to_lowercase();
            tools
                .into_iter()
                .filter(|t| {
                    format!("{:?}", t.category)
                        .to_lowercase()
                        .contains(&cat_lower)
                })
                .collect()
        } else {
            tools
        };

        let entries: Vec<ToolEntry> = tools
            .iter()
            .map(|t| ToolEntry {
                name: t.name.clone(),
                category: format!("{:?}", t.category),
                path: t.path.clone(),
                description: t.description.clone(),
            })
            .collect();

        // Add external tools from clients
        let mut external_entries = Vec::new();
        {
            let cm = self.clients.read().await;
            let all_external = cm.list_all_tools().await;
            for (srv_name, tools) in all_external {
                for tool in tools {
                    if let (Some(name), Some(desc)) =
                        (tool["name"].as_str(), tool["description"].as_str())
                    {
                        external_entries.push(ToolEntry {
                            name: format!("{}/{}", srv_name, name),
                            category: "ExternalMCP".to_string(),
                            path: format!("mcp://{}", srv_name),
                            description: desc.to_string(),
                        });
                    }
                }
            }
        }

        let mut total_entries = entries;
        total_entries.extend(external_entries);

        // Add Native Utility Tools from cached Registry
        total_entries.extend(self.builtin_registry.list_tools());

        let response = ToolListResponse {
            total_count: total_entries.len(),
            tools: total_entries,
        };

        serde_json::to_value(response)
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to serialize ToolListResponse");
                serde_json::json!({ "success": false, "error": "Internal serialization error" })
            })
            .unwrap_or_else(|e| e)
    }

    /// Handle: execute_tool
    pub async fn handle_execute(&self, input: ExecuteToolInput) -> serde_json::Value {
        // Check if it's an external tool (prefix: server/tool)
        if let Some((srv_name, tool_name)) = input.binary.split_once('/') {
            let args_val = input.structured_args.clone().unwrap_or_else(|| {
                serde_json::to_value(&input.args).unwrap_or(serde_json::Value::Array(vec![]))
            });

            let start = std::time::Instant::now();
            let mut result = {
                let cm = self.clients.read().await;
                cm.call_external_tool(srv_name, tool_name, args_val.clone())
                    .await
            };

            // Check if Self-Healing is required (PHOENIX_RESTART_REQUIRED)
            if let Err(McpClientError::ConnectionFailed(ref msg)) = result {
                if msg.contains("PHOENIX_RESTART_REQUIRED") || msg.contains("Server is offline") {
                    tracing::warn!(server = %srv_name, "PHOENIX: Autonomous recovery initiated");

                    // Attempt restart
                    {
                        let mut cm_write = self.clients.write().await;
                        if let Err(e) = cm_write.restart_server(srv_name).await {
                            tracing::error!(server = %srv_name, error = %e, "PHOENIX: Recovery failed");
                        } else {
                            tracing::info!(server = %srv_name, "PHOENIX: Recovery successful, retrying tool call");
                            // Retry once
                            result = cm_write
                                .call_external_tool(srv_name, tool_name, args_val)
                                .await;
                        }
                    }
                }
            }

            let duration = start.elapsed();
            match result {
                Ok(mut res) => {
                    // Inject telemetry
                    if let Some(obj) = res.as_object_mut() {
                        obj.insert(
                            "execution_time_ms".to_string(),
                            serde_json::json!(duration.as_millis()),
                        );
                    }
                    return res;
                }
                Err(e) => {
                    return serde_json::json!({
                        "error": format!("External MCP execute failed: {}", e),
                        "success": false,
                        "execution_time_ms": duration.as_millis(),
                    })
                }
            }
        }

        // Check for Native Utility Tools via cached Registry
        if self
            .builtin_registry
            .list_tools()
            .iter()
            .any(|t| t.name == input.binary)
        {
            match self
                .builtin_registry
                .execute(&input.binary, input.structured_args.clone())
                .await
            {
                Ok(res) => return res,
                Err(e) => {
                    return serde_json::json!({
                        "success": false,
                        "error": format!("Native tool execution failed: {}", e),
                    })
                }
            }
        }

        let args: Vec<&str> = input.args.iter().map(|s| s.as_str()).collect();

        // Resolve binary name to absolute path if possible
        let binary_path = {
            let discovery = self.discovery.read().await;
            discovery
                .get(&input.binary)
                .map(|t| t.path.clone())
                .unwrap_or(input.binary.clone())
        };

        let config_read = self._config.read().await;
        let default_workspace = std::env::temp_dir().join(format!(
            "{}-workspace",
            config_read.agent.name.to_lowercase()
        ));
        if !default_workspace.exists() {
            let _ = std::fs::create_dir_all(&default_workspace);
        }

        match self
            .executor
            .execute(&binary_path, &args, &default_workspace, None)
            .await
        {
            Ok(result) => serde_json::to_value(result).unwrap_or_default(),
            Err(e) => serde_json::json!({
                "error": e.to_string(),
                "success": false,
            }),
        }
    }

    /// Handle: get_tool_help
    pub async fn handle_get_help(&self, input: GetToolHelpInput) -> serde_json::Value {
        // 1. Check local binary discovery
        let discovery = self.discovery.read().await;
        if let Some(help_text) = discovery.get_help(&input.tool_name).await {
            return serde_json::json!({
                "tool": input.tool_name,
                "help": help_text,
                "source": "discovery"
            });
        }

        // 2. Check built-in native registry
        if let Some(help_text) = self.builtin_registry.get_help(&input.tool_name) {
            return serde_json::json!({
                "tool": input.tool_name,
                "help": help_text,
                "source": "native_registry"
            });
        }

        // 3. Fallback: explain that no help was found
        serde_json::json!({
            "tool": input.tool_name,
            "error": "No help documentation found for this tool.",
            "success": false
        })
    }

    /// Handle: list_resources
    pub async fn handle_list_resources(&self, input: ListResourcesInput) -> serde_json::Value {
        let cm = self.clients.read().await;
        if let Some(srv_name) = input.server {
            match cm.list_external_resources(&srv_name).await {
                Ok(res) => serde_json::json!({ "server": srv_name, "resources": res }),
                Err(e) => {
                    serde_json::json!({ "error": format!("Failed to list resources from {}: {}", srv_name, e) })
                }
            }
        } else {
            // Better: just list all resources from all servers
            let mut all_resources = serde_json::Map::new();
            for name in cm.get_server_names() {
                if let Ok(res) = cm.list_external_resources(&name).await {
                    all_resources.insert(name, serde_json::Value::Array(res));
                }
            }
            serde_json::Value::Object(all_resources)
        }
    }

    /// Handle: read_resource
    pub async fn handle_read_resource(&self, input: ReadResourceInput) -> serde_json::Value {
        let cm = self.clients.read().await;
        match cm.read_external_resource(&input.server, &input.uri).await {
            Ok(res) => res,
            Err(e) => {
                serde_json::json!({ "error": format!("Failed to read resource {} from {}: {}", input.uri, input.server, e) })
            }
        }
    }

    /// Handle: list_prompts
    pub async fn handle_list_prompts(&self, input: ListPromptsInput) -> serde_json::Value {
        let cm = self.clients.read().await;
        if let Some(srv_name) = input.server {
            match cm.list_external_prompts(&srv_name).await {
                Ok(res) => serde_json::json!({ "server": srv_name, "prompts": res }),
                Err(e) => {
                    serde_json::json!({ "error": format!("Failed to list prompts from {}: {}", srv_name, e) })
                }
            }
        } else {
            let mut all_prompts = serde_json::Map::new();
            for name in cm.get_server_names() {
                if let Ok(res) = cm.list_external_prompts(&name).await {
                    all_prompts.insert(name, serde_json::Value::Array(res));
                }
            }
            serde_json::Value::Object(all_prompts)
        }
    }

    /// Handle: get_prompt
    pub async fn handle_get_prompt(&self, input: GetPromptInput) -> serde_json::Value {
        let cm = self.clients.read().await;
        match cm
            .get_external_prompt(&input.server, &input.name, input.arguments)
            .await
        {
            Ok(res) => res,
            Err(e) => {
                serde_json::json!({ "error": format!("Failed to get prompt {} from {}: {}", input.name, input.server, e) })
            }
        }
    }

    /// Get access to the tool discovery system.
    pub fn discovery(&self) -> Arc<RwLock<ToolDiscovery>> {
        Arc::clone(&self.discovery)
    }

    /// Get access to the tool executor.
    pub fn executor(&self) -> Arc<ToolExecutor> {
        Arc::clone(&self.executor)
    }

    /// Get access to the MCP clients.
    pub fn clients(&self) -> Arc<RwLock<crate::mcp::client::McpClientManager>> {
        Arc::clone(&self.clients)
    }

    /// Get access to the built-in tool registry.
    pub fn builtin_registry(&self) -> Arc<crate::builtin_tools::registry::BuiltinRegistry> {
        Arc::clone(&self.builtin_registry)
    }

    /// Gracefully shut down all MCP clients.
    pub async fn shutdown(&self) {
        tracing::info!("MCP Server: Triggering global client shutdown");

        // Abort background tasks (M-07 fix)
        {
            let mut tasks = self.background_tasks.lock().await;
            for handle in tasks.drain(..) {
                handle.abort();
            }
        }

        let mut cm = self.clients.write().await;
        let _ = cm.close_all().await;
    }
}
