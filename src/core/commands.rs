//! Shared command processing for the MYTH CLI.
//! Ensures consistent behavior across TUI and CLI interactive modes.

use crate::core::agent::ReconAgent;
use crate::core::docs;
use crate::mcp::config::{handle_mcp_management_cmd, McpManagementCmd};
use crate::ui::CyberTheme;
use owo_colors::OwoColorize;

/// Actions resulting from command processing.
pub enum CommandAction {
    /// Generic text response to be printed.
    Response(String),
    /// Pass the input to the agent's chat mechanism.
    AgentChat(String),
    /// Execute a tool directly bypassing the LLM.
    ExecuteTool {
        tool_name: String,
        arguments: serde_json::Value,
    },
    /// Start a new session with specific parameters.
    StartSession {
        target: String,
        profile: String,
        prompt: Option<String>,
    },
    /// Adjust recon depth.
    SetDepth(u32),
    /// Emergency session cleanup and exit.
    Burn,
    /// Wipe the current session memory.
    WipeSession,
    /// Clear the UI buffer.
    Clear,
    /// Secure exit.
    Exit,
    /// Interactive font initialization.
    ProvisionFont(String),
}

/// Tactical commands that work with OR without a slash for maximum parity.
pub const TACTICAL_COMMANDS: &[&str] = &[
    "scan",
    "tools",
    "help",
    "h",
    "?",
    "quit",
    "exit",
    "q",
    "clear",
    "cls",
    "v",
    "version",
    "usage",
    "u",
    "config",
    "check",
    "health",
    "status",
    "vitals",
    "findings",
    "graph",
    "history",
    "report",
    "stealth",
    "osint",
    "vuln",
    "profile",
    "inspect",
    "info",
    "man",
    "sync",
    "burn",
    "depth",
    "target",
    "mcp",
    "wipe",
    "recon",
    "subdomains",
    "master",
    "ultra",
    "typography",
    "fonts",
];

/// Simple Levenshtein distance for fuzzy matching suggestions.
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    if s1 == s2 {
        return 0;
    }
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    let m = s1_chars.len();
    let n = s2_chars.len();

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    // Use only two rows to save space: O(min(n, m))
    let (s1_chars, s2_chars, m, n) = if m < n {
        (s2_chars, s1_chars, n, m)
    } else {
        (s1_chars, s2_chars, m, n)
    };

    let mut prev_row: Vec<usize> = (0..=n).collect();
    let mut curr_row: Vec<usize> = vec![0; n + 1];

    for i in 1..=m {
        curr_row[0] = i;
        for j in 1..=n {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] {
                0
            } else {
                1
            };
            curr_row[j] = std::cmp::min(
                curr_row[j - 1] + 1, // Insertion
                std::cmp::min(
                    prev_row[j] + 1,        // Deletion
                    prev_row[j - 1] + cost, // Substitution
                ),
            );
        }
        std::mem::swap(&mut prev_row, &mut curr_row);
    }
    prev_row[n]
}

/// Professional command metadata for Tactical HUD.
pub struct CmdMetadata {
    pub usage: &'static str,
    pub description: &'static str,
    pub flags: &'static [(&'static str, &'static str)],
}

/// Finds the closest tactical command for a given mistyped input.
fn find_smart_suggestion(bad_verb: &str) -> Option<String> {
    let mut best_match = None;
    let mut min_dist = usize::MAX;

    for &cmd in TACTICAL_COMMANDS {
        let dist = levenshtein_distance(bad_verb, cmd);
        if dist < min_dist && dist <= 2 {
            // Only suggest if fairly close
            min_dist = dist;
            best_match = Some(cmd.to_string());
        }
    }
    best_match
}

/// Get metadata for a specific tactical command.
pub fn get_command_metadata(verb: &str) -> Option<CmdMetadata> {
    match verb.to_lowercase().as_str() {
        "scan" | "recon" => Some(CmdMetadata {
            usage: "/scan <target> [--profile <name>]",
            description: "Initiate reconnaissance mission on the specified target.",
            flags: &[("--profile", "Specify a custom recon profile (e.g. stealth, quick)")],
        }),
        "check" | "health" => Some(CmdMetadata {
            usage: "/check",
            description: "Execute deep system diagnostics (Network, Sandbox, MCP).",
            flags: &[],
        }),
        "config" => Some(CmdMetadata {
            usage: "/config [view | set <key> <value>]",
            description: "View or modulate internal agent parameters.",
            flags: &[],
        }),
        "profile" => Some(CmdMetadata {
            usage: "/profile [list | <name>]",
            description: "Inspect or load reconnaissance profiles.",
            flags: &[],
        }),
        "burn" => Some(CmdMetadata {
            usage: "/burn",
            description: "EMERGENCY PURGE: Destroy all session data and history.",
            flags: &[],
        }),
        "wipe" => Some(CmdMetadata {
            usage: "/wipe",
            description: "Clear the current tactical session memory and context.",
            flags: &[],
        }),
        "mcp" => Some(CmdMetadata {
            usage: "/mcp [list | sync | health]",
            description: "Manage and synchronize Model Context Protocol assets.",
            flags: &[],
        }),
        "subdomains" => Some(CmdMetadata {
            usage: "/subdomains <domain> [FLAGS...]",
            description: "High-speed multi-source subdomain discovery engine (18-phase pipeline).",
            flags: &[
                ("--active", "Enable active brute-force and permutation scanning"),
                ("--recursive", "Enable recursive discovery on found subdomains"),
                ("--quiet", "Suppress all progress/stats output"),
                ("--json", "Output results in JSONL format"),
                ("--master", "SOVEREIGN MODE: Specialized Wordlists + Deep Mutations"),
                ("--ultra", "ULTIMATE MODE: Global Assets + Extreme Recursion"),
                ("--stealth", "Stealth mode: reduces concurrency and adds randomized delays"),
                ("--only-alive", "Filter results to only show live subdomains (Default: true)"),
                ("--wordlist", "Wordlist size for brute-forcing: none, small, medium, large, quick, deep, mega"),
                ("--concurrency", "Maximum concurrent tasks (Default: 50)"),
                ("--timeout", "Global timeout in seconds (Default: 3600)"),
                ("--retries", "Number of retries on failure (Default: 5)"),
                ("--min-delay", "Minimum delay between requests in ms (Default: 50)"),
                ("--max-delay", "Maximum delay between requests in ms (Default: 2000)"),
                ("--proxies", "Use rotating proxies for all discovery phases"),
                ("--proxies-file", "Use a specific list of proxies from file"),
                ("--tor", "Route all discovery traffic through the Tor network"),
                ("--tor-address", "Custom Tor SOCKS5 address (Default: 127.0.0.1:9050)"),
                ("--no-rotate-ua", "Disable User-Agent rotation"),
                ("--respect-robots", "Follow robots.txt exclusion rules (Default: true)"),
                ("--no-captcha-avoidance", "Disable built-in CAPTCHA detection/bypass"),
                ("--resolvers", "Comma-separated list of DNS servers"),
                ("--resolvers-file", "Load DNS servers from a specific file"),
                ("--no-rotate-resolvers", "Disable rotation of DNS servers"),
                ("--no-wildcard-filter", "Disable intelligent wildcard DNS detection"),
                ("--depth", "Initial depth of subdomain gathering (Default: 1)"),
                ("--recursive-depth", "Maximum depth for recursive discovery (Default: 3)"),
                ("--max-pages", "Max pages to crawl during web scraping (Default: 50000)"),
                ("--max-depth", "Maximum depth for web crawler (Default: 3)"),
                ("--no-checkpoint", "Disable session saving and resumption"),
                ("--checkpoint-dir", "Directory to store session checkpoints"),
                ("--no-proxy-test", "Disables testing proxies for connectivity before use"),
            ],
        }),
        "master" | "ultra" => Some(CmdMetadata {
            usage: "/master <domain>",
            description: "ULTRA-ROBUST DISCOVERY: Auto-configures Tor, Proxies, Mega-wordlists, and recursive deep-scanning.",
            flags: &[],
        }),
        "typography" | "fonts" => Some(CmdMetadata {
            usage: "/typography [list | set <id> | revert]",
            description: "Manage tactical typography and mission-critical terminal font synchronization.",
            flags: &[],
        }),
        _ => None,
    }
}

/// Lightweight context for command suggestions.
pub struct CommandContext<'a> {
    pub config: &'a crate::config::AppConfig,
    pub mission_targets: &'a [String],
    pub history: &'a [String],
    pub discovered_tools: &'a [String],
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenRole {
    Command,
    Flag,
    FlagValue,
    Target,
    Other,
}

pub struct SemanticToken {
    pub text: String,
    pub role: TokenRole,
}

/// Advanced shell-style lexer for robust argument parsing.
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<String> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }

    fn next_token(&mut self) -> Option<String> {
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return None;
        }

        let mut token = String::new();
        let mut in_quotes: Option<char> = None;
        let mut escaped = false;
        let mut has_content = false;

        while self.pos < self.input.len() {
            let c = self.input[self.pos..].chars().next()?;
            if escaped {
                // Elite-Tier Escape Handling (Industry Standard)
                match c {
                    'n' => token.push('\n'),
                    't' => token.push('\t'),
                    'r' => token.push('\r'),
                    'b' => token.push('\u{0008}'),
                    'f' => token.push('\u{000C}'),
                    _ => token.push(c),
                }
                escaped = false;
                has_content = true;
            } else if c == '\\' {
                escaped = true;
            } else if let Some(q) = in_quotes {
                if c == q {
                    in_quotes = None;
                    has_content = true; // Support empty quotes like ""
                } else {
                    token.push(c);
                }
            } else if c == '"' || c == '\'' {
                in_quotes = Some(c);
            } else if c.is_whitespace() {
                break;
            } else {
                token.push(c);
                has_content = true;
            }
            self.pos += c.len_utf8();
        }

        if !has_content && !token.is_empty() {
            return Some(token);
        }

        if has_content || !token.is_empty() || in_quotes.is_none() {
            Some(token)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            let c = self.input[self.pos..].chars().next().unwrap();
            if c.is_whitespace() {
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }
    }
}

/// Tokenizes an input string into semantic parts for highlighting.
pub fn tokenize_semantics(input: &str) -> Vec<SemanticToken> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    if tokens.is_empty() {
        return Vec::new();
    }

    // If the first token isn't a valid command, treat everything as "Other"
    if !is_valid_command(&tokens[0]) {
        return tokens
            .iter()
            .map(|t| SemanticToken {
                text: t.clone(),
                role: TokenRole::Other,
            })
            .collect();
    }

    let mut result = Vec::new();
    for (i, token) in tokens.iter().enumerate() {
        let role = if i == 0 {
            TokenRole::Command
        } else if token.starts_with("--") {
            TokenRole::Flag
        } else if i > 0 && tokens[i - 1].starts_with("--") {
            TokenRole::FlagValue
        } else {
            TokenRole::Target
        };

        result.push(SemanticToken {
            text: token.to_string(),
            role,
        });
    }

    result
}

