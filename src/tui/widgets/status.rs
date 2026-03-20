//! Neural Header widget — System telemetry and mission status.

use crate::tui::theme::Theme;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

/// Properties for the StatusWidget.
#[derive(Debug, Clone)]
pub struct StatusProps {
    pub agent_name: String,
    pub version: String,
    pub state: String,
    pub iteration: u32,
    pub max_iterations: u32,
    pub findings_count: usize,
    pub target: String,
    pub is_thinking: bool,
}

/// Neural Header at the top of the HUD.
pub struct StatusWidget {
    pub props: StatusProps,
}

impl StatusWidget {
    pub fn new(props: StatusProps) -> Self {
        Self { props }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let block = Block::default()
            .borders(Borders::BOTTOM)
            .border_style(theme.border_style());

        let inner = block.inner(area);
        block.render(area, buf);

        let link_style = if self.props.is_thinking {
            Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::SLOW_BLINK)
        } else {
            Style::default().fg(theme.accent)
        };

        let link_text = if self.props.is_thinking {
            format!("SYNCING: {} 🛰", self.props.target)
        } else {
            format!("NEURAL LINK: {}", self.props.target)
        };

        let status_text = Line::from(vec![
            Span::styled(
                format!(
                    " 🧠 {} HUD v{} ",
                    self.props.agent_name.to_uppercase(),
                    self.props.version
                ),
                Style::default()
                    .fg(theme.background)
                    .bg(theme.primary)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" [ MODE: {} ] ", self.props.state),
                Style::default()
                    .fg(theme.warning)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" │ ", theme.dim_style()),
            Span::styled(
                format!(
                    "CYCLES: {}/{}",
                    self.props.iteration, self.props.max_iterations
                ),
                theme.text_style(),
            ),
            Span::styled(" │ ", theme.dim_style()),
            Span::styled(
                format!("FINDINGS: {}", self.props.findings_count),
                Style::default().fg(theme.secondary),
            ),
            Span::styled(" │ ", theme.dim_style()),
            Span::styled(link_text, link_style),
            Span::styled(" │ ", theme.dim_style()),
            Span::styled("🔒 SECURE ", Style::default().fg(theme.success)),
        ]);

        let paragraph = Paragraph::new(status_text)
            .alignment(Alignment::Left)
            .style(theme.status_bar_style());

        paragraph.render(inner, buf);
    }
}
