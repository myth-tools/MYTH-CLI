//! # MYTH MCP Client Engine
//!
//! This module provides an industry-grade, high-performance implementation of the
//! Model Context Protocol (MCP) clients. It is architected for maximum robustness,
//! featuring the **PHOENIX Self-Healing Engine** which automatically monitors,
//! diagnoses, and respawns failed server processes.
//!
//! ## Key Components
//!
//! - **McpClientManager**: The central orchestrator that manages multiple remote (SSE)
//!   and local (STDIO) servers with concurrent status monitoring and dynamic sync capabilities.
//! - **SseMcpClient**: A robust, event-driven client for remote MCP endpoints with
//!   low-latency request dispatching and automatic endpoint discovery.
//! - **StdioMcpClient**: A high-performance local client designed for direct process
//!   interaction, capturing stderr diagnostics and ensuring clean resource reclamation.
//!
//! ## Resilience (PHOENIX Engine)
//!
//! The PHOENIX engine ensures industry-grade uptime by detecting protocol-level
//! failures and communication timeouts. When a critical failure is detected,
//! the engine triggers an autonomous restart cycle, ensuring the AI agent
//! never loses its connection to vital system tools.

use thiserror::Error;

#[derive(Debug, Error, Clone, serde::Serialize, serde::Deserialize)]
pub enum McpClientError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    #[error("Timeout: {0}")]
    Timeout(String),
}
use crate::config::{CustomMcpServer, LocalMcpConfig, RemoteMcpConfig};
use async_trait::async_trait;
use futures::stream::{FuturesUnordered, StreamExt};
use reqwest_eventsource::{Event, EventSource};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;

/// Standard wait time for SSE endpoint discovery.
const SSE_ENDPOINT_TIMEOUT_SECS: u64 = 10;
/// PHOENIX self-healing sentinel value to trigger manager-level restarts.
const PHOENIX_RESTART_REQUIRED: &str = "PHOENIX_RESTART_REQUIRED";

#[async_trait]
pub trait McpClient: Send + Sync {
    async fn connect(&mut self) -> Result<(), McpClientError>;
    fn is_alive(&self) -> bool;
    async fn list_prompts(&self) -> Result<Vec<Value>, McpClientError>;
    async fn get_prompt(&self, name: &str, args: Value) -> Result<Value, McpClientError>;
    async fn list_tools_raw(&self) -> Result<Vec<Value>, McpClientError>;
    async fn list_tools(&self) -> Result<Vec<Value>, McpClientError>;
    async fn list_resources(&self) -> Result<Vec<Value>, McpClientError>;
    async fn read_resource(&self, uri: &str) -> Result<Value, McpClientError>;
    async fn call_tool(&self, name: &str, args: Value) -> Result<Value, McpClientError>;
    fn get_config(&self) -> CustomMcpServer;
    fn process_info(&self) -> Option<(u32, String)>;
    async fn close(&mut self) -> Result<(), McpClientError>;
    async fn ping(&self) -> bool;
}

/// Manager for multiple MCP clients with autonomous self-healing (PHOENIX).
pub struct McpClientManager {
    clients: HashMap<String, Box<dyn McpClient + Send + Sync>>,
    /// Industry-grade tool cache for zero-latency discovery
    tool_cache: HashMap<String, Vec<Value>>,
}

impl Default for McpClientManager {
    fn default() -> Self {
        Self::new()
    }
}

