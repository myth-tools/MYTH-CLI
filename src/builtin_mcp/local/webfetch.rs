//! WebFetch MCP Server definition.
//!
//! Provides functionality to fetch web content using @iflow-mcp/manooll-webfetch-mcp.

use crate::config::{CustomMcpServer, LocalMcpConfig, McpTransport};
use std::collections::HashMap;

pub fn get_config() -> CustomMcpServer {
    CustomMcpServer::Local(LocalMcpConfig {
        enabled: true,
        command: "npx".to_string(),
        args: vec![
            "-y".to_string(),
            "@iflow-mcp/manooll-webfetch-mcp".to_string(),
        ],
        env: HashMap::new(),
        description: Some("WebFetch — Fast web content extraction".to_string()),
        working_dir: None,
        timeout: 60,
        allowed_tools: vec![],
        transport: McpTransport::Stdio,
        install_script: None,
    })
}
