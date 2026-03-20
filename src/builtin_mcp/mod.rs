use crate::config::CustomMcpServer;
use std::collections::HashMap;

pub mod custom;
pub mod local;
pub mod remote;

/// Aggregates all built-in tactical MCP assets into a central "Factory Defaults" registry.
pub fn get_factory_defaults() -> HashMap<String, CustomMcpServer> {
    let mut defaults = HashMap::new();

    // ─── LOCAL TACTICAL ASSETS ───
    defaults.insert(
        "filesystem".to_string(),
        local::filesystem::get_config(vec![]),
    );
    defaults.insert("sqlite".to_string(), local::sqlite::get_config());
    defaults.insert("playwright".to_string(), local::playwright::get_config());
    defaults.insert("webfetch".to_string(), local::webfetch::get_config());
    defaults.insert(
        "llm_researcher".to_string(),
        local::researcher::get_config(),
    );
    defaults.insert(
        "open-websearch".to_string(),
        local::open_websearch::get_config(),
    );

    // ─── REMOTE INTELLIGENCE ASSETS ───
    defaults.insert("fetch".to_string(), remote::fetch::get_config());
    defaults.insert("github".to_string(), remote::github::get_config());
    defaults.insert("exa".to_string(), remote::exa::get_config());
    defaults.insert("jina".to_string(), remote::jina::get_config());

    // ─── CUSTOM SECURITY ASSETS ───
    defaults.insert("codegate".to_string(), custom::codegate::get_config());

    defaults
}