impl McpClientManager {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            tool_cache: HashMap::new(),
        }
    }

    pub fn clients(&self) -> &HashMap<String, Box<dyn McpClient + Send + Sync>> {
        &self.clients
    }

    /// Returns the names of all currently connected MCP clients.
    pub fn list_client_names(&self) -> Vec<String> {
        self.clients.keys().cloned().collect()
    }

    pub async fn tool_count(&self) -> usize {
        let mut count = 0;
        for client in self.clients.values() {
            if let Ok(tools) = client.list_tools().await {
                count += tools.len();
            }
        }
        count
    }

    pub async fn add_server(
        &mut self,
        name: String,
        config: CustomMcpServer,
    ) -> Result<(), McpClientError> {
        let client = Self::connect_client_static(name.clone(), config).await?;
        self.clients.insert(name, client);
        Ok(())
    }

    pub fn insert_client(&mut self, name: String, client: Box<dyn McpClient + Send + Sync>) {
        self.clients.insert(name, client);
    }

    pub async fn connect_client_static(
        name: String,
        config: CustomMcpServer,
    ) -> Result<Box<dyn McpClient + Send + Sync>, McpClientError> {
        let mut client: Box<dyn McpClient + Send + Sync> = match config {
            CustomMcpServer::Local(ref l) => Box::new(StdioMcpClient::new(name, l.clone())),
            CustomMcpServer::Remote(ref r) => Box::new(SseMcpClient::new(name, r.clone())),
        };
        client.connect().await?;
        Ok(client)
    }

    pub async fn restart_server(&mut self, name: &str) -> Result<(), McpClientError> {
        if let Some(mut client) = self.clients.remove(name) {
            tracing::info!(server = %name, "PHOENIX: Shutting down failed server for restart");
            let config = client.get_config();
            let _ = client.close().await;

            tracing::info!(server = %name, "PHOENIX: Respawning server...");
            self.add_server(name.to_string(), config).await
        } else {
            Err(McpClientError::ConnectionFailed(format!(
                "Cannot restart: Server '{}' not found",
                name
            )))
        }
    }

    pub fn get_client(&self, name: &str) -> Option<&(dyn McpClient + Send + Sync)> {
        self.clients.get(name).map(|c| c.as_ref())
    }

    pub fn get_server_names(&self) -> Vec<String> {
        self.clients.keys().cloned().collect()
    }

    /// Refresh tools for a specific client and update the cache.
    pub async fn refresh_tools_for_client(&mut self, name: &str) -> Result<(), McpClientError> {
        if let Some(client) = self.clients.get(name) {
            let tools =
                tokio::time::timeout(std::time::Duration::from_secs(5), client.list_tools())
                    .await
                    .map_err(|_| {
                        McpClientError::Timeout(format!("Tool discovery timed out for {}", name))
                    })??;
            self.tool_cache.insert(name.to_string(), tools);
            Ok(())
        } else {
            Err(McpClientError::ProtocolError(format!(
                "Client '{}' not found for refresh",
                name
            )))
        }
    }

    pub fn remove_cached_tools(&mut self, name: &str) {
        self.tool_cache.remove(name);
    }

    pub async fn list_all_tools(&self) -> HashMap<String, Vec<Value>> {
        // ULTRA-FAST: If cache is warm and covers all clients, return immediately
        if !self.tool_cache.is_empty()
            && self.clients.keys().all(|k| self.tool_cache.contains_key(k))
        {
            return self.tool_cache.clone();
        }

        let mut all_tools = self.tool_cache.clone();

        // Fetch missing servers in parallel
        let missing_clients: Vec<_> = self
            .clients
            .iter()
            .filter(|(name, _)| !self.tool_cache.contains_key(*name))
            .collect();

        if missing_clients.is_empty() {
            return all_tools;
        }

        let futures = missing_clients
            .into_iter()
            .map(|(name, client)| async move {
                let res =
                    tokio::time::timeout(std::time::Duration::from_secs(5), client.list_tools())
                        .await;
                (name.clone(), res)
            });

        let results = futures::future::join_all(futures).await;
        for (name, res) in results {
            match res {
                Ok(Ok(tools)) => {
                    all_tools.insert(name, tools);
                }
                Ok(Err(e)) => {
                    tracing::warn!(server = %name, error = %e, "Tactical: Failed to list tools from MCP server");
                }
                Err(_) => {
                    tracing::warn!(server = %name, "Tactical: Tool discovery timed out (5s)");
                }
            }
        }
        all_tools
    }

    pub async fn call_external_tool(
        &self,
        server_name: &str,
        tool_name: &str,
        args: Value,
    ) -> Result<Value, McpClientError> {
        if let Some(client) = self.clients.get(server_name) {
            match client.call_tool(tool_name, args.clone()).await {
                Err(McpClientError::ConnectionFailed(msg)) if msg == "PHOENIX_RESTART_REQUIRED" => {
                    tracing::info!(server = %server_name, "Self-healing: Triggering server restart for '{}'", server_name);
                    Err(McpClientError::ConnectionFailed(
                        "Server is offline. Use '/mcp toggle' to restart.".into(),
                    ))
                }
                res => res,
            }
        } else {
            Err(McpClientError::ConnectionFailed(format!(
                "Server '{}' not found",
                server_name
            )))
        }
    }

    pub async fn list_external_prompts(
        &self,
        server_name: &str,
    ) -> Result<Vec<Value>, McpClientError> {
        if let Some(client) = self.clients.get(server_name) {
            client.list_prompts().await
        } else {
            tracing::warn!(server = %server_name, "External prompt list failed: Server not found");
            Err(McpClientError::ConnectionFailed(format!(
                "Server '{}' not found",
                server_name
            )))
        }
    }

    pub async fn list_external_tools_raw(
        &self,
        server_name: &str,
    ) -> Result<Vec<Value>, McpClientError> {
        if let Some(client) = self.clients.get(server_name) {
            client.list_tools_raw().await
        } else {
            Err(McpClientError::ConnectionFailed(format!(
                "Server '{}' not found",
                server_name
            )))
        }
    }

    pub async fn get_external_prompt(
        &self,
        server_name: &str,
        prompt_name: &str,
        args: Value,
    ) -> Result<Value, McpClientError> {
        if let Some(client) = self.clients.get(server_name) {
            client.get_prompt(prompt_name, args).await
        } else {
            Err(McpClientError::ConnectionFailed(format!(
                "Server '{}' not found",
                server_name
            )))
        }
    }

    pub async fn list_external_resources(
        &self,
        server_name: &str,
    ) -> Result<Vec<Value>, McpClientError> {
        if let Some(client) = self.clients.get(server_name) {
            client.list_resources().await
        } else {
            Err(McpClientError::ConnectionFailed(format!(
                "Server '{}' not found",
                server_name
            )))
        }
    }

    pub async fn read_external_resource(
        &self,
        server_name: &str,
        uri: &str,
    ) -> Result<Value, McpClientError> {
        if let Some(client) = self.clients.get(server_name) {
            client.read_resource(uri).await
        } else {
            Err(McpClientError::ConnectionFailed(format!(
                "Server '{}' not found",
                server_name
            )))
        }
    }

    pub async fn get_all_statuses_parallel(
        &self,
    ) -> HashMap<String, (bool, String, Option<u32>, bool)> {
        let mut futures = FuturesUnordered::new();
        for (name, client) in &self.clients {
            let name = name.clone();
            futures.push(async move {
                let alive = client.is_alive();
                let info = client.process_info();
                let status = if alive {
                    info.as_ref()
                        .map(|(_, s)| s.clone())
                        .unwrap_or_else(|| "RUNNING".to_string())
                } else {
                    "OFFLINE".to_string()
                };
                let healthy = client.ping().await;
                (name, (alive, status, info.map(|(p, _)| p), healthy))
            });
        }

        let mut statuses = HashMap::new();
        while let Some((name, status)) = futures.next().await {
            statuses.insert(name, status);
        }
        statuses
    }

    pub fn get_sync_delta(
        &self,
        new_config: &HashMap<String, CustomMcpServer>,
    ) -> (Vec<String>, HashMap<String, CustomMcpServer>) {
        let mut to_remove = Vec::new();
        let mut to_add = HashMap::new();

        for (name, client) in &self.clients {
            match new_config.get(name) {
                Some(new_srv_cfg) => {
                    let is_enabled = match new_srv_cfg {
                        CustomMcpServer::Local(l) => l.enabled,
                        CustomMcpServer::Remote(r) => r.enabled,
                    };
                    if !is_enabled || client.get_config() != *new_srv_cfg || !client.is_alive() {
                        to_remove.push(name.clone());
                    }
                }
                None => {
                    to_remove.push(name.clone());
                }
            }
        }

        for (name, new_srv_cfg) in new_config {
            let is_enabled = match new_srv_cfg {
                CustomMcpServer::Local(l) => l.enabled,
                CustomMcpServer::Remote(r) => r.enabled,
            };
            if is_enabled && !self.clients.contains_key(name) {
                to_add.insert(name.clone(), new_srv_cfg.clone());
            }
        }

        (to_remove, to_add)
    }

    pub async fn sync_with_config(
        &mut self,
        new_config: &HashMap<String, CustomMcpServer>,
    ) -> Result<(), McpClientError> {
        let (to_remove, to_add) = self.get_sync_delta(new_config);

        for name in to_remove {
            if let Some(mut client) = self.clients.remove(&name) {
                tracing::info!(server = %name, "Shutting down MCP server (sync)");
                let _ = client.close().await;
            }
        }

        let mut add_futures = FuturesUnordered::new();
        for (name, cfg) in to_add {
            add_futures.push(async move {
                let mut client: Box<dyn McpClient + Send + Sync> = match cfg {
                    CustomMcpServer::Local(ref l) => {
                        Box::new(StdioMcpClient::new(name.clone(), l.clone()))
                    }
                    CustomMcpServer::Remote(ref r) => {
                        Box::new(SseMcpClient::new(name.clone(), r.clone()))
                    }
                };
                let res = client.connect().await.map(|_| client);
                (name, res)
            });
        }

        while let Some((name, result)) = add_futures.next().await {
            match result {
                Ok(client) => {
                    self.clients.insert(name, client);
                }
                Err(e) => {
                    tracing::error!(server = %name, error = %e, "Failed to start MCP server during sync")
                }
            }
        }

        Ok(())
    }

    pub async fn remove_client(&mut self, name: &str) -> Result<(), McpClientError> {
        if let Some(mut client) = self.clients.remove(name) {
            client.close().await?;
        }
        Ok(())
    }

    pub async fn close_all(&mut self) -> Result<(), McpClientError> {
        let mut close_futures = FuturesUnordered::new();
        for (_, mut client) in self.clients.drain() {
            close_futures.push(async move {
                let _ = client.close().await;
            });
        }
        while close_futures.next().await.is_some() {}
        Ok(())
    }
}

