//! ReconGraph — directed state machine for reconnaissance workflows.
//!
//! States: Planning → Execution → Analysis → Pivot (or Report)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Reconnaissance workflow state.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReconState {
    /// Initial planning — LLM decides what to scan first
    Planning,
    /// Tool execution — running a tool in the sandbox
    Executing,
    /// Output analysis — LLM analyzes tool output
    Analyzing,
    /// Pivoting — LLM found new targets/leads to follow
    Pivoting,
    /// Reporting — generating final report
    Reporting,
    /// Session complete
    Done,
    /// Error state
    Error(String),
}

impl std::fmt::Display for ReconState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Planning => write!(f, "📋 Planning"),
            Self::Executing => write!(f, "⚡ Executing"),
            Self::Analyzing => write!(f, "🔍 Analyzing"),
            Self::Pivoting => write!(f, "🔄 Pivoting"),
            Self::Reporting => write!(f, "📊 Reporting"),
            Self::Done => write!(f, "✅ Done"),
            Self::Error(e) => write!(f, "❌ Error: {}", e),
        }
    }
}

/// A finding discovered during reconnaissance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub title: String,
    pub severity: Severity,
    pub description: String,
    pub evidence: String,
    pub tool_used: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Critical => write!(f, "🔴 Critical"),
            Self::High => write!(f, "🟠 High"),
            Self::Medium => write!(f, "🟡 Medium"),
            Self::Low => write!(f, "🔵 Low"),
            Self::Informational => write!(f, "⚪ Info"),
        }
    }
}

/// The recon graph state machine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconGraph {
    current_state: ReconState,
    iteration: u32,
    max_iterations: u32,
    findings: Vec<Finding>,
    targets: Vec<String>,
    tools_used: Vec<String>,
    history: Vec<(ReconState, String)>,
    pub current_phase: u8,
    pub phase_summaries: HashMap<u8, String>,
}

impl ReconGraph {
    /// Create a new recon graph.
    pub fn new(initial_target: &str, max_iterations: u32) -> Self {
        Self {
            current_state: ReconState::Planning,
            iteration: 0,
            max_iterations,
            findings: Vec::new(),
            targets: vec![initial_target.to_string()],
            tools_used: Vec::new(),
            history: Vec::new(),
            current_phase: 0,
            phase_summaries: HashMap::new(),
        }
    }

    /// Get the current state.
    pub fn state(&self) -> &ReconState {
        &self.current_state
    }

    /// Set the max iterations for the reconnaissance graph.
    pub fn set_max_iterations(&mut self, iterations: u32) {
        self.max_iterations = iterations;
    }

    /// Get the current iteration count.
    pub fn iteration(&self) -> u32 {
        self.iteration
    }

    /// Transition to the next state.
    pub fn transition(&mut self, next_state: ReconState) {
        let reason = format!(
            "Iteration {}: {:?} → {:?}",
            self.iteration, self.current_state, next_state
        );
        tracing::info!(
            from = %self.current_state,
            to = %next_state,
            iteration = self.iteration,
            "State transition"
        );
        self.history.push((self.current_state.clone(), reason));
        self.current_state = next_state;
    }

    /// Add a tool to the list of tools used, if not already present.
    pub fn update_tools_used(&mut self, tool_name: &str) {
        let name = tool_name.to_string();
        if !self.tools_used.contains(&name) {
            self.tools_used.push(name);
        }
    }

    /// Move to execution state and increment iteration counter.
    pub fn begin_execution(&mut self, tool_name: &str) {
        self.iteration += 1;
        self.update_tools_used(tool_name);
        self.transition(ReconState::Executing);
    }

    /// Move to analysis after execution completes.
    pub fn begin_analysis(&mut self) {
        self.transition(ReconState::Analyzing);
    }

    /// Add a finding.
    pub fn add_finding(&mut self, finding: Finding) {
        tracing::info!(
            title = %finding.title,
            severity = %finding.severity,
            "New finding discovered"
        );
        self.findings.push(finding);
    }

    /// Add a new target discovered during recon.
    pub fn add_target(&mut self, target: String) {
        if !self.targets.contains(&target) {
            tracing::info!(target = %target, "New target discovered");
            self.targets.push(target);
        }
    }

    /// Check if we should continue or stop.
    pub fn should_continue(&self) -> bool {
        self.iteration < self.max_iterations
            && !matches!(self.current_state, ReconState::Done | ReconState::Error(_))
    }

    /// Get all findings.
    pub fn findings(&self) -> &[Finding] {
        &self.findings
    }

    /// Get all discovered targets.
    pub fn targets(&self) -> &[String] {
        &self.targets
    }

    /// Get tools used so far.
    pub fn tools_used(&self) -> &[String] {
        &self.tools_used
    }

    /// Get state history.
    pub fn history(&self) -> &[(ReconState, String)] {
        &self.history
    }

    /// Get a summary of the current session.
    pub fn summary(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert(
            "state".into(),
            serde_json::json!(format!("{}", self.current_state)),
        );
        map.insert("iteration".into(), serde_json::json!(self.iteration));
        map.insert(
            "max_iterations".into(),
            serde_json::json!(self.max_iterations),
        );
        map.insert(
            "current_phase".into(),
            serde_json::json!(self.current_phase),
        );
        map.insert(
            "findings_count".into(),
            serde_json::json!(self.findings.len()),
        );
        map.insert("targets".into(), serde_json::json!(self.targets));
        map.insert(
            "tools_used_count".into(),
            serde_json::json!(self.tools_used.len()),
        );
        map
    }

    /// Advance to the next phase (0-based indexing).
    pub fn advance_phase(&mut self, phase_num: u8, summary: String) {
        // L-07 Fix: Ensure 0-based indexing consistency
        self.current_phase = phase_num + 1;
        self.phase_summaries.insert(phase_num, summary);
        tracing::info!(
            current = phase_num,
            next = self.current_phase,
            "Mission phase advanced"
        );
    }
}
