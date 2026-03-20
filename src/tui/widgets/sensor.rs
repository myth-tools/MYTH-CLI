//! Sensor Feed widget — real-time swarm telemetry.

use crate::tui::theme::Theme;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

/// A single sensor reading (active tool).
#[derive(Debug, Clone)]
pub struct SensorReading {
    pub tool_name: String,
    pub status: SensorStatus,
    pub progress: f64, // 0.0 to 1.0
}

#[derive(Debug, Clone, PartialEq)]
pub enum SensorStatus {
    Active,
    Complete,
    Failed,
}

/// Active sensorial feed panel.
pub struct SensorWidget {
    readings: Vec<SensorReading>,
}

impl SensorWidget {
    pub fn new() -> Self {
        Self {
            readings: Vec::new(),
        }
    }

    pub fn update_reading(&mut self, tool: &str, status: SensorStatus, progress: f64) {
        if let Some(r) = self.readings.iter_mut().find(|r| r.tool_name == tool) {
            r.status = status;
            // L-02 Fix: Quantize progress to avoid excessive flickering on tiny updates
            r.progress = (progress * 100.0).round() / 100.0;
        } else {
            self.readings.push(SensorReading {
                tool_name: tool.to_string(),
                status,
                progress,
            });
        }
    }

    pub fn clear_finished(&mut self) {
        self.readings.retain(|r| r.status == SensorStatus::Active);
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme.border_style())
            .title(" 📡 Sensor Feed ")
            .title_style(theme.title_style());

        let inner = block.inner(area);
        block.render(area, buf);

        if self.readings.is_empty() {
            let empty_text = Paragraph::new("⚡ [ STANDBY ]")
                .style(theme.dim_style())
                .alignment(Alignment::Center);

            let vertical_center = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(45),
                    Constraint::Length(1),
                    Constraint::Percentage(45),
                ])
                .split(inner)[1];

            empty_text.render(vertical_center, buf);
            return;
        }

        let list_items: Vec<ListItem> = self
            .readings
            .iter()
            .map(|r| {
                let (icon, color) = match r.status {
                    SensorStatus::Active => ("⚡", theme.primary),
                    SensorStatus::Complete => ("🏁", theme.success),
                    SensorStatus::Failed => ("⚠️", theme.error),
                };

                let progress_bar = if r.status == SensorStatus::Active {
                    let filled = (r.progress * 10.0) as usize;
                    format!(" [{}{}]", "█".repeat(filled), "░".repeat(10 - filled))
                } else {
                    "".to_string()
                };

                ListItem::new(Line::from(vec![
                    Span::styled(format!(" {} ", icon), Style::default().fg(color)),
                    Span::styled(
                        r.tool_name.clone(),
                        Style::default().fg(theme.text).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(progress_bar, theme.dim_style()),
                ]))
            })
            .collect();

        let list = List::new(list_items);
        Widget::render(list, inner, buf);
    }
}

impl Default for SensorWidget {
    fn default() -> Self {
        Self::new()
    }
}
