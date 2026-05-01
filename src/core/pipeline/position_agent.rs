//! Generic runner for pipeline positions.

use crate::config::AgentSlotConfig;
use crate::core::agent::AgentError;
use crate::llm::NimClient;
use rig::completion::Prompt;
use serde::de::DeserializeOwned;
use std::sync::Arc;

pub struct PositionAgent {
    slot_config: AgentSlotConfig,
    nim_client: Arc<NimClient>,
    system_prompt: String,
}

impl PositionAgent {
    pub fn new(
        slot_config: AgentSlotConfig,
        nim_client: Arc<NimClient>,
        system_prompt: String,
    ) -> Self {
        Self {
            slot_config,
            nim_client,
            system_prompt,
        }
    }

    /// Run a single-turn structured extraction.
    pub async fn instruct<T: DeserializeOwned>(&self, prompt: &str) -> Result<T, AgentError> {
        let builder = self.nim_client.agent_builder_for_slot(&self.slot_config);

        let agent = builder.preamble(&self.system_prompt).build();

        let mut enriched_prompt = prompt.to_string();
        enriched_prompt.push_str("\n\nYou MUST respond ONLY with valid JSON matching the exact required structure. Do not wrap it in markdown code blocks. Just output the raw JSON.");

        let response = agent
            .prompt(&enriched_prompt)
            .await
            .map_err(|e| AgentError::Llm(format!("Prompt error: {}", e)))?;

        let clean = response
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        serde_json::from_str::<T>(clean)
            .map_err(|e| AgentError::Llm(format!("JSON Parse error: {}\nRaw Output: {}", e, clean)))
    }
}
