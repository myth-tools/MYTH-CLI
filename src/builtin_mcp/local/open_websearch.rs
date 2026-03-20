//! Open WebSearch MCP Server definition.
//!
//! Provides free web search capabilities using open-websearch.

use crate::config::{CustomMcpServer, LocalMcpConfig, McpTransport};
use std::collections::HashMap;

pub fn get_config() -> CustomMcpServer {
    CustomMcpServer::Local(LocalMcpConfig {
        enabled: true,
        command: "npx".to_string(),
        args: vec!["-y".to_string(), "open-websearch@latest".to_string()],
        env: HashMap::new(),
        description: Some(
            "Open WebSearch — Free web search engine for real-time information retrieval"
                .to_string(),
        ),
        working_dir: None,
        timeout: 120,
        allowed_tools: vec![],
        transport: McpTransport::Stdio,
    })
}
