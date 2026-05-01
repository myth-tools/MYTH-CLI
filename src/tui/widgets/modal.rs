//! Modal widget — high-fidelity tactical overlays.

use crate::tui::app::Modal;
use crate::tui::theme::Theme;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};

pub struct ModalWidget;

impl ModalWidget {
    pub fn render(area: Rect, buf: &mut Buffer, theme: &Theme, modal: &Modal) {
        // Calculate modal size (centered)
        let modal_area = Self::centered_rect(60, 40, area);

        // Clear the area under the modal
        Clear.render(modal_area, buf);

        // ─── Shadow Effect (Block gradient) ───
        let shadow_area = Rect {
            x: modal_area.x + 2,
            y: modal_area.y + 1,
            width: modal_area.width,
            height: modal_area.height,
        };
        if shadow_area.right() < area.right() && shadow_area.bottom() < area.bottom() {
            for y in shadow_area.y..shadow_area.y + shadow_area.height {
                for x in shadow_area.x..shadow_area.x + shadow_area.width {
                    if let Some(cell) = buf.cell_mut((x, y)) {
                        cell.set_style(Style::default().fg(Color::Indexed(234)).bg(Color::Black));
                        cell.set_char(' '); // Solid dark shadow
                    }
                }
            }
        }

        // ─── Main Modal Box ───
        let (title, content, style) = match modal {
            Modal::Help => (
                " ◈ NEURAL COMMAND MATRIX [HELP] ",
                vec![
                    Line::from(vec![Span::styled(
                        " GLOBAL CONTROLS ",
                        theme.primary_style().add_modifier(Modifier::BOLD),
                    )]),
                    Line::from(vec![
                        Span::styled("  ?          ", theme.secondary_style()),
                        Span::raw(" Toggle this Help Matrix"),
                    ]),
                    Line::from(vec![
                        Span::styled("  Ctrl+C     ", theme.secondary_style()),
                        Span::raw(" Abort Mission / Exit"),
                    ]),
                    Line::from(vec![
                        Span::styled("  Tab        ", theme.secondary_style()),
                        Span::raw(" Cycle Focus (Input/Chat/Tree)"),
                    ]),
                    Line::from(vec![
                        Span::styled("  Esc        ", theme.secondary_style()),
                        Span::raw(" Close Modals / MCP Manager"),
                    ]),
                    Line::from(""),
                    Line::from(vec![Span::styled(
                        " VIEWPORT MODULATION ",
                        theme.primary_style().add_modifier(Modifier::BOLD),
                    )]),
                    Line::from(vec![
                        Span::styled("  Ctrl+B     ", theme.secondary_style()),
                        Span::raw(" Toggle Recon Map (Left)"),
                    ]),
                    Line::from(vec![
                        Span::styled("  Ctrl+N     ", theme.secondary_style()),
                        Span::raw(" Toggle Sensor Array (Right)"),
                    ]),
                    Line::from(vec![
                        Span::styled("  [ / ]      ", theme.secondary_style()),
                        Span::raw(" Toggle Panels (Quick)"),
                    ]),
                    Line::from(vec![
                        Span::styled("  < / >      ", theme.secondary_style()),
                        Span::raw(" Resize Recon Map Panel"),
                    ]),
                    Line::from(vec![
                        Span::styled("  { / }      ", theme.secondary_style()),
                        Span::raw(" Resize Sensor Array Panel"),
                    ]),
                    Line::from(""),
                    Line::from(vec![Span::styled(
                        " TACTICAL NAVIGATION ",
                        theme.primary_style().add_modifier(Modifier::BOLD),
                    )]),
                    Line::from(vec![
                        Span::styled("  Ctrl+P     ", theme.secondary_style()),
                        Span::raw(" Neural Command Palette (Fuzzy)"),
                    ]),
                    Line::from(vec![
                        Span::styled("  Ctrl+M     ", theme.secondary_style()),
                        Span::raw(" MCP Tactical Manager"),
                    ]),
                    Line::from(vec![
                        Span::styled("  H / L      ", theme.secondary_style()),
                        Span::raw(" Switch Active Screens (Mission/Settings/MCP)"),
                    ]),
                    Line::from(""),
                    Line::from(vec![Span::styled(
                        " INTERACTIVE HUD ",
                        theme.primary_style().add_modifier(Modifier::BOLD),
                    )]),
                    Line::from(vec![
                        Span::styled("  MOUSE_SCROLL ", theme.success_style()),
                        Span::raw(" Scroll focused view"),
                    ]),
                    Line::from(vec![
                        Span::styled("  LEFT_CLICK   ", theme.success_style()),
                        Span::raw(" Focus / Action / Resize Border"),
                    ]),
                    Line::from(vec![
                        Span::styled("  RIGHT_CLICK  ", theme.success_style()),
                        Span::raw(" Context Menu (Node/Sensor)"),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled(" (Press ", theme.dim_style()),
                        Span::styled("Esc", theme.secondary_style()),
                        Span::styled(" or ", theme.dim_style()),
                        Span::styled("?", theme.secondary_style()),
                        Span::styled(" to dismiss) ", theme.dim_style()),
                    ]),
                ],
                theme.primary_style(),
            ),
            Modal::Warning(msg) => (
                " ◈ SYSTEM WARNING ",
                vec![Line::from(vec![
                    Span::styled(" [!] ", theme.error_style()),
                    Span::raw(msg),
                ])],
                theme.error_style(),
            ),
            Modal::CommandConfirm(cmd) => (
                " ◈ EXECUTION CONFIRMATION ",
                vec![
                    Line::from(vec![Span::raw("Confirm deployment of command:")]),
                    Line::from(vec![Span::styled(
                        format!("  {}  ", cmd),
                        theme.secondary_style(),
                    )]),
                    Line::from(""),
                    Line::from(vec![
                        Span::raw("Press "),
                        Span::styled("Enter", theme.success_style()),
                        Span::raw(" to confirm or "),
                        Span::styled("Esc", theme.error_style()),
                        Span::raw(" to abort."),
                    ]),
                ],
                theme.warning_style(),
            ),
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(style)
            .title(Span::styled(title, style.add_modifier(Modifier::BOLD)))
            .bg(theme.background);

        let inner_area = block.inner(modal_area);
        block.render(modal_area, buf);

        let text = Paragraph::new(content)
            .alignment(Alignment::Left)
            .wrap(ratatui::widgets::Wrap { trim: true });

        text.render(inner_area, buf);
    }

    /// Helper to create a centered rect of a certain percentage
    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}
