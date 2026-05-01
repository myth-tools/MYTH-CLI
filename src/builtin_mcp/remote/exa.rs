//! Exa Search MCP Server definition.
//!
//! Exa is an LLM-native search engine that provides high-quality markdown results.

use crate::config::{CustomMcpServer, LocalMcpConfig, McpTransport};

pub fn get_config() -> CustomMcpServer {
    CustomMcpServer::Local(LocalMcpConfig {
        enabled: true,
        command: "npx".to_string(),
        args: vec!["-y".to_string(), "exa-mcp-server".to_string()],
        env: crate::mcp_env! {
            "EXA_API_KEY" => ""
        },
        description: Some("Exa Search — LLM-native search engine".to_string()),
        working_dir: None,
        timeout: 60,
        allowed_tools: vec![],
        transport: McpTransport::Stdio,
        install_script: None,
    })
}