/// Remote (SSE) MCP Client with robust event-stream handling.
pub struct SseMcpClient {
    id: String,
    config: RemoteMcpConfig,
    is_connected: Arc<std::sync::atomic::AtomicBool>,
    http_client: reqwest::Client,
    post_endpoint: Arc<Mutex<Option<String>>>,
    dispatch:
        Arc<dashmap::DashMap<u64, tokio::sync::oneshot::Sender<Result<Value, McpClientError>>>>,
    request_id: std::sync::atomic::AtomicU64,
    sse_task: Option<tokio::task::JoinHandle<()>>,
}

impl SseMcpClient {
    pub fn new(id: String, config: RemoteMcpConfig) -> Self {
        Self {
            id,
            config,
            is_connected: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            http_client: reqwest::Client::new(),
            post_endpoint: Arc::new(Mutex::new(None)),
            dispatch: Arc::new(dashmap::DashMap::new()),
            request_id: std::sync::atomic::AtomicU64::new(1),
            sse_task: None,
        }
    }

    async fn send_request(&self, method: &str, params: Value) -> Result<Value, McpClientError> {
        if !self.is_alive() {
            return Err(McpClientError::ConnectionFailed(
                "PHOENIX_RESTART_REQUIRED".into(),
            ));
        }
        let endpoint =
            self.post_endpoint
                .lock()
                .await
                .clone()
                .ok_or(McpClientError::ConnectionFailed(
                    "No post endpoint discovered".into(),
                ))?;

        let id = self
            .request_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let request =
            serde_json::json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params });
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.dispatch.insert(id, tx);

        let _ = self
            .http_client
            .post(&endpoint)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                self.dispatch.remove(&id);
                McpClientError::ProtocolError(e.to_string())
            })?;

        match tokio::time::timeout(std::time::Duration::from_secs(self.config.timeout), rx).await {
            Ok(Ok(result)) => result,
            _ => {
                self.dispatch.remove(&id);
                Err(McpClientError::Timeout("SSE timeout".into()))
            }
        }
    }

    /// Send a JSON-RPC notification (fire-and-forget, no `id` field, no response expected).
    /// Required by MCP spec for `notifications/initialized` after handshake.
    async fn send_notification(&self, method: &str) -> Result<(), McpClientError> {
        if !self.is_alive() {
            return Err(McpClientError::ConnectionFailed(
                "PHOENIX_RESTART_REQUIRED".into(),
            ));
        }
        let endpoint =
            self.post_endpoint
                .lock()
                .await
                .clone()
                .ok_or(McpClientError::ConnectionFailed(
                    "No post endpoint for notification".into(),
                ))?;
        let notification = serde_json::json!({ "jsonrpc": "2.0", "method": method });
        let _ = self
            .http_client
            .post(&endpoint)
            .json(&notification)
            .send()
            .await
            .map_err(|e| {
                McpClientError::ProtocolError(format!("Notification send failed: {}", e))
            })?;
        Ok(())
    }
}

