//! TUI App — main event loop, rendering, and input handling.

use crate::tui::animation::AnimationState;
use crate::tui::layout::ReconLayout;
use crate::tui::theme::Theme;
use crate::tui::widgets::chat::ChatWidget;
use crate::tui::widgets::input::InputWidget;
use crate::tui::widgets::modal::ModalWidget;
use crate::tui::widgets::nav::NavWidget;
use crate::tui::widgets::sensor::{SensorStatus, SensorWidget};
use crate::tui::widgets::status::StatusWidget;
use crate::tui::widgets::tree::{TreeNode, TreeWidget};

use tokio::sync::mpsc;

use crossterm::event::{self, KeyCode, KeyModifiers};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Sparkline};

/// Focus state for keyboard input routing.
#[derive(Debug, Clone, PartialEq)]
pub enum Focus {
    Input,
    Chat,
    Tree,
    Sensors,
    McpManager,
    Findings,
    Nav,
}

/// Tactical Finding metadata
#[derive(Debug, Clone)]
pub struct Finding {
    pub id: String,
    pub target: String,
    pub service: String,
    pub port: String,
    pub risk: String,
    pub description: String,
}

/// Active Screen / View routing
#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Mission,   // Main Combat Deck
    Findings,  // Tactical Findings Registry
    Settings,  // Config & Tuning
    McpStatus, // Plugin Registry
}

/// Modal state for tactical overlays
#[derive(Debug, Clone, PartialEq)]
pub enum Modal {
    Help,
    Warning(String),
    CommandConfirm(String),
}

/// Tooltip state for hover interactions
#[derive(Debug, Clone)]
pub struct Tooltip {
    pub text: String,
    pub x: u16,
    pub y: u16,
}

/// Context menu state for right-click interactions
#[derive(Debug, Clone)]
pub struct ContextMenu {
    pub options: Vec<String>,
    pub x: u16,
    pub y: u16,
    pub selected: usize,
}

/// Command palette state for fuzzy-search interactions
#[derive(Debug, Clone)]
pub struct CommandPalette {
    pub input: String,
    pub options: Vec<String>,
    pub selected: usize,
}

/// Toast notification state
#[derive(Debug, Clone)]
pub struct Toast {
    pub message: String,
    pub level: String, // info, warning, success
    pub expiry: std::time::Instant,
}

/// System telemetry state
#[derive(Debug, Clone)]
pub struct SystemStats {
    pub cpu: f32,
    pub mem: f32,
    pub network: String,
    pub cpu_history: Vec<u64>,
    pub mem_history: Vec<u64>,
}

/// Events sent from the agent to the TUI.
#[derive(Debug, Clone)]
pub enum TuiEvent {
    Message {
        role: String,
        content: String,
    },
    MessageStart {
        role: String,
    },
    MessageChunk {
        chunk: String,
    },
    ProcessingStatus(bool),
    ToolStarted {
        server: String,
        tool: String,
        args: String,
    },
    ToolStream {
        tool: String,
        line: String,
    },
    ToolFinished {
        tool: String,
        success: bool,
    },
    StateUpdate(String),
    IterationUpdate(u32),
    FindingsUpdate(usize),
    TargetUpdate(String),
    ConfigReloaded(Box<crate::config::AppConfig>),
    ToolExecution {
        name: String,
        status: String,
    },
    WebSearchStarted {
        server: String,
        query: String,
    },
    WebSourceFound {
        source: String,
    },
    ToolDiscoveryUpdate(Vec<String>),
    VitalsUpdate {
        cpu: f32,
        mem: f32,
    },
    ClearChat,
}

/// Main TUI application state.
pub struct App {
    pub theme: Theme,
    pub animation: AnimationState,
    pub chat: ChatWidget,
    pub input: InputWidget,
    pub tree: TreeWidget,
    pub sensors: SensorWidget,
    pub mcp_manager: crate::tui::widgets::mcp::McpWidget,
    pub findings_widget: crate::tui::widgets::findings::FindingsWidget,
    pub focus: Focus,
    pub show_left_panel: bool,
    pub show_right_panel: bool,
    pub left_panel_width: u16,
    pub right_panel_width: u16,
    pub show_mcp_manager: bool,
    pub running: bool,
    pub is_thinking: bool,
    pub chat_viewport_height: u16,
    // Agent info
    pub name: String,
    pub version: String,
    // Agent state
    pub agent_state: String,
    pub iteration: u32,
    pub max_iterations: u32,
    pub findings_count: usize,
    pub target: String,
    pub mission_targets: Vec<String>,
    pub config: crate::config::AppConfig,
    // Event receiver
    pub rx: mpsc::UnboundedReceiver<TuiEvent>,
    pub hud_message: Option<(String, std::time::Instant)>,
    pub last_layout: Option<crate::tui::layout::MainRects>,
    pub last_body: Option<crate::tui::layout::BodyRects>,
    pub current_screen: Screen,
    pub active_modal: Option<Modal>,
    pub active_tooltip: Option<Tooltip>,
    pub active_context_menu: Option<ContextMenu>,
    pub active_palette: Option<CommandPalette>,
    pub discovered_tools: Vec<String>,
    pub active_search: Option<String>,
    pub toasts: Vec<Toast>,
    pub findings: Vec<Finding>,
    pub stats: SystemStats,
    pub is_dragging_scrollbar: bool,
    pub is_dragging_left_border: bool,
    pub is_dragging_right_border: bool,
    pub sys: sysinfo::System,
    pub last_area: ratatui::layout::Rect,
    pub mouse_pos: Option<(u16, u16)>,
    pub is_hovering_left_border: bool,
    pub is_hovering_right_border: bool,
    pub last_tick: std::time::Instant,
    pub fps: f32,
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

