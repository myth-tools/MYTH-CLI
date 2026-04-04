use aes_gcm_siv::{
    aead::{Aead, KeyInit},
    Aes256GcmSiv, Nonce,
};
use rusqlite::params;
use rustyline::history::{DefaultHistory, History};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
struct HistoryContainer {
    entries: Vec<String>,
    version: u32,
}

/// Standardized tactical history path for both CLI and TUI (Industry Grade).
pub fn get_history_path() -> PathBuf {
    crate::config::AppConfig::user_config_path()
        .parent()
        .map(|p| p.join(".myth_history"))
        .unwrap_or_else(|| PathBuf::from(".myth_history"))
}

/// New SQLite vault path for atomic persistence.
pub fn get_vault_path() -> PathBuf {
    crate::config::AppConfig::user_config_path()
        .parent()
        .map(|p| p.join(".myth_history.db"))
        .unwrap_or_else(|| PathBuf::from(".myth_history.db"))
}

/// Dynamic Key Derivation (Industry Standard Silicon-Grade)
/// Based on system identity to ensure Zero-Touch professional security.
fn derive_tactical_key() -> [u8; 32] {
    let home = std::env::var("HOME")
        .unwrap_or_else(|_| std::env::temp_dir().to_string_lossy().into_owned());
    let hostname = nix::unistd::gethostname()
        .map(|h| h.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "unknown_spectre".to_string());

    // Stable machine identifier (Hardware-Locked Identity)
    let machine_id = if let Ok(id) = fs::read_to_string("/etc/machine-id") {
        id.trim().to_string()
    } else if let Ok(id) = fs::read_to_string("/var/lib/dbus/machine-id") {
        id.trim().to_string()
    } else {
        "legacy_volatile_identity".to_string()
    };

    // Salted tactical seed
    let salt = b"MYTH_TACTICAL_ENCRYPTION_v2_SHADOW_KEY";

    let mut hasher = Sha256::new();
    hasher.update(home.as_bytes());
    hasher.update(hostname.as_bytes());
    hasher.update(machine_id.as_bytes());
    hasher.update(salt);

    let mut key = [0u8; 32];
    key.copy_from_slice(&hasher.finalize());
    key
}

/// Encrypts and saves raw history strings atomically.
pub fn save_history_vec(entries: &[String], path: &Path) -> anyhow::Result<()> {
    let container = HistoryContainer {
        entries: entries.to_vec(),
        version: 1,
    };

    let serialized = serde_json::to_vec(&container)?;
    let key = derive_tactical_key();
    let cipher = Aes256GcmSiv::new(&key.into());

    let mut nonce_hasher = Sha256::new();
    nonce_hasher.update(&serialized);
    let nonce_bytes = &nonce_hasher.finalize()[..12];
    let nonce = Nonce::from_slice(nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, serialized.as_ref())
        .map_err(|_| anyhow::anyhow!("Silicon-Grade Encryption Failed"))?;

    let mut final_blob = nonce_bytes.to_vec();
    final_blob.extend_from_slice(&ciphertext);

    let tmp_path = path.with_extension("tmp");
    {
        let mut file = fs::File::create(&tmp_path)?;
        file.write_all(&final_blob)?;
        file.sync_all()?;
    }
    fs::rename(&tmp_path, path)?;

    Ok(())
}

/// Encrypts and saves the command history atomically.
pub fn save_encrypted_history(history: &DefaultHistory, path: &Path) -> anyhow::Result<()> {
    let mut entries = Vec::new();
    for i in 0..history.len() {
        if let Ok(Some(entry)) = history.get(i, rustyline::history::SearchDirection::Forward) {
            entries.push(entry.entry.to_string());
        }
    }

    let container = HistoryContainer {
        entries,
        version: 1,
    };

    let serialized = serde_json::to_vec(&container)?;
    let key = derive_tactical_key();
    let cipher = Aes256GcmSiv::new(&key.into());

    // Deterministic Nonce for History (based on content hash to ensure stability)
    let mut nonce_hasher = Sha256::new();
    nonce_hasher.update(&serialized);
    let nonce_bytes = &nonce_hasher.finalize()[..12];
    let nonce = Nonce::from_slice(nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, serialized.as_ref())
        .map_err(|_| anyhow::anyhow!("Silicon-Grade Encryption Failed"))?;

    // Prepend nonce for decryption
    let mut final_blob = nonce_bytes.to_vec();
    final_blob.extend_from_slice(&ciphertext);

    // Atomic Write with Explicit Hardware Sync (Industry Standard Resilience)
    let tmp_path = path.with_extension("tmp");
    {
        let mut file = fs::File::create(&tmp_path)?;
        file.write_all(&final_blob)?;
        file.sync_all()?; // Forces the OS to physically commit to drive
    }
    fs::rename(&tmp_path, path)?;

    Ok(())
}

