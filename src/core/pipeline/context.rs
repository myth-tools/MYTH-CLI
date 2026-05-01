//! Shared Mission Context Bus for the Multi-Agent Pipeline.

use crate::core::recon_graph::ReconGraph;
use crate::memory::embeddings::EmbeddingGenerator;
use crate::memory::qdrant::InMemoryStore;
use dashmap::DashMap;
use std::sync::atomic::AtomicU8;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use super::types::{ExecutionPlan, GatheredIntel, RawFinding, ScopeDecision, ValidatedFinding};

/// Single Source of Truth passed through all 8 pipeline positions.
pub struct PipelineContext {
    // Shared identity
    pub target: String,

    // Read-write by all positions
    pub scope: Arc<RwLock<Option<ScopeDecision>>>,
    pub gathered_intel: Arc<RwLock<Option<GatheredIntel>>>,
    pub execution_plan: Arc<RwLock<Option<ExecutionPlan>>>,

    // Concurrently written finding streams
    pub raw_findings: Arc<DashMap<String, RawFinding>>,
    pub validated_findings: Arc<DashMap<String, ValidatedFinding>>,
    pub final_report: Arc<RwLock<Option<String>>>,

    // Shared infrastructure (all positions read-only)
    pub recon_graph: Arc<Mutex<ReconGraph>>,
    pub memory: Arc<InMemoryStore>,
    pub generator: Arc<dyn EmbeddingGenerator>,

    // Telemetry
    pub current_position: Arc<AtomicU8>,
    pub pipeline_start: std::time::Instant,
}

impl PipelineContext {
    pub fn new(
        target: String,
        recon_graph: Arc<Mutex<ReconGraph>>,
        memory: Arc<InMemoryStore>,
        generator: Arc<dyn EmbeddingGenerator>,
    ) -> Self {
        Self {
            target,
            scope: Arc::new(RwLock::new(None)),
            gathered_intel: Arc::new(RwLock::new(None)),
            execution_plan: Arc::new(RwLock::new(None)),
            raw_findings: Arc::new(DashMap::new()),
            validated_findings: Arc::new(DashMap::new()),
            final_report: Arc::new(RwLock::new(None)),
            recon_graph,
            memory,
            generator,
            current_position: Arc::new(AtomicU8::new(0)),
            pipeline_start: std::time::Instant::now(),
        }
    }
}