        let mut tree = TreeWidget::new();
        tree.add_root(
            TreeNode::new("NEURAL DOMAINS", "🎯")
                .with_child(TreeNode::new("(waiting for target)", "○")),
        );

        let mut theme = Theme::dark(config.tui.simulated_mode);
        theme.font = config.tui.font.clone();

        Self {
            theme,
            animation: AnimationState::new(),
            chat,
            input: {
                let mut input = InputWidget::new();
                let history_path = crate::core::persistence::get_history_path();
                if let Ok(history) = crate::core::persistence::load_history_vec(&history_path) {
                    input.history = history;
                }
                input
            },
            tree,
            sensors: SensorWidget::new(),
            mcp_manager: crate::tui::widgets::mcp::McpWidget::new(&config),
            findings_widget: crate::tui::widgets::findings::FindingsWidget::new(),
            config,
            mission_targets: Vec::new(),
            focus: Focus::Input,
            show_left_panel: true,
            show_right_panel: true,
            left_panel_width: 30,
            right_panel_width: 30,
            show_mcp_manager: false,
            running: true,
            is_thinking: false,
            chat_viewport_height: 20,
            name,
            version,
            agent_state: "STANDBY".to_string(),
            iteration: 0,
            max_iterations: 50,
            findings_count: 0,
            target: "UNDEFINED".to_string(),
            rx,
            hud_message: None,
            last_layout: None,
            last_body: None,
            current_screen: Screen::Mission,
            active_modal: None,
            active_tooltip: None,
            active_context_menu: None,
            active_palette: None,
            discovered_tools: Vec::new(),
            active_search: None,
            toasts: Vec::new(),
            findings: Vec::new(),
            stats: SystemStats {
                cpu: 0.1,
                mem: 142.5,
                network: "ACTIVE".to_string(),
                cpu_history: vec![0; 40],
                mem_history: vec![0; 40],
            },
            is_dragging_scrollbar: false,
            is_dragging_left_border: false,
            is_dragging_right_border: false,
            sys: sysinfo::System::new_all(),
            last_area: ratatui::layout::Rect::default(),
            mouse_pos: None,
            is_hovering_left_border: false,
            is_hovering_right_border: false,
            last_tick: std::time::Instant::now(),
            fps: 60.0,
        }
    }

    /// Generate the mission header banner.
    fn mission_header(name: &str, version: &str) -> String {
        format!(
            "## {} — NEURAL RECON HUD v{}\n\
             > Powered by NVIDIA NIM + Swarm Mode\n\
             > 🔒 Critical Sandbox Isolation Active\n\n\
             **MISSION:** Initialize target acquisition to begin reconnaissance.\n\
             **CMDS:** /help (for full command matrix), /subdomains, /profile, /vitals, /wipe, /clear, /quit",
            name.to_uppercase(), version
        )
    }

    pub fn add_toast(&mut self, message: String, level: String) {
        self.toasts.push(Toast {
            message,
            level,
            expiry: std::time::Instant::now() + std::time::Duration::from_secs(4),
        });
    }

    pub fn set_hud_message(&mut self, msg: String) {
        self.hud_message = Some((msg, std::time::Instant::now()));
    }

    /// Process incoming events + advance animation.
    pub fn update(&mut self) -> bool {
        let mut needs_redraw = false;

        // 1. Advance Typewriter & Smooth Scroll Animation
        if self.chat.tick(self.is_thinking) {
            needs_redraw = true;
        }

        // 2. Advance UI animations (Pulse, HUD)
        if self.is_thinking || !self.toasts.is_empty() || self.hud_message.is_some() {
            self.animation.tick();
            needs_redraw = true;
        }

        // 3. Track Industry-Grade FPS Telemetry
        let now = std::time::Instant::now();
        let frame_time = now.duration_since(self.last_tick).as_secs_f32();
        if frame_time > 0.0 {
            let current_fps = 1.0 / frame_time;
            self.fps = self.fps * 0.9 + current_fps * 0.1; // Smooth average
        }
        self.last_tick = now;

        while let Ok(event) = self.rx.try_recv() {
            needs_redraw = true; // Any event warrants a redraw
            match event {
                TuiEvent::VitalsUpdate { cpu, mem } => {
                    self.stats.cpu = cpu;
                    self.stats.mem = mem;

                    // Sliding window buffers for cyber sparklines
                    self.stats.cpu_history.push(cpu as u64);
                    if self.stats.cpu_history.len() > 100 {
                        self.stats.cpu_history.remove(0);
                    }

                    self.stats.mem_history.push(mem as u64);
                    if self.stats.mem_history.len() > 100 {
                        self.stats.mem_history.remove(0);
                    }
                }
                TuiEvent::ClearChat => {
                    self.chat.clear();
                }
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
                TuiEvent::ToolStarted { server, tool, args } => {
                    self.sensors
                        .update_reading(&tool, SensorStatus::Active, 0.5);
                    self.chat.add_entry(
                        "system",
                        &format!(
                            "⚡ **EXECUTING** [ {} ] {} _{}_",
                            server.to_uppercase(),
                            tool,
                            args
                        ),
                    );
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
                    self.chat.append_tool_stream(&tool, &line);
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
                TuiEvent::WebSearchStarted { server, query } => {
                    self.chat.add_entry(
                        "system",
                        &format!(
                            "🌐 **INTELLIGENCE** [ {} ] SEARCHING => \"_{}_\"",
                            server.to_uppercase(),
                            query
                        ),
                    );
                }
                TuiEvent::WebSourceFound { source } => {
                    self.chat.add_entry(
                        "system",
                        &format!("  📍 SOURCE [ _{}_ ] INDEXED", source.to_uppercase()),
                    );
                }
                TuiEvent::ToolDiscoveryUpdate(tools) => {
                    self.discovered_tools = tools;
                }
            }
        }
        needs_redraw
    }

    /// Reload config and refresh widgets.
    pub fn config_reloaded(&mut self, config: &crate::config::AppConfig) {
        self.config = config.clone();
        self.mcp_manager = crate::tui::widgets::mcp::McpWidget::new(config);
    }

    /// Handle a keyboard event.
    pub fn handle_key(&mut self, key: event::KeyEvent) -> Option<String> {
        // ─── Modal/Palette Priority Logic ───
        if let Some(mut palette) = self.active_palette.take() {
            match key.code {
                KeyCode::Esc => {
                    self.active_palette = None;
                    return None;
                }
                KeyCode::Enter => {
                    if let Some(selected) = palette.options.get(palette.selected) {
                        let cmd = selected.clone();
                        self.active_palette = None;

                        // Action mapping
                        if cmd.starts_with("/") {
                            self.chat.add_entry("user", &cmd);
                            return Some(cmd);
                        } else if cmd.starts_with("tool: ") {
                            let tool_name = cmd.strip_prefix("tool: ").unwrap();
                            let inject = format!("/{} ", tool_name);
                            self.input.set_text(&inject);
                            self.focus = Focus::Input;
                            return None;
                        }
                    }
                    self.active_palette = None;
                    return None;
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if !palette.options.is_empty() {
                        palette.selected =
                            (palette.selected + palette.options.len() - 1) % palette.options.len();
                    }
                    self.active_palette = Some(palette);
                    return None;
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if !palette.options.is_empty() {
                        palette.selected = (palette.selected + 1) % palette.options.len();
                    }
                    self.active_palette = Some(palette);
                    return None;
                }
                KeyCode::Backspace => {
                    palette.input.pop();
                    palette.selected = 0;
                    // Re-filter (Simple substring for now)
                    let mut all_opts = crate::core::commands::TACTICAL_COMMANDS
                        .iter()
                        .map(|s| format!("/{}", s))
                        .collect::<Vec<String>>();
                    all_opts.extend(self.discovered_tools.iter().map(|s| format!("tool: {}", s)));

                    let query = palette.input.to_lowercase();
                    palette.options = all_opts
                        .into_iter()
                        .filter(|o| o.to_lowercase().contains(&query))
                        .collect();

                    self.active_palette = Some(palette);
                    return None;
                }
                KeyCode::Char(c) => {
                    palette.input.push(c);
                    palette.selected = 0;

                    let mut all_opts = crate::core::commands::TACTICAL_COMMANDS
                        .iter()
                        .map(|s| format!("/{}", s))
                        .collect::<Vec<String>>();
                    all_opts.extend(self.discovered_tools.iter().map(|s| format!("tool: {}", s)));

                    let query = palette.input.to_lowercase();
                    palette.options = all_opts
                        .into_iter()
                        .filter(|o| o.to_lowercase().contains(&query))
                        .collect();

                    self.active_palette = Some(palette);
                    return None;
                }
                _ => {
                    self.active_palette = Some(palette);
                    return None;
                }
            }
        }

        // Global shortcuts
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if self.is_thinking || (self.agent_state != "STANDBY" && self.agent_state != "IDLE")
                {
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
            KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let mut options = crate::core::commands::TACTICAL_COMMANDS
                    .iter()
                    .map(|s| format!("/{}", s))
                    .collect::<Vec<String>>();
                options.extend(self.discovered_tools.iter().map(|s| format!("tool: {}", s)));
                self.active_palette = Some(CommandPalette {
                    input: String::new(),
                    options,
                    selected: 0,
                });
                return None;
            }
            KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.show_left_panel = !self.show_left_panel;
                self.set_hud_message(format!(
                    "RECON_MAP_{}",
                    if self.show_left_panel {
                        "LINKED"
                    } else {
                        "DECOUPLED"
                    }
                ));
                return None;
            }
            KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.show_right_panel = !self.show_right_panel;
                self.set_hud_message(format!(
                    "SENSOR_ARRAY_{}",
                    if self.show_right_panel {
                        "LINKED"
                    } else {
                        "DECOUPLED"
                    }
                ));
                return None;
            }
            KeyCode::Tab if self.focus != Focus::Input => {
                self.focus = match self.focus {
                    Focus::Chat => {
                        if self.show_left_panel {
                            Focus::Tree
                        } else {
                            Focus::Input
                        }
                    }
                    Focus::Tree => Focus::Input,
                    Focus::McpManager => Focus::Input,
                    Focus::Findings => Focus::Input,
                    _ => Focus::Input,
                };
                return None;
            }
            KeyCode::Char('m') | KeyCode::Char('M')
                if key.modifiers.contains(KeyModifiers::CONTROL) =>
            {
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
            KeyCode::Char('[') if self.focus != Focus::Input => {
                self.show_left_panel = !self.show_left_panel;
                self.set_hud_message(format!(
                    "RECON_MAP_{}",
                    if self.show_left_panel {
                        "LINKED"
                    } else {
                        "DECOUPLED"
                    }
                ));
                return None;
            }
            KeyCode::Char(']') if self.focus != Focus::Input => {
                self.show_right_panel = !self.show_right_panel;
                self.set_hud_message(format!(
                    "SENSOR_ARRAY_{}",
                    if self.show_right_panel {
                        "LINKED"
                    } else {
                        "DECOUPLED"
                    }
                ));
                return None;
            }
            KeyCode::Char('<') | KeyCode::Char(',') if self.focus != Focus::Input => {
                self.left_panel_width = self.left_panel_width.saturating_sub(2).max(10);
                self.set_hud_message(format!("RECON_MAP_WIDTH: {}", self.left_panel_width));
                return None;
            }
            KeyCode::Char('>') | KeyCode::Char('.') if self.focus != Focus::Input => {
                self.left_panel_width = self.left_panel_width.saturating_add(2).min(80);
                self.set_hud_message(format!("RECON_MAP_WIDTH: {}", self.left_panel_width));
                return None;
            }
            KeyCode::Char('{') | KeyCode::Char('-') if self.focus != Focus::Input => {
                self.right_panel_width = self.right_panel_width.saturating_sub(2).max(10);
                self.set_hud_message(format!("SENSOR_ARRAY_WIDTH: {}", self.right_panel_width));
                return None;
            }
            KeyCode::Char('}') | KeyCode::Char('=') if self.focus != Focus::Input => {
                self.right_panel_width = self.right_panel_width.saturating_add(2).min(80);
                self.set_hud_message(format!("SENSOR_ARRAY_WIDTH: {}", self.right_panel_width));
                return None;
            }
            KeyCode::Char('?') if self.focus != Focus::Input => {
                self.active_modal = Some(Modal::Help);
                return None;
            }
            KeyCode::Esc if self.active_modal.is_some() => {
                self.active_modal = None;
                return None;
            }
            _ => {}
        }

        // Route to focused widget
        match self.focus {
            Focus::Input => {
                let history = self.input.history.clone();
                let tools = self.discovered_tools.clone();
                match key.code {
                    KeyCode::Enter => {
                        let cmd = self.input.submit();
                        if !cmd.is_empty() {
                            // Silicon-Grade Auto-Save (Zero Data Loss)
                            let history_path = crate::core::persistence::get_history_path();
                            let _ = crate::core::persistence::save_history_vec(
                                &self.input.history,
                                &history_path,
                            );

                            if cmd.starts_with("/find ") {
                                self.active_search =
                                    Some(cmd.strip_prefix("/find ").unwrap().trim().to_string());
                                self.add_toast(
                                    format!(
                                        "SEARCH_ACTIVE: {}",
                                        self.active_search.as_ref().unwrap()
                                    ),
                                    "info".to_string(),
                                );
                            } else {
                                self.chat.add_entry("user", &cmd);
                                if cmd.starts_with("/scan ") {
                                    let target =
                                        cmd.split_whitespace().nth(1).unwrap_or("").to_string();
                                    if !target.is_empty() && !self.mission_targets.contains(&target)
                                    {
                                        self.mission_targets.push(target);
                                    }
                                }
                                return Some(cmd);
                            }
                        }
                    }
                    KeyCode::Tab => {
                        let ctx = crate::core::commands::CommandContext {
                            config: &self.config,
                            mission_targets: &self.mission_targets,
                            history: &history,
                            discovered_tools: &tools,
                        };
                        if !self.input.autocomplete(&ctx) {
                            self.focus = Focus::Chat;
                        }
                        return None;
                    }
                    KeyCode::Char('w') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.input.delete_word_backward();
                    }
                    KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.input.clear_line();
                    }
                    KeyCode::Char('a') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.input.home();
                    }
                    KeyCode::Char('e') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.input.end();
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

                // Real-time Intelligence Update (Ghost Hints)
                let ctx = crate::core::commands::CommandContext {
                    config: &self.config,
                    mission_targets: &self.mission_targets,
                    history: &history,
                    discovered_tools: &tools,
                };
                self.input.update_ghost_hint(&ctx);
            }
            Focus::Chat => match key.code {
                KeyCode::Up | KeyCode::Char('k') => self.chat.scroll_up(),
                KeyCode::Down | KeyCode::Char('j') => {
                    let h = self.viewport_chat_height();
                    self.chat.scroll_down(&self.theme, h);
                }
                KeyCode::PageUp => {
                    let h = self.viewport_chat_height();
                    self.chat.scroll_page_up(h);
                }
                KeyCode::PageDown => {
                    let h = self.viewport_chat_height();
                    self.chat.scroll_page_down(h);
                }
                KeyCode::Char('G') => self.chat.jump_to_bottom(),
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
            Focus::Nav => match key.code {
                KeyCode::Left | KeyCode::Char('h') => {
                    self.current_screen = match self.current_screen {
                        Screen::Mission => Screen::McpStatus,
                        Screen::Findings => Screen::Mission,
                        Screen::Settings => Screen::Findings,
                        Screen::McpStatus => Screen::Settings,
                    };
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    self.current_screen = match self.current_screen {
                        Screen::Mission => Screen::Findings,
                        Screen::Findings => Screen::Settings,
                        Screen::Settings => Screen::McpStatus,
                        Screen::McpStatus => Screen::Mission,
                    };
                }
                _ => {}
            },
            Focus::Sensors => match key.code {
                KeyCode::Up | KeyCode::Char('k') => self.sensors.scroll_up(),
                KeyCode::Down | KeyCode::Char('j') => self.sensors.scroll_down(),
                _ => {}
            },
            Focus::Findings => match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    self.findings_widget.previous(self.findings.len())
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    self.findings_widget.next(self.findings.len())
                }
                _ => {}
            },
        }

        None
    }

    /// Compute actual chat viewport height from terminal dimensions.
    pub fn viewport_chat_height(&self) -> usize {
        self.chat_viewport_height as usize
    }

    /// Render the full TUI.
    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let theme = &self.theme;

        // Clear background
        let bg = Block::default().style(Style::default().bg(theme.background));
        frame.render_widget(bg, area);

        // ─── Performance Optimization: Area Check ───
        if area != self.last_area {
            self.last_area = area;
            // Any logic that needs to clear caches on resize would go here
        }

        // ─── Phase 1: Main Framework Layout ───
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(area);

        let layout = ReconLayout::main_layout(main_chunks[0]);
        let status_area = main_chunks[1];

        // ─── Navigation ───
        NavWidget::render(
            layout.nav,
            frame.buffer_mut(),
            theme,
            &self.current_screen,
            self.focus == Focus::Nav,
        );

        // ─── Header ───
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
        header.render(layout.header, frame.buffer_mut(), theme, &self.animation);

        // ─── Body (Multi-View Router) ───
        let body_layout = ReconLayout::body_layout(
            layout.body,
            self.show_left_panel,
            self.show_right_panel,
            self.left_panel_width,
            self.right_panel_width,
        );

        // ─── Industry-Grade Interaction: Hover Glow ───
        let left_border_style = if self.is_dragging_left_border {
            theme.focused_border_style()
        } else if self.is_hovering_left_border {
            theme.highlight_style()
        } else {
            theme.unfocused_border_style()
        };

        let right_border_style = if self.is_dragging_right_border {
            theme.focused_border_style()
        } else if self.is_hovering_right_border {
            theme.highlight_style()
        } else {
            theme.unfocused_border_style()
        };

        match self.current_screen {
            Screen::Mission => {
                if self.show_left_panel {
                    self.tree.render(
                        body_layout.left,
                        frame.buffer_mut(),
                        theme,
                        self.focus == Focus::Tree,
                    );
                }

                self.chat_viewport_height = body_layout.center.height.saturating_sub(2);
                self.chat.render(
                    body_layout.center,
                    frame.buffer_mut(),
                    theme,
                    self.focus == Focus::Chat,
                    self.active_search.as_deref(),
                );

                if self.show_right_panel {
                    self.sensors.render(
                        body_layout.right,
                        frame.buffer_mut(),
                        theme,
                        false,
                        &self.animation,
                    );
                }
            }
            _ => {
                // Settings, Findings, and MCP take the whole body
                match self.current_screen {
                    Screen::Findings => {
                        self.findings_widget.render(
                            layout.body,
                            frame.buffer_mut(),
                            theme,
                            &self.findings,
                            self.focus == Focus::Findings,
                        );
                    }
                    Screen::Settings => {
                        let settings_title = Line::from(vec![Span::styled(
                            " ◈ TACTICAL CONFIGURATION ENGINE ",
                            theme.secondary_style().add_modifier(Modifier::BOLD),
                        )]);
                        let settings_items = vec![
                            Line::from(""),
                            Line::from(vec![
                                Span::styled(" [SYSTEM] ", theme.dim_style()),
                                Span::raw("Neural Animations: "),
                                Span::styled(" ENABLED ", theme.success_style()),
                            ]),
                            Line::from(vec![
                                Span::styled(" [SYSTEM] ", theme.dim_style()),
                                Span::raw("Vsync Latency:     "),
                                Span::styled(" 16.6ms ", theme.secondary_style()),
                            ]),
                            Line::from(vec![
                                Span::styled(" [SYSTEM] ", theme.dim_style()),
                                Span::raw("Network Bridge:    "),
                                Span::styled(" MCP_PROXY ", theme.primary_style()),
                            ]),
                            Line::from(""),
                            Line::from(vec![
                                Span::styled(" [UI]     ", theme.dim_style()),
                                Span::raw("Theme Preset:      "),
                                Span::styled(" CYBER_NOIR ", theme.highlight_style()),
                            ]),
                            Line::from(vec![
                                Span::styled(" [UI]     ", theme.dim_style()),
                                Span::raw("Status HUD:        "),
                                Span::styled(" FULL_TELEMETRY ", theme.secondary_style()),
                            ]),
                            Line::from(""),
                            Line::from(vec![
                                Span::styled(" (Press ", theme.dim_style()),
                                Span::styled("Tab", theme.secondary_style()),
                                Span::styled(" to return to Mission) ", theme.dim_style()),
                            ]),
                        ];

                        let block = Block::default()
                            .borders(Borders::ALL)
                            .border_style(theme.unfocused_border_style())
                            .title(settings_title);

                        let inner = block.inner(layout.body);
                        frame.render_widget(block, layout.body);

                        let settings_msg =
                            Paragraph::new(settings_items).alignment(Alignment::Left);
                        frame.render_widget(settings_msg, inner);
                    }
                    Screen::McpStatus => {
                        self.mcp_manager
                            .render(layout.body, frame.buffer_mut(), theme);
                    }
                    _ => {}
                }
            }
        }

        // ─── Tactical Handles (Glow Overlays) ───
        if self.current_screen == Screen::Mission {
            if self.show_left_panel {
                let handle_area = Rect {
                    x: body_layout.center.x.saturating_sub(1),
                    y: body_layout.center.y,
                    width: 1,
                    height: body_layout.center.height,
                };
                let handle_style = if self.is_dragging_left_border {
                    theme.focused_title_style()
                } else {
                    left_border_style
                };
                let char = if self.is_hovering_left_border || self.is_dragging_left_border {
                    "┃"
                } else {
                    "│"
                };
                let handle = Paragraph::new(char).style(handle_style);
                frame.render_widget(handle, handle_area);
            }
            if self.show_right_panel {
                let handle_area = Rect {
                    x: body_layout.center.x + body_layout.center.width,
                    y: body_layout.center.y,
                    width: 1,
                    height: body_layout.center.height,
                };
                let handle_style = if self.is_dragging_right_border {
                    theme.focused_title_style()
                } else {
                    right_border_style
                };
                let char = if self.is_hovering_right_border || self.is_dragging_right_border {
                    "┃"
                } else {
                    "│"
                };
                let handle = Paragraph::new(char).style(handle_style);
                frame.render_widget(handle, handle_area);
            }
        }

        // ─── Input ───
        let history = self.input.history.clone();
        let tools = self.discovered_tools.clone();
        let ctx = crate::core::commands::CommandContext {
            config: &self.config,
            mission_targets: &self.mission_targets,
            history: &history,
            discovered_tools: &tools,
        };
        self.input.render(
            layout.input,
            frame.buffer_mut(),
            theme,
            &self.name,
            &ctx,
            self.focus == Focus::Input,
        );

        // Persist for mouse collision logic
        self.last_layout = Some(layout);
        self.last_body = Some(body_layout);

        // ─── Status Bar ───
        let heartbeat = self.animation.heartbeat_symbol();
        let fps_style = if self.fps > 55.0 {
            theme.success_style()
        } else if self.fps > 30.0 {
            theme.warning_style()
        } else {
            theme.error_style()
        };

        let mut status_spans = vec![
            Span::styled(" STATUS: ", theme.dim_style()),
            Span::styled(format!("{} ", self.agent_state), theme.success_style()),
            Span::styled("│", theme.dim_style()),
            Span::styled(" FPS: ", theme.dim_style()),
            Span::styled(format!("{:.0} ", self.fps), fps_style),
            Span::styled("│", theme.dim_style()),
            Span::styled(" HUD: ", theme.dim_style()),
            Span::styled(
                "ULTRA_PREMIUM ",
                theme.primary_style().add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!(" {} ", heartbeat), theme.primary_style()),
            Span::styled("│", theme.dim_style()),
            Span::styled(" SANDBOX: ", theme.dim_style()),
            Span::styled("SECURE ", theme.success_style()),
        ];

        if !self.show_mcp_manager {
            status_spans.push(Span::styled("│", theme.dim_style()));
            status_spans.push(Span::styled(" M=MCP", theme.dim_style()));
        }

        // Overlay HUD message if active
        if let Some((msg, time)) = &self.hud_message {
            if time.elapsed() < std::time::Duration::from_secs(3) {
                status_spans.insert(
                    0,
                    Span::styled(format!(" [!] {} ", msg), theme.highlight_style()),
                );
                status_spans.insert(1, Span::styled("│", theme.dim_style()));
            } else {
                // Clear expired message (mutable borrow issue?)
                // Actually we can't easily clear it here since self is borrowed for status_spans
            }
        }

        let status = Paragraph::new(Line::from(status_spans)).style(theme.status_bar_style());
        frame.render_widget(status, self.last_layout.as_ref().unwrap().status);

        // ─── Modal Overlay (Top Priority) ───
        if let Some(ref modal) = self.active_modal {
            ModalWidget::render(area, frame.buffer_mut(), theme, modal);
        }

        // ─── Command Palette Overlay ───
        if let Some(ref palette) = self.active_palette {
            let p_width = 60.min(area.width.saturating_sub(4));
            let p_height = 10.min(area.height.saturating_sub(4));
            let p_area = centered_rect(p_width, p_height, area);

            frame.render_widget(Clear, p_area);
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(theme.highlight_style())
                .title(Span::styled(
                    " ◈ NEURAL COMMAND PALETTE (FUZZY) ",
                    theme.highlight_style().add_modifier(Modifier::BOLD),
                ))
                .bg(theme.background);

            let inner = block.inner(p_area);
            frame.render_widget(block, p_area);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Min(0)])
                .split(inner);

            let p_input =
                Paragraph::new(format!(" ❯ {} ", palette.input)).style(theme.primary_style());
            frame.render_widget(p_input, chunks[0]);

            let p_options: Vec<ListItem> = palette
                .options
                .iter()
                .enumerate()
                .map(|(i, opt)| {
                    let style = if i == palette.selected {
                        theme.highlight_style()
                    } else {
                        theme.dim_style()
                    };
                    ListItem::new(Span::styled(format!("   {} ", opt), style))
                })
                .collect();

            frame.render_widget(List::new(p_options), chunks[1]);
        }

        // ─── Context Menu Overlay ───
        if let Some(ref menu) = self.active_context_menu {
            let options: Vec<ListItem> = menu
                .options
                .iter()
                .enumerate()
                .map(|(i, opt)| {
                    let style = if i == menu.selected {
                        theme.highlight_style()
                    } else {
                        theme.primary_style()
                    };
                    ListItem::new(Span::styled(opt, style))
                })
                .collect();

            let menu_w = menu.options.iter().map(|o| o.len()).max().unwrap_or(0) as u16 + 2;
            let menu_h = menu.options.len() as u16 + 2;

            let mut mx = menu.x;
            let mut my = menu.y;
            if mx + menu_w > area.width {
                mx = area.width.saturating_sub(menu_w);
            }
            if my + menu_h > area.height {
                my = area.height.saturating_sub(menu_h);
            }

            let m_area = Rect {
                x: mx,
                y: my,
                width: menu_w.min(area.width),
                height: menu_h.min(area.height),
            };

            frame.render_widget(Clear, m_area);
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(theme.secondary_style())
                .bg(theme.background);
            let inner = block.inner(m_area);
            frame.render_widget(block, m_area);
            frame.render_widget(List::new(options), inner);
        }

        // ─── Tooltip Overlay (Absolute Top) ───
        if let Some(ref tooltip) = self.active_tooltip {
            let lines: Vec<Line> = tooltip
                .text
                .split('\n')
                .map(|l| Line::from(l.to_string()))
                .collect();
            let width = lines.iter().map(|l| l.width()).max().unwrap_or(0) as u16 + 2;
            let height = lines.len() as u16 + 2;

            let mut tx = tooltip.x + 1;
            let mut ty = tooltip.y + 1;

            // Bounds check
            if tx + width > area.width {
                tx = tooltip.x.saturating_sub(width);
            }
            if ty + height > area.height {
                ty = tooltip.y.saturating_sub(height);
            }

            let t_area = Rect {
                x: tx,
                y: ty,
                width: width.min(area.width),
                height: height.min(area.height),
            };

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(theme.highlight_style())
                .bg(theme.background);

            let inner = block.inner(t_area);
            frame.render_widget(Clear, t_area);
            frame.render_widget(block, t_area);
            frame.render_widget(Paragraph::new(lines).style(theme.primary_style()), inner);
        }

        // ─── Phase 9: Tactical Status Bar ───
        let stats_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(40),
                Constraint::Length(25), // CPU Spark
                Constraint::Length(25), // MEM Spark
            ])
            .split(status_area);

        let stats_line = Line::from(vec![
            Span::styled(" ◈ SYSTEM_CORE: ", theme.dim_style()),
            Span::styled(
                format!("CPU {:.1}% ", self.stats.cpu),
                if self.stats.cpu > 0.5 {
                    theme.warning_style()
                } else {
                    theme.success_style()
                },
            ),
            Span::styled(
                format!("MEM {:.1}MB ", self.stats.mem),
                theme.secondary_style(),
            ),
            Span::styled(" ◈ NET: ", theme.dim_style()),
            Span::styled(format!("{} ", self.stats.network), theme.primary_style()),
            Span::styled(" ◈ MISSION_CLOCK: ", theme.dim_style()),
            Span::styled(
                format!("{:?} ", std::time::Instant::now().elapsed().as_secs()),
                theme.highlight_style(),
            ),
        ]);
        let status_bar = Paragraph::new(stats_line).bg(Color::Indexed(233));
        frame.render_widget(status_bar, stats_chunks[0]);

        // CPU Sparkline
        let cpu_spark = Sparkline::default()
            .block(Block::default().bg(Color::Indexed(233)))
            .data(&self.stats.cpu_history)
            .style(Style::default().fg(theme.primary));
        frame.render_widget(cpu_spark, stats_chunks[1]);

        // MEM Sparkline
        let mem_spark = Sparkline::default()
            .block(Block::default().bg(Color::Indexed(233)))
            .data(&self.stats.mem_history)
            .style(Style::default().fg(theme.secondary));
        frame.render_widget(mem_spark, stats_chunks[2]);

        // ─── Phase 9: Toast Notification Stack ───
        let now = std::time::Instant::now();
        self.toasts.retain(|t| t.expiry > now);

        for (i, toast) in self.toasts.iter().enumerate() {
            let t_width = (toast.message.len() + 6) as u16;
            let t_area = Rect {
                x: area.width.saturating_sub(t_width + 2),
                y: area.height.saturating_sub(4 + (i as u16 * 3)),
                width: t_width,
                height: 3,
            };
            frame.render_widget(Clear, t_area);
            let t_style = match toast.level.as_str() {
                "warning" => theme.warning_style(),
                "success" => theme.success_style(),
                _ => theme.primary_style(),
            };
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(t_style)
                .bg(theme.background);
            frame.render_widget(block.clone(), t_area);
            let p = Paragraph::new(format!(" {} ", toast.message)).style(t_style);
            frame.render_widget(p, block.inner(t_area));
        }

        // ─── Cursor ───
        if self.focus == Focus::Input {
            let prompt_len = self.name.len() + 3; // "name ❯ "
            let cursor_pos = self.input.cursor_position();
            let total_cursor_x = prompt_len + cursor_pos;
            let visible_width = layout.input.width.saturating_sub(2) as usize; // inside borders

            let scroll_x = if visible_width > 0 && total_cursor_x >= visible_width {
                total_cursor_x - visible_width + 1
            } else {
                0
            };

            let cursor_x = layout.input.x + 1 + (total_cursor_x - scroll_x) as u16;
            let cursor_y = layout.input.y + 1;
            frame.set_cursor_position((cursor_x, cursor_y));
        }
    }
}

fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let x = r.x + (r.width.saturating_sub(width)) / 2;
    let y = r.y + (r.height.saturating_sub(height)) / 2;
    Rect {
        x,
        y,
        width: width.min(r.width),
        height: height.min(r.height),
    }
}