/// Institutional-Grade Mission Vault (SQLite-Backed)
pub struct HistoryVault {
    conn: rusqlite::Connection,
}

impl HistoryVault {
    /// Initializes the vault with institution-grade performance tuning and schema.
    pub fn init(path: &Path) -> anyhow::Result<Self> {
        let conn = rusqlite::Connection::open(path)?;

        // Durability Hardening (Industry Standard)
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;

        // Performance Hardening (Institution-Grade)
        // Memory-mapped I/O allows near-instant reads by letting the OS treat the file as RAM.
        conn.pragma_update(None, "mmap_size", "268435456")?; // 256MB mmap
        conn.pragma_update(None, "cache_size", "-2000")?; // 2MB cache
        conn.pragma_update(None, "temp_store", "MEMORY")?; // RAM-only temp files

        // Initialize Schema
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tactical_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                command_blob BLOB NOT NULL
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    /// Appends a new mission command to the vault with compression and row-level encryption.
    pub fn append(&self, command: &str) -> anyhow::Result<()> {
        let key = derive_tactical_key();
        let cipher = aes_gcm_siv::Aes256GcmSiv::new(&key.into());

        // Institution-Grade Compression (Zstd Compression Level 3)
        // Reduces tactical footprint and speeds up I/O for large histories.
        let compressed = zstd::encode_all(command.as_bytes(), 3)?;

        let mut nonce_hasher = Sha256::new();
        nonce_hasher.update(&compressed);
        let nonce_bytes = &nonce_hasher.finalize()[..12];
        let nonce = aes_gcm_siv::Nonce::from_slice(nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, compressed.as_slice())
            .map_err(|_| anyhow::anyhow!("Silicon-Grade Encryption Failed"))?;

        let mut final_blob = nonce_bytes.to_vec();
        final_blob.extend_from_slice(&ciphertext);

        let timestamp = chrono::Local::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO tactical_history (timestamp, command_blob) VALUES (?1, ?2)",
            params![timestamp, final_blob],
        )?;

        Ok(())
    }

