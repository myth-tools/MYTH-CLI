//! TUI App — main event loop, rendering, and input handling.

use crate::tui::layout::ReconLayout;
use crate::tui::theme::Theme;
use crate::tui::widgets::chat::ChatWidget;
use crate::tui::widgets::input::InputWidget;
use crate::tui::widgets::sensor::{SensorStatus, SensorWidget};
use crate::tui::widgets::status::StatusWidget;
use crate::tui::widgets::tree::{TreeNode, TreeWidget};

use tokio::sync::mpsc;

use crossterm::event::{self, KeyCode, KeyModifiers};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

/// Focus state for keyboard input routing.
#[derive(Debug, Clone, PartialEq)]
pub enum Focus {
    Input,
    Chat,
    Tree,
    McpManager,
}

/// Events sent from the agent to the TUI.
#[derive(Debug, Clone)]
pub enum TuiEvent {
    Message { role: String, content: String },
    MessageStart { role: String },
    MessageChunk { chunk: String },
    ProcessingStatus(bool),
    ToolStarted { tool: String, command: String },
    ToolStream { tool: String, line: String },
    ToolFinished { tool: String, success: bool },
    StateUpdate(String),
    IterationUpdate(u32),
    FindingsUpdate(usize),
    TargetUpdate(String),
    ConfigReloaded(Box<crate::config::AppConfig>),
    ToolExecution { name: String, status: String },
    ClearChat,
}

/// Main TUI application state.
pub struct App {
    pub theme: Theme,
    pub chat: ChatWidget,
    pub input: InputWidget,
    pub tree: TreeWidget,
    pub sensors: SensorWidget,
    pub mcp_manager: crate::tui::widgets::mcp::McpWidget,
    pub focus: Focus,
    pub show_sidebars: bool,
    pub show_mcp_manager: bool,
    pub running: bool,
    pub is_thinking: bool,
    // Agent info
    pub name: String,
    pub version: String,
    // Agent state (for status bar)
    pub agent_state: String,
    pub iteration: u32,
    pub max_iterations: u32,
    pub findings_count: usize,
    pub target: String,
    pub mission_targets: Vec<String>,
    pub config: crate::config::AppConfig,
    // Event receiver
    pub rx: mpsc::UnboundedReceiver<TuiEvent>,
}

impl App {
    pub fn new(
        name: String,
        version: String,
        _author: String,
        rx: mpsc::UnboundedReceiver<TuiEvent>,
        config: crate::config::AppConfig,
    ) -> Self {
        let mut chat = ChatWidget::new();
        chat.add_entry("system", &Self::mission_header(&name, &version));

        // Default tree structure
        let mut tree = TreeWidget::new();
        tree.add_root(
            TreeNode::new("NEURAL DOMAINS", "🎯")
                .with_child(TreeNode::new("(waiting for target)", "○")),
        );

        Self {
            theme: Theme::dark(),
            chat,
            input: InputWidget::new(),
            tree,
            sensors: SensorWidget::new(),
            mcp_manager: crate::tui::widgets::mcp::McpWidget::new(&config),
            config,
            mission_targets: Vec::new(),
            focus: Focus::Input,
            show_sidebars: true,
            show_mcp_manager: false,
            running: true,
            is_thinking: false,
            name,
            version,
            agent_state: "STANDBY".to_string(),
            iteration: 0,
            max_iterations: 50,
            findings_count: 0,
            target: "UNDEFINED".to_string(),
            rx,
        }
    }

    /// Generate the mission header banner text.
    fn mission_header(name: &str, version: &str) -> String {
        format!(
            "╔══════════════════════════════════════════════╗\n\
             ║   {name} — Neural Recon HUD v{version}         ║\n\
             ║   Powered by NVIDIA NIM + Swarm Mode        ║\n\
             ║   🔒 Critical Sandbox Isolation Active     ║\n\
             ╚══════════════════════════════════════════════╝\n\
             \n\
             MISSION: Initialize target acquisition to begin reconnaissance.\n\
             CMDS: /help (for full command matrix), /subdomains, /profile, /vitals, /wipe, /clear, /quit",
            name = name.to_uppercase(), version = version
        )
    }

