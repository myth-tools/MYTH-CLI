use crate::config::{AppConfig, CustomMcpServer};
use crate::tui::theme::Theme;
use ratatui::prelude::*;
use ratatui::widgets::*;

/// Widget to display and manage MCP servers.
pub struct McpWidget {
    pub selected_index: usize,
    pub servers: Vec<(String, CustomMcpServer, bool)>, // (Name, Config, IsBuiltin)
    pub state: ratatui::widgets::ListState,
}

impl McpWidget {
    pub fn new(app_config: &AppConfig) -> Self {
        let mut servers = Vec::new();
        let factory_defaults = crate::builtin_mcp::get_factory_defaults();
        for (name, srv) in &app_config.mcp.mcp_servers {
            let is_builtin = factory_defaults.contains_key(name);
            servers.push((name.clone(), srv.clone(), is_builtin));
        }

        servers.sort_by(|a, b| a.0.cmp(&b.0));

        let mut state = ratatui::widgets::ListState::default();
        if !servers.is_empty() {
            state.select(Some(0));
        }

        Self {
            selected_index: 0,
            servers,
            state,
        }
    }

    pub fn next(&mut self) {
        if !self.servers.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.servers.len();
            self.state.select(Some(self.selected_index));
        }
    }

    pub fn previous(&mut self) {
        if !self.servers.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.servers.len() - 1;
            } else {
                self.selected_index -= 1;
            }
            self.state.select(Some(self.selected_index));
        }
    }

    pub fn toggle_selected(&mut self) -> Option<(String, bool)> {
        if let Some((name, srv, _)) = self.servers.get_mut(self.selected_index) {
            let new_state = match srv {
                CustomMcpServer::Local(l) => {
                    l.enabled = !l.enabled;
                    l.enabled
                }
                CustomMcpServer::Remote(r) => {
                    r.enabled = !r.enabled;
                    r.enabled
                }
            };

            // Persist to mcp.json
            if let Ok(mut storage) = crate::config::McpStorage::load() {
                storage.mcp_servers.insert(name.clone(), srv.clone());
                let _ = storage.save();
            }

            return Some((name.clone(), new_state));
        }
        None
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        // ─── Double-border modal ───
        let block = Block::default()
            .title(Span::styled(
                " ◈ NEURAL TOOL-BRIDGE (MCP) ",
                Style::default()
                    .fg(theme.primary)
                    .add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_style(
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            )
            .border_type(BorderType::Double);

        let inner_area = block.inner(area);
        Clear.render(area, buf);

        // Paint background for contrast
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_bg(theme.surface);
                }
            }
        }

        block.render(area, buf);

        let mut items = Vec::new();
        for (i, (name, srv, is_builtin)) in self.servers.iter().enumerate() {
            let is_enabled = match srv {
                CustomMcpServer::Local(l) => l.enabled,
                CustomMcpServer::Remote(r) => r.enabled,
            };

            let status_symbol = if is_enabled { "●" } else { "○" };
            let status_style = if is_enabled {
                Style::default()
                    .fg(theme.success)
                    .add_modifier(Modifier::BOLD)
            } else {
                theme.dim_style()
            };

            let mcp_type = match srv {
                CustomMcpServer::Local(_) => "LOCAL",
                CustomMcpServer::Remote(_) => "REMOTE",
            };
            let type_color = match srv {
                CustomMcpServer::Local(_) => theme.primary,
                CustomMcpServer::Remote(_) => theme.accent,
            };

            let name_style = if i == self.selected_index {
                Style::default().fg(theme.text).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.primary)
            };

            let mut spans = vec![
                Span::styled(format!(" {} ", status_symbol), status_style),
                Span::styled(format!("{:<20}", name), name_style),
                Span::styled(format!(" [{}]", mcp_type), Style::default().fg(type_color)),
            ];

            if *is_builtin {
                spans.push(Span::styled(
                    " CORE",
                    Style::default()
                        .fg(theme.dim)
                        .add_modifier(Modifier::ITALIC),
                ));
            }

            items.push(ListItem::new(Line::from(spans)));
        }

        let list = List::new(items)
            .highlight_style(
                Style::default()
                    .bg(theme.surface_bright)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▸ ");

        // Reserve space for help bar
        let list_area = Rect::new(
            inner_area.x,
            inner_area.y,
            inner_area.width,
            inner_area.height.saturating_sub(1),
        );
        StatefulWidget::render(list, list_area, buf, &mut self.state);

        // ─── Help Bar ───
        let help_text = vec![Line::from(vec![
            Span::styled(" ↑↓ ", Style::default().fg(theme.dim)),
            Span::styled("Navigate", Style::default().fg(theme.primary)),
            Span::styled("  ⏎ ", Style::default().fg(theme.dim)),
            Span::styled("Toggle", Style::default().fg(theme.primary)),
            Span::styled("  ESC ", Style::default().fg(theme.dim)),
            Span::styled("Close", Style::default().fg(theme.primary)),
        ])];
        let help = Paragraph::new(help_text).alignment(Alignment::Center);

        let help_area = Rect::new(
            inner_area.x,
            inner_area.y + inner_area.height.saturating_sub(1),
            inner_area.width,
            1,
        );
        help.render(help_area, buf);
    }
}
