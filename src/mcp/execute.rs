//! Tool Executor — orchestrates sandboxed command execution.

use crate::config::AppConfig;
use crate::sandbox::{BubblewrapSandbox, SecurityPolicy};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecuteError {
    #[error("Command blocked: {0}")]
    Blocked(String),

    #[error("Sandbox error: {0}")]
    SandboxError(#[from] crate::sandbox::bwrap::SandboxError),

    #[error("Output too large: {size} bytes (max: {max} bytes)")]
    OutputTooLarge { size: usize, max: usize },

    #[error("Redundant command detected: {0}. Do not repeat failing or blocked commands. Analyze the failure and choose a different tactical approach.")]
    RedundantCommand(String),

    #[error("Tool binary not found: {0}. Please ensure the tool is installed and in your PATH.")]
    MissingBinary(String),
}

/// Truncate a string at the nearest valid UTF-8 character boundary.
/// Prevents panics from slicing in the middle of a multibyte char.
fn truncate_at_char_boundary(s: &str, max_bytes: usize) -> &str {
    if max_bytes >= s.len() {
        return s;
    }
    // Walk backwards from max_bytes to find a valid char boundary
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

/// Executes tools inside the bubblewrap sandbox with policy enforcement.
pub struct ToolExecutor {
    sandbox: BubblewrapSandbox,
    policy: SecurityPolicy,
    sandbox_enabled: bool,
    timeout_seconds: u64,
    /// Tracks the last executed command to prevent hallucinations and infinite loops.
    redundancy_monitor: std::sync::Arc<std::sync::Mutex<RedundancyMonitor>>,
    /// Multi-turn tool output cache with TTL (10 minutes)
    output_cache: std::sync::Arc<dashmap::DashMap<String, (ExecutionResult, std::time::Instant)>>,
    proxy_config: crate::config::ProxyConfig,
    max_tool_executions: u32,
}

use std::collections::VecDeque;

#[derive(Default)]
struct RedundancyMonitor {
    history: VecDeque<(String, bool)>, // (command, success)
    execution_count: u32,              // total tools executed in this session turn
}

/// Result returned to the LLM after tool execution.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExecutionResult {
    pub command: String,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub duration_ms: u64,
    pub success: bool,
    pub truncated: bool,
}

pub type OutputCallback = crate::sandbox::bwrap::OutputCallback;

pub struct BatchCommand {
    pub binary: String,
    pub args: Vec<String>,
    pub callback: Option<OutputCallback>,
}

impl ToolExecutor {
    /// Create from config.
    pub fn from_config(config: &AppConfig) -> Self {
        Self {
            sandbox: BubblewrapSandbox::from_config(config),
            policy: SecurityPolicy::from_config(config),
            sandbox_enabled: config.sandbox.enabled,
            timeout_seconds: config.agent.timeout_seconds,
            redundancy_monitor: std::sync::Arc::new(std::sync::Mutex::new(
                RedundancyMonitor::default(),
            )),
            output_cache: std::sync::Arc::new(dashmap::DashMap::new()),
            proxy_config: config.proxy.clone(),
            max_tool_executions: config.agent.max_iterations,
        }
    }

    /// Execute a tool with full operational pipeline:
    /// 1. Validate against policy
    /// 2. Run in sandbox (or unsandboxed if disabled)
    /// 3. Capture & truncate output
    pub async fn execute(
        &self,
        binary: &str,
        args: &[&str],
        workspace_path: &std::path::Path,
        callback: Option<OutputCallback>,
    ) -> Result<ExecutionResult, ExecuteError> {
        // Step 1: Policy check
        self.policy
            .is_allowed(binary)
            .map_err(ExecuteError::Blocked)?;

        self.policy
            .validate_args(args)
            .map_err(ExecuteError::Blocked)?;

        let full_cmd = format!("{} {}", binary, args.join(" "));

        // Step 1.2: Cache Check (ENH-06 - Performance & Resilience)
        if let Some(cached) = self.output_cache.get(&full_cmd) {
            let (res, ts) = cached.value();
            if ts.elapsed() < std::time::Duration::from_secs(600) {
                tracing::info!(cmd = %full_cmd, "Neural Cache HIT: Returning recent tactical results");
                return Ok(res.clone());
            }
        }

        // Step 1.5: Redundancy Check (Crush hallucinations)
        {
            let mut monitor = self.redundancy_monitor.lock().unwrap();

            // Check session bounds (H-04: configurable cap for complex workflows)
            monitor.execution_count += 1;
            if monitor.execution_count > self.max_tool_executions {
                return Err(ExecuteError::RedundantCommand(
                    format!("Maximum tool execution limit ({}) reached for this session turn. STOP executing tools immediately and Synthesize your findings. Do not call any more tools.", self.max_tool_executions)
                ));
            }

            // Check A->B->A loops or immediate repeats
            let recent_history: Vec<_> = monitor.history.iter().take(4).collect();
            let exact_match = recent_history.iter().find(|(cmd, _)| cmd == &full_cmd);

            if let Some((_, success)) = exact_match {
                if !success {
                    // It failed recently, do not run it again.
                    return Err(ExecuteError::RedundantCommand(format!(
                        "{} failed recently. Do NOT retry the exact same failing command. Analyze the failure and change your approach.", full_cmd
                    )));
                } else {
                    // It succeeded recently, but we are repeating it exactly within 4 commands.
                    // This is suspicious. Count total occurrences in history.
                    let total_occurrences = monitor
                        .history
                        .iter()
                        .filter(|(cmd, _)| cmd == &full_cmd)
                        .count();
                    if total_occurrences >= 2 {
                        return Err(ExecuteError::RedundantCommand(format!(
                            "{} has already been run multiple times. Stop repeating successful commands endlessly. Move on to the next phase or Synthesize.", full_cmd
                        )));
                    }
                }
            }
        }

        // Transform arguments (e.g., inject -c 4 for ping)
        let owned_args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        let transformed_args = self.policy.transform_arguments(binary, &owned_args);
        let arg_refs: Vec<&str> = transformed_args.iter().map(|s| s.as_str()).collect();

        tracing::info!(
            binary,
            args = ?arg_refs,
            sandboxed = self.sandbox_enabled,
            "Executing tool"
        );

        // Step 1.8: Pre-Execution IP Rotation via Tor
        crate::mcp::tor::rotate_ip_if_enabled(&self.proxy_config).await;

        // Step 2: Execute
        let result = if self.sandbox_enabled {
            match self
                .sandbox
                .execute(binary, &arg_refs, workspace_path, callback)
                .await
            {
                Ok(res) => res,
                Err(crate::sandbox::bwrap::SandboxError::BinaryNotFound(b)) => {
                    return Err(ExecuteError::MissingBinary(b));
                }
                Err(e) => return Err(ExecuteError::SandboxError(e)),
            }
        } else {
            match crate::sandbox::BubblewrapSandbox::execute_unsandboxed(
                binary,
                &arg_refs,
                self.timeout_seconds,
                callback,
            )
            .await
            {
                Ok(res) => res,
                Err(crate::sandbox::bwrap::SandboxError::BinaryNotFound(b)) => {
                    return Err(ExecuteError::MissingBinary(b));
                }
                Err(e) => return Err(ExecuteError::SandboxError(e)),
            }
        };

        // Internal bwrap "command not found" detection (usually exit code 127 if shell is involved, or specific error msg)
        // Bubblewrap doesn't always return 127, it might return 1 with an error message.
        if result.exit_code != 0
            && (result.stderr.contains("No such file or directory")
                || result.stderr.contains("command not found"))
        {
            return Err(ExecuteError::MissingBinary(binary.to_string()));
        }

        // Step 3: Build response (truncate at safe UTF-8 boundary if needed)
        let max_bytes = self.policy.max_output_bytes();
        let mut truncated = false;

        let stdout = if result.stdout.len() > max_bytes {
            truncated = true;
            let safe_slice = truncate_at_char_boundary(&result.stdout, max_bytes);
            format!(
                "{}... [TRUNCATED: {} total bytes]",
                safe_slice,
                result.stdout.len()
            )
        } else {
            result.stdout
        };

        let stderr_max = max_bytes / 4;
        let stderr = if result.stderr.len() > stderr_max {
            truncated = true;
            let safe_slice = truncate_at_char_boundary(&result.stderr, stderr_max);
            format!("{}... [TRUNCATED]", safe_slice)
        } else {
            result.stderr
        };

        let execution_result = ExecutionResult {
            command: result.command,
            stdout,
            stderr,
            exit_code: result.exit_code,
            duration_ms: result.duration_ms,
            success: result.exit_code == 0,
            truncated,
        };

        // Update redundancy monitor with result
        {
            let mut monitor = self.redundancy_monitor.lock().unwrap();
            monitor
                .history
                .push_front((full_cmd.clone(), execution_result.success));
            if monitor.history.len() > 8 {
                monitor.history.pop_back();
            }
        }

        // Update output cache
        self.output_cache.insert(
            full_cmd,
            (execution_result.clone(), std::time::Instant::now()),
        );

        Ok(execution_result)
    }

    pub async fn execute_batch(
        &self,
        commands: Vec<BatchCommand>,
        workspace_path: &std::path::Path,
    ) -> Vec<Result<ExecutionResult, ExecuteError>> {
        use futures::future::join_all;

        let workspace_path_buf = workspace_path.to_path_buf();
        let futures = commands.into_iter().map(|cmd| {
            let executor = self;
            let workspace = workspace_path_buf.clone();

            async move {
                let arg_refs: Vec<&str> = cmd.args.iter().map(|s| s.as_str()).collect();
                executor
                    .execute(&cmd.binary, &arg_refs, &workspace, cmd.callback)
                    .await
            }
        });

        join_all(futures).await
    }

    /// Reset the redundancy monitor (usage count) for a new agent turn.
    pub fn reset_redundancy_monitor(&self) {
        let mut monitor = self.redundancy_monitor.lock().unwrap();
        monitor.execution_count = 0;
        // We keep the history to prevent loops across turns, but reset the hard count limit.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_ascii() {
        let s = "hello world";
        assert_eq!(truncate_at_char_boundary(s, 5), "hello");
    }

    #[test]
    fn test_truncate_multibyte_safe() {
        // "こんにちは" = 5 chars × 3 bytes each = 15 bytes
        let s = "こんにちは";
        // Truncating at byte 7 should back up to byte 6 (boundary of 2nd char)
        let result = truncate_at_char_boundary(s, 7);
        assert_eq!(result, "こん");
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_truncate_no_panic_on_emoji() {
        let s = "🔴🟠🟡🔵⚪";
        // Each emoji is 4 bytes; truncate at 5 should give only the first emoji
        let result = truncate_at_char_boundary(s, 5);
        assert_eq!(result, "🔴");
    }

    #[test]
    fn test_truncate_larger_than_string() {
        let s = "short";
        assert_eq!(truncate_at_char_boundary(s, 100), "short");
    }

    #[test]
    fn test_truncate_zero() {
        let s = "anything";
        assert_eq!(truncate_at_char_boundary(s, 0), "");
    }
}