    /// Process incoming events from the agent thread.
    pub fn update(&mut self) {
        while let Ok(event) = self.rx.try_recv() {
            match event {
                TuiEvent::Message { role, content } => {
                    self.chat.add_entry(&role, &content);
                }
                TuiEvent::MessageStart { role } => {
                    self.chat.add_entry(&role, "");
                }
                TuiEvent::MessageChunk { chunk } => {
                    self.chat.append_chunk(&chunk);
                }
                TuiEvent::ProcessingStatus(thinking) => {
                    self.is_thinking = thinking;
                }
                TuiEvent::ToolStarted { tool, .. } => {
                    self.sensors
                        .update_reading(&tool, SensorStatus::Active, 0.5);
                }
                TuiEvent::ToolFinished { tool, success } => {
                    let status = if success {
                        SensorStatus::Complete
                    } else {
                        SensorStatus::Failed
                    };
                    self.sensors.update_reading(&tool, status, 1.0);
                }
                TuiEvent::ToolStream { tool, line } => {
                    // Stream live tool output into the chat widget for real-time visibility
                    self.chat.add_entry("tool", &format!("[{}] {}", tool, line));
                }
                TuiEvent::StateUpdate(state) => {
                    self.agent_state = state;
                }
                TuiEvent::IterationUpdate(iter) => {
                    self.iteration = iter;
                }
                TuiEvent::FindingsUpdate(count) => {
                    self.findings_count = count;
                }
                TuiEvent::TargetUpdate(target) => {
                    self.target = target;
                }
                TuiEvent::ConfigReloaded(config) => {
                    self.config_reloaded(&config);
                }
                TuiEvent::ToolExecution { name, status } => {
                    self.chat.add_entry(
                        "system",
                        &format!("[DIRECT EXECUTION] {} -> {}", name, status),
                    );
                }
                TuiEvent::ClearChat => {
                    self.chat.clear();
                    // Tactical Refresh: Re-add the mission header for a "fresh start" look
                    self.chat
                        .add_entry("system", &Self::mission_header(&self.name, &self.version));
                }
            }
        }
    }

    /// Reload the configuration and refresh widgets.
    pub fn config_reloaded(&mut self, config: &crate::config::AppConfig) {
        self.config = config.clone();
        self.mcp_manager = crate::tui::widgets::mcp::McpWidget::new(config);
    }

