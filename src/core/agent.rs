//! Agent — the main AI agent that orchestrates the reconnaissance loop.

use crate::config::AppConfig;
use crate::core::recon_graph::{ReconGraph, ReconState};
use crate::core::session::Session;
use crate::llm::prompts;
use crate::llm::NimClient;
use crate::mcp::McpServer;
use crate::memory::qdrant::{InMemoryStore, MemoryEntry, MemoryEntryType};

use rig::agent::MultiTurnStreamItem;
use rig::completion::Message as RigMessage;
use rig::message::{AssistantContent, UserContent};
use rig::prelude::CompletionClient;
use rig::streaming::StreamedAssistantContent;
use rig::streaming::StreamingChat;
use rig::OneOrMany;

use crate::tui::app::TuiEvent;
use tokio::sync::mpsc;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("LLM error: {0}")]
    Llm(String),

    #[error("MCP error: {0}")]
    Mcp(String),

    #[error("Session error: {0}")]
    Session(String),

    #[error("Memory error: {0}")]
    Memory(String),
}

/// The reconnaissance agent.
pub struct ReconAgent {
    nim_client: NimClient,
    mcp_server: McpServer,
    recon_graph: Arc<Mutex<ReconGraph>>,
    config: AppConfig,
    messages: Vec<Message>,
    session: Option<Session>,
    memory: Arc<InMemoryStore>,
    pub generator: Arc<dyn crate::memory::embeddings::EmbeddingGenerator>,
    pub event_tx: Option<mpsc::UnboundedSender<TuiEvent>>,
    pub missing_tools: Arc<Mutex<HashSet<String>>>,
}

/// A message in the conversation history.
#[derive(Debug, Clone)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool(String), // tool name
}

impl ReconAgent {
    /// Create a new agent with parallelized initialization.
    pub async fn new(config: AppConfig) -> Result<Self, AgentError> {
        // 1. Initialize NIM client
        let nim_client =
            NimClient::from_config(&config).map_err(|e| AgentError::Llm(e.to_string()))?;

        // 2. Initialize Memory
        let mut memory_store = InMemoryStore::from_config(&config);
        if config.memory.enabled {
            let _ = memory_store.connect().await;
        }
        let memory = Arc::new(memory_store);

        // 3. Initialize Embedding Generator
        let generator: Arc<dyn crate::memory::embeddings::EmbeddingGenerator> =
            if let Ok(keys) = config.llm.resolve_api_keys() {
                if let Some(key) = keys.first() {
                    Arc::new(crate::memory::embeddings::NimEmbeddingGenerator::new(
                        key.clone(),
                        config.llm.base_url.clone(),
                        "nvidia/nv-embedqa-e5-v5".to_string(),
                    ))
                } else {
                    Arc::new(crate::memory::embeddings::FallbackGenerator::new(
                        config.memory.vector_size as usize,
                    ))
                }
            } else {
                Arc::new(crate::memory::embeddings::FallbackGenerator::new(
                    config.memory.vector_size as usize,
                ))
            };

        // 4. Initialize Recon Graph
        let recon_graph = Arc::new(Mutex::new(ReconGraph::new("", config.agent.max_iterations)));

        // 5. Initialize MCP Server (Injected with dependencies)
        let mcp_server = McpServer::new(
            &config,
            recon_graph.clone(),
            memory.clone(),
            generator.clone(),
            None,
        )
        .await;

        // 6. Initialize Messages
        let messages = vec![Message {
            role: MessageRole::System,
            content: prompts::system_prompt(
                &config.agent.name,
                &config.agent.version,
                &config.agent.all_report_path,
                &config.creator,
                &config.agent.user_name,
                "(MCP servers initializing... use `discover_tools` to check availability)",
            ),
            timestamp: chrono::Utc::now(),
        }];

        tracing::info!(
            architect = %config.creator.name,
            organization = %config.creator.organization,
            clearance = %config.creator.clearance_level,
            "Neural lineage synchronized. Agent authorized for high-stakes operations."
        );

        Ok(Self {
            nim_client,
            mcp_server,
            recon_graph,
            config,
            messages,
            session: None,
            memory,
            generator,
            event_tx: None,
            missing_tools: Arc::new(Mutex::new(HashSet::new())),
        })
    }

