//! Fetch MCP Server definition.
//!
//! Uses uvx to run the official MCP Fetch server.

use crate::config::{CustomMcpServer, LocalMcpConfig, McpTransport};
use std::collections::HashMap;

pub fn get_config() -> CustomMcpServer {
    CustomMcpServer::Local(LocalMcpConfig {
        enabled: true,
        command: "uvx".to_string(),
        args: vec![
            "--no-progress".to_string(),
            "--quiet".to_string(),
            "mcp-server-fetch".to_string(),
        ],
        env: HashMap::new(),
        description: Some("Official MCP Fetch server".to_string()),
        working_dir: None,
        timeout: 60,
        allowed_tools: vec![],
        transport: McpTransport::Stdio,
        install_script: None,
    })
}
