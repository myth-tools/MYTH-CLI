//! Embeddings — text to vector conversion for semantic memory.
//!
//! Uses NVIDIA NIM's embedding endpoint or falls back to simple TF-IDF.

use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmbeddingError {
    #[error("Embedding generation failed: {0}")]
    GenerationError(String),
}

/// Generator for text embeddings.
#[async_trait]
pub trait EmbeddingGenerator: Send + Sync {
    /// Generate a vector for the given text.
    async fn generate(&self, text: &str) -> Vec<f32>;
}

/// Fallback generator using character/word frequency hashing.
pub struct FallbackGenerator {
    dimensions: usize,
}

impl FallbackGenerator {
    pub fn new(dimensions: usize) -> Self {
        Self { dimensions }
    }
}

#[async_trait]
impl EmbeddingGenerator for FallbackGenerator {
    async fn generate(&self, text: &str) -> Vec<f32> {
        simple_text_hash(text, self.dimensions)
    }
}

pub struct NimEmbeddingGenerator {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
    model: String,
    /// Configured dimensions for consistent fallback vector sizes
    dimensions: usize,
}

impl NimEmbeddingGenerator {
    pub fn new(api_key: String, base_url: String, model: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
            base_url,
            model,
            dimensions: 1024, // Default NIM embedding size
        }
    }

    /// Set custom dimensions for fallback consistency.
    pub fn with_dimensions(mut self, dimensions: usize) -> Self {
        self.dimensions = dimensions;
        self
    }
}

#[async_trait]
impl EmbeddingGenerator for NimEmbeddingGenerator {
    async fn generate(&self, text: &str) -> Vec<f32> {
        let url = format!("{}/embeddings", self.base_url);
        let payload = serde_json::json!({
            "input": text,
            "model": self.model,
            "encoding_format": "float",
            "input_type": "query"
        });

        match self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    if let Some(data) = json.get("data").and_then(|d| d.as_array()) {
                        if let Some(first) = data.first() {
                            if let Some(embedding) =
                                first.get("embedding").and_then(|e| e.as_array())
                            {
                                return embedding
                                    .iter()
                                    .filter_map(|v| v.as_f64().map(|f| f as f32))
                                    .collect();
                            }
                        }
                    }
                }
            }
            Ok(resp) => {
                let status = resp.status();
                let txt = resp.text().await.unwrap_or_default();
                tracing::warn!("NIM embedding failed: {} - {}", status, txt);
            }
            Err(e) => {
                tracing::warn!("NIM embedding request error: {}", e);
            }
        }

        // Fallback to local TF-IDF with configured dimensions on failure
        simple_text_hash(text, self.dimensions)
    }
}

/// Improved text embedding using character trigram hashing (H-03 fix).
/// Uses overlapping character trigrams with FNV-like hashing for better
/// discrimination between texts with similar character distributions.
/// In production, the NIM embedding API handles this; this is the offline fallback.
pub fn simple_text_hash(text: &str, dimensions: usize) -> Vec<f32> {
    let mut vector = vec![0.0f32; dimensions];
    let lower = text.to_lowercase();
    let bytes = lower.as_bytes();

    // 1. Character trigram hashing (much better discrimination than single-char positional)
    if bytes.len() >= 3 {
        for window in bytes.windows(3) {
            // FNV-1a inspired hash of the trigram
            let mut hash: u64 = 0xcbf29ce484222325;
            for &b in window {
                hash ^= b as u64;
                hash = hash.wrapping_mul(0x100000001b3);
            }
            let idx = (hash as usize) % dimensions;
            vector[idx] += 1.0;
        }
    }

    // 2. Word-level positional hashing for additional signal
    for (word_pos, word) in lower.split_whitespace().enumerate() {
        let word_bytes = word.as_bytes();
        let mut hash: u64 = 0xcbf29ce484222325;
        for &b in word_bytes {
            hash ^= b as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        // Spread across different dimensions using word position
        let idx1 = (hash as usize) % dimensions;
        let idx2 = (hash as usize ^ (word_pos * 7919)) % dimensions;
        vector[idx1] += 0.5;
        vector[idx2] += 0.3;
    }

    // 3. Single character fallback for very short texts
    if bytes.len() < 3 {
        for (i, &byte) in bytes.iter().enumerate() {
            let idx = (byte as usize * (i + 1)) % dimensions;
            vector[idx] += 1.0;
        }
    }

    // Normalize to unit vector
    let magnitude: f32 = vector.iter().map(|x| x * x).sum::<f32>().sqrt();
    if magnitude > 0.0 {
        for v in &mut vector {
            *v /= magnitude;
        }
    }

    vector
}

/// Compute cosine similarity between two vectors.
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a * norm_b)
}
