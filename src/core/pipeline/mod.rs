//! Multi-Agent Pipeline Architecture

pub mod channels;
pub mod context;
pub mod orchestrator;
pub mod position_agent;
pub mod types;

pub use context::PipelineContext;
pub use orchestrator::{MissionPipeline, PipelineResult};
pub use position_agent::PositionAgent;