    /// Loads all history into a DefaultHistory object for the interactive session.
    pub fn load_into(&self, history: &mut DefaultHistory) -> anyhow::Result<()> {
        let mut stmt = self
            .conn
            .prepare("SELECT command_blob FROM tactical_history ORDER BY id ASC")?;
        let rows = stmt.query_map([], |row| row.get::<_, Vec<u8>>(0))?;

        let key = derive_tactical_key();
        let cipher = aes_gcm_siv::Aes256GcmSiv::new(&key.into());

        history.clear()?;
        for blob in rows {
            let blob = blob?;
            if blob.len() < 12 {
                continue;
            }

            let nonce_bytes = &blob[..12];
            let ciphertext = &blob[12..];
            let nonce = aes_gcm_siv::Nonce::from_slice(nonce_bytes);

            if let Ok(decrypted_compressed) = cipher.decrypt(nonce, ciphertext) {
                // Decompression Layer (Zstd)
                if let Ok(decrypted) = zstd::decode_all(decrypted_compressed.as_slice()) {
                    if let Ok(cmd) = String::from_utf8(decrypted) {
                        history.add(&cmd)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Migrates legacy flat-file history into the atomic vault.
    pub fn migrate_legacy(&self, legacy_path: &Path) -> anyhow::Result<bool> {
        if !legacy_path.exists() {
            return Ok(false);
        }

        // Use the existing logic to attempt decryption
        let mut temp_history = DefaultHistory::new();
        if load_encrypted_history(&mut temp_history, legacy_path).is_ok() {
            for i in 0..temp_history.len() {
                if let Ok(Some(entry)) =
                    temp_history.get(i, rustyline::history::SearchDirection::Forward)
                {
                    let _ = self.append(entry.entry.as_ref());
                }
            }

            // Rename to rescue trail
            let legacy_trail = legacy_path.with_extension("legacy");
            let _ = fs::rename(legacy_path, legacy_trail);
            return Ok(true);
        }

        Ok(false)
    }
}

/// Decrypts and loads the command history into the editor (RETAINED FOR MIGRATION).
pub fn load_encrypted_history(history: &mut DefaultHistory, path: &Path) -> anyhow::Result<()> {
    if !path.exists() {
        return Ok(());
    }

    let raw_blob = fs::read(path)?;
    if raw_blob.len() < 12 {
        anyhow::bail!("Tactical History Corrupted: Block Fragmented");
    }

    let nonce_bytes = &raw_blob[..12];
    let ciphertext = &raw_blob[12..];

    let key = derive_tactical_key();
    let cipher = Aes256GcmSiv::new(&key.into());
    let nonce = Nonce::from_slice(nonce_bytes);

    let decrypted = match cipher.decrypt(nonce, ciphertext) {
        Ok(data) => data,
        Err(_) => {
            // Automatic Recovery Logic (Industry Grade Resilience)
            let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
            let backup_path = path.with_extension(format!("vault_fail_{}.bak", timestamp));
            let _ = fs::rename(path, &backup_path);

            anyhow::bail!(
                "Identity Shift Detected: Mission history context isolated to {:?}",
                backup_path
            );
        }
    };

    let container: HistoryContainer = serde_json::from_slice(&decrypted)?;

    // Clear and refill (robust loading)
    history.clear()?;
    for entry in container.entries {
        history.add(&entry)?;
    }

    Ok(())
}

/// Decrypts and loads encrypted history into a raw string vector.
pub fn load_history_vec(path: &Path) -> anyhow::Result<Vec<String>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let raw_blob = fs::read(path)?;
    if raw_blob.len() < 12 {
        anyhow::bail!("Tactical History Corrupted: Too Small");
    }

    let nonce_bytes = &raw_blob[..12];
    let ciphertext = &raw_blob[12..];

    let key = derive_tactical_key();
    let cipher = Aes256GcmSiv::new(&key.into());
    let nonce = Nonce::from_slice(nonce_bytes);

    let decrypted = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| anyhow::anyhow!("Tactical Memory Decryption Failed: Invalid Identity"))?;

    let container: HistoryContainer = serde_json::from_slice(&decrypted)?;
    Ok(container.entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_sqlite_vault_roundtrip() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("vault.db");
        let vault = HistoryVault::init(&path)?;

        let cmd1 = "nmap -sV target.internal";
        let cmd2 = "recon --stealth --deep";

        vault.append(cmd1)?;
        vault.append(cmd2)?;

        let mut rl_history = DefaultHistory::new();
        vault.load_into(&mut rl_history)?;

        assert_eq!(rl_history.len(), 2);
        assert_eq!(
            rl_history
                .get(0, rustyline::history::SearchDirection::Forward)?
                .unwrap()
                .entry,
            cmd1
        );
        assert_eq!(
            rl_history
                .get(1, rustyline::history::SearchDirection::Forward)?
                .unwrap()
                .entry,
            cmd2
        );

        Ok(())
    }

    #[test]
    fn test_sqlite_vault_migration() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let legacy_path = dir.path().join(".myth_history");
        let vault_path = dir.path().join("vault.db");

        // 1. Create legacy history
        let entries = vec!["whoami".to_string(), "ls -la".to_string()];
        save_history_vec(&entries, &legacy_path)?;

        // 2. Initialize vault and migrate
        let vault = HistoryVault::init(&vault_path)?;
        let migrated = vault.migrate_legacy(&legacy_path)?;

        assert!(migrated);
        assert!(!legacy_path.exists());
        assert!(dir.path().join(".myth_history.legacy").exists());

        // 3. Verify content
        let mut rl_history = DefaultHistory::new();
        vault.load_into(&mut rl_history)?;
        assert_eq!(rl_history.len(), 2);

        Ok(())
    }
}