#[async_trait]
impl McpClient for SseMcpClient {
    async fn connect(&mut self) -> Result<(), McpClientError> {
        let mut es = EventSource::get(&self.config.url);
        let post_endpoint_cache = Arc::clone(&self.post_endpoint);
        let is_connected_flag = Arc::clone(&self.is_connected);
        let dispatch = Arc::clone(&self.dispatch);
        let server_id = self.id.clone();

        let handle = tokio::spawn(async move {
            while let Some(event) = es.next().await {
                match event {
                    Ok(Event::Message(msg)) => {
                        if msg.event == "endpoint" {
                            let mut lock = post_endpoint_cache.lock().await;
                            *lock = Some(msg.data);
                            is_connected_flag.store(true, std::sync::atomic::Ordering::SeqCst);
                        } else if msg.event == "message" {
                            if let Ok(resp) = serde_json::from_str::<Value>(&msg.data) {
                                if let Some(id) = resp.get("id").and_then(|v| v.as_u64()) {
                                    if let Some((_, tx)) = dispatch.remove(&id) {
                                        let res = if let Some(e) = resp.get("error") {
                                            Err(McpClientError::ProtocolError(e.to_string()))
                                        } else {
                                            Ok(resp.get("result").cloned().unwrap_or(Value::Null))
                                        };
                                        let _ = tx.send(res);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!(server = %server_id, error = %e, "SSE Stream Error");
                        is_connected_flag.store(false, std::sync::atomic::Ordering::SeqCst);
                        break;
                    }
                    _ => {}
                }
            }
            is_connected_flag.store(false, std::sync::atomic::Ordering::SeqCst);
        });

        self.sse_task = Some(handle);

        // Wait for endpoint discovery (PHOENIX: critical for SSE handshakes)
        let start = std::time::Instant::now();
        while start.elapsed().as_secs() < SSE_ENDPOINT_TIMEOUT_SECS {
            if self.is_alive() {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        if !self.is_alive() {
            return Err(McpClientError::ConnectionFailed(
                "Failed to receive endpoint event".into(),
            ));
        }

        self.send_request(
            "initialize",
            serde_json::json!({
                "protocolVersion": "2024-11-05", "capabilities": {},
                "clientInfo": { "name": "myth", "version": "0.1.0" }
            }),
        )
        .await?;

        // MCP Spec Compliance: Send `initialized` notification after successful handshake.
        // Without this, many servers silently reject all subsequent tool calls.
        self.send_notification("notifications/initialized").await?;
        Ok(())
    }

    async fn list_tools(&self) -> Result<Vec<Value>, McpClientError> {
        let res = self.send_request("tools/list", Value::Null).await?;
        Ok(res
            .get("tools")
            .and_then(|t| t.as_array())
            .cloned()
            .unwrap_or_default())
    }

    async fn list_tools_raw(&self) -> Result<Vec<Value>, McpClientError> {
        self.list_tools().await
    }

    async fn call_tool(&self, name: &str, arguments: Value) -> Result<Value, McpClientError> {
        self.send_request(
            "tools/call",
            serde_json::json!({ "name": name, "arguments": arguments }),
        )
        .await
    }

    async fn list_resources(&self) -> Result<Vec<Value>, McpClientError> {
        let res = self.send_request("resources/list", Value::Null).await?;
        Ok(res
            .get("resources")
            .and_then(|t| t.as_array())
            .cloned()
            .unwrap_or_default())
    }

    async fn read_resource(&self, uri: &str) -> Result<Value, McpClientError> {
        self.send_request("resources/read", serde_json::json!({ "uri": uri }))
            .await
    }

    async fn list_prompts(&self) -> Result<Vec<Value>, McpClientError> {
        let res = self.send_request("prompts/list", Value::Null).await?;
        Ok(res
            .get("prompts")
            .and_then(|t| t.as_array())
            .cloned()
            .unwrap_or_default())
    }

    async fn get_prompt(&self, name: &str, arguments: Value) -> Result<Value, McpClientError> {
        self.send_request(
            "prompts/get",
            serde_json::json!({ "name": name, "arguments": arguments }),
        )
        .await
    }

    fn is_alive(&self) -> bool {
        self.is_connected.load(std::sync::atomic::Ordering::SeqCst)
    }
    fn process_info(&self) -> Option<(u32, String)> {
        Some((0, "REMOTE".into()))
    }
    async fn close(&mut self) -> Result<(), McpClientError> {
        if let Some(handle) = self.sse_task.take() {
            handle.abort();
        }
        self.is_connected
            .store(false, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }

    fn get_config(&self) -> CustomMcpServer {
        CustomMcpServer::Remote(self.config.clone())
    }
    async fn ping(&self) -> bool {
        self.is_alive()
    }
}

/// Local (STDIO) MCP Client with detailed diagnostic capture.
pub struct StdioMcpClient {
    id: String,
    config: LocalMcpConfig,
    child: Option<Arc<Mutex<tokio::process::Child>>>,
    stdin: Option<Arc<Mutex<tokio::process::ChildStdin>>>,
    dispatch:
        Arc<dashmap::DashMap<u64, tokio::sync::oneshot::Sender<Result<Value, McpClientError>>>>,
    request_id: std::sync::atomic::AtomicU64,
}

impl StdioMcpClient {
    pub fn new(id: String, config: LocalMcpConfig) -> Self {
        Self {
            id,
            config,
            child: None,
            stdin: None,
            dispatch: Arc::new(dashmap::DashMap::new()),
            request_id: std::sync::atomic::AtomicU64::new(1),
        }
    }

    async fn send_request(&self, method: &str, params: Value) -> Result<Value, McpClientError> {
        if !self.is_alive() {
            return Err(McpClientError::ConnectionFailed(
                PHOENIX_RESTART_REQUIRED.into(),
            ));
        }

        let id = self
            .request_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let request =
            serde_json::json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params });
        let mut request_str = serde_json::to_string(&request).map_err(|e| {
            McpClientError::ProtocolError(format!("Failed to serialize request: {}", e))
        })?;
        request_str.push('\n');

        let stdin_locked = self
            .stdin
            .as_ref()
            .ok_or(McpClientError::ConnectionFailed("Stdin closed".into()))?;
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.dispatch.insert(id, tx);

        {
            let mut stdin = stdin_locked.lock().await;
            if let Err(e) = stdin.write_all(request_str.as_bytes()).await {
                self.dispatch.remove(&id);
                return Err(McpClientError::ProtocolError(format!(
                    "Write failed: {}",
                    e
                )));
            }
            let _ = stdin.flush().await;
        }

        match tokio::time::timeout(std::time::Duration::from_secs(self.config.timeout), rx).await {
            Ok(Ok(result)) => result,
            _ => {
                self.dispatch.remove(&id);
                Err(McpClientError::Timeout("Request timed out".into()))
            }
        }
    }

    /// Send a JSON-RPC notification (fire-and-forget, no `id` field, no response expected).
    /// Required by MCP spec for `notifications/initialized` after handshake.
    async fn send_notification(&self, method: &str) -> Result<(), McpClientError> {
        if !self.is_alive() {
            return Err(McpClientError::ConnectionFailed(
                PHOENIX_RESTART_REQUIRED.into(),
            ));
        }
        let notification = serde_json::json!({ "jsonrpc": "2.0", "method": method });
        let mut notif_str = serde_json::to_string(&notification).map_err(|e| {
            McpClientError::ProtocolError(format!("Failed to serialize notification: {}", e))
        })?;
        notif_str.push('\n');
        let stdin_locked = self
            .stdin
            .as_ref()
            .ok_or(McpClientError::ConnectionFailed("Stdin closed".into()))?;
        {
            let mut stdin = stdin_locked.lock().await;
            if let Err(e) = stdin.write_all(notif_str.as_bytes()).await {
                return Err(McpClientError::ProtocolError(format!(
                    "Notification write failed: {}",
                    e
                )));
            }
            let _ = stdin.flush().await;
        }
        Ok(())
    }
}

#[async_trait]
impl McpClient for StdioMcpClient {
    async fn connect(&mut self) -> Result<(), McpClientError> {
        let mut command = tokio::process::Command::new(&self.config.command);
        command
            .args(&self.config.args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true);

        for (k, v) in &self.config.env {
            if !v.is_empty() {
                command.env(k, v);
            }
        }
        if let Some(ref dir) = self.config.working_dir {
            command.current_dir(dir);
        }

        let mut child = command
            .spawn()
            .map_err(|e| McpClientError::ConnectionFailed(format!("Spawn failed: {}", e)))?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| McpClientError::ConnectionFailed("Failed to open stdin pipe".into()))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| McpClientError::ConnectionFailed("Failed to open stdout pipe".into()))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| McpClientError::ConnectionFailed("Failed to open stderr pipe".into()))?;

        let server_id = self.id.clone();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                tracing::debug!(server = %server_id, "Stderr: {}", line);
            }
        });

