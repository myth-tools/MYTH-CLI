//! TUI Layout — defines the panel structure using ratatui's Flex layout.

use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// The main layout structure for MYTH TUI.
pub struct ReconLayout;

impl ReconLayout {
    /// Create the main HUD layout: Header, Mid (Body), Bottom (Input + Status)
    pub fn main_layout(area: Rect) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // HUD Header / Neural Telemetry
                Constraint::Min(10),   // Main Combat Deck
                Constraint::Length(3), // Neural Input
                Constraint::Length(1), // Telemetry Status
            ])
            .split(area)
            .to_vec()
    }

    /// Split the body into 3 vertical panels:
    /// [Sidebar (Recon Tree)] [Main (Chat)] [Sensors (Swarm Tracker)]
    pub fn body_layout(area: Rect, show_sidebars: bool) -> Vec<Rect> {
        if show_sidebars {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(20), // Neural Tree (Left)
                    Constraint::Percentage(55), // tactical intelligence (Center)
                    Constraint::Percentage(25), // Active Sensorial Feed (Right)
                ])
                .split(area)
                .to_vec()
        } else {
            vec![area] // Mission Focused View (Center Only)
        }
    }
}
