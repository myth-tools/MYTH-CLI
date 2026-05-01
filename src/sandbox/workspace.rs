//! Workspace management — tmpfs RAM disk lifecycle.

use crate::config::AppConfig;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkspaceError {
    #[error("Failed to create workspace directory: {0}")]
    Create(String),

    #[error("Failed to mount tmpfs: {0}")]
    Mount(String),

    #[error("Failed to cleanup workspace: {0}")]
    Cleanup(String),

    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

/// Manages the in-memory workspace (tmpfs) for scan outputs and session data.
pub struct Workspace {
    base_path: PathBuf,
    size_mb: u32,
    mounted: bool,
}

impl Workspace {
    /// Create workspace config from settings and session id (does NOT mount yet).
    pub fn from_config(config: &AppConfig, session_id: &str) -> Self {
        let name = config.agent.name.to_lowercase();
        Self {
            base_path: std::env::temp_dir().join(format!("{}-workspace-{}", name, session_id)),
            size_mb: config.sandbox.workspace_size_mb,
            mounted: false,
        }
    }

    /// Initialize the workspace directory structure.
    /// Falls back to a regular temp directory if tmpfs mount fails (no root).
    pub async fn init(&mut self) -> Result<(), WorkspaceError> {
        // Create directory structure
        let subdirs = ["scans", "findings", "logs", "reports"];
        for dir in &subdirs {
            let path = self.base_path.join(dir);
            tokio::fs::create_dir_all(&path)
                .await
                .map_err(|e| WorkspaceError::Create(e.to_string()))?;
        }

        tracing::info!(path = %self.base_path.display(), "Workspace initialized");

        // Try to mount tmpfs (requires root)
        match self.try_mount_tmpfs().await {
            Ok(_) => {
                self.mounted = true;
                tracing::info!(size_mb = self.size_mb, "tmpfs mounted (volatile)");
            }
            Err(e) => {
                tracing::warn!(
                    error = %e,
                    "Could not mount tmpfs (needs root). Using regular directory — data may persist."
                );
            }
        }

        Ok(())
    }

    /// Attempt to mount tmpfs at the workspace path.
    async fn try_mount_tmpfs(&self) -> Result<(), WorkspaceError> {
        tracing::warn!(
            path = %self.base_path.display(),
            "Executing host system mount command for tmpfs isolation. Process must have CAP_SYS_ADMIN."
        );
        let output = tokio::process::Command::new("mount")
            .arg("-t")
            .arg("tmpfs")
            .arg("-o")
            .arg(format!("size={}M,mode=0700", self.size_mb))
            .arg("tmpfs")
            .arg(&self.base_path)
            .output()
            .await
            .map_err(|e| WorkspaceError::Mount(e.to_string()))?;

        if !output.status.success() {
            return Err(WorkspaceError::Mount(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(())
    }

    /// Get the base workspace path.
    pub fn path(&self) -> &Path {
        &self.base_path
    }

    /// Get path for scan outputs.
    pub fn scans_dir(&self) -> PathBuf {
        self.base_path.join("scans")
    }

    /// Get path for findings.
    pub fn findings_dir(&self) -> PathBuf {
        self.base_path.join("findings")
    }

    /// Get path for logs.
    pub fn logs_dir(&self) -> PathBuf {
        self.base_path.join("logs")
    }

    /// Get path for reports.
    pub fn reports_dir(&self) -> PathBuf {
        self.base_path.join("reports")
    }

    /// Cleanup workspace — remove all files and unmount.
    pub async fn cleanup(&mut self) -> Result<(), WorkspaceError> {
        tracing::info!("Cleaning up workspace");

        // Unmount FIRST if we mounted tmpfs (must happen before removing directory)
        if self.mounted {
            tracing::warn!(
                path = %self.base_path.display(),
                "Executing host system umount command to cleanup tmpfs isolation."
            );
            let _ = tokio::process::Command::new("umount")
                .arg(&self.base_path)
                .output()
                .await;
            self.mounted = false;
        }

        // Remove contents after unmounting
        if self.base_path.exists() {
            tokio::fs::remove_dir_all(&self.base_path)
                .await
                .map_err(|e| WorkspaceError::Cleanup(e.to_string()))?;
        }

        Ok(())
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        // Best-effort sync cleanup — async cleanup() should be called first when possible
        if self.base_path.exists() {
            let path = self.base_path.clone();
            std::thread::spawn(move || {
                // Robust system-level cleanup for the workstation RAM disk
                let _ = std::process::Command::new("rm")
                    .arg("-rf")
                    .arg(&path)
                    .output();
            });
        }
    }
}