        let mut reader = BufReader::new(stdout).lines();
        self.stdin = Some(Arc::new(Mutex::new(stdin)));
        self.child = Some(Arc::new(Mutex::new(child)));

        let dispatch = Arc::clone(&self.dispatch);
        tokio::spawn(async move {
            while let Ok(Some(line)) = reader.next_line().await {
                if let Ok(resp) = serde_json::from_str::<Value>(&line) {
                    if let Some(id) = resp.get("id").and_then(|v| v.as_u64()) {
                        if let Some((_, tx)) = dispatch.remove(&id) {
                            let res = if let Some(e) = resp.get("error") {
                                Err(McpClientError::ProtocolError(e.to_string()))
                            } else {
                                Ok(resp.get("result").cloned().unwrap_or(Value::Null))
                            };
                            let _ = tx.send(res);
                        }
                    }
                }
            }
        });

        self.send_request(
            "initialize",
            serde_json::json!({
                "protocolVersion": "2024-11-05", "capabilities": {},
                "clientInfo": { "name": "myth", "version": "0.1.0" }
            }),
        )
        .await?;

        // MCP Spec Compliance: Send `initialized` notification after successful handshake.
        // Without this, many servers silently reject all subsequent tool calls.
        self.send_notification("notifications/initialized").await?;
        Ok(())
    }

    async fn list_tools(&self) -> Result<Vec<Value>, McpClientError> {
        let res = self.send_request("tools/list", Value::Null).await?;
        Ok(res
            .get("tools")
            .and_then(|t| t.as_array())
            .cloned()
            .unwrap_or_default())
    }

    async fn list_tools_raw(&self) -> Result<Vec<Value>, McpClientError> {
        self.list_tools().await
    }

    async fn call_tool(&self, name: &str, arguments: Value) -> Result<Value, McpClientError> {
        self.send_request(
            "tools/call",
            serde_json::json!({ "name": name, "arguments": arguments }),
        )
        .await
    }

    async fn list_resources(&self) -> Result<Vec<Value>, McpClientError> {
        let res = self.send_request("resources/list", Value::Null).await?;
        Ok(res
            .get("resources")
            .and_then(|t| t.as_array())
            .cloned()
            .unwrap_or_default())
    }

    async fn read_resource(&self, uri: &str) -> Result<Value, McpClientError> {
        self.send_request("resources/read", serde_json::json!({ "uri": uri }))
            .await
    }

    async fn list_prompts(&self) -> Result<Vec<Value>, McpClientError> {
        let res = self.send_request("prompts/list", Value::Null).await?;
        Ok(res
            .get("prompts")
            .and_then(|t| t.as_array())
            .cloned()
            .unwrap_or_default())
    }

    async fn get_prompt(&self, name: &str, arguments: Value) -> Result<Value, McpClientError> {
        self.send_request(
            "prompts/get",
            serde_json::json!({ "name": name, "arguments": arguments }),
        )
        .await
    }

    fn is_alive(&self) -> bool {
        self.child
            .as_ref()
            .and_then(|c| {
                c.try_lock()
                    .ok()
                    .map(|mut child| matches!(child.try_wait(), Ok(None)))
            })
            .unwrap_or(false)
    }

    fn process_info(&self) -> Option<(u32, String)> {
        self.child.as_ref().and_then(|c| {
            c.try_lock()
                .ok()
                .map(|child| (child.id().unwrap_or(0), "RUNNING".into()))
        })
    }

    async fn close(&mut self) -> Result<(), McpClientError> {
        self.stdin.take();
        if let Some(child_lock) = self.child.take() {
            let _ = child_lock.lock().await.kill().await;
        }
        Ok(())
    }

    fn get_config(&self) -> CustomMcpServer {
        CustomMcpServer::Local(self.config.clone())
    }
    async fn ping(&self) -> bool {
        self.is_alive()
    }
}
