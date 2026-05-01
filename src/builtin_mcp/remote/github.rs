//! GitHub MCP Server definition.
//!
//! Provides advanced repository management and automation using mcp-server-github.

use crate::config::{CustomMcpServer, LocalMcpConfig, McpTransport};

pub fn get_config() -> CustomMcpServer {
    CustomMcpServer::Local(LocalMcpConfig {
        enabled: true,
        command: "npx".to_string(),
        args: vec![
            "-y".to_string(),
            "@modelcontextprotocol/server-github".to_string(),
        ],
        env: crate::mcp_env! {
            "GITHUB_PERSONAL_ACCESS_TOKEN" => ""
        },
        description: Some("GitHub — Repository management and automation".to_string()),
        working_dir: None,
        timeout: 120,
        allowed_tools: vec![],
        transport: McpTransport::Stdio,
        install_script: None,
    })
}
