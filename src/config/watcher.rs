use notify::{Event, RecursiveMode, Result, Watcher};
use std::path::Path;
use tokio::sync::mpsc;

/// Discriminatory configuration update events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigUpdateEvent {
    /// Global user settings changed (user.yaml)
    UserConfig,
    /// Neural link registry changed (mcp.json)
    McpRegistry,
}

/// Elite Configuration Watcher with discriminatory intelligence.
pub struct ConfigWatcher {
    _watcher: notify::RecommendedWatcher,
}

impl ConfigWatcher {
    /// Create a new watcher that monitors the configuration folder for discriminatory events.
    pub fn new(config_dir: &Path, tx: mpsc::UnboundedSender<ConfigUpdateEvent>) -> Result<Self> {
        let config_dir = config_dir.to_path_buf();
        let last_event = std::sync::Arc::new(std::sync::Mutex::new(std::time::Instant::now()));

        // Track the last 50ms to debounce rapid-fire events from certain editors (atomic swaps)
        let debounce_threshold = std::time::Duration::from_millis(50);

        let mut watcher = notify::recommended_watcher(move |res: Result<Event>| {
            match res {
                Ok(event) => {
                    // Filter: Only care about modification/creation of relevant files
                    // We also ignore .tmp files to prevent self-triggering during atomic writes
                    if event.kind.is_modify() || event.kind.is_create() {
                        let now = std::time::Instant::now();
                        {
                            let mut last = last_event.lock().unwrap();
                            if now.duration_since(*last) < debounce_threshold {
                                return;
                            }
                            *last = now;
                        }

                        for path in event.paths {
                            if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                                if filename == "user.yaml" {
                                    let _ = tx.send(ConfigUpdateEvent::UserConfig);
                                } else if filename == "mcp.json" {
                                    let _ = tx.send(ConfigUpdateEvent::McpRegistry);
                                }
                            }
                        }
                    }
                }
                Err(e) => tracing::error!("Neural link watcher diagnostic error: {:?}", e),
            }
        })?;

        // Watch the directory non-recursively to capture file renames/swaps
        watcher.watch(&config_dir, RecursiveMode::NonRecursive)?;

        Ok(Self { _watcher: watcher })
    }
}
