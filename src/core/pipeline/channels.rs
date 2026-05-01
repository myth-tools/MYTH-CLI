//! Channel types for inter-agent message passing.
//!
//! Positions use these channels as conveyor belts for typed, zero-copy data transfer.

use super::types::{RawFinding, ValidatedFinding};
use tokio::sync::mpsc;

/// Stream of raw findings emitted by Position 5 (Executor) and consumed by Position 6 (Validator).
pub type RawFindingSender = mpsc::UnboundedSender<RawFinding>;
pub type RawFindingReceiver = mpsc::UnboundedReceiver<RawFinding>;

/// Stream of validated findings emitted by Position 6 (Validator) and consumed by Position 7 (Reporter).
pub type ValidatedFindingSender = mpsc::UnboundedSender<ValidatedFinding>;
pub type ValidatedFindingReceiver = mpsc::UnboundedReceiver<ValidatedFinding>;
