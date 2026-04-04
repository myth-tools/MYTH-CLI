//! SQLite MCP Server definition.
//!
//! Uses uvx to run the official MCP SQLite server.

use crate::config::{CustomMcpServer, LocalMcpConfig, McpTransport};
use std::collections::HashMap;

pub fn get_config() -> CustomMcpServer {
    CustomMcpServer::Local(LocalMcpConfig {
        enabled: true,
        command: "uvx".to_string(),
        args: vec![
            "--no-progress".to_string(),
            "--quiet".to_string(),
            "mcp-server-sqlite".to_string(),
            "--db-path".to_string(),
            format!("{}/myth_mission.db", std::env::temp_dir().to_string_lossy()),
        ],
        env: HashMap::new(),
        description: Some("Relational database interaction".to_string()),
        working_dir: None,
        timeout: 60,
        allowed_tools: vec![],
        transport: McpTransport::Stdio,
        install_script: None,
    })
}
