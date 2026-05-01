//! Session — lifecycle management (start, run, cleanup).

use crate::config::AppConfig;
use crate::sandbox::Workspace;
use uuid::Uuid;

/// Represents a single agent session from start to finish.
pub struct Session {
    pub id: String,
    pub target: String,
    pub profile: String,
    pub workspace: Workspace,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub operator_name: String,
}

impl Session {
    pub fn new(target: &str, profile: &str, config: &AppConfig) -> Self {
        let id = Uuid::new_v4().to_string();
        Self {
            id: id.clone(),
            target: target.to_string(),
            profile: profile.to_string(),
            workspace: Workspace::from_config(config, &id),
            started_at: chrono::Utc::now(),
            operator_name: config.agent.user_name.clone(),
        }
    }

    /// Initialize workspace and prepare for scanning.
    pub async fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.workspace.init().await?;
        tracing::info!(
            session_id = %self.id,
            target = %self.target,
            profile = %self.profile,
            "Session initialized"
        );
        Ok(())
    }

    /// Get session uptime.
    pub fn uptime(&self) -> chrono::Duration {
        chrono::Utc::now() - self.started_at
    }

    /// Clean up session (remove workspace, free memory).
    pub async fn cleanup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.workspace.cleanup().await?;

        // Clear last mission context on explicit cleanup
        let ctx_path = AppConfig::mission_context_path();
        if ctx_path.exists() {
            if let Err(e) = std::fs::remove_file(&ctx_path) {
                tracing::warn!(path = %ctx_path.display(), error = %e, "Failed to remove mission context file during cleanup");
            }
        }

        tracing::info!(
            session_id = %self.id,
            uptime_secs = self.uptime().num_seconds(),
            "Session cleaned up — all data destroyed"
        );
        Ok(())
    }

    /// Persist mission metadata to disk for CLI parity.
    /// Uses the session workspace path for scoped file generation (C-05 fix).
    pub async fn persist_metadata(
        &self,
        graph: &crate::core::recon_graph::ReconGraph,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let meta = MissionMetadata {
            session_id: self.id.clone(),
            target: self.target.clone(),
            profile: self.profile.clone(),
            started_at: self.started_at,
            operator_name: self.operator_name.clone(),
            graph: graph.clone(),
        };

        let path = AppConfig::mission_context_path();
        let content = serde_json::to_string_pretty(&meta)?;

        // Write directly — no need for FileGenerator for a simple metadata file
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::write(&path, content).await?;

        Ok(())
    }
}

/// Lightweight snapshot of a mission for cross-process CLI access.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MissionMetadata {
    pub session_id: String,
    pub target: String,
    pub profile: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub operator_name: String,
    pub graph: crate::core::recon_graph::ReconGraph,
}

impl MissionMetadata {
    pub fn load() -> Option<Self> {
        let path = AppConfig::mission_context_path();
        if !path.exists() {
            return None;
        }
        let content = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&content).ok()
    }
}
