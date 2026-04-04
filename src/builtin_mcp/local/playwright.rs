//! Playwright MCP Server definition.
//!
//! Browser automation for interacting with complex JS-heavy web targets.

use crate::config::{CustomMcpServer, LocalMcpConfig, McpTransport};

pub fn get_config() -> CustomMcpServer {
    CustomMcpServer::Local(LocalMcpConfig {
        enabled: true,
        command: "npx".to_string(),
        args: vec!["-y".to_string(), "@playwright/mcp@latest".to_string()],
        env: crate::mcp_env! {
            "CDP_URL" => "http://127.0.0.1:9222",
            "PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD" => "1"
        },
        description: Some(
            "Hybrid Web Automation — PRIMARY Lightpanda (Elite) with seamless Chromium (Native) fallback. Powered by the Elite CDP Bridge.".to_string(),
        ),
        working_dir: None,
        timeout: 300,
        allowed_tools: vec![],
        transport: McpTransport::Stdio,
        install_script: None,
    })
}
