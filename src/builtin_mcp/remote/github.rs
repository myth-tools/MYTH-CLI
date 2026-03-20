//! GitHub MCP Server definition.
//!
//! Provides advanced repository management and automation using mcp-server-github.

use crate::config::{CustomMcpServer, LocalMcpConfig, McpTransport};

pub fn get_config() -> CustomMcpServer {
    CustomMcpServer::Local(LocalMcpConfig {
        enabled: true,
        command: "uvx".to_string(),
        args: vec!["mcp-server-github".to_string()],
        env: crate::mcp_env! {
            "GITHUB_PERSONAL_ACCESS_TOKEN" => std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN").unwrap_or_default()
        },
        description: Some("GitHub — Repository management and automation".to_string()),
        working_dir: None,
        timeout: 60,
        allowed_tools: vec![],
        transport: McpTransport::Stdio,
    })
}
