//! NVIDIA NIM Client — via Rig's OpenAI-compatible provider.
//!
//! NVIDIA NIM exposes an OpenAI-compatible API at `https://integrate.api.nvidia.com/v1`.
//! We use Rig's OpenAI provider repointed to the NIM endpoint.

use crate::config::AppConfig;
use rig::providers::openai;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NimError {
    #[error("Missing API key: {0}")]
    MissingApiKey(String),

    #[error("Failed to create NIM client: {0}")]
    ClientError(String),
}

/// NVIDIA NIM client wrapper using Rig's OpenAI provider, with key rotation.
pub struct NimClient {
    clients: Vec<openai::CompletionsClient>,
    current_idx: Arc<AtomicUsize>,
    active_model: Arc<std::sync::RwLock<String>>,
    primary_model: String,
    fallback_models: Vec<String>,
    temperature: f32,
    max_tokens: u32,
    /// Track indices of keys that have permanently failed (M-04 fix)
    failed_keys: dashmap::DashSet<usize>,
}

impl NimClient {
    /// Create a new NIM client from config, initializing multiple instances for key rotation.
    pub fn from_config(config: &AppConfig) -> Result<Self, NimError> {
        let api_keys = config
            .llm
            .resolve_api_keys()
            .map_err(|e| NimError::MissingApiKey(e.to_string()))?;
        let clients: Vec<openai::CompletionsClient> = api_keys
            .into_iter()
            .filter_map(|api_key| {
                openai::Client::builder()
                    .api_key(&api_key)
                    .base_url(&config.llm.base_url)
                    .build()
                    .map(|c| c.completions_api())
                    .map_err(|e| {
                        tracing::error!("Failed to build OpenAI client: {}", e);
                        e
                    })
                    .ok()
            })
            .collect();

        if clients.is_empty() {
            return Err(NimError::ClientError("No valid clients created".into()));
        }

        let mut fallback_models = Vec::new();
        if let Some(ref fb) = config.llm.fallback_model {
            fallback_models.push(fb.clone());
        }

        tracing::info!(
            provider = %config.llm.provider,
            model = %config.llm.model,
            keys_loaded = clients.len(),
            "NVIDIA NIM client initialized with multiple keys and fallback models"
        );

        Ok(Self {
            clients,
            current_idx: Arc::new(AtomicUsize::new(0)),
            active_model: Arc::new(std::sync::RwLock::new(config.llm.model.clone())),
            primary_model: config.llm.model.clone(),
            fallback_models,
            temperature: config.llm.temperature,
            max_tokens: config.llm.max_tokens,
            failed_keys: dashmap::DashSet::new(),
        })
    }

    /// Get the currently active Rig OpenAI client, skipping known-bad keys.
    pub fn client(&self) -> &openai::CompletionsClient {
        let len = self.clients.len();
        let base = self.current_idx.load(Ordering::Relaxed);

        // Try to find a non-failed key, starting from current index
        for offset in 0..len {
            let idx = (base + offset) % len;
            if !self.failed_keys.contains(&idx) {
                return &self.clients[idx];
            }
        }

        // All keys marked as failed — clear failures and retry from current
        tracing::warn!("All API keys marked as failed. Clearing failure log and retrying.");
        self.failed_keys.clear();
        &self.clients[base % len]
    }

    /// Rotate to the next available API key/client and mark current as failed.
    pub fn rotate_key(&self) {
        let prev = self.current_idx.load(Ordering::Relaxed);
        let len = self.clients.len();
        let old_idx = prev % len;

        // Mark the current key as failed
        self.failed_keys.insert(old_idx);

        let new_idx = (prev + 1) % len;
        self.current_idx.store(prev + 1, Ordering::Relaxed);

        tracing::warn!(
            old_index = old_idx,
            new_index = new_idx,
            total_keys = len,
            failed_count = self.failed_keys.len(),
            "Rotating NVIDIA NIM API key due to failure or rate limiting."
        );
    }

    /// Rotate the active model to the next fallback in a round-robin cycle.
    /// Each call advances to the next model, looping back to the primary model.
    pub fn rotate_model(&self) {
        let current = self.model();
        // Collect all unique models: primary + fallbacks
        let mut all_models = vec![self.primary_model.clone()];
        for m in &self.fallback_models {
            if !all_models.contains(m) {
                all_models.push(m.clone());
            }
        }
        if all_models.len() <= 1 {
            tracing::warn!("No fallback models configured — cannot rotate.");
            return;
        }
        // Find current position and advance to next
        let current_pos = all_models.iter().position(|m| m == &current).unwrap_or(0);
        let next_pos = (current_pos + 1) % all_models.len();
        let next = &all_models[next_pos];
        if let Ok(mut active) = self.active_model.write() {
            *active = next.clone();
            tracing::error!(
                from = %current,
                to = %next,
                position = format!("{}/{}", next_pos + 1, all_models.len()),
                "LLM model failure detected. Rotating to fallback model for robustness."
            );
        }
    }

    /// Get the total number of configured API keys.
    pub fn client_count(&self) -> usize {
        self.clients.len()
    }

    /// Get the currently active model name.
    pub fn model(&self) -> String {
        self.active_model
            .read()
            .map(|m| m.clone())
            .unwrap_or_else(|_| self.primary_model.clone())
    }

    /// Get configured temperature.
    pub fn temperature(&self) -> f32 {
        self.temperature
    }

    /// Get configured max tokens.
    pub fn max_tokens(&self) -> u32 {
        self.max_tokens
    }
}
