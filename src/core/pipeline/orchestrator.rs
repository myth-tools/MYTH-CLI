//! MissionPipeline — Orchestrates the 8-position pipeline.

use crate::config::AppConfig;
use crate::core::agent::{AgentError, ReconAgent};
use crate::llm::NimClient;
use crate::mcp::McpServer;
use std::sync::Arc;

use super::context::PipelineContext;
use super::position_agent::PositionAgent;
use super::types::*;

pub struct MissionPipeline {
    config: AppConfig,
    nim_client: Arc<NimClient>,
    context: Arc<PipelineContext>,
    mcp_server: Option<McpServer>,
}

pub struct PipelineResult {
    pub success: bool,
    pub report: Option<String>,
}

impl MissionPipeline {
    pub async fn new(
        config: AppConfig,
        nim_client: Arc<NimClient>,
        context: Arc<PipelineContext>,
    ) -> Result<Self, AgentError> {
        Ok(Self {
            config,
            nim_client,
            context,
            mcp_server: None, // Will be initialized if needed by Executor
        })
    }

    pub fn set_mcp_server(&mut self, mcp: McpServer) {
        self.mcp_server = Some(mcp);
    }

    pub async fn run(&mut self, user_input: &str) -> Result<PipelineResult, AgentError> {
        // --- POSITION 1: Evaluator ---
        if self.config.pipeline.evaluator.enabled {
            tracing::info!("Starting Position 1: Evaluator");
            let p1 = PositionAgent::new(
                self.config.pipeline.evaluator.clone(),
                self.nim_client.clone(),
                crate::llm::prompts::evaluator_prompt(&self.context.target),
            );

            let scope: ScopeDecision = p1.instruct(user_input).await?;
            if !scope.allowed {
                tracing::warn!("Mission aborted by Evaluator: {}", scope.reasoning);
                return Ok(PipelineResult {
                    success: false,
                    report: Some(scope.reasoning),
                });
            }

            *self.context.scope.write().await = Some(scope.clone());
        }

        // --- POSITION 2: Gatherer (Parallel) ---
        let mut gathered = GatheredIntel::default();
        if self.config.pipeline.gatherer.enabled {
            tracing::info!("Starting Position 2: Data Gathering Orchestrator");
            let mut handles = vec![];

            for sub_config in &self.config.pipeline.gatherer.sub_agents {
                let target = self.context.target.clone();
                let sub_role = sub_config.description.clone();
                // Map SubAgentConfig to AgentSlotConfig for PositionAgent
                let slot_config = crate::config::AgentSlotConfig {
                    enabled: true,
                    model: sub_config.model.clone(),
                    temperature: sub_config.temperature,
                    max_tokens: sub_config.max_tokens,
                    description: sub_role.clone(),
                };

                let p2 = PositionAgent::new(
                    slot_config,
                    self.nim_client.clone(),
                    crate::llm::prompts::gatherer_prompt(&target, &sub_role),
                );

                let input = user_input.to_string();
                handles.push(tokio::spawn(async move {
                    p2.instruct::<GatheredIntel>(&input).await
                }));
            }

            for handle in handles {
                if let Ok(Ok(partial_intel)) = handle.await {
                    gathered.web_sources.extend(partial_intel.web_sources);
                    gathered
                        .recommended_tools
                        .extend(partial_intel.recommended_tools);
                    gathered
                        .passive_findings
                        .extend(partial_intel.passive_findings);
                }
            }

            *self.context.gathered_intel.write().await = Some(gathered.clone());
        }

        // --- POSITION 3: Aggregator ---
        let mut aggregated = AggregatedContext::default();
        if self.config.pipeline.aggregator.enabled {
            tracing::info!("Starting Position 3: Data Aggregator");
            let p3 = PositionAgent::new(
                self.config.pipeline.aggregator.clone(),
                self.nim_client.clone(),
                crate::llm::prompts::aggregator_prompt(&self.context.target),
            );

            let prompt = format!("Gathered Intel: {:?}", gathered);
            aggregated = p3.instruct(&prompt).await?;
        }

        // --- POSITION 4: Planner ---
        let mut plan = ExecutionPlan::default();
        if self.config.pipeline.planner.enabled {
            tracing::info!("Starting Position 4: Planner");
            let p4 = PositionAgent::new(
                self.config.pipeline.planner.clone(),
                self.nim_client.clone(),
                crate::llm::prompts::planner_prompt(&self.context.target),
            );

            let prompt = format!("Aggregated Context: {:?}", aggregated);
            plan = p4.instruct(&prompt).await?;
            *self.context.execution_plan.write().await = Some(plan.clone());
        }

        // --- POSITION 5: Executor ---
        let mut raw_findings = Vec::new();
        if self.config.pipeline.executor.enabled {
            tracing::info!("Starting Position 5: Executor");

            // To maintain compatibility, we build a ReconAgent from context
            // and pass the plan as a chat input to simulate the classic loop
            let mut executor_agent = ReconAgent::from_pipeline_context(
                self.config.clone(),
                self.context.clone(),
                self.mcp_server
                    .take()
                    .expect("McpServer must be set before executor"),
                self.nim_client.clone(),
            )
            .await?;

            let prompt = format!("Execute the following plan: {:?}", plan);
            // This is simplified. In a full implementation, you might want to call
            // a specific loop or extract findings from the agent's memory afterwards.
            let _ = executor_agent.chat_stream(&prompt).await;

            // For now, mock some findings (since extracting from ReconAgent stream requires larger refactor)
            // Real implementation would pass a channel to the executor tool bridge.
            raw_findings.push(RawFinding {
                title: "Example Finding".into(),
                description: "This finding is from the execution phase.".into(),
                tool_used: "executor".into(),
                severity: "Info".into(),
            });

            // Restore mcp server
            // (Normally you'd extract it back from executor_agent or share via Arc)
        }

        // --- POSITION 6: Validator ---
        let mut validated_findings = Vec::new();
        if self.config.pipeline.validator.enabled {
            tracing::info!("Starting Position 6: Validator");
            let mut handles = vec![];

            for raw in raw_findings {
                let target = self.context.target.clone();
                let sub_config = self
                    .config
                    .pipeline
                    .validator
                    .sub_agents
                    .first()
                    .cloned()
                    .unwrap_or_default();

                let slot_config = crate::config::AgentSlotConfig {
                    enabled: true,
                    model: sub_config.model,
                    temperature: sub_config.temperature,
                    max_tokens: sub_config.max_tokens,
                    description: sub_config.description,
                };

                let nim = self.nim_client.clone();

                handles.push(tokio::spawn(async move {
                    let p6 = PositionAgent::new(
                        slot_config,
                        nim,
                        crate::llm::prompts::validator_prompt(&target),
                    );
                    let prompt = format!("Raw Finding: {:?}", raw);
                    p6.instruct::<ValidatedFinding>(&prompt).await
                }));
            }

            for handle in handles {
                if let Ok(Ok(vf)) = handle.await {
                    if vf.is_valid {
                        validated_findings.push(vf);
                    }
                }
            }
        }

        // --- POSITION 7: Reporter ---
        let mut final_report = String::new();
        if self.config.pipeline.reporter.enabled {
            tracing::info!("Starting Position 7: Reporter");
            let p7 = PositionAgent::new(
                self.config.pipeline.reporter.clone(),
                self.nim_client.clone(),
                crate::llm::prompts::reporter_prompt(&self.context.target),
            );

            let prompt = format!("Validated Findings: {:?}", validated_findings);
            // We use a struct to extract from JSON
            #[derive(serde::Deserialize)]
            struct ReportWrapper {
                report: String,
            }

            let prompt_json = format!(
                "{}\n\nReturn JSON: {{ \"report\": \"<markdown>\" }}",
                prompt
            );
            if let Ok(res) = p7.instruct::<ReportWrapper>(&prompt_json).await {
                final_report = res.report;
                *self.context.final_report.write().await = Some(final_report.clone());
            }
        }

        // --- POSITION 8: Human Review ---
        if self.config.pipeline.human_review.enabled
            && !self.config.pipeline.human_review.auto_approve
        {
            tracing::info!("Starting Position 8: Human Review Gate");
            println!("\n=== FINAL REPORT DRAFT ===");
            println!("{}", final_report);
            println!("==========================");
            println!("Do you approve this report? (y/n)");

            // Read from stdin
            let mut input = String::new();
            if std::io::stdin().read_line(&mut input).is_ok()
                && !input.trim().eq_ignore_ascii_case("y")
            {
                return Ok(PipelineResult {
                    success: false,
                    report: Some("Rejected by human review".into()),
                });
            }
        }

        Ok(PipelineResult {
            success: true,
            report: Some(final_report),
        })
    }
}
