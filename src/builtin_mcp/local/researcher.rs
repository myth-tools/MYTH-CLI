//! LLM Researcher MCP Server definition.
//!
//! Provides advanced research capabilities using @iflow-mcp/light-research-mcp.

use crate::config::{CustomMcpServer, LocalMcpConfig, McpTransport};
use std::collections::HashMap;

pub fn get_config() -> CustomMcpServer {
    CustomMcpServer::Local(LocalMcpConfig {
        enabled: true,
        command: "npx".to_string(),
        args: vec![
            "-y".to_string(),
            "@iflow-mcp/light-research-mcp".to_string(),
            "--mcp".to_string(),
        ],
        env: HashMap::new(),
        description: Some("LLM Researcher — Advanced autonomous research".to_string()),
        working_dir: None,
        timeout: 120,
        allowed_tools: vec![],
        transport: McpTransport::Stdio,
    })
}
