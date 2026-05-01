//! Data structures passed between agents in the MissionPipeline.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeDecision {
    pub allowed: bool,
    pub rules: Vec<String>,
    pub reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GatheredIntel {
    pub web_sources: Vec<String>,
    pub recommended_tools: Vec<String>,
    pub passive_findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AggregatedContext {
    pub target_summary: String,
    pub attack_surface: Vec<String>,
    pub prioritized_vectors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExecutionPlan {
    pub steps: Vec<PlanStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStep {
    pub id: String,
    pub description: String,
    pub tool_name: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawFinding {
    pub title: String,
    pub description: String,
    pub tool_used: String,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedFinding {
    pub finding: RawFinding,
    pub is_valid: bool,
    pub verification_notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanDecision {
    pub approved: bool,
    pub feedback: Option<String>,
}