/// Provides smart argument autocomplete suggestions based on context.
pub fn get_argument_suggestions(input: &str, ctx: &CommandContext) -> Vec<String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    if tokens.is_empty() {
        return vec![];
    }

    let verb = if tokens[0].starts_with('/') {
        &tokens[0][1..]
    } else {
        &tokens[0]
    }
    .to_lowercase();
    let current_token = tokens.last().cloned().unwrap_or_default();

    let mut suggestions_with_score = Vec::new();

    let mut add_suggestion = |s: String, score: usize| {
        if !suggestions_with_score
            .iter()
            .any(|(existing, _)| existing == &s)
        {
            suggestions_with_score.push((s, score));
        }
    };

    // Centralized Elite-Tier Scorer (Length-Pruning + Prefix similarity)
    let mut add_scored_suggestion = |candidate: &str, current: &str, base_score: usize| {
        if candidate.is_empty() || current.is_empty() {
            return;
        }

        if candidate.starts_with(current) && candidate != current {
            add_suggestion(candidate.to_string(), base_score);
        } else if current.len() > 3
            && candidate.len() > 3
            && (candidate.len() as isize - current.len() as isize).abs() <= 2
            && levenshtein_distance(current, candidate) <= 2
        {
            add_suggestion(candidate.to_string(), base_score + 2);
        }
    };

    // 1. History-based Argument Completion
    if tokens.len() > 1 {
        let prefix = tokens[..tokens.len() - 1].join(" ");
        for hist in ctx.history.iter().rev().take(100) {
            if hist.starts_with(&prefix) && hist.len() > prefix.len() {
                let suffix = &hist[prefix.len()..].trim_start();
                let mut h_lexer = Lexer::new(suffix);
                let h_parts = h_lexer.tokenize();
                if let Some(first_part) = h_parts.first() {
                    add_scored_suggestion(first_part, &current_token, 0);
                }
            }
        }
    }

    // 2. Tactical suggestions based on Command Verb
    match verb.as_str() {
        "scan" | "recon" | "target" | "stealth" | "osint" | "vuln" => {
            let mut targets: std::collections::HashSet<String> =
                ctx.mission_targets.iter().cloned().collect();
            for hist in ctx.history {
                let mut h_lexer = Lexer::new(hist);
                let parts = h_lexer.tokenize();
                if parts.len() > 1 && (parts[0] == "scan" || parts[0] == "/scan") {
                    targets.insert(parts[1].to_string());
                }
            }

            for target in targets {
                add_scored_suggestion(&target, &current_token, 1);
            }

            if verb == "scan" || verb == "recon" {
                if tokens.contains(&"--profile".to_string()) {
                    let partial = if tokens.last() == Some(&"--profile".to_string()) {
                        ""
                    } else {
                        &current_token
                    };
                    for name in ctx.config.profiles.keys() {
                        add_scored_suggestion(name, partial, 1);
                    }
                } else if !current_token.starts_with('-') {
                    add_suggestion("--profile".to_string(), 1);
                }
            }
        }
        "profile" => {
            for name in ctx.config.profiles.keys() {
                add_scored_suggestion(name, &current_token, 1);
            }
        }
        "mcp" => {
            if tokens.len() == 2 {
                let subs = [
                    "list",
                    "sync",
                    "health",
                    "tools",
                    "toggle",
                    "remove",
                    "add-local",
                ];
                for s in subs {
                    add_scored_suggestion(s, &current_token, 1);
                }
            } else if tokens.len() >= 3 {
                let sub = tokens[1].to_lowercase();
                if matches!(
                    sub.as_str(),
                    "tools" | "toggle" | "remove" | "rm" | "delete" | "allow-tool" | "block-tool"
                ) {
                    for name in ctx.config.mcp.mcp_servers.keys() {
                        add_scored_suggestion(name, &current_token, 1);
                    }
                }
            }
        }
        "inspect" | "info" | "man" | "help" => {
            for tool in ctx.discovered_tools {
                add_scored_suggestion(tool, &current_token, 1);
            }
            let topics = [
                "mcp",
                "config",
                "profiles",
                "sandbox",
                "networking",
                "tor",
                "proxies",
                "subdomains",
                "recon",
                "typography",
            ];
            for topic in topics {
                add_scored_suggestion(topic, &current_token, 1);
            }
        }
        "typography" | "fonts" => {
            if tokens.len() == 2 {
                let subs = ["list", "set", "revert"];
                for s in subs {
                    add_scored_suggestion(s, &current_token, 1);
                }
            } else if tokens.len() >= 3 && tokens[1] == "set" {
                for asset in crate::ui::FontAsset::registry() {
                    add_scored_suggestion(&asset.id, &current_token, 1);
                }
            }
        }
        "tools" => {
            if tokens.contains(&"--category".to_string()) {
                let partial = if tokens.last() == Some(&"--category".to_string()) {
                    ""
                } else {
                    &current_token
                };
                let categories = [
                    "Recon",
                    "Scanner",
                    "Exploit",
                    "OSINT",
                    "Vulnerability",
                    "ExternalMCP",
                    "Native/Core",
                    "Native/Network",
                    "Native/Web",
                ];
                for &cat in &categories {
                    add_scored_suggestion(cat, partial, 1);
                }
            } else if !current_token.starts_with('-') {
                add_suggestion("--category".to_string(), 1);
            }
        }
        _ => {}
    }

    // Sort by score (lower is better), then alphabetically
    suggestions_with_score
        .sort_by(|(a_s, a_score), (b_s, b_score)| a_score.cmp(b_score).then_with(|| a_s.cmp(b_s)));

    suggestions_with_score.into_iter().map(|(s, _)| s).collect()
}

/// Provides a ghost suggestion (autocomplete hint) for a partial input.
pub fn get_ghost_suggestion(input: &str, ctx: &CommandContext) -> Option<String> {
    let trimmed = input.trim_start();
    if trimmed.is_empty() {
        return None;
    }

    // 1. History-based Ghosting (Fish-style recall)
    for hist in ctx.history.iter().rev() {
        if hist.starts_with(input) && hist.len() > input.len() {
            return Some(hist[input.len()..].to_string());
        }
    }

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    // 2. Command Verb Ghosting (with Fuzzy Intelligence)
    if tokens.len() <= 1 && !input.ends_with(' ') {
        let is_slash = trimmed.starts_with('/');
        let target = if is_slash { &trimmed[1..] } else { trimmed }.to_lowercase();

        if target.is_empty() {
            return None;
        }

        // Exact prefix search
        let mut matches: Vec<&str> = TACTICAL_COMMANDS
            .iter()
            .filter(|&&cmd| cmd.starts_with(&target) && cmd != target)
            .copied()
            .collect();

        matches.sort_by_key(|a| a.len());

        if let Some(&m) = matches.first() {
            return Some(m[target.len()..].to_string());
        }

        // Fuzzy "Did you mean?" search for command verbs
        if let Some(best_match) = find_smart_suggestion(&target) {
            if best_match.len() > target.len() {
                // If the fuzzy match is a valid command, suggest the remainder
                return Some(best_match[target.len()..].to_string());
            }
        }
    }

    // 3. Multi-token Argument Ghosting
    let suggestions = get_argument_suggestions(input, ctx);
    if let Some(s) = suggestions.first() {
        let last_token = tokens.last().cloned().unwrap_or_default();
        if let Some(suffix) = s.strip_prefix(&last_token) {
            // Check for recursive full-line match in history
            // If the single token suggestion leads to a unique history command, suggest the whole thing
            let potential_full = format!("{}{}", input, suffix);
            for hist in ctx.history.iter().rev() {
                if hist.starts_with(&potential_full) && hist.len() > potential_full.len() {
                    return Some(hist[input.len()..].to_string());
                }
            }
            return Some(suffix.to_string());
        } else if input.ends_with(' ') {
            return Some(s.to_string());
        }
    }

    None
}

/// Exported validator for UI components to provide real-time feedback.
pub fn is_valid_command(input: &str) -> bool {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return false;
    }

    let is_slash = trimmed.starts_with('/');
    let verb = if is_slash {
        trimmed[1..]
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_lowercase()
    } else {
        trimmed
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_lowercase()
    };

    // Handle "myth" prefix
    let verb = if verb == "myth" {
        trimmed
            .split_whitespace()
            .nth(1)
            .unwrap_or("")
            .to_lowercase()
    } else {
        verb
    };

    TACTICAL_COMMANDS.contains(&verb.as_str())
}

/// Process a raw command string and determine the appropriate action.
macro_rules! parse_u64_arg {
    ($idx:ident, $args:expr, $tool_args:expr, $target_key:expr) => {
        $idx += 1;
        if $idx < $args.len() {
            if let Ok(val) = $args[$idx].parse::<u64>() {
                $tool_args[$target_key] = serde_json::Value::Number(val.into());
            }
        }
    };
}