    /// Handle a keyboard event.
    pub fn handle_key(&mut self, key: event::KeyEvent) -> Option<String> {
        // Global shortcuts
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if self.is_thinking || self.agent_state != "STANDBY" && self.agent_state != "IDLE" {
                    crate::signals::abort_mission();
                    self.chat.add_entry(
                        "system",
                        "⚠️ MISSION ABORT SIGNAL BROADCAST. Terminating active tasks...",
                    );
                } else {
                    self.running = false;
                }
                return None;
            }
            KeyCode::Tab => {
                self.focus = match self.focus {
                    Focus::Input => Focus::Chat,
                    Focus::Chat => {
                        if self.show_sidebars {
                            Focus::Tree
                        } else {
                            Focus::Input
                        }
                    }
                    Focus::Tree => Focus::Input,
                    Focus::McpManager => Focus::Input,
                };
                return None;
            }
            KeyCode::Char('m') | KeyCode::Char('M') if self.focus != Focus::Input => {
                self.show_mcp_manager = !self.show_mcp_manager;
                if self.show_mcp_manager {
                    self.focus = Focus::McpManager;
                } else {
                    self.focus = Focus::Input;
                }
                return None;
            }
            KeyCode::Esc if self.show_mcp_manager => {
                self.show_mcp_manager = false;
                self.focus = Focus::Input;
                return None;
            }
            _ => {}
        }

        // Route to focused widget
        match self.focus {
            Focus::Input => {
                let ctx = crate::core::commands::CommandContext {
                    config: &self.config,
                    mission_targets: &self.mission_targets,
                };
                match key.code {
                    KeyCode::Enter => {
                        let cmd = self.input.submit();
                        if !cmd.is_empty() {
                            // If it's a scan, add to mission targets
                            if cmd.starts_with("/scan ") {
                                let target =
                                    cmd.split_whitespace().nth(1).unwrap_or("").to_string();
                                if !target.is_empty() && !self.mission_targets.contains(&target) {
                                    self.mission_targets.push(target);
                                }
                            }
                            return Some(cmd);
                        }
                    }
                    KeyCode::Tab => {
                        if self.input.autocomplete(&ctx) {
                            return None;
                        }
                    }
                    KeyCode::Char(c) => self.input.insert_char(c),
                    KeyCode::Backspace => self.input.backspace(),
                    KeyCode::Delete => self.input.delete(),
                    KeyCode::Left => self.input.move_left(),
                    KeyCode::Right => self.input.move_right(),
                    KeyCode::Home => self.input.home(),
                    KeyCode::End => self.input.end(),
                    KeyCode::Up => self.input.history_up(),
                    KeyCode::Down => self.input.history_down(),
                    _ => {}
                }
            }
            Focus::Chat => match key.code {
                KeyCode::Up | KeyCode::Char('k') => self.chat.scroll_up(),
                KeyCode::Down | KeyCode::Char('j') => self.chat.scroll_down(&self.theme),
                _ => {}
            },
            Focus::Tree => match key.code {
                KeyCode::Up | KeyCode::Char('k') => self.tree.previous(),
                KeyCode::Down | KeyCode::Char('j') => self.tree.next(),
                _ => {}
            },
            Focus::McpManager => match key.code {
                KeyCode::Up | KeyCode::Char('k') => self.mcp_manager.previous(),
                KeyCode::Down | KeyCode::Char('j') => self.mcp_manager.next(),
                KeyCode::Char(' ') | KeyCode::Enter => {
                    if let Some((name, state)) = self.mcp_manager.toggle_selected() {
                        let action = if state { "ENABLED" } else { "DISABLED" };
                        self.chat.add_entry(
                            "system",
                            &format!("MCP Server '{}' synchronized: {}.", name, action),
                        );
                        return Some("/sync".to_string());
                    }
                }
                _ => {}
            },
        }

        None
    }

    /// Render the full TUI.
    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let theme = &self.theme;

        // Clear background
        let bg = Block::default().style(Style::default().bg(theme.background));
        frame.render_widget(bg, area);

        // Main layout: header, body, input, status
        let layout = ReconLayout::main_layout(area);

        // ─── Header (Neural Telemetry) ───
        let header = StatusWidget::new(crate::tui::widgets::status::StatusProps {
            agent_name: self.name.clone(),
            version: self.version.clone(),
            state: self.agent_state.clone(),
            iteration: self.iteration,
            max_iterations: self.max_iterations,
            findings_count: self.findings_count,
            target: self.target.clone(),
            is_thinking: self.is_thinking,
        });
        header.render(layout[0], frame.buffer_mut(), theme);

        // ─── Body (Tree + Chat + Sensors) ───
        let body_layout = ReconLayout::body_layout(layout[1], self.show_sidebars);

        if self.show_sidebars && body_layout.len() > 2 {
            // Neural Tree
            self.tree.render(body_layout[0], frame.buffer_mut(), theme);

            // Tactical Chat
            self.chat.render(body_layout[1], frame.buffer_mut(), theme);

            // Sensor Feed
            self.sensors
                .render(body_layout[2], frame.buffer_mut(), theme);
        } else {
            // Full-width tactical view
            self.chat.render(body_layout[0], frame.buffer_mut(), theme);
        }

        // ─── Input ───
        let ctx = crate::core::commands::CommandContext {
            config: &self.config,
            mission_targets: &self.mission_targets,
        };
        self.input
            .render(layout[2], frame.buffer_mut(), theme, &self.name, &ctx);

        // ─── Status Bar (Telemetry Feed) ───
        let mut status_lines = vec![
            Span::styled(" MISSION STATUS: ", theme.dim_style()),
            Span::styled(format!("{} ", self.agent_state), theme.success_style()),
            Span::styled(" │ ", theme.dim_style()),
            Span::styled(" SANDBOX: ", theme.dim_style()),
            Span::styled("SECURE ", theme.success_style()),
            Span::styled(" │ ", theme.dim_style()),
            Span::styled(" MCP: ", theme.dim_style()),
            Span::styled("SYNCED ", theme.accent_style()),
            Span::styled("〰 ", theme.accent_style().bold()), // The Heartbeat symbol
        ];

        if !self.show_mcp_manager {
            status_lines.push(Span::styled(" │ ", theme.dim_style()));
            status_lines.push(Span::styled("Press 'M' for MCP", theme.accent_style()));
        }

        let status = Paragraph::new(Line::from(status_lines)).style(theme.status_bar_style());
        frame.render_widget(status, layout[3]);

        // ─── MCP Manager Overlay ───
        if self.show_mcp_manager {
            let overlay_area = centered_rect(60, 40, area);
            self.mcp_manager
                .render(overlay_area, frame.buffer_mut(), theme);
        }

        // ─── Cursor ───
        if self.focus == Focus::Input {
            let prompt_len = (self.name.len() + 2) as u16; // name + "> "
            let cursor_x = layout[2].x + 1 + prompt_len + self.input.cursor_position() as u16; // 1 border + prompt
            let cursor_y = layout[2].y + 1; // 1 border
            frame.set_cursor_position((cursor_x, cursor_y));
        }
    }
}

/// Helper function to create a centered rect using up certain percentage of the available rect `r`.
/// Clamped to avoid u16 underflow on very small terminals.
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let px = percent_x.min(100);
    let py = percent_y.min(100);

    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - py) / 2),
            Constraint::Percentage(py),
            Constraint::Percentage((100 - py) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - px) / 2),
            Constraint::Percentage(px),
            Constraint::Percentage((100 - px) / 2),
        ])
        .split(popup_layout[1])[1]
}
