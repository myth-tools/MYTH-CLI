use crate::config::{AppConfig, CustomMcpServer};
use crate::tui::theme::Theme;
use ratatui::prelude::*;
use ratatui::widgets::*;

/// Widget to display and manage MCP servers.
pub struct McpWidget {
    pub selected_index: usize,
    pub servers: Vec<(String, CustomMcpServer, bool)>, // (Name, Config, IsBuiltin)
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

        Self {
            selected_index: 0,
            servers,
        }
    }

    pub fn next(&mut self) {
        if !self.servers.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.servers.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.servers.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.servers.len() - 1;
            } else {
                self.selected_index -= 1;
            }
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

            // Save to mcp.json
            if let Ok(mut storage) = crate::config::McpStorage::load() {
                storage.mcp_servers.insert(name.clone(), srv.clone());
                let _ = storage.save();
            }

            return Some((name.clone(), new_state));
        }
        None
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let block = Block::default()
            .title(Span::styled(
                " ⚡ NEURAL TOOL-BRIDGE (MCP) ",
                theme.title_style(),
            ))
            .borders(Borders::ALL)
            .border_style(theme.accent_style())
            .border_type(BorderType::Thick);

        let inner_area = block.inner(area);
        Clear.render(area, buf); // Ensure we clear the background for the modal
        block.render(area, buf);

        let mut items = Vec::new();
        for (i, (name, srv, is_builtin)) in self.servers.iter().enumerate() {
            let is_enabled = match srv {
                CustomMcpServer::Local(l) => l.enabled,
                CustomMcpServer::Remote(r) => r.enabled,
            };

            let status_symbol = if is_enabled { "●" } else { "○" };
            let status_style = if is_enabled {
                theme.success_style()
            } else {
                theme.dim_style()
            };

            let mcp_type = match srv {
                CustomMcpServer::Local(_) => "LOCAL",
                CustomMcpServer::Remote(_) => "REMOTE",
            };

            let mut spans = vec![
                Span::styled(format!(" {} ", status_symbol), status_style),
                Span::styled(
                    format!("{:<20}", name),
                    if i == self.selected_index {
                        theme.accent_style().bold()
                    } else {
                        theme.primary_style()
                    },
                ),
                Span::styled(format!(" [{}]", mcp_type), theme.dim_style()),
            ];

            if *is_builtin {
                spans.push(Span::styled(" (CORE)", theme.dim_style().italic()));
            }

            items.push(ListItem::new(Line::from(spans)));
        }

        let list = List::new(items)
            .highlight_style(Style::default().bg(theme.surface))
            .highlight_symbol("> ");

        let mut state = ListState::default();
        state.select(Some(self.selected_index));

        StatefulWidget::render(list, inner_area, buf, &mut state);

        // ─── Help Matrix ───
        let help_text = vec![Line::from(vec![
            Span::styled(" [↑/↓] ", theme.dim_style()),
            Span::styled("Navigate ", theme.primary_style()),
            Span::styled(" [SPACE/ENTER] ", theme.dim_style()),
            Span::styled("Toggle Asset ", theme.primary_style()),
            Span::styled(" [M/ESC] ", theme.dim_style()),
            Span::styled("Close Registry", theme.primary_style()),
        ])];
        let help = Paragraph::new(help_text).alignment(Alignment::Center);

        let help_area = Rect::new(
            inner_area.x,
            inner_area.y + inner_area.height - 1,
            inner_area.width,
            1,
        );
        help.render(help_area, buf);
    }
}
