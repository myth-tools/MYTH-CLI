//! Findings widget — Tactical registry of discovered vulnerabilities and targets.

use crate::tui::app::Finding;
use crate::tui::theme::Theme;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Cell, Row, Table};

pub struct FindingsWidget {
    pub selected_index: usize,
    pub state: ratatui::widgets::TableState,
}

impl Default for FindingsWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl FindingsWidget {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            state: ratatui::widgets::TableState::default().with_selected(Some(0)),
        }
    }

    pub fn next(&mut self, len: usize) {
        if len == 0 {
            return;
        }
        self.selected_index = (self.selected_index + 1) % len;
        self.state.select(Some(self.selected_index));
    }

    pub fn previous(&mut self, len: usize) {
        if len == 0 {
            return;
        }
        self.selected_index = (self.selected_index + len - 1) % len;
        self.state.select(Some(self.selected_index));
    }

    pub fn render(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        theme: &Theme,
        findings: &[Finding],
        focused: bool,
    ) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(if focused {
                BorderType::Thick
            } else {
                BorderType::Plain
            })
            .border_style(if focused {
                theme.focused_border_style()
            } else {
                theme.unfocused_border_style()
            })
            .title(Span::styled(
                " ◈ TACTICAL FINDINGS REGISTRY ",
                if focused {
                    theme.focused_title_style()
                } else {
                    theme.title_style()
                },
            ))
            .bg(theme.background);

        let header_cells = ["ID", "TARGET", "SERVICE", "PORT", "RISK", "DESCRIPTION"]
            .iter()
            .map(|h| {
                Cell::from(*h).style(
                    Style::default()
                        .fg(theme.primary)
                        .add_modifier(Modifier::BOLD),
                )
            });
        let header = Row::new(header_cells)
            .style(Style::default().bg(theme.surface))
            .height(1)
            .bottom_margin(1);

        let rows = findings.iter().map(|f| {
            let risk_style = match f.risk.to_uppercase().as_str() {
                "CRITICAL" => Style::default()
                    .fg(theme.error)
                    .add_modifier(Modifier::BOLD),
                "HIGH" => Style::default()
                    .fg(theme.warning)
                    .add_modifier(Modifier::BOLD),
                "MEDIUM" => Style::default().fg(theme.secondary),
                "LOW" => Style::default().fg(theme.success),
                _ => theme.dim_style(),
            };

            Row::new(vec![
                Cell::from(f.id.clone()).style(theme.dim_style()),
                Cell::from(f.target.clone()).style(theme.primary_style()),
                Cell::from(f.service.clone()).style(theme.secondary_style()),
                Cell::from(f.port.clone()).style(theme.accent_style()),
                Cell::from(f.risk.clone()).style(risk_style),
                Cell::from(f.description.clone()).style(theme.text_style()),
            ])
        });

        let table = Table::new(
            rows,
            [
                Constraint::Length(8),  // ID
                Constraint::Min(20),    // TARGET
                Constraint::Length(15), // SERVICE
                Constraint::Length(8),  // PORT
                Constraint::Length(10), // RISK
                Constraint::Min(40),    // DESCRIPTION
            ],
        )
        .header(header)
        .block(block)
        .row_highlight_style(
            Style::default()
                .bg(theme.surface_bright)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▸ ");

        ratatui::widgets::StatefulWidget::render(table, area, buf, &mut self.state);
    }
}
