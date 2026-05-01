//! Neural Header widget — elite system telemetry and mission status.

use crate::tui::animation::AnimationState;
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

    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme, anim: &AnimationState) {
        let block = Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(theme.dim));

        let inner = block.inner(area);
        block.render(area, buf);

        // ─── State Badge ───
        let (state_fg, state_bg) = match self.props.state.as_str() {
            "SCANNING" | "RECON" | "EXECUTING" => (theme.background, theme.secondary),
            "THINKING" | "SYNCING" => (theme.background, theme.primary),
            "IDLE" | "STANDBY" => (theme.text, theme.surface_bright),
            _ => (theme.text, theme.surface),
        };

        // ─── Thinking Indicator ───
        let thinking_span = if self.props.is_thinking {
            let spinner = anim.spinner_char();
            Span::styled(
                format!(" {} SYNCING ", spinner),
                Style::default()
                    .fg(theme.background)
                    .bg(theme.primary)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled(
                format!(" {} ", self.props.state),
                Style::default()
                    .fg(state_fg)
                    .bg(state_bg)
                    .add_modifier(Modifier::BOLD),
            )
        };

        // ─── Target Span ───
        let target_span = if self.props.is_thinking {
            Span::styled(
                format!(" {} {} ", anim.scan_bar(), self.props.target),
                Style::default()
                    .fg(theme.primary)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled(
                format!(" ◈ {} ", self.props.target),
                Style::default().fg(theme.accent),
            )
        };

        // ─── Heartbeat ───
        let heartbeat = anim.heartbeat_symbol();

        let status_text = Line::from(vec![
            // Agent badge
            Span::styled(
                format!(
                    " {} v{} ",
                    self.props.agent_name.to_uppercase(),
                    self.props.version
                ),
                Style::default()
                    .fg(theme.background)
                    .bg(theme.primary)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ", theme.dim_style()),
            // Mode
            thinking_span,
            Span::styled(" │ ", theme.dim_style()),
            // Cycles
            Span::styled(
                format!("⟳ {}/{}", self.props.iteration, self.props.max_iterations),
                theme.text_style(),
            ),
            Span::styled(" │ ", theme.dim_style()),
            // Findings
            Span::styled(
                format!("◆ {}", self.props.findings_count),
                Style::default().fg(theme.secondary),
            ),
            Span::styled(" │ ", theme.dim_style()),
            // Target
            target_span,
            Span::styled(" │ ", theme.dim_style()),
            // Security
            Span::styled("● SECURE ", Style::default().fg(theme.success)),
            // Heartbeat
            Span::styled(
                format!("{} ", heartbeat),
                if anim.blink(30, 60) {
                    Style::default().fg(theme.primary)
                } else {
                    Style::default().fg(theme.dim)
                },
            ),
        ]);

        let paragraph = Paragraph::new(status_text)
            .alignment(Alignment::Left)
            .style(theme.status_bar_style());

        paragraph.render(inner, buf);
    }
}
