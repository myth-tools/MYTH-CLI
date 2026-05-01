//! SystemContext: Advanced Path Registry & Environment Sensing for Architectural Harmony.
//!
//! This module acts as the authoritative engine for all system-wide paths and
//! environmental context. It natively supports Standard Linux distributions
//! (Debian, Arch, Fedora) and sandboxed environments like Termux (Android).

use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DistroFamily {
    Debian,
    Fedora,
    Arch,
    Termux,
    Generic,
}

/// Sensing the strategic host environment (Distribution, Paths, Containers).
#[derive(Debug, Clone)]
pub struct SystemContext {
    /// The tactical root (e.g., $PREFIX on Termux, "" on Standard Linux).
    pub prefix: Option<String>,
    /// Whether we are currently engaged in a Termux environment.
    pub is_termux: bool,
    /// The detected distribution family.
    pub family: DistroFamily,
    /// Whether the process is executing with effective root privileges.
    pub is_root: bool,
    /// True if executing inside a containerized sandbox (e.g., Docker, LXC).
    pub is_container: bool,
}

impl Default for SystemContext {
    fn default() -> Self {
        Self::sense()
    }
}

impl SystemContext {
    /// Senses the environment root, distribution family, and sandbox status.
    pub fn sense() -> Self {
        let prefix = env::var("PREFIX").ok().filter(|s| !s.is_empty());
        let is_termux = prefix.as_ref().is_some_and(|p| p.contains("com.termux"));

        let family = if is_termux {
            DistroFamily::Termux
        } else {
            Self::detect_distro()
        };

        let is_root =
            env::var("USER").unwrap_or_default() == "root" || unsafe { libc::geteuid() == 0 };
        let is_container =
            Path::new("/.dockerenv").exists() || Path::new("/run/.containerenv").exists();

        Self {
            prefix,
            is_termux,
            family,
            is_root,
            is_container,
        }
    }

    fn detect_distro() -> DistroFamily {
        // Simple heuristic based on common release files
        if Path::new("/etc/debian_version").exists() {
            return DistroFamily::Debian;
        } else if Path::new("/etc/fedora-release").exists()
            || Path::new("/etc/redhat-release").exists()
        {
            return DistroFamily::Fedora;
        } else if Path::new("/etc/arch-release").exists() {
            return DistroFamily::Arch;
        }

        // Deeper inspection via /etc/os-release
        if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
            let lower = content.to_lowercase();
            if lower.contains("id_like=debian") || lower.contains("id=debian") {
                return DistroFamily::Debian;
            } else if lower.contains("id_like=fedora") || lower.contains("id=fedora") {
                return DistroFamily::Fedora;
            } else if lower.contains("id_like=arch") || lower.contains("id=arch") {
                return DistroFamily::Arch;
            }
        }

        DistroFamily::Generic
    }

    /// Derives the industrial-grade binary root.
    pub fn bin_root(&self) -> PathBuf {
        if let Some(ref p) = self.prefix {
            PathBuf::from(format!("{}/bin", p))
        } else {
            // Priority: /usr/local/bin -> /usr/bin/
            let local = PathBuf::from("/usr/local/bin");
            if local.is_dir() {
                local
            } else {
                PathBuf::from("/usr/bin")
            }
        }
    }

    /// Returns a list of candidate binary search paths natively adjusted to the environment.
    pub fn bin_search_paths(&self) -> Vec<PathBuf> {
        if let Some(ref p) = self.prefix {
            vec![PathBuf::from(format!("{}/bin", p))]
        } else {
            vec![
                PathBuf::from("/usr/local/bin"),
                PathBuf::from("/usr/bin"),
                PathBuf::from("/bin"),
                PathBuf::from("/sbin"),
                PathBuf::from("/usr/sbin"),
            ]
        }
    }

    /// Derives the system-wide configuration root.
    pub fn config_root(&self) -> PathBuf {
        if let Some(ref p) = self.prefix {
            PathBuf::from(format!("{}/etc", p))
        } else {
            PathBuf::from("/etc")
        }
    }

    /// Returns priority-ordered candidate locations for reading configuration files.
    /// Priority: 1. User ~/.config/myth 2. Optional Environment Override 3. System /etc/myth
    pub fn effective_config_dirs(&self) -> Vec<PathBuf> {
        let mut dirs = Vec::new();

        // 1. User specific definition
        if let Some(mut base) = dirs::config_dir() {
            base.push("myth");
            dirs.push(base);
        }

        // 2. System Definition
        let sys_config = self.config_root().join("myth");
        dirs.push(sys_config.clone());

        // 3. Optional fallback on standard Linux
        if !self.is_termux && sys_config.as_path() != std::path::Path::new("/etc/myth") {
            dirs.push(PathBuf::from("/etc/myth"));
        }

        dirs.into_iter().filter(|d| d.exists()).collect()
    }

    /// Derives the operational logging root natively adjusted for privileges.
    pub fn log_root(&self) -> PathBuf {
        if let Some(ref p) = self.prefix {
            PathBuf::from(format!("{}/var/log", p))
        } else if self.is_root {
            PathBuf::from("/var/log")
        } else {
            // Fallback for non-root execution (Standard Linux)
            dirs::state_dir().unwrap_or_else(|| PathBuf::from("/tmp"))
        }
    }

    /// Derives the persistent state/library root natively adjusted for privileges.
    pub fn lib_root(&self) -> PathBuf {
        if let Some(ref p) = self.prefix {
            PathBuf::from(format!("{}/var/lib", p))
        } else if self.is_root {
            PathBuf::from("/var/lib")
        } else {
            // Fallback for non-root execution (Standard Linux)
            dirs::data_dir().unwrap_or_else(|| PathBuf::from("/tmp"))
        }
    }

    /// Returns a specific candidate for a tactical asset (e.g. myth config).
    pub fn join_config<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.config_root().join(path)
    }

    /// Returns a specific candidate for a tactical log file.
    pub fn join_log<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.log_root().join(path)
    }

    /// Searches for a binary within the sensed environment's executable pathing.
    pub fn find_binary(&self, name: &str) -> Option<PathBuf> {
        // Direct absolute path check
        if Path::new(name).is_absolute() && Path::new(name).is_executable() {
            return Some(PathBuf::from(name));
        }

        // Context-aware PATH scanning
        for dir in self.bin_search_paths() {
            let candidate = dir.join(name);
            if candidate.is_executable() {
                return Some(candidate);
            }
        }
        None
    }
}

// Helper trait to abstract executable check across platforms
trait ExecutablePath {
    fn is_executable(&self) -> bool;
}

#[cfg(unix)]
impl ExecutablePath for Path {
    fn is_executable(&self) -> bool {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = std::fs::metadata(self) {
            metadata.is_file() && metadata.permissions().mode() & 0o111 != 0
        } else {
            false
        }
    }
}

#[cfg(not(unix))]
impl ExecutablePath for Path {
    fn is_executable(&self) -> bool {
        // Fallback for non-Unix
        self.is_file()
    }
}