    /// Set the TUI event sender for live HUD updates.
    pub fn with_event_tx(mut self, tx: mpsc::UnboundedSender<TuiEvent>) -> Self {
        self.event_tx = Some(tx);
        self
    }

    /// Dynamically update the TUI event transmitter (useful for CLI mode multiplexing).
    pub fn set_event_tx(&mut self, tx: Option<mpsc::UnboundedSender<TuiEvent>>) {
        self.event_tx = tx;
    }

    /// Emit an event to the TUI if a sender is configured.
    fn emit(&self, event: TuiEvent) {
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(event);
        }
    }

    /// Start a new reconnaissance session on a target.
    pub async fn start_session(&mut self, target: &str, profile: &str) {
        // Create and initialize session
        let mut session = Session::new(target, profile, &self.config);
        if let Err(e) = session.init().await {
            tracing::warn!(error = %e, "Session workspace init failed, continuing without workspace");
        }

        // Persist mission metadata for standalone CLI parity
        let graph = self.recon_graph.lock().await;
        if let Err(e) = session.persist_metadata(&graph).await {
            tracing::error!(error = %e, "Mission context persistence failed");
        }
        drop(graph);

        self.session = Some(session);
        self.recon_graph = Arc::new(Mutex::new(ReconGraph::new(
            target,
            self.config.agent.max_iterations,
        )));

        // Resolve the profile config for User Mode tool restrictions
        let profile_config = self.config.profiles.get(profile).cloned();
        let session_prompt =
            prompts::session_start_prompt(target, profile, profile_config.as_ref());
        self.messages.push(Message {
            role: MessageRole::User,
            content: session_prompt.clone(),
            timestamp: chrono::Utc::now(),
        });

        self.emit(TuiEvent::TargetUpdate(target.to_string()));
        self.emit(TuiEvent::StateUpdate("PLANNING".to_string()));
        self.emit(TuiEvent::Message {
            role: "system".to_string(),
            content: format!("Neural link established with target: {}", target),
        });

        tracing::info!(target, profile, "New reconnaissance session started");
    }

    /// Clear the current reconnaissance session and memory.
    pub async fn clear_session(&mut self) {
        self.messages.clear();
        self.messages.push(Message {
            role: MessageRole::System,
            content: prompts::system_prompt(
                &self.config.agent.name,
                &self.config.agent.version,
                &self.config.agent.all_report_path,
                &self.config.creator,
                &self.config.agent.user_name,
                &self.build_mcp_server_info().await,
            ),
            timestamp: chrono::Utc::now(),
        });

        self.session = None;
        self.emit(TuiEvent::ClearChat);
        self.emit(TuiEvent::TargetUpdate("".to_string()));
        self.emit(TuiEvent::StateUpdate("IDLE".to_string()));
        self.emit(TuiEvent::Message {
            role: "system".to_string(),
            content: "Session memory and tactical context cleared. Awaiting new orders."
                .to_string(),
        });

        tracing::info!("Reconnaissance session and neural metadata cleared by user");
    }

    /// Process a user message/command.
    pub fn add_user_message(&mut self, content: &str) {
        self.messages.push(Message {
            role: MessageRole::User,
            content: content.to_string(),
            timestamp: chrono::Utc::now(),
        });
    }

    /// Synchronize the agent with a persisted mission state.
    pub async fn restore_session(&mut self, meta: crate::core::session::MissionMetadata) {
        let mut session = Session::new(&meta.target, &meta.profile, &self.config);
        session.id = meta.session_id;
        session.operator_name = meta.operator_name;

        // Note: Workspace is volatile, but we reconstruct session pointers
        self.session = Some(session);
        self.recon_graph = Arc::new(Mutex::new(meta.graph));

        tracing::info!(target = %meta.target, "Re-synchronized with mission context");
    }

    /// Get the current recon state.
    pub async fn recon_state(&self) -> ReconState {
        let graph = self.recon_graph.lock().await;
        graph.state().clone()
    }

    /// Get the recon graph for inspection.
    pub fn recon_graph(&self) -> Arc<Mutex<ReconGraph>> {
        self.recon_graph.clone()
    }

    /// Set the max iterations for the reconnaissance agent.
    pub async fn set_max_iterations(&mut self, iterations: u32) {
        self.config.agent.max_iterations = iterations;
        let mut graph = self.recon_graph.lock().await;
        graph.set_max_iterations(iterations);
    }

    /// Get the MCP server.
    pub fn mcp_server(&self) -> &McpServer {
        &self.mcp_server
    }

    /// Get the NIM client.
    pub fn nim_client(&self) -> &NimClient {
        &self.nim_client
    }

    /// Get conversation history.
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    /// Get the config.
    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    /// Update the agent's internal configuration.
    pub fn config_mut(&mut self) -> &mut AppConfig {
        &mut self.config
    }

    /// Synchronize MCP tools based on the current configuration.
    pub async fn sync_mcp(&mut self) {
        self.mcp_server.reload_tools().await;
        self.mcp_server.sync_clients(&self.config).await;

        // Emit updated tool list to TUI
        let clients = self.mcp_server.clients();
        let cm = clients.read().await;
        let all_tools = cm.list_all_tools().await;
        let mut tool_names = Vec::new();
        for tools in all_tools.values() {
            for t in tools {
                if let Some(name) = t["name"].as_str() {
                    tool_names.push(name.to_string());
                }
            }
        }
        tool_names.sort();
        self.emit(TuiEvent::ToolDiscoveryUpdate(tool_names));
    }

    /// Hot-reload the configuration from disk and re-sync MCP tools.
    /// Supports delta reloads for elite performance.
    pub async fn reload_config(
        &mut self,
        event: Option<crate::config::watcher::ConfigUpdateEvent>,
    ) -> Result<(), AgentError> {
        let new_config = AppConfig::load_merged()
            .map_err(|e| AgentError::Session(format!("Config reload failed: {}", e)))?;

        self.config = new_config;

        match event {
            Some(crate::config::watcher::ConfigUpdateEvent::McpRegistry) => {
                tracing::info!(
                    "Delta reload: Neutral link registry synchronization in progress..."
                );
                self.sync_mcp().await;
            }
            Some(crate::config::watcher::ConfigUpdateEvent::UserConfig) => {
                tracing::info!("Delta reload: Global operator parameters updated.");
                // Some user config changes might require tool re-sync (e.g. tool_paths)
                self.sync_mcp().await;
            }
            None => {
                // Global reload
                self.sync_mcp().await;
            }
        }

        // Notify TUI of the refresh
        self.emit(TuiEvent::ConfigReloaded(Box::new(self.config.clone())));

        tracing::info!("Agent configuration hot-reloaded successfully");
        Ok(())
    }

    /// Get the active session, if any.
    pub fn session(&self) -> Option<&Session> {
        self.session.as_ref()
    }

    /// Add an assistant message to history.
    pub fn add_assistant_message(&mut self, content: &str) {
        self.messages.push(Message {
            role: MessageRole::Assistant,
            content: content.to_string(),
            timestamp: chrono::Utc::now(),
        });
    }

    /// Add a tool result to history.
    pub fn add_tool_result(&mut self, tool_name: &str, content: &str) {
        self.messages.push(Message {
            role: MessageRole::Tool(tool_name.to_string()),
            content: content.to_string(),
            timestamp: chrono::Utc::now(),
        });
    }

    /// Clean up the current session safely.
    pub async fn cleanup(&mut self) {
        if let Some(mut session) = self.session.take() {
            if let Err(e) = session.cleanup().await {
                tracing::warn!(error = %e, "Session cleanup failed");
            }
        } else {
            // Even if no active session in memory, try to clear persisted context
            let ctx_path = AppConfig::mission_context_path();
            if ctx_path.exists() {
                let _ = std::fs::remove_file(ctx_path);
            }
        }

        // --- Industrial Resource Destruction ---
        self.mcp_server.shutdown().await;
        self.memory.clear();

        tracing::info!("Agent cleanup complete — all volatile data destroyed");
    }

    /// Fully reset the agent state, clearing history, session, and memory.
    pub async fn reset_session(&mut self) {
        // Clear messages and re-add system prompt
        self.messages = vec![Message {
            role: MessageRole::System,
            content: prompts::system_prompt(
                &self.config.agent.name,
                &self.config.agent.version,
                &self.config.agent.all_report_path,
                &self.config.creator,
                &self.config.agent.user_name,
                &self.build_mcp_server_info().await,
            ),
            timestamp: chrono::Utc::now(),
        }];

        // Clean up current session files if active
        if let Some(ref mut session) = self.session {
            let _ = session.cleanup().await;
        }

        // Reset session and graph
        self.session = None;
        self.recon_graph = Arc::new(Mutex::new(ReconGraph::new(
            "",
            self.config.agent.max_iterations,
        )));

        // Clear memory
        self.memory.clear();

        self.emit(TuiEvent::TargetUpdate("(no target)".to_string()));
        self.emit(TuiEvent::StateUpdate("IDLE".to_string()));

        tracing::info!("Agent session reset — memory and history purged");
    }

    /// Resolve the workspace path, creating it if needed.
    fn resolve_workspace(&self) -> std::path::PathBuf {
        let workspace_path = self
            .session
            .as_ref()
            .map(|s| s.workspace.path().to_path_buf())
            .unwrap_or_else(|| {
                std::env::temp_dir().join(format!(
                    "{}-workspace-fallback",
                    self.config.agent.name.to_lowercase()
                ))
            });
        if !workspace_path.exists() {
            let _ = std::fs::create_dir_all(&workspace_path);
        }
        workspace_path
    }

    /// Build a formatted string listing connected MCP servers and their tools.
    async fn build_mcp_server_info(&self) -> String {
        let clients = self.mcp_server.clients();
        let cm = clients.read().await;
        let names = cm.list_client_names();
        if names.is_empty() {
            return "No MCP servers currently connected. Use `discover_tools` to check if servers are still initializing.".to_string();
        }

        let mut info = String::new();
        // Global safety: 10s timeout for all MCP tool aggregation
        let all_tools =
            match tokio::time::timeout(std::time::Duration::from_secs(10), cm.list_all_tools())
                .await
            {
                Ok(res) => res,
                Err(_) => {
                    tracing::warn!("Tactical: Global MCP tool discovery timed out (10s)");
                    HashMap::new()
                }
            };
        for name in &names {
            let tools = all_tools.get(name);
            let tool_list = if let Some(tools) = tools {
                tools
                    .iter()
                    .filter_map(|t| t["name"].as_str())
                    .map(|n| format!("`{}/{}`", name, n))
                    .collect::<Vec<_>>()
                    .join(", ")
            } else {
                "(loading tools...)".to_string()
            };
            info.push_str(&format!("- **{}**: {}\n", name, tool_list));
        }
        info
    }

    /// Build the complete Rig agent with ALL tool bridges attached.
    /// Single source of truth — used by chat(), chat_stream(), and retry paths.
    async fn build_rig_agent(
        &self,
        executed_tools: Arc<tokio::sync::Mutex<Vec<(String, String)>>>,
        workspace_path: std::path::PathBuf,
    ) -> rig::agent::Agent<rig::providers::openai::CompletionModel> {
        use crate::llm::tools::{
            BridgeContext, DiscoverToolsBridge, ExecuteToolBridge, GetToolHelpBridge,
        };

        let bridge_ctx = BridgeContext {
            executor: self.mcp_server.executor(),
            discovery: self.mcp_server.discovery(),
            clients: self.mcp_server.clients(),
            executed_tools: executed_tools.clone(),
            missing_tools: self.missing_tools.clone(),
            workspace_path: workspace_path.clone(),
            event_tx: self.event_tx.clone(),
            builtin_registry: self.mcp_server.builtin_registry(),
            rate_limit_ms: self.config.llm.rate_limit_ms,
        };

        let execute_tool = ExecuteToolBridge::new(bridge_ctx)
            .with_memory(self.memory_arc(), self.generator.clone());

        // Re-instantiate ctx for batch since tool builders consume it or need their own access
        // Actually BridgeContext is cheap to clone if we add Clone, but let's just make another one for clarity or clone the arcs
        let bridge_ctx_batch = BridgeContext {
            executor: self.mcp_server.executor(),
            discovery: self.mcp_server.discovery(),
            clients: self.mcp_server.clients(),
            executed_tools: executed_tools.clone(),
            missing_tools: self.missing_tools.clone(),
            workspace_path,
            event_tx: self.event_tx.clone(),
            builtin_registry: self.mcp_server.builtin_registry(),
            rate_limit_ms: self.config.llm.rate_limit_ms,
        };

        let execute_batch = crate::llm::tools::ExecuteBatchBridge::new(bridge_ctx_batch)
            .with_memory(self.memory_arc(), self.generator.clone());

        let discover_tools = DiscoverToolsBridge::new(
            self.mcp_server.discovery(),
            self.mcp_server.clients(),
            self.mcp_server.builtin_registry(),
            self.event_tx.clone(),
        );
        let get_tool_help =
            GetToolHelpBridge::new(self.mcp_server.discovery(), self.event_tx.clone());
        let report_phase = crate::llm::tools::ReportPhaseCompletionBridge::new(
            self.recon_graph.clone(),
            self.event_tx.clone(),
        );
        let report_finding = crate::llm::tools::ReportFindingBridge::new(
            self.recon_graph.clone(),
            self.event_tx.clone(),
        );
        let search_memory = crate::llm::tools::SearchMemoryBridge::new(
            self.memory_arc(),
            self.generator.clone(),
            self.event_tx.clone(),
        );

        // Full MCP Protocol Bridges
        let list_resources = crate::llm::tools::ListResourcesBridge::new(
            self.mcp_server.clients(),
            self.event_tx.clone(),
        );
        let read_resource = crate::llm::tools::ReadResourceBridge::new(
            self.mcp_server.clients(),
            self.event_tx.clone(),
        );
        let list_prompts = crate::llm::tools::ListPromptsBridge::new(
            self.mcp_server.clients(),
            self.event_tx.clone(),
        );
        let get_prompt = crate::llm::tools::GetPromptBridge::new(
            self.mcp_server.clients(),
            self.event_tx.clone(),
        );

        // Native Utility Bridges
        let generate_file = crate::llm::tools::GenerateFileBridge::new(
            self.mcp_server.builtin_registry(),
            self.event_tx.clone(),
        );
        let append_to_file = crate::llm::tools::AppendToFileBridge::new(
            self.mcp_server.builtin_registry(),
            self.event_tx.clone(),
        );
        let generate_secure_asset = crate::llm::tools::GenerateSecureAssetBridge::new(
            self.mcp_server.builtin_registry(),
            self.event_tx.clone(),
        );
        let generate_batch = crate::llm::tools::GenerateBatchBridge::new(
            self.mcp_server.builtin_registry(),
            self.event_tx.clone(),
        );
        let generate_secure_batch = crate::llm::tools::GenerateSecureBatchBridge::new(
            self.mcp_server.builtin_registry(),
            self.event_tx.clone(),
        );
        let patch_json = crate::llm::tools::PatchJsonBridge::new(
            self.mcp_server.builtin_registry(),
            self.event_tx.clone(),
        );
        let read_mmap = crate::llm::tools::ReadMmapBridge::new(self.mcp_server.builtin_registry());
        let browse = crate::llm::tools::BrowseBridge::new(
            self.mcp_server.builtin_registry(),
            self.event_tx.clone(),
        );
        let web_action = crate::llm::tools::WebActionBridge::new(
            self.mcp_server.builtin_registry(),
            self.event_tx.clone(),
        );
        let web_login = crate::llm::tools::WebLoginBridge::new(
            self.mcp_server.builtin_registry(),
            self.event_tx.clone(),
        );
        let generate_payload = crate::llm::tools::GeneratePayloadBridge::new(
            self.mcp_server.builtin_registry(),
            self.event_tx.clone(),
        );
        let generate_payload_file = crate::llm::tools::GeneratePayloadFileBridge::new(
            self.mcp_server.builtin_registry(),
            self.event_tx.clone(),
        );
        let generate_with_metadata = crate::llm::tools::GenerateWithMetadataBridge::new(
            self.mcp_server.builtin_registry(),
            self.event_tx.clone(),
        );
        let generate_compressed = crate::llm::tools::GenerateCompressedBridge::new(
            self.mcp_server.builtin_registry(),
            self.event_tx.clone(),
        );
        let generate_compressed_batch = crate::llm::tools::GenerateCompressedBatchBridge::new(
            self.mcp_server.builtin_registry(),
            self.event_tx.clone(),
        );
        let get_statistics = crate::llm::tools::GetStatisticsBridge::new(
            self.mcp_server.builtin_registry(),
            self.event_tx.clone(),
        );

        self.nim_client
            .client()
            .agent(self.nim_client.model())
            .default_max_turns(self.config.agent.max_iterations as usize)
            .preamble(&prompts::system_prompt(
                &self.config.agent.name,
                &self.config.agent.version,
                &self.config.agent.all_report_path,
                &self.config.creator,
                &self.config.agent.user_name,
                &self.build_mcp_server_info().await,
            ))
            .max_tokens(self.config.llm.max_tokens as u64)
            .temperature(self.config.llm.temperature as f64)
            .tool(execute_tool)
            .tool(execute_batch)
            .tool(discover_tools)
            .tool(get_tool_help)
            .tool(report_phase)
            .tool(report_finding)
            .tool(search_memory)
            .tool(list_resources)
            .tool(read_resource)
            .tool(list_prompts)
            .tool(get_prompt)
            .tool(generate_file)
            .tool(append_to_file)
            .tool(browse)
            .tool(web_action)
            .tool(web_login)
            .tool(generate_secure_asset)
            .tool(generate_batch)
            .tool(generate_secure_batch)
            .tool(patch_json)
            .tool(read_mmap)
            .tool(generate_payload)
            .tool(generate_payload_file)
            .tool(generate_with_metadata)
            .tool(generate_compressed)
            .tool(generate_compressed_batch)
            .tool(get_statistics)
            .build()
    }

    /// Convert internal message history to Rig format.
    /// Skips the system prompt (handled by preamble) and the last message (current prompt).
    fn build_history(&self) -> Vec<RigMessage> {
        let mut history = vec![];
        let msg_count = self.messages.len();

        // Sliding Window (Industry-Grade Memory Management)
        // We skip the last message (current user prompt handled separately by Rig)
        // and enforce the max_history_turns limit.
        let limit = self.config.agent.max_history_turns;
        let skip_count = if msg_count > limit + 1 {
            msg_count - limit - 1
        } else {
            0
        };

        if skip_count > 0 {
            tracing::debug!(
                skipped = skip_count,
                limit = limit,
                "Sliding context window active: purging stale short-term turns"
            );
        }

        for (i, msg) in self.messages.iter().enumerate().skip(skip_count) {
            if i == msg_count - 1 {
                continue;
            }
            match msg.role {
                MessageRole::User => {
                    history.push(RigMessage::User {
                        content: OneOrMany::one(UserContent::text(msg.content.clone())),
                    });
                }
                MessageRole::Assistant => {
                    history.push(RigMessage::Assistant {
                        id: None,
                        content: OneOrMany::one(AssistantContent::text(msg.content.clone())),
                    });
                }
                MessageRole::Tool(ref name) => {
                    history.push(RigMessage::User {
                        content: OneOrMany::one(UserContent::text(format!(
                            "[Tool Output - {}]\n{}",
                            name, msg.content
                        ))),
                    });
                }
                MessageRole::System => {
                    history.push(RigMessage::User {
                        content: OneOrMany::one(UserContent::text(format!(
                            "[System Event]\n{}",
                            msg.content
                        ))),
                    });
                }
            }
        }
        history
    }

    /// Process a message through the Rig Agent with full tool-calling support.
    pub async fn chat(&mut self, prompt: &str) -> Result<String, AgentError> {
        use rig::completion::Chat;

        self.add_user_message(prompt);

        if self.config.memory.enabled {
            self.store_memory(
                prompt,
                MemoryEntryType::Note,
                None,
                serde_json::json!({"role": "user"}),
            )
            .await;
        }

        let workspace_path = self.resolve_workspace();
        let executed_tools =
            std::sync::Arc::new(tokio::sync::Mutex::new(Vec::<(String, String)>::new()));
        let ai_agent = self
            .build_rig_agent(executed_tools.clone(), workspace_path)
            .await;
        let history = self.build_history();

        let timeout_duration =
            tokio::time::Duration::from_secs(self.config.agent.timeout_seconds + 60);

        let response =
            match tokio::time::timeout(timeout_duration, ai_agent.chat(prompt, history.clone()))
                .await
            {
                Ok(Ok(resp)) => resp,
                _ => {
                    tracing::warn!("LLM call failed or timed out, rotating and retrying...");
                    if self.nim_client.client_count() > 1 {
                        self.nim_client.rotate_key();
                    } else {
                        self.nim_client.rotate_model();
                    }

                    // Rebuild with rotated credentials — same full tool set
                    let workspace_path2 = self.resolve_workspace();
                    let executed_tools2 =
                        std::sync::Arc::new(
                            tokio::sync::Mutex::new(Vec::<(String, String)>::new()),
                        );
                    let retry_agent = self.build_rig_agent(executed_tools2, workspace_path2).await;

                    match tokio::time::timeout(timeout_duration, retry_agent.chat(prompt, history))
                        .await
                    {
                        Ok(Ok(resp)) => resp,
                        Ok(Err(e)) => return Err(AgentError::Llm(format!("Retry failed: {}", e))),
                        Err(_) => {
                            tracing::error!("Retry also timed out. Forcing model rotation.");
                            self.nim_client.rotate_model();
                            return Err(AgentError::Llm(
                                "LLM response timed out twice. Rotated model for next attempt."
                                    .to_string(),
                            ));
                        }
                    }
                }
            };

        self.add_assistant_message(&response);

        if self.config.memory.enabled {
            self.store_memory(
                &response,
                MemoryEntryType::Analysis,
                None,
                serde_json::json!({"role": "assistant"}),
            )
            .await;
        }
        Ok(response)
    }

    /// Get shared lock to memory.
    pub fn memory_arc(&self) -> Arc<InMemoryStore> {
        self.memory.clone()
    }

    /// Get the current length of the memory store.
    pub fn memory_len(&self) -> usize {
        self.memory.len()
    }

    /// Helper to store memory with automatic vector generation.
    pub async fn store_memory(
        &mut self,
        text: &str,
        entry_type: MemoryEntryType,
        tool_name: Option<String>,
        metadata: serde_json::Value,
    ) {
        if !self.config.memory.enabled {
            return;
        }

        let gen = self.generator.clone();
        let mem = self.memory.clone();
        let content = text.to_string();
        let target = self.session.as_ref().map(|s| s.target.clone());
        let timestamp = chrono::Utc::now().to_rfc3339();

        // Zero-Latency Lightning Offloading
        tokio::spawn(async move {
            let vector = gen.generate(&content).await;
            let entry = MemoryEntry {
                id: uuid::Uuid::new_v4().to_string(),
                content,
                entry_type,
                tool_name,
                target,
                timestamp,
                metadata,
                vector: Some(vector),
            };
            let m = mem.clone();
            let _ = m.store(entry).await;
        });
    }

    /// Print summary of executed tools.
    pub async fn print_tool_summary(
        &self,
        executed_tools: Arc<tokio::sync::Mutex<Vec<(String, String)>>>,
    ) {
        let executed = executed_tools.lock().await;
        if !executed.is_empty() {
            crate::ui::print_audit(&executed);
        }

        // Professional missing tool notification
        let missing = self.missing_tools.lock().await;
        if !missing.is_empty() {
            use owo_colors::OwoColorize;

            println!(
                "\n  {}",
                "┌───────────────────────────────────────────────────────────┐".bright_black()
            );
            println!(
                "  {}  {} {}",
                "│".bright_black(),
                "⚠️".yellow(),
                "SYSTEM ADVISORY: Missing External Dependencies"
                    .bold()
                    .yellow()
            );
            println!(
                "  {}",
                "├───────────────────────────────────────────────────────────┤".bright_black()
            );
            println!(
                "  {}  The agent attempted to use tools not found on this system:",
                "│".bright_black()
            );

            for tool in missing.iter() {
                println!("  {}  • {}", "│".bright_black(), tool.bright_cyan().bold());
            }

            println!("  {} ", "│".bright_black());
            println!(
                "  {}  {} To restore full capability, install the missing tools.",
                "│".bright_black(),
                "TIP:".green().italic()
            );

            // Smarter suggestions
            let mut suggest_sudo = false;
            let mut commands = Vec::new();
            for tool in missing.iter() {
                let pkg = match tool.as_str() {
                    "nmap" => Some("nmap"),
                    "whois" => Some("whois"),
                    "dig" => Some("dnsutils"),
                    "host" => Some("bind9-host"),
                    "curl" => Some("curl"),
                    "subfinder" => Some("subfinder"),
                    "nuclei" => Some("nuclei"),
                    "httpx" => Some("httpx-toolkit"),
                    "amass" => Some("amass"),
                    _ => None,
                };
                if let Some(p) = pkg {
                    commands.push(p);
                    suggest_sudo = true;
                }
            }

            if suggest_sudo {
                let is_termux = std::env::var("PREFIX")
                    .map(|p| p.contains("com.termux"))
                    .unwrap_or(false);
                let cmd = if is_termux {
                    format!("pkg install {}", commands.join(" "))
                } else if std::process::Command::new("which")
                    .arg("dnf")
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false)
                {
                    format!("sudo dnf install -y {}", commands.join(" "))
                } else if std::process::Command::new("which")
                    .arg("pacman")
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false)
                {
                    format!("sudo pacman -S --noconfirm {}", commands.join(" "))
                } else {
                    format!(
                        "sudo apt update && sudo apt install -y {}",
                        commands.join(" ")
                    )
                };

                println!(
                    "  {}  {} {}",
                    "│".bright_black(),
                    "Run:".dimmed(),
                    cmd.bright_white().italic()
                );
            }

            println!(
                "  {}  {} Run {} for a detailed health check.",
                "│".bright_black(),
                "      ".dimmed(),
                "myth check".bold().cyan()
            );
            println!(
                "  {}",
                "└───────────────────────────────────────────────────────────┘".bright_black()
            );
            println!();
        }
    }

    /// Process a stream of messages through the Rig Agent for live transparency.
    pub async fn chat_stream(
        &mut self,
        prompt: &str,
    ) -> Result<
        (
            impl futures::Stream<Item = Result<String, AgentError>>,
            Arc<tokio::sync::Mutex<Vec<(String, String)>>>,
        ),
        AgentError,
    > {
        self.add_user_message(prompt);

        // Reset execution limit for this turn
        self.mcp_server.executor().reset_redundancy_monitor();

        if self.config.memory.enabled {
            self.store_memory(
                prompt,
                MemoryEntryType::Note,
                None,
                serde_json::json!({"role": "user"}),
            )
            .await;
        }

        let workspace_path = self.resolve_workspace();
        let executed_tools =
            std::sync::Arc::new(tokio::sync::Mutex::new(Vec::<(String, String)>::new()));
        let ai_agent = self
            .build_rig_agent(executed_tools.clone(), workspace_path)
            .await;
        let history = self.build_history();

        use futures::StreamExt;

        let stream = match tokio::time::timeout(
            tokio::time::Duration::from_secs(30),
            ai_agent.stream_chat(prompt, history),
        )
        .await
        {
            Ok(s) => s,
            Err(_) => {
                tracing::error!(
                    "LLM stream connection timed out. Rotating model for next attempt."
                );
                self.nim_client.rotate_model();
                return Err(AgentError::Llm("LLM stream connection timed out. We've rotated the model; please try your request again.".to_string()));
            }
        };

        let mapped_stream = stream.map(|res| match res {
            Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Text(
                text_obj,
            ))) => Ok(text_obj.text),
            Ok(_) => Ok(String::new()),
            Err(e) => Err(AgentError::Llm(e.to_string())),
        });

        Ok((mapped_stream, executed_tools))
    }

    /// Execute a tool directly bypassing the LLM.
    /// This is used for high-performance tactical commands like /subdomains.
    pub async fn execute_tool_directly(
        &self,
        tool_name: &str,
        arguments: serde_json::Value,
    ) -> Result<serde_json::Value, AgentError> {
        let input = crate::mcp::schemas::ExecuteToolInput {
            binary: tool_name.to_string(),
            args: vec![], // Not used for structured calls
            structured_args: Some(arguments),
            working_dir: None,
        };

        let result = self.mcp_server.handle_execute(input).await;

        // Log to telemetry if enabled
        if let Some(ref tx) = self.event_tx {
            let _ = tx.send(TuiEvent::ToolExecution {
                name: tool_name.to_string(),
                status: if result["success"].as_bool().unwrap_or(true) {
                    "success".to_string()
                } else {
                    "failure".to_string()
                },
            });
        }

        Ok(result)
    }
}
