//! Playwright MCP Server definition.
//!
//! Browser automation for interacting with complex JS-heavy web targets.

use crate::config::{CustomMcpServer, LocalMcpConfig, McpTransport};
use std::collections::HashMap;

pub fn get_config() -> CustomMcpServer {
    CustomMcpServer::Local(LocalMcpConfig {
        enabled: true,
        command: "uvx".to_string(),
        args: vec!["mcp-server-playwright".to_string()],
        env: HashMap::new(),
        description: Some("Browser automation and testing".to_string()),
        working_dir: None,
        timeout: 120,
        allowed_tools: vec![],
        transport: McpTransport::Stdio,
    })
}
