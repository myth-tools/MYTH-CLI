//! Filesystem MCP Server definition.
//!
//! Uses npx to run the official MCP Filesystem server.

use crate::config::{CustomMcpServer, LocalMcpConfig, McpTransport};
use std::collections::HashMap;

pub fn get_config(mut allowed_directories: Vec<String>) -> CustomMcpServer {
    // Advanced: Auto-detect current working directory if none provided for industry-grade UX
    if allowed_directories.is_empty() {
        if let Ok(cwd) = std::env::current_dir() {
            allowed_directories.push(cwd.to_string_lossy().to_string());
        }
    }

    CustomMcpServer::Local(LocalMcpConfig {
        enabled: false,
        command: "npx".to_string(),
        args: {
            let mut args = vec![
                "-y".to_string(),
                "@modelcontextprotocol/server-filesystem".to_string(),
            ];
            args.extend(allowed_directories);
            args
        },
        env: HashMap::new(),
        description: Some(
            "Filesystem — Local file operations and secure workspace management".to_string(),
        ),
        working_dir: None,
        timeout: 60,
        allowed_tools: vec![],
        transport: McpTransport::Stdio,
    })
}
