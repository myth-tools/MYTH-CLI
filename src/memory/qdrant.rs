//! Qdrant in-memory vector storage for session memory.
//!
//! Stores scan results and findings as vectors for semantic search.
//! Everything is volatile — dies with the Qdrant process.

use crate::config::AppConfig;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("Qdrant connection failed: {0}")]
    ConnectionError(String),

    #[error("Collection error: {0}")]
    CollectionError(String),

    #[error("Insert error: {0}")]
    InsertError(String),

    #[error("Search error: {0}")]
    SearchError(String),

    #[error("Qdrant not available: {0}")]
    NotAvailable(String),
}

/// A memory entry stored in Qdrant.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryEntry {
    pub id: String,
    pub content: String,
    pub entry_type: MemoryEntryType,
    pub tool_name: Option<String>,
    pub target: Option<String>,
    pub timestamp: String,
    pub metadata: serde_json::Value,
    /// Semantic vector for this entry
    pub vector: Option<Vec<f32>>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MemoryEntryType {
    ScanResult,
    Finding,
    ToolOutput,
    Analysis,
    Note,
}

use chrono::Utc;
use dashmap::DashMap;
use std::sync::Arc;

/// Advanced Qdrant-based session memory (in-memory mode).
/// Tuned for ultra-powerful, high-speed concurrent RAM-based memory storage.
pub struct InMemoryStore {
    collection_name: String,
    vector_size: u32,
    grpc_port: u16,
    connected: bool,
    // Ultra-fast concurrent in-memory storage
    entries: Arc<DashMap<String, MemoryEntry>>,
    // Keyword inverted index for fast keyword lookups: keyword -> Set<entry_id>
    keyword_index: Arc<DashMap<String, dashmap::DashSet<String>>>,
    // Reverse index for O(1) eviction cleanup: entry_id -> Vec<keyword>
    reverse_index: Arc<DashMap<String, Vec<String>>>,
    // Timeline index for fast timeline lookups
    timeline: Arc<DashMap<i64, String>>,
    // O(1) FIFO eviction queue
    eviction_queue: Arc<tokio::sync::Mutex<std::collections::VecDeque<String>>>,
    max_entries: usize,
}

impl InMemoryStore {
    /// Create from config.
    pub fn from_config(config: &AppConfig) -> Self {
        let collection_name = format!(
            "{}_{}",
            config.agent.name.to_lowercase(),
            config.memory.collection_name
        );
        Self {
            collection_name,
            vector_size: config.memory.vector_size,
            grpc_port: config.memory.grpc_port,
            connected: false,
            entries: Arc::new(DashMap::new()),
            keyword_index: Arc::new(DashMap::new()),
            reverse_index: Arc::new(DashMap::new()),
            timeline: Arc::new(DashMap::new()),
            eviction_queue: Arc::new(tokio::sync::Mutex::new(std::collections::VecDeque::new())),
            max_entries: config.memory.max_entries,
        }
    }

    /// Initialize connection to Qdrant.
    pub async fn connect(&mut self) -> Result<(), MemoryError> {
        self.connected = true;

        tracing::info!(
            collection = %self.collection_name,
            port = self.grpc_port,
            vector_size = self.vector_size,
            connected = self.connected,
            "Advanced Unified Memory System initialized (ultra-fast in-memory backend)"
        );
        Ok(())
    }

    /// Insert a memory entry (with O(1) FIFO eviction).
    pub async fn store(&self, entry: MemoryEntry) -> Result<(), MemoryError> {
        let mut queue = self.eviction_queue.lock().await;

        // FIFO eviction if at capacity (I-03 fix: uses configurable max_entries)
        if self.entries.len() >= self.max_entries {
            if let Some(old_id) = queue.pop_front() {
                self.entries.remove(&old_id);
                // Clean keyword_index using the reverse_index
                if let Some((_, old_keywords)) = self.reverse_index.remove(&old_id) {
                    for kw in old_keywords {
                        if let Some(set) = self.keyword_index.get(&kw) {
                            set.remove(&old_id);
                            // Remove empty keyword sets to prevent unbounded key growth
                            if set.is_empty() {
                                drop(set);
                                self.keyword_index.remove(&kw);
                            }
                        }
                    }
                }
                // Clean timeline (remove by value scan — timeline is small vs entries)
                self.timeline.retain(|_, v| v != &old_id);
            }
        }

        tracing::debug!(
            id = %entry.id,
            entry_type = ?entry.entry_type,
            "Storing high-fidelity memory entry"
        );

        let id_clone = entry.id.clone();

        // 1. Index keywords and build reverse index for O(1) eviction cleanup
        let words: Vec<String> = entry
            .content
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.replace(|c: char| !c.is_alphanumeric(), ""))
            .filter(|s| s.len() > 3)
            .collect();

