//! CodeGate MCP Server definition.
//!
//! Security proxy to protect the agent from prompt injections and data leaks.

use crate::config::{CustomMcpServer, LocalMcpConfig, McpTransport};
use std::collections::HashMap;

pub fn get_config() -> CustomMcpServer {
    CustomMcpServer::Local(LocalMcpConfig {
        enabled: true,
        command: "npx".to_string(),
        args: vec!["-y".to_string(), "codegate-mcp".to_string()],
        env: HashMap::new(),
        description: Some("CodeGate — Security proxy and protection".to_string()),
        working_dir: None,
        timeout: 60,
        allowed_tools: vec![],
        transport: McpTransport::Stdio,
    })
}