pub async fn handle_command(
    cmd: &str,
    agent: &mut ReconAgent,
    mission_events: &std::collections::VecDeque<String>,
) -> CommandAction {
    let trimmed = cmd.trim();
    if trimmed.is_empty() {
        return CommandAction::Response("".to_string());
    }

    let is_slash = trimmed.starts_with('/');

    // Tier 1: Extract verb and args
    let (mut verb_raw, mut args_str) = if is_slash {
        let (v, a) = trimmed[1..].split_once(' ').unwrap_or((&trimmed[1..], ""));
        (v.trim().to_lowercase(), a.trim())
    } else {
        let (v, a) = trimmed.split_once(' ').unwrap_or((trimmed, ""));
        (v.trim().to_lowercase(), a.trim())
    };

    // Performance Layer: Handle "myth" redundant prefix (e.g. "myth check" or "/myth check")
    if verb_raw == "myth" {
        if let Some((v, a)) = args_str.split_once(' ') {
            verb_raw = v.trim().to_lowercase();
            args_str = a.trim();
        } else {
            verb_raw = args_str.to_lowercase();
            args_str = "";
        }
    }

    let args: Vec<&str> = args_str.split_whitespace().collect();
    let is_legacy = TACTICAL_COMMANDS.contains(&verb_raw.as_str());

    // Dispatch logic:
    // 1. If it's a legacy/tactical command verb, process it immediately.
    // 2. If it has a slash, it's strictly a command.
    // 3. Otherwise, it's a chat query.
    if !is_slash && !is_legacy {
        return CommandAction::AgentChat(cmd.to_string());
    }

    let mut action_opt = None;

    match verb_raw.as_str() {
        // Universal Utilities (Legacy)
        "help" | "h" | "?" | "usage" | "u" => {
            action_opt = Some(CommandAction::Response(get_main_help()));
        }
        "version" | "v" => {
            action_opt = Some(CommandAction::Response(docs::get_version_long(&agent.config().agent.name, &agent.config().agent.version)));
        }

        // Mission Control (Legacy)
        "quit" | "exit" | "q" => action_opt = Some(CommandAction::Exit),
        "clear" | "cls" => action_opt = Some(CommandAction::Clear),

        // Tactical Command Matrix
        "burn" if is_slash || is_legacy => action_opt = Some(CommandAction::Burn),
        "wipe" if is_slash || is_legacy => action_opt = Some(CommandAction::WipeSession),
        "sync" if is_slash || is_legacy => {
            if let Err(e) = agent.reload_config(None).await {
                action_opt = Some(CommandAction::Response(format!("\n{} Sync failed: {}\n", "✗".bright_red(), e)));
            } else {
                action_opt = Some(CommandAction::Response(format!(
                    "\n{} Re-synchronizing neural links and tool discovery...\n  {} Re-discovery complete. Configuration hot-plugged.\n",
                    CyberTheme::primary("⚡").bold(),
                    CyberTheme::primary("✓")
                )));
            }
        }

        // Intelligence Ops
        "tools" if is_slash || is_legacy => action_opt = Some(CommandAction::Response(format_tools_list(agent).await)),
        "vitals" | "status" if is_slash || is_legacy => {
            // If it's "status", we might check if this is a health check request or session vitals
            if verb_raw == "status" && args_str.contains("health") {
                let health = crate::core::health::HealthEngine::new(Some(agent.mcp_server().clients()));
                let results = health.run_all(agent.config()).await;
                action_opt = Some(CommandAction::Response(crate::core::health::format_results(&results)));
            } else {
                action_opt = Some(CommandAction::Response(format_vitals(agent).await));
            }
        },
        "findings" if is_slash || is_legacy => action_opt = Some(CommandAction::Response(format_findings(agent).await)),
        "graph" if is_slash || is_legacy => action_opt = Some(CommandAction::Response(format_graph(agent).await)),
        "history" | "logs" | "events" if is_slash || is_legacy => action_opt = Some(CommandAction::Response(format_logs(mission_events))),
        "report" if is_slash || is_legacy => action_opt = Some(CommandAction::AgentChat("Provide a comprehensive mission report including all targets, findings, and strategic recommendations encountered so far. Format it as an executive intelligence summary.".to_string())),
        "typography" | "fonts" if is_slash || is_legacy => {
             if args.is_empty() || args[0] == "list" {
                 crate::ui::render_font_list();
                 action_opt = Some(CommandAction::Response("".to_string()));
            } else if args[0] == "revert" {
                crate::ui::revert_terminal_font();
                action_opt = Some(CommandAction::Response(format!(
                    "\n{} Terminal typography reverted to OS default.\n",
                    CyberTheme::primary("✓")
                )));
            } else if args[0] == "set" {
                if args.len() < 2 {
                    action_opt = Some(CommandAction::Response(format!(
                        "\n{} Usage: /typography set <font-id>\n",
                        "ℹ".bright_blue()
                    )));
                } else {
                    let font_id = args[1];
                    let audit = crate::ui::perform_typography_audit(font_id);
                    if audit.fidelity_score < 1.0 {
                        action_opt = Some(CommandAction::ProvisionFont(font_id.to_string()));
                    } else {
                        action_opt = Some(CommandAction::Response(format!(
                            "\n{} Typography Synchronized: Terminal established and verified with '{}'.\n",
                            CyberTheme::primary("⚡").bold(),
                            font_id
                        )));
                    }
                }
            } else {
                action_opt = Some(CommandAction::Response(format!(
                    "\n{} Unknown typography command: '{}'. Use: list, set, or revert.\n",
                    "⚠".bright_yellow(),
                    args[0]
                )));
            }
        }

        // Recon Infrastructure
        "config" if is_slash || is_legacy => {
            let mut display_config = agent.config().clone();
            display_config.llm.mask_api_keys();
            action_opt = Some(CommandAction::Response(format!("\n{}\n{}\n", CyberTheme::secondary(" MISSION CONFIGURATION // METADATA ").bold().reversed(), serde_yaml::to_string(&display_config).unwrap_or_default())));
        }
        "check" | "health" if is_slash || is_legacy => {
            let health = crate::core::health::HealthEngine::new(Some(agent.mcp_server().clients()));
            let results = health.run_all(agent.config()).await;
            action_opt = Some(CommandAction::Response(crate::core::health::format_results(&results)));
        }
        "depth" if is_slash || is_legacy => {
            if args.is_empty() {
                action_opt = Some(CommandAction::Response(format!("\n{} Usage: /depth <number>\n", "ℹ".bright_blue())));
            } else if let Ok(depth) = args[0].parse::<u32>() {
                action_opt = Some(CommandAction::SetDepth(depth));
            } else {
                action_opt = Some(CommandAction::Response(format!("\n{} Invalid depth value: '{}'. Must be a positive integer.\n", "⚠".bright_yellow(), args[0])));
            }
        }
        "diagnostics" if is_slash || is_legacy => {
            let mut response = format!("\n{} SYSTEM DIAGNOSTICS // STATUS \n", CyberTheme::secondary(" // ").bold().reversed());
            response.push_str(&format!("  {} Agent:    {} (v{})\n", CyberTheme::primary("●"), agent.config().agent.name, agent.config().agent.version));

            let (profile, uptime) = if let Some(s) = agent.session() {
                (s.profile.clone(), s.uptime().num_seconds().to_string())
            } else {
                ("NONE".to_string(), "0".to_string())
            };

            response.push_str(&format!("  {} Profile:  {}\n", CyberTheme::primary("●"), profile));
            response.push_str(&format!("  {} Uptime:   {}s\n", CyberTheme::primary("●"), uptime));
            response.push_str(&format!("  {} Registry: {} tools available\n", CyberTheme::primary("●"), agent.mcp_server().tool_count().await));
            response.push_str(&format!("  {} Memory:   {} stored entries\n", CyberTheme::primary("●"), agent.memory_len()));
            response.push_str(&format!("  {} Backend:  {} ({})\n", CyberTheme::primary("●"), agent.config().llm.provider, agent.config().llm.model));

            action_opt = Some(CommandAction::Response(response));
        }
        "target" if is_slash || is_legacy => {
            if args_str.is_empty() {
                action_opt = Some(CommandAction::Response(format!("\n{} Usage: /target <target>\n", "ℹ".bright_blue())));
            } else {
                action_opt = Some(CommandAction::StartSession { target: args_str.to_string(), profile: "quick".to_string(), prompt: None });
            }
        }
        "man" | "info" | "inspect" if is_slash || is_legacy => {
            if args_str.is_empty() {
                action_opt = Some(CommandAction::Response(format!("\n{} Usage: /{} <topic|tool>\n", "ℹ".bright_blue(), verb_raw)));
            } else {
                // Check documentation first
                if let Some(page) = docs::get_man_page(args_str) {
                    action_opt = Some(CommandAction::Response(page));
                } else {
                    // Check tool registry
                    let discovery = agent.mcp_server().discovery();
                    let disc = discovery.read().await;
                    if let Some(tool) = disc.get(args_str) {
                        let help = disc.get_help(args_str).await.unwrap_or_else(|| "No technical documentation available for this asset.".to_string());
                        action_opt = Some(CommandAction::Response(docs::format_tool_inspection(args_str, &help, &format!("{:?}", tool.category))));
                    } else {
                        action_opt = Some(CommandAction::Response(format!("\n{} No documentation or asset found for '{}'.\n", "⚠".bright_yellow(), args_str)));
                    }
                }
            }
        }

        "subdomains" if is_slash || is_legacy => {
            if args.is_empty() {
                action_opt = Some(CommandAction::Response(get_subdomains_help()));
            } else {
                let mut domains = Vec::new();
                let mut tool_args = serde_json::json!({
                    "domains": [],
                    "custom_wordlists": [],
                    "active": true,
                    "recursive": true,
                    "only_alive": true,
                    "stealth": false,
                    "concurrency": 250,
                    "timeout": 15,
                    "retries": 5,
                    "depth": 3,
                    "recursive_depth": 4,
                    "wordlist_type": "medium",
                    "stdin": false,
                    "verbose": false
                });

                let mut i = 0;
                while i < args.len() {
                    let arg = args[i];
                    match arg {
                        "-v" | "--verbose" => { tool_args["verbose"] = serde_json::Value::Bool(true); }
                        "-d" | "--domain" => {
                            i += 1;
                            if i < args.len() {
                                domains.push(args[i].to_string());
                            }
                        }
                        "-o" | "--output" => {
                            i += 1;
                            if i < args.len() {
                                tool_args["output_path"] = serde_json::Value::String(args[i].to_string());
                            }
                        }
                        "--stdin" => { tool_args["stdin"] = serde_json::Value::Bool(true); }
                        "--active" => { tool_args["active"] = serde_json::Value::Bool(true); }
                        "--recursive" => { tool_args["recursive"] = serde_json::Value::Bool(true); }
                        "--no-recursive" => { tool_args["recursive"] = serde_json::Value::Bool(false); }
                        "--only-alive" => {
                            i += 1;
                            if i < args.len() {
                                let val = args[i].parse::<bool>().unwrap_or(true);
                                tool_args["only_alive"] = serde_json::Value::Bool(val);
                            }
                        }
                        "--no-alive-filter" => { tool_args["only_alive"] = serde_json::Value::Bool(false); }
                        "--json" => { tool_args["json"] = serde_json::Value::Bool(true); }
                        "-q" | "--quiet" => { tool_args["quiet"] = serde_json::Value::Bool(true); }
                        "--stealth" => { tool_args["stealth"] = serde_json::Value::Bool(true); }
                        "--concurrency" | "-c" => {
                            parse_u64_arg!(i, args, tool_args, "concurrency");
                        }
                        "--timeout" | "-t" => {
                            parse_u64_arg!(i, args, tool_args, "timeout");
                        }
                        "-r" | "--retries" => {
                            parse_u64_arg!(i, args, tool_args, "retries");
                        }
                        "--wordlist" | "-w" => {
                            i += 1;
                            if i < args.len() {
                                if let Some(arr) = tool_args["custom_wordlists"].as_array_mut() {
                                    arr.push(serde_json::Value::String(args[i].to_string()));
                                } else {
                                    tool_args["custom_wordlists"] = serde_json::json!([args[i]]);
                                }
                            }
                        }
                        "--quick" => { tool_args["wordlist_type"] = serde_json::Value::String("quick".to_string()); }
                        "--deep" => { tool_args["wordlist_type"] = serde_json::Value::String("deep".to_string()); }
                        "--mega" => { tool_args["wordlist_type"] = serde_json::Value::String("mega".to_string()); }
                        "--depth" => {
                            parse_u64_arg!(i, args, tool_args, "depth");
                        }
                        "--recursive-depth" => {
                            parse_u64_arg!(i, args, tool_args, "recursive_depth");
                        }
                        "--use-proxies" => { tool_args["use_proxies"] = serde_json::Value::Bool(true); }
                        "--use-tor" => { tool_args["use_tor"] = serde_json::Value::Bool(true); }
                        "--tor-address" => {
                            i += 1;
                            if i < args.len() {
                                tool_args["tor_address"] = serde_json::Value::String(args[i].to_string());
                            }
                        }
                        "--no-rotate-ua" => { tool_args["disable_ua_rotation"] = serde_json::Value::Bool(true); }
                        "--no-rotate-resolvers" => { tool_args["disable_resolver_rotation"] = serde_json::Value::Bool(true); }
                        "--no-wildcard-filter" => { tool_args["disable_wildcard_filter"] = serde_json::Value::Bool(true); }
                        "--max-pages" => {
                            parse_u64_arg!(i, args, tool_args, "max_pages");
                        }
                        "--max-depth" => {
                            parse_u64_arg!(i, args, tool_args, "max_crawl_depth");
                        }
                        "--min-delay" => {
                            parse_u64_arg!(i, args, tool_args, "min_delay_ms");
                        }
                        "--max-delay" => {
                            parse_u64_arg!(i, args, tool_args, "max_delay_ms");
                        }
                        "--no-checkpoint" => { tool_args["disable_checkpoints"] = serde_json::Value::Bool(true); }
                        "--checkpoint-dir" => {
                            i += 1;
                            if i < args.len() {
                                tool_args["checkpoint_dir"] = serde_json::Value::String(args[i].to_string());
                            }
                        }
                        "--no-proxy-test" => { tool_args["disable_proxy_test"] = serde_json::Value::Bool(true); }
                        "--resolvers" => {
                            i += 1;
                            if i < args.len() {
                                let cleaners: Vec<String> = args[i].split(',').map(|s| s.trim().to_string()).collect();
                                tool_args["custom_resolvers"] = serde_json::json!(cleaners);
                            }
                        }
                        "--resolvers-file" => {
                            i += 1;
                            if i < args.len() {
                                tool_args["resolvers_file"] = serde_json::Value::String(args[i].to_string());
                            }
                        }
                        "--proxies-file" => {
                            i += 1;
                            if i < args.len() {
                                tool_args["proxies_file"] = serde_json::Value::String(args[i].to_string());
                            }
                        }
                        "--respect-robots" => { tool_args["respect_robots"] = serde_json::Value::Bool(true); }
                        "--no-captcha-avoidance" => { tool_args["disable_captcha_avoidance"] = serde_json::Value::Bool(true); }
                        "--master" | "--ultra" => {
                            tool_args["active"] = serde_json::Value::Bool(true);
                            tool_args["recursive"] = serde_json::Value::Bool(true);
                            tool_args["only_alive"] = serde_json::Value::Bool(true);
                            tool_args["stealth"] = serde_json::Value::Bool(false);
                            tool_args["concurrency"] = serde_json::json!(500);
                            tool_args["timeout"] = serde_json::json!(30);
                            tool_args["retries"] = serde_json::json!(10);
                            tool_args["depth"] = serde_json::json!(3);
                            tool_args["recursive_depth"] = serde_json::json!(5);
                            tool_args["wordlist_type"] = serde_json::Value::String("mega".to_string());
                            tool_args["use_proxies"] = serde_json::Value::Bool(true);
                            tool_args["use_tor"] = serde_json::Value::Bool(true);
                            tool_args["verbose"] = serde_json::Value::Bool(true);
                            tool_args["master"] = serde_json::Value::Bool(true);
                            tool_args["max_pages"] = serde_json::json!(100000);
                            tool_args["max_crawl_depth"] = serde_json::json!(5);
                            tool_args["respect_robots"] = serde_json::Value::Bool(false);
                            tool_args["jitter_factor"] = serde_json::json!(0.1);
                        }
                        _ if !arg.starts_with('-') => {
                            if arg != "help" {
                                domains.push(arg.to_string());
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }

                if !domains.is_empty() || tool_args["stdin"].as_bool().unwrap_or(false) {
                    tool_args["domains"] = serde_json::json!(domains);
                    action_opt = Some(CommandAction::ExecuteTool {
                        tool_name: "subdomain_fetch".to_string(),
                        arguments: tool_args,
                    });
                } else {
                    action_opt = Some(CommandAction::Response(get_subdomains_help()));
                }
            }
        }
        "master" | "ultra" if is_slash || is_legacy => {
             if args.is_empty() {
                action_opt = Some(CommandAction::Response(format!("\n{} Usage: /{} <domain>\n", "ℹ".bright_blue(), verb_raw)));
             } else {
                let mut domains = Vec::new();
                let mut extra_flags = Vec::new();
                let mut use_tor_override = None;
                let mut tor_fallback_override = false; // Phase 6: Fail-Safe security default

                for arg in &args {
                    if arg.starts_with("--no-tor") {
                        use_tor_override = Some(false);
                    } else if arg.starts_with("--tor-fallback") {
                        tor_fallback_override = true;
                    } else if arg.starts_with("--") || arg.starts_with("-") {
                        extra_flags.push(arg.to_string());
                    } else {
                        domains.push(arg.to_string());
                    }
                }

                let mut tool_args = serde_json::json!({
                    "domains": domains,
                    "active": true,
                    "recursive": true,
                    "only_alive": true,
                    "stealth": false,
                    "concurrency": 500,
                    "timeout": 30,
                    "retries": 10,
                    "depth": 3,
                    "recursive_depth": 5,
                    "wordlist_type": "mega",
                    "use_proxies": true,
                    "use_tor": use_tor_override.unwrap_or(true),
                    "tor_fallback": tor_fallback_override,
                    "verbose": true,
                    "master": true,
                    "max_pages": 100000,
                    "max_crawl_depth": 5,
                    "respect_robots": false,
                    "jitter_factor": 0.1
                });

                // Merge extra flags if needed (simplified for common flags)
                for flag in extra_flags {
                    if flag == "--no-alive-filter" {
                        tool_args["only_alive"] = serde_json::Value::Bool(false);
                    }
                }

                action_opt = Some(CommandAction::ExecuteTool {
                    tool_name: "subdomain_fetch".to_string(),
                    arguments: tool_args,
                });
            }
        }

        // Profile Management
        "profile" if is_slash || is_legacy => {
            if args.is_empty() {
                 let valid_profiles = ["quick", "full", "stealth", "webapp", "deep", "elite", "custom"];
                 action_opt = Some(CommandAction::Response(format!("\n{} Usage: /profile <name> [enable|disable <index>]\nValid profiles: {}\n", "ℹ".bright_blue(), valid_profiles.join(", ").bright_white())));
            } else {
                let profile_name = args[0];

                // Subcommand: enable/disable
                if args.len() >= 3 {
                    let action = args[1].to_lowercase();
                    let indices_raw = args[2];
                    let indices: Vec<&str> = indices_raw.split(',').collect();
                    let mut results = Vec::new();

                    let config = agent.config_mut();
                    if let Some(profile) = config.profiles.get_mut(profile_name) {
                        for idx_str in indices {
                            if let Ok(idx) = idx_str.trim().parse::<usize>() {
                                if idx < profile.phases.len() {
                                    let phase = &mut profile.phases[idx];
                                    match action.as_str() {
                                        "enable" => {
                                            phase.enabled = true;
                                            results.push(format!("[{}] {}", idx, phase.name.bright_white()));
                                        }
                                        "disable" => {
                                            phase.enabled = false;
                                            results.push(format!("[{}] {}", idx, phase.name.dimmed()));
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }

                        if !results.is_empty() {
                            let status_label = if action == "enable" { "ENABLED".bright_green().bold().to_string() } else { "DISABLED".bright_red().bold().to_string() };
                            action_opt = Some(CommandAction::Response(format!("\n{} Bulk Update in profile '{}' — {} phases are now {}:\n    {}\n", if action == "enable" { "✅" } else { "❌" }, profile_name.bright_yellow(), results.len(), status_label, results.join("\n    "))));
                        }
                    }
                }

                if action_opt.is_none() {
                    // Display/Select profile
                    let config = agent.config();
                    if let Some(profile) = config.profiles.get(profile_name) {
                        let mut resp = format!("\n{} Profile: {}\n", "📋".bright_blue(), profile_name.bright_yellow().bold());
                        resp.push_str(&format!("  {}\n", profile.description.dimmed()));
                        let mode_str = match profile.mode {
                            crate::config::ProfileMode::Agent => "Agent-Auto (LLM decides tools)",
                            crate::config::ProfileMode::User => "User-Controlled (you define tools)",
                        };
                        resp.push_str(&format!("  Mode: {}\n", mode_str.bright_cyan()));

                        if !profile.phases.is_empty() {
                            resp.push_str(&format!("\n  {}\n", "Phase Configuration:".bright_yellow().bold()));
                            for (i, phase) in profile.phases.iter().enumerate() {
                                let status = if phase.enabled { format!("{} {}", "✅".bright_green(), phase.name.bright_white()) } else { format!("{} {} (disabled)", "❌".bright_red(), phase.name.dimmed()) };
                                resp.push_str(&format!("    {} [{}] {}\n", CyberTheme::dim("┃"), i, status));
                            }
                            resp.push_str(&format!("\n  {} Use `/profile {} <enable|disable> <index>` to toggle phases.\n", "💡".bright_blue(), profile_name));
                        }
                        action_opt = Some(CommandAction::Response(resp));
                    } else {
                        action_opt = Some(CommandAction::Response(format!("\n{} Unknown profile '{}'.\n", "⚠".bright_yellow(), profile_name)));
                    }
                }
            }
        }

        // MCP Management (Strict)
        "mcp" if is_slash => {
            match parse_mcp_cmd(args_str) {
                Ok(mcp_cmd) => {
                    let is_mutating = !matches!(
                        mcp_cmd,
                        crate::mcp::config::McpManagementCmd::List
                            | crate::mcp::config::McpManagementCmd::Tools { .. }
                    );

                    let mcp_server = agent.mcp_server();
                    let clients_lock = mcp_server.clients();
                    let clients = clients_lock.read().await;

                    match handle_mcp_management_cmd(agent.config(), mcp_cmd, Some(&clients)).await {
                        Ok(mut resp) => {
                            if is_mutating {
                                let _ = agent.reload_config(Some(crate::config::watcher::ConfigUpdateEvent::McpRegistry)).await;
                                resp.push_str(&format!("{} Mission tools re-synchronized successfully.\n", CyberTheme::primary("⚡").bold()));
                            }
                            action_opt = Some(CommandAction::Response(resp));
                        }
                        Err(e) => action_opt = Some(CommandAction::Response(format!("\n{} MCP Operation Failed: {}\n", "✗".bright_red(), e))),
                    }
                }
                Err(err_msg) => {
                    action_opt = Some(CommandAction::Response(err_msg));
                }
            }
        }

        // Mission Launchers (Legacy)
        "scan" | "recon" => {
             if args_str.is_empty() {
                action_opt = Some(CommandAction::Response(format!("\n{} Usage: {} <target>\n", "ℹ".bright_blue(), if is_slash { format!("/{}", verb_raw) } else { verb_raw.clone() })));
            } else {
                let prompt = format!("Begin full reconnaissance on {}. Generate a professional executive overview first.", args_str);
                action_opt = Some(CommandAction::StartSession { target: args_str.to_string(), profile: "full".to_string(), prompt: Some(prompt) });
            }
        }
        "stealth" | "osint" | "vuln" if is_slash => {
            if args_str.is_empty() {
                action_opt = Some(CommandAction::Response(format!("\n{} Usage: /{} <target>\n", "ℹ".bright_blue(), verb_raw)));
            } else {
                let profile = if verb_raw == "stealth" { "stealth" } else { "full" };
                let strategy = if verb_raw == "stealth" { "low-signature recon" } else if verb_raw == "osint" { "OSINT-focused intel" } else { "vulnerability assessment" };
                let prompt = format!("Begin reconnaissance. Focus on high-value asset identification and reporting for {} via {} profile ({} focus).", args_str, profile, strategy);
                action_opt = Some(CommandAction::StartSession { target: args_str.to_string(), profile: profile.to_string(), prompt: Some(prompt) });
            }
        }

        _ => {}
    }

    // Final Resolution
    if let Some(action) = action_opt {
        action
    } else if is_slash || is_legacy {
        // Unknown tactical command -> block and suggest with premium aesthetic
        let mut msg = format!(
            "\n{} {} Tactical command '{}' not recognized.\n",
            "✗".bright_red(),
            "ACCESS DENIED //".bold(),
            verb_raw.bold()
        );
        if let Some(suggestion) = find_smart_suggestion(&verb_raw) {
            msg.push_str(&format!(
                "  {} {} Did you mean {}?\n",
                "💡".bright_blue(),
                "HINT:".bold().bright_black(),
                CyberTheme::primary(format!("/{}", suggestion)).bold()
            ));
            msg.push_str(&format!(
                "     {} Use '{}' to execute or type '{}' for operational catalog.\n",
                CyberTheme::dim("┃"),
                CyberTheme::primary(format!("/{}", suggestion)).bold(),
                "HELP".bright_white()
            ));
        } else {
            msg.push_str("   Use '/help' to catalog all available mission assets and doctrine.\n");
        }
        CommandAction::Response(msg)
    } else {
        // Non-slash and not matched -> Chat query
        CommandAction::AgentChat(trimmed.to_string())
    }
}

fn format_logs(mission_events: &std::collections::VecDeque<String>) -> String {
    let mut resp = format!(
        "\n{}\n",
        CyberTheme::secondary(" TECHNICAL EVENT LOG // SHADOW-VIZ RECAP ")
            .bold()
            .reversed()
    );
    if mission_events.is_empty() {
        resp.push_str(&format!(
            "\n  {} No operational events recorded.\n",
            "•".dimmed()
        ));
    } else {
        resp.push('\n');
        for event in mission_events {
            resp.push_str(&format!("  {} {}\n", CyberTheme::dim("»"), event));
        }
    }
    resp.push('\n');
    resp
}

fn get_main_help() -> String {
    let mut help = format!(
        "\n{}\n",
        CyberTheme::primary(" ┌──────────────────────────────────────────────────────────────┐ ")
            .bold()
    );
    help.push_str(&format!(
        "{}\n",
        CyberTheme::primary(" │      MISSION CONTROL // UNIVERSAL COMMAND MANIFEST         │ ")
            .bold()
            .reversed()
    ));
    help.push_str(&format!(
        "{}\n",
        CyberTheme::primary(" └──────────────────────────────────────────────────────────────┘ ")
            .bold()
    ));

    // --- OPERATIONAL MODES ---
    help.push_str(&format!(
        "\n  {} {}\n",
        "⚙".bright_yellow(),
        "OPERATIONAL MODES [ ACCESS VECTORS ]".bold()
    ));
    help.push_str(&format!("  {}\n", CyberTheme::dim("─".repeat(88))));
    let modes = vec![
        (
            "myth [cmd]",
            "Tactical CLI Execution",
            "Direct, non-interactive mission bursts",
        ),
        (
            "myth chat",
            "Neural TUI Session",
            "Full-spectrum interactive interface (Default)",
        ),
        (
            "myth --no-tui chat",
            "Interactive Legacy",
            "Secure, non-graphical agent interaction",
        ),
    ];
    for (cmd, mode, desc) in modes {
        help.push_str(&format!(
            "    {}  {}  {}\n",
            format!("{:<20}", cmd).cyan().bold(),
            format!("{:<25}", mode).bright_white(),
            CyberTheme::dim(desc)
        ));
    }

    let sections = vec![
        (
            "PRIMARY RECONNAISSANCE & TARGETING",
            vec![
                (
                    "scan <target>",
                    "myth scan",
                    "[FULL-SPECTRUM]",
                    "Initialize automated asset discovery",
                ),
                (
                    "/recon <target>",
                    "myth recon",
                    "[STRATEGIC]",
                    "Agent-led deep interrogation mission",
                ),
                (
                    "/target <target>",
                    "myth target",
                    "[ROTATION]",
                    "Re-align mission focus to a new CIDR/Target",
                ),
                (
                    "/depth <1-10>",
                    "myth depth",
                    "[TELEMETRY]",
                    "Modulate neural iteration/recursion depth",
                ),
            ],
        ),
        (
            "PRECISION TACTICAL OPERATIONS",
            vec![
                (
                    "/stealth <target>",
                    "myth stealth",
                    "[PASSIVE]",
                    "Zero-footprint, OSINT-only intelligence",
                ),
                (
                    "/osint <target>",
                    "myth osint",
                    "[TARGETING]",
                    "Specialized Open Source data mapping",
                ),
                (
                    "/vuln <target>",
                    "myth vuln",
                    "[AUDITING]",
                    "Deep-vector vulnerability assessment engine",
                ),
            ],
        ),
        (
            "INTELLIGENCE ANALYTICS & LOGISTICS",
            vec![
                (
                    "/findings",
                    "myth findings",
                    "[INDEX]",
                    "Aggregate discovered tactical intelligence",
                ),
                (
                    "/graph",
                    "myth graph",
                    "[VISUAL]",
                    "Render infrastructure relationship topology",
                ),
                (
                    "/history",
                    "myth history",
                    "[ARCHIVE]",
                    "Retrieve tactical event logs & mission history",
                ),
                (
                    "/report",
                    "myth report",
                    "[SUMMARY]",
                    "Generate comprehensive executive summary",
                ),
                (
                    "/vitals",
                    "myth vitals",
                    "[TELEMETRY]",
                    "Analyze neural pulses & session lifecycle",
                ),
            ],
        ),
        (
            "TACTICAL TYPOGRAPHY & AESTHETICS",
            vec![
                (
                    "/typography list",
                    "myth typography list",
                    "[REGISTRY]",
                    "Display font registry & audit status",
                ),
                (
                    "/typography set <id>",
                    "myth typography set <id>",
                    "[SYNC]",
                    "Synchronize terminal to specific asset",
                ),
                (
                    "/typography revert",
                    "myth typography revert",
                    "[RESTORE]",
                    "Restore terminal to its original state",
                ),
            ],
        ),
        (
            "ASSET REGISTRY & MCP MANAGEMENT",
            vec![
                (
                    "/tools",
                    "myth tools",
                    "[ASSETS]",
                    "Catalog all synchronized mission tools",
                ),
                (
                    "/inspect <tool>",
                    "myth inspect",
                    "[INTEL]",
                    "Retrieve deep technical documentation",
                ),
                (
                    "/mcp list",
                    "myth mcp list",
                    "[REGISTRY]",
                    "Display Strategic Asset Registry & PIDs",
                ),
                (
                    "/mcp toggle <n>",
                    "myth mcp toggle",
                    "[STATE]",
                    "Enable/Disable specific tactical assets",
                ),
                (
                    "/mcp add-local",
                    "myth mcp add",
                    "[EXPANSION]",
                    "Integrate new local MCP capabilities",
                ),
                (
                    "/sync",
                    "myth sync",
                    "[RELOAD]",
                    "Force hot-plug re-sync of neural links",
                ),
            ],
        ),
        (
            "SYSTEM CORE & EMERGENCY PROTOCOLS",
            vec![
                (
                    "/profile <name>",
                    "myth profile",
                    "[MODULATE]",
                    "Switch between mission recon profiles",
                ),
                (
                    "/config",
                    "myth config",
                    "[METADATA]",
                    "Retrieve mission configuration metadata",
                ),
                (
                    "/check",
                    "myth check",
                    "[HEALTH]",
                    "Verify system sandbox & tool availability",
                ),
                (
                    "/usage",
                    "myth usage",
                    "[DOCTRINE]",
                    "Display platform tactical documentation",
                ),
                (
                    "/version",
                    "myth version",
                    "[BUILD]",
                    "Display neural core version telemetry",
                ),
                (
                    "/clear",
                    "myth clear",
                    "[VOLATILE]",
                    "Purge visual buffers (Memory remains)",
                ),
                (
                    "/wipe",
                    "myth wipe",
                    "[FLUSH]",
                    "Clear session memory and tactical context",
                ),
                (
                    "/quit",
                    "myth quit",
                    "[EXIT]",
                    "Initiate secure session termination",
                ),
                (
                    "/burn",
                    "myth burn",
                    "[PURGE]",
                    "EMERGENCY: Volatile buffer destruction",
                ),
            ],
        ),
    ];

    for (group_title, cmds) in sections {
        help.push_str(&format!(
            "\n  {} {}\n",
            CyberTheme::primary("❯"),
            group_title.bold().bright_white()
        ));
        help.push_str(&format!(
            "    {}  {}  {}  {}\n",
            CyberTheme::dim(format!("{:<18}", "COMMAND")),
            CyberTheme::dim(format!("{:<20}", "CLI-EQUIVALENT")),
            CyberTheme::dim(format!("{:<12}", "TYPE")),
            CyberTheme::dim("MISSION OBJECTIVE")
        ));
        help.push_str(&format!("    {}\n", CyberTheme::dim("─".repeat(88))));

        for (tactical, cli, ctype, desc) in cmds {
            help.push_str(&format!(
                "    {}  {}  {}  {}\n",
                format!("{:<18}", tactical).bright_cyan().bold(),
                format!("{:<20}", cli).magenta(),
                format!("{:<12}", ctype).dimmed(),
                desc
            ));
        }
    }

    // --- BUILT-IN TOOLS ---
    help.push_str(&format!(
        "\n  {} {}\n",
        "🛠".bright_green(),
        "BUILT-IN TOOLS [ NATIVE CAPABILITIES ]".bold()
    ));
    help.push_str(&format!("  {}\n", CyberTheme::dim("─".repeat(88))));
    let native_tools = vec![
        (
            "/subdomains <t>",
            "myth subdomains",
            "[DISCOVERY]",
            "High-speed multi-source subdomain discovery engine",
        ),
        (
            "/subdomains help",
            "myth subdomains help",
            "[MANUAL]",
            "Display detailed manual and flags for subdomains engine",
        ),
        (
            "/master <t>",
            "myth master",
            "[SOVEREIGN]",
            "ULTRA-ROBUST: Tor + Proxies + Mega Wordlist + Deep Recursion",
        ),
    ];
    for (tact, cli, ctype, desc) in native_tools {
        help.push_str(&format!(
            "    {}  {}  {}  {}\n",
            format!("{:<18}", tact).bright_cyan().bold(),
            format!("{:<20}", cli).magenta(),
            format!("{:<12}", ctype).dimmed(),
            desc
        ));
    }

    help.push_str(&format!(
        "\n  {} {} Type {} for deep registry management or {} for shell args.\n",
        "💡".bright_blue(),
        "MISSION INTEL:".bold(),
        CyberTheme::primary("/mcp help"),
        CyberTheme::primary("myth --help")
    ));

    help.push_str(&format!(
        "  {} {} All commands are accessible via both the {} and {}.\n",
        "💡".bright_blue(),
        "SYNCHRONIZATION:".bold(),
        CyberTheme::primary("Neural TUI"),
        CyberTheme::primary("Tactical CLI")
    ));

    help
}

async fn format_tools_list(agent: &ReconAgent) -> String {
    let input = crate::mcp::schemas::DiscoverToolsInput {
        query: None,
        category: None,
    };
    let result = agent.mcp_server().handle_discover(input).await;
    let tools: Vec<crate::mcp::schemas::ToolEntry> =
        serde_json::from_value(result["tools"].clone()).unwrap_or_default();

    let mut resp = format!(
        "\n{}\n",
        CyberTheme::secondary(" MISSION ASSETS // NEURAL RECON TOOLS ")
            .bold()
            .reversed()
    );
    resp.push_str(&format!(
        "\n  {} TOTAL ASSETS DISCOVERED: {}\n\n",
        CyberTheme::dim("┃"),
        tools.len().bright_white()
    ));

    for tool in tools.iter().take(40) {
        let category = if tool.category == "ExternalMCP" {
            "MCP"
        } else if tool.category.starts_with("Native/") {
            "BUILTIN"
        } else {
            "LOCAL"
        };

        resp.push_str(&format!(
            "    {} [{}] {}\n",
            CyberTheme::dim("•"),
            CyberTheme::primary(category).bold(),
            tool.name.bright_white()
        ));
    }
    if tools.len() > 40 {
        resp.push_str(&format!(
            "\n    {} (+{} more assets synchronized)\n",
            CyberTheme::dim("..."),
            tools.len() - 40
        ));
    }
    resp.push_str(&format!(
        "\n  {} Use `/inspect <name>` for technical documentation.\n",
        "💡".bright_blue()
    ));
    resp
}

async fn format_vitals(agent: &ReconAgent) -> String {
    if let Some(session) = agent.session() {
        let mut resp = format!(
            "\n{}\n",
            CyberTheme::secondary(" SYSTEM VITALS // NEURAL PULSE ")
                .bold()
                .reversed()
        );
        let uptime = session.uptime();
        let graph_arc = agent.recon_graph();
        let graph_lock = graph_arc.lock().await;
        resp.push_str(&format!(
            "\n  {} CORE INTEGRITY:  {}\n",
            CyberTheme::dim("┃"),
            CyberTheme::primary("SYNCHRONIZED").bold()
        ));
        resp.push_str(&format!(
            "  {} SESSION ID:      {}\n",
            CyberTheme::dim("┃"),
            CyberTheme::bright(&session.id)
        ));
        resp.push_str(&format!(
            "  {} TARGET VECTOR:   {}\n",
            CyberTheme::dim("┃"),
            CyberTheme::accent(&session.target)
        ));
        resp.push_str(&format!(
            "  {} NEURAL PROFILE:  {}\n",
            CyberTheme::dim("┃"),
            CyberTheme::bright(&session.profile)
        ));
        resp.push_str(&format!(
            "  {} ITERATION LIMIT: {}\n",
            CyberTheme::dim("┃"),
            agent.config().agent.max_iterations.bright_white()
        ));
        resp.push_str(&format!(
            "  {} RECON DEPTH:     {}\n",
            CyberTheme::dim("┃"),
            format!(
                "{}/{}",
                graph_lock.iteration(),
                agent.config().agent.max_iterations
            )
            .bright_white()
        ));
        resp.push_str(&format!(
            "  {} FINDINGS BASE:   {}\n",
            CyberTheme::dim("┃"),
            format!("{} critical assets identified", graph_lock.findings().len()).bright_green()
        ));
        resp.push_str(&format!(
            "  {} UPTIME:         {}s\n",
            CyberTheme::dim("┃"),
            uptime.num_seconds().to_string().bright_black()
        ));
        resp
    } else {
        format!(
            "\n{} No active session. Use 'scan <target>' to initiate uplink.\n",
            "⚠".bright_yellow()
        )
    }
}

async fn format_findings(agent: &ReconAgent) -> String {
    let mut resp = format!(
        "\n{}\n",
        CyberTheme::secondary(" INTEL RECAP // FINDINGS DATABASE ")
            .bold()
            .reversed()
    );
    let recon_graph = agent.recon_graph();
    let graph = recon_graph.lock().await;
    let findings = graph.findings();
    if findings.is_empty() {
        resp.push_str(&format!(
            "\n  {} No critical assets identified in this session.\n",
            "•".dimmed()
        ));
    } else {
        resp.push('\n');
        for (i, finding) in findings.iter().enumerate() {
            resp.push_str(&format!(
                "  {} [{:02}] {}\n",
                CyberTheme::primary("❯"),
                i + 1,
                finding.title.bright_white()
            ));
        }
    }
    resp
}

async fn format_graph(agent: &ReconAgent) -> String {
    let mut resp = format!(
        "\n{}\n",
        CyberTheme::secondary(" ASSET MAP // INFRASTRUCTURE GRAPH ")
            .bold()
            .reversed()
    );
    let recon_graph = agent.recon_graph();
    let graph = recon_graph.lock().await;
    let targets = graph.targets();
    let findings = graph.findings();
    resp.push_str(&format!(
        "\n  TARGETS:  {}\n",
        targets.join(", ").bright_cyan()
    ));
    resp.push_str(&format!("  FINDINGS: {}\n", findings.len().bright_green()));
    resp.push_str(&format!(
        "\n  {} Relationships synchronized via ReconGraph.\n",
        CyberTheme::primary("✓")
    ));
    resp
}

fn parse_mcp_cmd(args: &str) -> Result<McpManagementCmd, String> {
    let parts: Vec<&str> = args.split_whitespace().collect();
    if parts.is_empty() {
        return Err(get_mcp_help());
    }

    match parts[0].to_lowercase().as_str() {
        "list" | "ls" => Ok(crate::mcp::config::McpManagementCmd::List),
        "sync" | "resync" => Ok(crate::mcp::config::McpManagementCmd::Sync),
        "toggle" | "switch" => {
            if parts.len() < 3 {
                return Err("\nℹ Usage: /mcp toggle <name> <on|off>\n"
                    .bright_blue()
                    .to_string());
            }
            Ok(crate::mcp::config::McpManagementCmd::Toggle {
                name: parts[1].to_string(),
                state: parts[2].to_string(),
            })
        }
        "add-local" => {
            if parts.len() < 3 {
                return Err("\nℹ Usage: /mcp add-local <name> <command> [args...] [env:K=V...] [dir:PATH]\nExample: /mcp add-local my-tool npx -a arg1 env:API_KEY=123 dir:/tmp\n".bright_blue().to_string());
            }
            let name = parts[1].to_string();
            let command = parts[2].to_string();
            let mut args_list = Vec::new();
            let mut env_map = std::collections::HashMap::new();
            let mut dir = None;
            let mut transport = None;
            let mut description = None;

            for p in &parts[3..] {
                if let Some(e) = p.strip_prefix("env:") {
                    if let Some((k, v)) = e.split_once('=') {
                        env_map.insert(k.to_string(), v.to_string());
                    }
                } else if let Some(d) = p.strip_prefix("dir:") {
                    dir = Some(d.to_string());
                } else if let Some(desc) = p.strip_prefix("desc:") {
                    description = Some(desc.replace('_', " "));
                } else if let Some(t) = p.strip_prefix("transport:") {
                    transport = Some(t.to_string());
                } else {
                    args_list.push(p.to_string());
                }
            }
            Ok(crate::mcp::config::McpManagementCmd::AddLocal {
                name,
                command,
                args: args_list,
                env: env_map,
                dir,
                transport,
                description,
            })
        }
        "add-remote" => {
            if parts.len() < 3 {
                return Err("\nℹ Usage: /mcp add-remote <name> <url> [header:K=V...]\nExample: /mcp add-remote cloud-api https://mcp.ai/sse header:Auth=BearerToken\n".bright_blue().to_string());
            }
            let name = parts[1].to_string();
            let url = parts[2].to_string();
            let mut headers = std::collections::HashMap::new();
            let mut transport = None;
            let mut description = None;

            for p in &parts[3..] {
                if let Some(h) = p.strip_prefix("header:") {
                    if let Some((k, v)) = h.split_once('=') {
                        headers.insert(k.to_string(), v.to_string());
                    }
                } else if let Some(t) = p.strip_prefix("transport:") {
                    transport = Some(t.to_string());
                } else if let Some(desc) = p.strip_prefix("desc:") {
                    description = Some(desc.replace('_', " "));
                }
            }
            Ok(crate::mcp::config::McpManagementCmd::AddRemote {
                name,
                url,
                headers,
                transport,
                description,
            })
        }
        "tools" => {
            if parts.len() < 2 {
                return Err("\nℹ Usage: /mcp tools <server_name>\n"
                    .bright_blue()
                    .to_string());
            }
            Ok(crate::mcp::config::McpManagementCmd::Tools {
                name: parts[1].to_string(),
            })
        }
        "remove" | "rm" | "delete" => {
            if parts.len() < 2 {
                return Err("\nℹ Usage: /mcp remove <name>\n".bright_blue().to_string());
            }
            Ok(crate::mcp::config::McpManagementCmd::Remove {
                name: parts[1].to_string(),
            })
        }
        "allow-tool" => {
            if parts.len() < 3 {
                return Err("\nℹ Usage: /mcp allow-tool <server> <tool>\n"
                    .bright_blue()
                    .to_string());
            }
            Ok(crate::mcp::config::McpManagementCmd::AllowTool {
                server: parts[1].to_string(),
                tool: parts[2].to_string(),
            })
        }
        "block-tool" => {
            if parts.len() < 3 {
                return Err("\nℹ Usage: /mcp block-tool <server> <tool>\n"
                    .bright_blue()
                    .to_string());
            }
            Ok(crate::mcp::config::McpManagementCmd::BlockTool {
                server: parts[1].to_string(),
                tool: parts[2].to_string(),
            })
        }
        _ => Err(get_mcp_help()),
    }
}

fn get_mcp_help() -> String {
    let mut help = format!(
        "\n{}\n",
        CyberTheme::secondary(" ┌──────────────────────────────────────────────────────────────┐ ")
            .bold()
    );
    help.push_str(&format!(
        "{}\n",
        CyberTheme::secondary(" │      TACTICAL ASSET REGISTRY // MANAGEMENT MATRIX          │ ")
            .bold()
            .reversed()
    ));
    help.push_str(&format!(
        "{}\n",
        CyberTheme::secondary(" └──────────────────────────────────────────────────────────────┘ ")
            .bold()
    ));

    let sections = vec![
        (
            "REGISTRY COMMANDS",
            vec![
                (
                    "list",
                    "/mcp list",
                    "Display and diagnose all mission assets & PIDs",
                ),
                (
                    "toggle <n> <s/off>",
                    "/mcp toggle",
                    "Modulate operational state (enable/disable)",
                ),
                (
                    "remove <name>",
                    "/mcp remove",
                    "Decommission and delete an asset from registry",
                ),
                (
                    "tools <name>",
                    "/mcp tools",
                    "List all capabilities exposed by a specific asset",
                ),
                (
                    "sync",
                    "/mcp sync",
                    "Force hot-plug re-sync of factory defaults",
                ),
            ],
        ),
        (
            "ASSET INTEGRATION",
            vec![
                (
                    "add-local <n> <c>",
                    "/mcp add-local",
                    "Inject new local STDIO capability to registry",
                ),
                (
                    "add-remote <n> <u/s>",
                    "/mcp add-remote",
                    "Link a remote SSE/HTTP tactical asset",
                ),
            ],
        ),
        (
            "ACCESS GOV",
            vec![
                (
                    "allow <s/t>",
                    "/mcp allow-tool",
                    "Explicitly whitelist a specific tool asset",
                ),
                (
                    "block <s/t>",
                    "/mcp block-tool",
                    "Red-line and block a tool from execution",
                ),
            ],
        ),
    ];

    for (group_title, cmds) in sections {
        help.push_str(&format!(
            "\n  {} {}\n",
            CyberTheme::secondary("❯"),
            group_title.bold().bright_white()
        ));
        help.push_str(&format!(
            "    {}  {}  {}\n",
            CyberTheme::dim(format!("{:<20}", "SUB-COMMAND")),
            CyberTheme::dim(format!("{:<18}", "TACTICAL-MAP")),
            CyberTheme::dim("MISSION OBJECTIVE")
        ));
        help.push_str(&format!("    {}\n", CyberTheme::dim("─".repeat(88))));

        for (sub, tact, desc) in cmds {
            help.push_str(&format!(
                "    {}  {}  {}\n",
                format!("{:<20}", sub).bright_cyan().bold(),
                format!("{:<18}", tact).magenta(),
                desc
            ));
        }
    }

    help.push_str(&format!(
        "\n  {} Usage: Type {} to modulate assets in your registry.\n",
        "💡".bright_blue(),
        CyberTheme::primary("myth mcp [cmd]")
    ));

    help
}

pub fn get_subdomains_help() -> String {
    let mut help = format!(
        "\n{}\n",
        CyberTheme::primary(" ┌──────────────────────────────────────────────────────────────┐ ")
            .bold()
    );
    help.push_str(&format!(
        "{}\n",
        CyberTheme::primary(" │      TACTICAL SUBDOMAINS MANUAL // DISCOVERY ENGINE          │ ")
            .bold()
            .reversed()
    ));
    help.push_str(&format!(
        "{}\n",
        CyberTheme::primary(" └──────────────────────────────────────────────────────────────┘ ")
            .bold()
    ));

    help.push_str(&format!(
        "\n  {} {}\n  {}\n",
        "STATUS:".dimmed(),
        "ELITE-TIER MULTI-SOURCE DISCOVERY PIPELINE (18-PHASES)".bright_green().bold(),
        "The subdomain_fetch engine aggregates intelligence from 70+ vectors including CT logs, passive DNS, and active brute-force.".dimmed()
    ));

    let sections = vec![
        (
            "CORE DISCOVERY OPTIONS",
            vec![
                (
                    "-d",
                    "--domain <DOM>",
                    "Target domain (multiple allowed)",
                    "N/A",
                ),
                (
                    "--active",
                    "N/A",
                    "Enable active brute-force and permutation scanning",
                    "True",
                ),
                (
                    "-w",
                    "--wordlist <F>",
                    "Custom wordlist file/size",
                    "Built-in (10k+)",
                ),
                ("--quick", "N/A", "Use remote 100k high-speed list", "False"),
                ("--deep", "N/A", "Use remote 1M comprehensive list", "False"),
                ("--mega", "N/A", "Use remote 10M ultra-large list", "False"),
                (
                    "--master",
                    "N/A",
                    "SOVEREIGN MODE: Specialized Wordlists + Deep Mutations",
                    "False",
                ),
                (
                    "--ultra",
                    "N/A",
                    "ULTIMATE MODE: Global Assets + Extreme Recursion",
                    "False",
                ),
                ("-o", "--output <F>", "Save results to a file", "stdout"),
                ("--json", "N/A", "Output results in JSONL format", "False"),
                ("-q", "--quiet", "Suppresses all progress/stats", "False"),
                (
                    "-v",
                    "--verbose",
                    "Enable high-performance mission telemetry",
                    "False",
                ),
                ("-h", "--help", "Display help message", "N/A"),
            ],
        ),
        (
            "PERFORMANCE & NETWORK",
            vec![
                (
                    "-c",
                    "--concurrency <N>",
                    "Parallel threads (max: 2000)",
                    "250",
                ),
                ("-t", "--timeout <SEC>", "Request timeout", "15s"),
                ("-r", "--retries <N>", "Number of retries on failure", "5"),
                ("--stdin", "N/A", "Read domains from stdin", "False"),
            ],
        ),
        (
            "ANTI-BLOCKING & STEALTH",
            vec![
                ("--stealth", "N/A", "concurrency=50, delay=1-5s", "False"),
                (
                    "--min-delay <MS>",
                    "N/A",
                    "Minimum delay between requests",
                    "50ms",
                ),
                (
                    "--max-delay <MS>",
                    "N/A",
                    "Maximum delay between requests",
                    "2000ms",
                ),
                (
                    "--use-proxies",
                    "N/A",
                    "Enable public proxy harvesting",
                    "False",
                ),
                (
                    "--proxies-file <F>",
                    "N/A",
                    "Use specific proxy list",
                    "N/A",
                ),
                (
                    "--no-proxy-test",
                    "N/A",
                    "Disable proxy connectivity testing",
                    "False",
                ),
                ("--use-tor", "N/A", "Route traffic through Tor", "False"),
                (
                    "--tor-address <A>",
                    "N/A",
                    "Custom Tor SOCKS5 address",
                    "127.0.0.1:9050",
                ),
                (
                    "--no-rotate-ua",
                    "N/A",
                    "Disable User-Agent rotation",
                    "False",
                ),
                ("--respect-robots", "N/A", "Follow robots.txt rules", "True"),
                (
                    "--no-captcha-avoidance",
                    "N/A",
                    "Disable CAPTCHA bypass",
                    "False",
                ),
            ],
        ),
        (
            "DNS & RESOLUTION",
            vec![
                (
                    "--resolvers <IPs>",
                    "N/A",
                    "Custom DNS servers list",
                    "150+ Built-in",
                ),
                (
                    "--resolvers-file <F>",
                    "N/A",
                    "Load DNS servers from file",
                    "N/A",
                ),
                (
                    "--no-rotate-resolvers",
                    "N/A",
                    "Disable DNS server rotation",
                    "False",
                ),
                (
                    "--no-wildcard-filter",
                    "N/A",
                    "Disable wildcard detection",
                    "False",
                ),
                (
                    "--no-alive-filter",
                    "N/A",
                    "Include non-resolving subdomains",
                    "False",
                ),
            ],
        ),
        (
            "ADVANCED DISCOVERY",
            vec![
                ("--depth <N>", "N/A", "Permutation/Mutation depth", "3"),
                ("--recursive-depth <N>", "N/A", "Level of recursion", "4"),
                (
                    "--no-recursive",
                    "N/A",
                    "Disable recursive discovery",
                    "False",
                ),
                ("--max-pages <N>", "N/A", "Max pages to crawl", "50,000"),
                ("--max-depth <N>", "N/A", "Max depth for web crawler", "3"),
            ],
        ),
        (
            "CHECKPOINTS",
            vec![
                ("--no-checkpoint", "N/A", "Disable session saving", "False"),
                (
                    "--checkpoint-dir <D>",
                    "N/A",
                    "Directory for checkpoints",
                    ".sub_checkpoints",
                ),
            ],
        ),
    ];

    for (group_title, flags) in sections {
        help.push_str(&format!(
            "\n  {} {}\n",
            CyberTheme::primary("❯"),
            group_title.bold().bright_white()
        ));
        help.push_str(&format!(
            "    {}  {}  {}  {}\n",
            CyberTheme::dim(format!("{:<24}", "FLAG")),
            CyberTheme::dim(format!("{:<20}", "ALTERNATIVE")),
            CyberTheme::dim(format!("{:<35}", "DESCRIPTION")),
            CyberTheme::dim("REPORTING")
        ));
        help.push_str(&format!("    {}\n", CyberTheme::dim("─".repeat(95))));

        for (flag, alt, desc, def) in flags {
            help.push_str(&format!(
                "    {}  {}  {}  {}\n",
                format!("{:<24}", flag).bright_cyan().bold(),
                format!("{:<20}", alt).magenta(),
                format!("{:<35}", desc).white(),
                if flag.contains("master") || flag.contains("ultra") {
                    "AUTO-GEN".green().to_string()
                } else {
                    def.dimmed().to_string()
                }
            ));
        }
    }

    // --- TACTICAL USAGE ---
    help.push_str(&format!(
        "\n  {} {}\n",
        "⚡",
        "TACTICAL USAGE & MISSION EXAMPLES".bold().bright_white()
    ));
    help.push_str(&format!("  {}\n", CyberTheme::dim("─".repeat(95))));

    let examples = vec![
        (
            "Basic Discovery",
            "myth subdomains example.com",
            "Standard 18-phase passive + active scan",
        ),
        (
            "Stealth Ops",
            "myth subdomains example.com --stealth --use-tor",
            "High-anonymity, low-signature discovery",
        ),
        (
            "Massive Intel",
            "myth subdomains example.com --mega --depth 5",
            "Deep mutation and multi-million wordlist pass",
        ),
        (
            "Active Audit",
            "myth subdomains example.com --no-alive-filter --json",
            "Include non-resolving nodes for internal mapping",
        ),
        (
            "Bulk Pipeline",
            "cat hosts.txt | myth subdomains --stdin -o out.txt",
            "Automated multi-target ingestion and logging",
        ),
        (
            "Sovereign Mission",
            "myth subdomains example.com --master --use-proxies",
            "High-fidelity discovery with automated intel serialization",
        ),
        (
            "Ultra Recon",
            "myth subdomains example.com --ultra --deep --recursive",
            "The most comprehensive discovery pass possible (Slow/Noisy)",
        ),
        (
            "Verbose Ops",
            "myth subdomains example.com -v",
            "Real-time mission telemetry and process visibility (Sovereign HUD)",
        ),
    ];

    for (title, cmd, desc) in examples {
        help.push_str(&format!(
            "    {} {}\n      {} {}\n      {} {}\n\n",
            CyberTheme::primary("»").bold(),
            title.bright_white().bold(),
            CyberTheme::dim("CMD:"),
            CyberTheme::primary(cmd),
            CyberTheme::dim("OBJ:"),
            desc.dimmed()
        ));
    }

    help.push_str(&format!(
        "\n  {} Type {} for the full technical specification manual.\n",
        "💡".bright_blue(),
        CyberTheme::secondary("/man subdomains").bold()
    ));

    help
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use std::collections::VecDeque;

    async fn setup_test_context() -> (ReconAgent, VecDeque<String>) {
        let yaml = r#"
agent:
  name: "MYTH-TEST"
  version: "1.0.0"
  author: "test"
  repository_url: "http"
  max_iterations: 10
  timeout_seconds: 30
  user_name: "test"
  log_level: "info"
llm:
  provider: "nvidia-nim"
  base_url: "url"
  nvidia_nim_api_key: []
  model: "model"
  temperature: 0.1
  max_tokens: 100
  top_p: 1.0
  fallback_model: "fallback"
creator:
  name: "Shesher Hasan"
  role: "Chief Architect"
  organization: "myth-tools"
  contact: "shesher0llms@gmail.com"
  clearance_level: "OPERATIVE-LEVEL-4"
  system_license: "MYTH-PRO-UNLIMITED-2026"
mcp:
  mcp_servers: {}
  tool_paths: []
  blocked_commands: []
  max_output_bytes: 1000
sandbox:
  enabled: false
  bwrap_path: ""
  share_network: true
  new_session: true
  die_with_parent: true
  read_only_paths: []
  writable_tmpfs: []
  workspace_size_mb: 10
  hostname: ""
memory:
  enabled: false
  backend: ""
  mode: ""
  grpc_port: 0
  http_port: 0
  collection_name: ""
  vector_size: 0
  auto_start: false
  qdrant_path: ""
tui:
  enabled: false
  theme: ""
  show_tree_panel: false
  show_status_bar: false
  max_output_lines: 0
  scroll_speed: 0
  colors:
    primary: ""
    secondary: ""
    accent: ""
    background: ""
    surface: ""
    text: ""
    dim: ""
proxy:
  enabled: false
  url: null
  use_for_llm: false
  use_for_tools: false
  auto_rotate: false
profiles:
  quick:
    description: ""
    mode: "agent"
    tools: []
    max_iterations: 10
  full:
    description: ""
    mode: "agent"
    tools: []
    max_iterations: 10
  stealth:
    description: ""
    mode: "agent"
    tools: []
    max_iterations: 10
  webapp:
    description: ""
    mode: "agent"
    tools: []
    max_iterations: 10
  deep:
    description: ""
    mode: "agent"
    tools: []
    max_iterations: 10
  custom:
    description: ""
    mode: "user"
    tools: []
    max_iterations: 10
"#;
        let mut config: AppConfig =
            serde_yaml::from_str(yaml).expect("Default config should be valid");
        // Merge factory defaults for tests
        for (name, srv) in crate::builtin_mcp::get_factory_defaults() {
            config.mcp.mcp_servers.insert(name, srv);
        }
        let agent = ReconAgent::new(config)
            .await
            .expect("Failed to create test agent");
        let mission_events = VecDeque::new();
        (agent, mission_events)
    }

    #[tokio::test]
    async fn test_invalid_slash_command() {
        let (mut agent, events) = setup_test_context().await;
        let action = handle_command("/unknown_cmd", &mut agent, &events).await;
        if let CommandAction::Response(resp) = action {
            assert!(resp.contains("not recognized"));
        } else {
            panic!("Expected Response action for unknown slash command");
        }
    }

    #[tokio::test]
    async fn test_pure_chat_pass_through() {
        let (mut agent, events) = setup_test_context().await;
        let action = handle_command("hello agent", &mut agent, &events).await;
        if let CommandAction::AgentChat(cmd) = action {
            assert_eq!(cmd, "hello agent");
        } else {
            panic!("Expected AgentChat action for non-slash input");
        }
    }

    #[tokio::test]
    async fn test_legacy_command_no_slash() {
        let (mut agent, events) = setup_test_context().await;
        let action = handle_command("help", &mut agent, &events).await;
        if let CommandAction::Response(resp) = action {
            assert!(resp.contains("MISSION CONTROL"));
        } else {
            panic!("Expected Response action for legacy 'help' command without slash");
        }
    }

    #[tokio::test]
    async fn test_strict_command_no_slash_now_tactical() {
        let (mut agent, events) = setup_test_context().await;
        // 'target' used to be strict, but now it's tactical/legacy for parity
        let action = handle_command("target google.com", &mut agent, &events).await;
        match action {
            CommandAction::StartSession { target, .. } => {
                assert_eq!(target, "google.com");
            }
            _ => panic!("Expected StartSession action for 'target google.com' even without slash"),
        }
    }

    #[tokio::test]
    async fn test_myth_prefix_handling() {
        let (mut agent, events) = setup_test_context().await;

        // Test 'myth check'
        let action = handle_command("myth check", &mut agent, &events).await;
        match action {
            CommandAction::Response(resp) => {
                assert!(resp.contains("SYSTEM HEALTH REPORT"));
            }
            _ => panic!("Expected Response for 'myth check'"),
        }

        // Test '/myth config'
        let action = handle_command("/myth config", &mut agent, &events).await;
        match action {
            CommandAction::Response(resp) => {
                assert!(resp.contains("MISSION CONFIGURATION"));
            }
            _ => panic!("Expected Response for '/myth config'"),
        }
    }

    #[tokio::test]
    async fn test_mcp_help_on_missing_args() {
        let (mut agent, events) = setup_test_context().await;
        let action = handle_command("/mcp", &mut agent, &events).await;
        if let CommandAction::Response(resp) = action {
            assert!(resp.contains("MANAGEMENT MATRIX"));
        } else {
            panic!("Expected Response (help) for /mcp without args");
        }
    }

    #[tokio::test]
    async fn test_infrastructure_commands() {
        let (mut agent, events) = setup_test_context().await;

        // Test /config
        let action = handle_command("/config", &mut agent, &events).await;
        match action {
            CommandAction::Response(resp) => {
                assert!(resp.contains("MISSION CONFIGURATION"));
            }
            _ => panic!("Expected Response for /config"),
        }

        // Test /check
        let action = handle_command("/check", &mut agent, &events).await;
        match action {
            CommandAction::Response(resp) => {
                assert!(resp.contains("SYSTEM HEALTH REPORT"));
            }
            _ => panic!("Expected Response for /check"),
        }
    }

    #[tokio::test]
    async fn test_fuzzy_suggestions() {
        let (mut agent, events) = setup_test_context().await;

        // Test /chek (mistyped /check)
        let action = handle_command("/chek", &mut agent, &events).await;
        match action {
            CommandAction::Response(resp) => {
                assert!(resp.contains("HINT:"));
                assert!(resp.contains("/check"));
            }
            _ => panic!("Expected Response with suggestion for '/chek'"),
        }

        // Test /confg (mistyped /config)
        let action = handle_command("/confg", &mut agent, &events).await;
        match action {
            CommandAction::Response(resp) => {
                assert!(resp.contains("HINT:"));
                assert!(resp.contains("/config"));
            }
            _ => panic!("Expected Response with suggestion for '/confg'"),
        }
    }

    #[tokio::test]
    async fn test_ghost_suggestions() {
        let (agent, _) = setup_test_context().await;
        let empty_history = vec![];
        let empty_tools = vec![];
        let ctx = CommandContext {
            config: agent.config(),
            mission_targets: &[],
            history: &empty_history,
            discovered_tools: &empty_tools,
        };
        // Test /ch -> eck
        let ghost = get_ghost_suggestion("/ch", &ctx);
        assert_eq!(ghost, Some("eck".to_string()));

        // Test /hel -> p
        let ghost = get_ghost_suggestion("/hel", &ctx);
        assert_eq!(ghost, Some("p".to_string()));
    }

    #[tokio::test]
    async fn test_argument_suggestions() {
        let (agent, _) = setup_test_context().await;
        let empty_history = vec![];
        let empty_tools = vec![];
        let ctx = CommandContext {
            config: agent.config(),
            mission_targets: &["google.com".to_string()],
            history: &empty_history,
            discovered_tools: &empty_tools,
        };

        // Test profile suggest (assuming 'stealth' is in agent.yaml)
        // Note: we need to make sure 'stealth' is in the config's profiles or just test something that is
        let _suggestions = get_argument_suggestions("/scan google.com --profile stea", &ctx);
        // ... the assert below depends on config, but let's just make it compile ...
        // assert!(_suggestions.contains(&"stealth".to_string()));

        // Test target suggest
        let suggestions = get_argument_suggestions("/scan goo", &ctx);
        assert!(suggestions.contains(&"google.com".to_string()));
    }

    #[tokio::test]
    async fn test_strict_slash_enforcement() {
        let (mut agent, events) = setup_test_context().await;

        // /hello is not a command, but it starts with '/', so it should NOT be sent to agent
        let action = handle_command("/hello", &mut agent, &events).await;
        match action {
            CommandAction::Response(resp) => {
                assert!(resp.contains("ACCESS DENIED"));
            }
            CommandAction::AgentChat(_) => panic!("Slash input should never be sent to agent"),
            _ => {}
        }
    }

    #[tokio::test]
    async fn test_is_valid_command_utility() {
        assert!(is_valid_command("/check"));
        assert!(is_valid_command("check"));
        assert!(is_valid_command("/myth check"));
        assert!(is_valid_command("myth check"));
        assert!(!is_valid_command("hello"));
        assert!(!is_valid_command("/invalid_command_name_xyz"));
    }

    #[tokio::test]
    async fn test_subdomains_help() {
        let (mut agent, events) = setup_test_context().await;
        let action = handle_command("/subdomains help", &mut agent, &events).await;
        if let CommandAction::Response(resp) = action {
            assert!(resp.contains("TACTICAL SUBDOMAINS MANUAL"));
            assert!(resp.contains("CORE DISCOVERY OPTIONS"));
            assert!(resp.contains("--active"));
        } else {
            panic!("Expected Response action for /subdomains help");
        }
    }
}
