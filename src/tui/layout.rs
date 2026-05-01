//! TUI Layout — defines the panel structure using ratatui's layout system.

use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// The main layout structure for MYTH TUI.
pub struct ReconLayout;

/// Main HUD layout components
#[derive(Debug, Clone, Copy)]
pub struct MainRects {
    pub nav: Rect,
    pub header: Rect,
    pub body: Rect,
    pub input: Rect,
    pub status: Rect,
}

/// Body panel components
#[derive(Debug, Clone, Copy)]
pub struct BodyRects {
    pub left: Rect,
    pub center: Rect,
    pub right: Rect,
}

impl ReconLayout {
    /// Create the main HUD layout: Nav, Header, Body, Input, Status
    pub fn main_layout(area: Rect) -> MainRects {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Top Nav Tabs
                Constraint::Length(3), // Neural Header
                Constraint::Min(10),   // Combat Deck / Multi-View
                Constraint::Length(3), // Neural Input
                Constraint::Length(1), // Telemetry Status
            ])
            .split(area);

        MainRects {
            nav: chunks[0],
            header: chunks[1],
            body: chunks[2],
            input: chunks[3],
            status: chunks[4],
        }
    }

    /// Split the body into 3 dynamic panels
    pub fn body_layout(
        area: Rect,
        mut show_left: bool,
        mut show_right: bool,
        left_width: u16,
        right_width: u16,
    ) -> BodyRects {
        // Safe responsiveness: Auto-collapse sidebars on extreme terminal constrictions
        if area.width < 60 {
            show_left = false;
            show_right = false;
        } else if area.width < 80 {
            show_right = false; // Prioritize Recon Map over Sensors on medium terminals
        }

        let l_c = if show_left {
            Constraint::Length(left_width)
        } else {
            Constraint::Length(0)
        };

        let r_c = if show_right {
            Constraint::Length(right_width)
        } else {
            Constraint::Length(0)
        };

        // Center panel dynamically absorbs all remaining space but refuses to crush past 20 chars
        let m_c = Constraint::Min(20);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([l_c, m_c, r_c])
            .split(area);

        BodyRects {
            left: chunks[0],
            center: chunks[1],
            right: chunks[2],
        }
    }
}
