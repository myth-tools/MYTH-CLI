//! Nav widget — clickable top-level mission navigation.

use crate::tui::app::Screen;
use crate::tui::theme::Theme;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

/// Mission navigation tabs.
pub struct NavWidget;

impl NavWidget {
    pub fn render(
        area: Rect,
        buf: &mut Buffer,
        theme: &Theme,
        current_screen: &Screen,
        focused: bool,
    ) {
        let tabs = [
            (Screen::Mission, " ◈ MISSION "),
            (Screen::Settings, " ◈ SETTINGS "),
            (Screen::McpStatus, " ◈ PLUGINS "),
        ];

        let mut spans = Vec::new();
        for (screen, label) in &tabs {
            let is_active = screen == current_screen;

            let style = if is_active {
                if focused {
                    theme.highlight_style()
                } else {
                    theme.secondary_style().add_modifier(Modifier::BOLD)
                }
            } else {
                theme.dim_style()
            };

            spans.push(Span::styled(*label, style));
            spans.push(Span::styled(" │ ", theme.dim_style()));
        }

        // Remove trailing separator
        if !spans.is_empty() {
            spans.pop();
        }

        let nav = Paragraph::new(Line::from(spans))
            .alignment(Alignment::Left)
            .style(Style::default().bg(theme.background));

        nav.render(area, buf);
    }

    /// Detect which screen was clicked.
    pub fn get_screen_at(area: Rect, col: u16, row: u16) -> Option<Screen> {
        if row != area.y {
            return None;
        }

        let tabs = [
            (Screen::Mission, " ◈ MISSION "),
            (Screen::Settings, " ◈ SETTINGS "),
            (Screen::McpStatus, " ◈ PLUGINS "),
        ];

        let mut current_x = area.x;
        for (screen, label) in &tabs {
            let width = label.len() as u16;
            if col >= current_x && col < current_x + width {
                return Some(screen.clone());
            }
            current_x += width + 3; // width of label + " │ "
        }
        None
    }
}