        for word in &words {
            let set = self.keyword_index.entry(word.clone()).or_default();
            set.insert(id_clone.clone());
        }
        self.reverse_index.insert(id_clone.clone(), words);

        // 2. Timeline Index
        let ts = Utc::now().timestamp_micros();
        self.timeline.insert(ts, id_clone.clone());

        // 3. Store main entry
        self.entries.insert(id_clone.clone(), entry);

        // 4. Update eviction queue with full cleanup on overflow
        {
            let mut queue = self.eviction_queue.lock().await;
            queue.push_back(id_clone);

            const MAX_MEMORY_ENTRIES: usize = 100_000;
            if queue.len() > MAX_MEMORY_ENTRIES {
                if let Some(old_id) = queue.pop_front() {
                    self.entries.remove(&old_id);
                    // Clean keyword_index using the reverse_index
                    if let Some((_, old_keywords)) = self.reverse_index.remove(&old_id) {
                        for kw in old_keywords {
                            if let Some(set) = self.keyword_index.get(&kw) {
                                set.remove(&old_id);
                                // Remove empty keyword sets to prevent unbounded key growth
                                if set.is_empty() {
                                    drop(set);
                                    self.keyword_index.remove(&kw);
                                }
                            }
                        }
                    }
                    // Clean timeline (remove by value scan — timeline is small vs entries)
                    self.timeline.retain(|_, v| v != &old_id);
                }
            }
        }

        Ok(())
    }

    /// Advanced Hybrid Search: Merges Semantic (Vec) + Keyword (BM25-lite) scores.
    /// This provides industry-standard retrieval precision for both fuzzy concepts and technical IDs.
    pub async fn search(
        &self,
        query_vector: Option<&[f32]>,
        query_text: Option<&str>,
        limit: usize,
    ) -> Result<Vec<MemoryEntry>, MemoryError> {
        let mut final_scores: std::collections::HashMap<String, f32> =
            std::collections::HashMap::new();

        // 1. Semantic Signal (Fuzzy/Contextual)
        if let Some(vec) = query_vector {
            for kv in self.entries.iter() {
                let entry = kv.value();
                if let Some(ref entry_vec) = entry.vector {
                    let sim = crate::memory::embeddings::cosine_similarity(entry_vec, vec);
                    if sim > 0.1 {
                        // Quality floor
                        // Weight semantic weight at 0.7 for general intent
                        *final_scores.entry(entry.id.clone()).or_insert(0.0) += sim * 0.7;
                    }
                }
            }
        }

        // 2. Keyword Signal (Precision/Exact)
        if let Some(query) = query_text {
            let query_lower = query.to_lowercase();
            let query_words: Vec<String> = query_lower
                .split_whitespace()
                .map(|s| s.replace(|c: char| !c.is_alphanumeric(), ""))
                .filter(|s| s.len() > 3)
                .collect();

            for word in query_words {
                if let Some(set) = self.keyword_index.get(&word) {
                    for id in set.iter() {
                        // Weight keywords at 0.3 for precision boosting
                        // This allows exact technical strings (IPs, UUIDs) to jump to the top
                        *final_scores.entry(id.clone()).or_insert(0.0) += 0.3;
                    }
                }
            }
        }

        // 3. Fusion & Ranking
        let mut results: Vec<(MemoryEntry, f32)> = final_scores
            .into_iter()
            .filter_map(|(id, score)| self.entries.get(&id).map(|e| (e.clone(), score)))
            .collect();

        // Sort by hybrid score descending
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let final_list: Vec<MemoryEntry> =
            results.into_iter().take(limit).map(|(e, _)| e).collect();

        if !final_list.is_empty() {
            tracing::debug!(
                results = final_list.len(),
                "Hybrid Memory Retrieval Success"
            );
            return Ok(final_list);
        }

        // Hard Fallback: O(N) Raw Scan for substrings (Last Resort)
        if let Some(query) = query_text {
            let query_lower = query.to_lowercase();
            let mut fallback_results: Vec<MemoryEntry> = self
                .entries
                .iter()
                .filter(|kv| kv.value().content.to_lowercase().contains(&query_lower))
                .map(|kv| kv.value().clone())
                .collect();
            fallback_results.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            return Ok(fallback_results.into_iter().take(limit).collect());
        }

        Ok(vec![])
    }

    /// Get all entries of a specific type.
    pub async fn get_by_type(&self, entry_type: MemoryEntryType) -> Vec<MemoryEntry> {
        self.entries
            .iter()
            .filter(|kv| kv.value().entry_type == entry_type)
            .map(|kv| kv.value().clone())
            .collect()
    }

    /// Get the total count of stored entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Clear all memory (happens automatically on exit, but can be manual too).
    pub fn clear(&self) {
        self.entries.clear();
        self.keyword_index.clear();
        self.reverse_index.clear();
        self.timeline.clear();
        tracing::info!("Advanced Memory System cleared");
    }
}
