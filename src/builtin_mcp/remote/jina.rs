//! Jina Reader MCP Server definition.
//!
//! Jina converts any URL into LLM-friendly markdown.

use crate::config::{CustomMcpServer, LocalMcpConfig, McpTransport};
use std::collections::HashMap;

pub fn get_config() -> CustomMcpServer {
    CustomMcpServer::Local(LocalMcpConfig {
        enabled: true,
        command: "npx".to_string(),
        args: vec!["-y".to_string(), "@jina-ai/mcp-server".to_string()],
        env: HashMap::new(),
        description: Some("Jina Reader — URL to Markdown".to_string()),
        working_dir: None,
        timeout: 60,
        allowed_tools: vec![],
        transport: McpTransport::Stdio,
    })
}
