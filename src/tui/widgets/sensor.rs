//! Sensor Feed widget — real-time tool telemetry with gradient progress.

use crate::tui::animation::AnimationState;
use crate::tui::theme::Theme;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

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
    pub state: ListState,
}

impl SensorWidget {
    pub fn new() -> Self {
        Self {
            readings: Vec::new(),
            state: ListState::default(),
        }
    }

    pub fn scroll_down(&mut self) {
        let total = self.readings.len();
        if total == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => (i + 1).min(total.saturating_sub(1)),
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn scroll_up(&mut self) {
        if self.readings.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => i.saturating_sub(1),
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn update_reading(&mut self, tool: &str, status: SensorStatus, progress: f64) {
        if let Some(r) = self.readings.iter_mut().find(|r| r.tool_name == tool) {
            r.status = status;
            r.progress = (progress * 100.0).round() / 100.0;
        } else {
            self.readings.push(SensorReading {
                tool_name: tool.to_string(),
                status,
                progress,
            });
        }
        // Enforce cap: Keep all Active, but limit Complete/Failed to the most recent 10.
        let inactive_count = self
            .readings
            .iter()
            .filter(|r| r.status != SensorStatus::Active)
            .count();
        if inactive_count > 10 {
            // Remove the oldest inactive reading
            if let Some(idx) = self
                .readings
                .iter()
                .position(|r| r.status != SensorStatus::Active)
            {
                self.readings.remove(idx);
            }
        }
    }

    pub fn clear_finished(&mut self) {
        self.readings.retain(|r| r.status == SensorStatus::Active);
    }

    /// Detect tool at mouse position for tooltips.
    pub fn get_tool_at(&self, area: Rect, col: u16, row: u16) -> Option<String> {
        let inner = Block::default().borders(Borders::ALL).inner(area);
        if col < inner.x
            || col >= inner.x + inner.width
            || row < inner.y
            || row >= inner.y + inner.height
        {
            return None;
        }

        let rel_row = (row - inner.y) as usize;
        let scroll = self.state.selected().unwrap_or(0);

        // This is a simple approximation; ideally we'd account for list wrapping/depth
        let idx = rel_row.saturating_add(scroll);
        self.readings.get(idx).map(|r| {
            format!(
                " [ TOOL: {} ]\\n STATUS: {:?}\\n PROGRESS: {}% ",
                r.tool_name,
                r.status,
                (r.progress * 100.0) as usize
            )
        })
    }

    /// Count active sensors.
    fn active_count(&self) -> usize {
        self.readings
            .iter()
            .filter(|r| r.status == SensorStatus::Active)
            .count()
    }

    pub fn render(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        theme: &Theme,
        focused: bool,
        anim: &AnimationState,
    ) {
        let active = self.active_count();
        let base_title = if active > 0 {
            format!(" SENSORS [{}] ", active)
        } else {
            " SENSORS ".to_string()
        };

        let title_text = if focused {
            format!(" ▶ ACTIVE :: {} ", base_title.trim())
        } else {
            format!(" ◈ {} ", base_title.trim())
        };
        let title_style = if focused {
            theme.focused_title_style()
        } else {
            theme.title_style()
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(if focused {
                theme.focused_border_style()
            } else {
                theme.unfocused_border_style()
            })
            .title(Span::styled(title_text, title_style));

        let inner = block.inner(area);
        block.render(area, buf);

        if self.readings.is_empty() {
            let hb = anim.heartbeat_symbol();
            let empty_text = Paragraph::new(format!("{} STANDBY", hb))
                .style(theme.dim_style())
                .alignment(Alignment::Center);

            let vertical_center = ratatui::layout::Layout::default()
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
                let (badge, badge_style) = match r.status {
                    SensorStatus::Active => (
                        "● LIVE",
                        Style::default()
                            .fg(theme.success)
                            .add_modifier(Modifier::BOLD),
                    ),
                    SensorStatus::Complete => ("◉ DONE", Style::default().fg(theme.primary)),
                    SensorStatus::Failed => (
                        "✗ FAIL",
                        Style::default()
                            .fg(theme.error)
                            .add_modifier(Modifier::BOLD),
                    ),
                };

                let progress_bar = if r.status == SensorStatus::Active {
                    let filled = (r.progress * 8.0) as usize;
                    let empty = 8_usize.saturating_sub(filled);
                    format!(" {}{}", "▰".repeat(filled), "▱".repeat(empty))
                } else {
                    String::new()
                };

                // Truncate tool name for display
                let display_name = if r.tool_name.len() > 18 {
                    format!("{}…", &r.tool_name[..17])
                } else {
                    r.tool_name.clone()
                };

                ListItem::new(Line::from(vec![
                    Span::styled(format!(" {} ", badge), badge_style),
                    Span::styled(
                        display_name,
                        Style::default().fg(theme.text).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(progress_bar, Style::default().fg(theme.primary)),
                ]))
            })
            .collect();

        let list = List::new(list_items)
            .highlight_style(theme.highlight_style())
            .highlight_symbol("▶ ");

        StatefulWidget::render(list, inner, buf, &mut self.state);
    }
}

impl Default for SensorWidget {
    fn default() -> Self {
        Self::new()
    }
}
