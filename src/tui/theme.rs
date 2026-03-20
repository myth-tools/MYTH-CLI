//! TUI Theme — colors, styles, and visual constants.

use ratatui::style::{Color, Modifier, Style};

/// MYTH color palette — premium cyber-HUD aesthetic.
pub struct Theme {
    pub primary: Color,        // Neon Cyan
    pub secondary: Color,      // Neon Green
    pub accent: Color,         // Neon Magenta
    pub background: Color,     // Deep Obsidian
    pub surface: Color,        // HUD Surface
    pub text: Color,           // High Contrast White
    pub dim: Color,            // Ghost Gray
    pub success: Color,        // Matrix Green
    pub warning: Color,        // Solar Flare
    pub error: Color,          // Blood Red
    pub table_header: Color,   // HUD Gold
    pub table_border: Color,   // Dim HUD
    pub flowchart_node: Color, // Accent Glow
}

impl Theme {
    /// Deep Obsidian HUD theme.
    pub fn dark() -> Self {
        Self {
            primary: Color::Rgb(255, 140, 0),        // #ff8c00 — Recon Orange
            secondary: Color::Rgb(255, 215, 0),      // #ffd700 — Tactical Gold
            accent: Color::Rgb(255, 69, 0),          // #ff4500 — Breach Red
            background: Color::Rgb(5, 5, 8),         // #050508 — Deep Obsidian
            surface: Color::Rgb(22, 18, 15),         // #16120f — Industrial Brown
            text: Color::Rgb(250, 245, 240),         // #faf5f0 — HUD White
            dim: Color::Rgb(120, 100, 80),           // #786450 — Muted Rust
            success: Color::Rgb(0, 255, 136),        // Matrix Green
            warning: Color::Rgb(255, 215, 0),        // Tactical Gold
            error: Color::Rgb(255, 69, 0),           // Breach Red
            table_header: Color::Rgb(255, 140, 0),   // Recon Orange
            table_border: Color::Rgb(60, 50, 40),    // Dark Rust
            flowchart_node: Color::Rgb(255, 140, 0), // Recon Orange
        }
    }

    /// Title style — Neon glow effect.
    pub fn title_style(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    /// Neural Text style — ultra crisp.
    pub fn text_style(&self) -> Style {
        Style::default().fg(self.text)
    }

    /// Ghost / Telemetry style.
    pub fn dim_style(&self) -> Style {
        Style::default().fg(self.dim)
    }

    /// HUD Highlight — High contrast.
    pub fn highlight_style(&self) -> Style {
        Style::default()
            .fg(self.background)
            .bg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    /// Standard Panel Border.
    pub fn border_style(&self) -> Style {
        Style::default().fg(self.surface)
    }

    /// Active Scanner/HUD Border.
    pub fn active_border_style(&self) -> Style {
        Style::default().fg(self.secondary)
    }

    /// Critical Alert Style.
    pub fn error_style(&self) -> Style {
        Style::default().fg(self.error).add_modifier(Modifier::BOLD)
    }

    /// Logic/Success Style.
    pub fn success_style(&self) -> Style {
        Style::default().fg(self.success)
    }

    /// Sensor Warning Style.
    pub fn warning_style(&self) -> Style {
        Style::default().fg(self.warning)
    }

    /// Cyan HUD Style.
    pub fn primary_style(&self) -> Style {
        Style::default().fg(self.primary)
    }

    /// Green HUD Style.
    pub fn secondary_style(&self) -> Style {
        Style::default().fg(self.secondary)
    }

    /// Magenta HUD Style.
    pub fn accent_style(&self) -> Style {
        Style::default().fg(self.accent)
    }

    /// HUD Telemetry Bar Style.
    pub fn status_bar_style(&self) -> Style {
        Style::default().bg(self.surface).fg(self.text)
    }

    /// Markdown Bold Style.
    pub fn markdown_bold(&self) -> Style {
        Style::default().fg(self.text).add_modifier(Modifier::BOLD)
    }

    /// Markdown Italic Style.
    pub fn markdown_italic(&self) -> Style {
        Style::default()
            .fg(self.text)
            .add_modifier(Modifier::ITALIC)
    }

    /// Markdown Code Style (Inline).
    pub fn markdown_code(&self) -> Style {
        Style::default().fg(self.primary).bg(self.surface)
    }

    /// Markdown Block Quote / List Style.
    pub fn markdown_lead(&self) -> Style {
        Style::default()
            .fg(self.secondary)
            .add_modifier(Modifier::BOLD)
    }

    /// Markdown Strikethrough Style.
    pub fn markdown_strikethrough(&self) -> Style {
        Style::default()
            .fg(self.dim)
            .add_modifier(Modifier::CROSSED_OUT)
    }

    /// Markdown Blockquote Style.
    pub fn markdown_blockquote(&self) -> Style {
        Style::default().fg(self.dim).add_modifier(Modifier::ITALIC)
    }

    /// Markdown Horizontal Rule Style.
    pub fn markdown_hr(&self) -> Style {
        Style::default().fg(self.dim)
    }

    /// Markdown Link Style.
    pub fn markdown_link(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::UNDERLINED)
    }

    /// Markdown H1 — Bold Magenta.
    pub fn markdown_h1(&self) -> Style {
        Style::default()
            .fg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    /// Markdown H2 — Bold Cyan.
    pub fn markdown_h2(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    /// Markdown H3 — Bold Green.
    pub fn markdown_h3(&self) -> Style {
        Style::default()
            .fg(self.secondary)
            .add_modifier(Modifier::BOLD)
    }

    /// Task list: completed item.
    pub fn markdown_task_done(&self) -> Style {
        Style::default()
            .fg(self.success)
            .add_modifier(Modifier::CROSSED_OUT)
    }

    /// Task list: pending item.
    pub fn markdown_task_pending(&self) -> Style {
        Style::default().fg(self.dim)
    }

    /// Bold + Italic combined.
    pub fn markdown_bold_italic(&self) -> Style {
        Style::default()
            .fg(self.text)
            .add_modifier(Modifier::BOLD | Modifier::ITALIC)
    }

    /// Table Header style — HUD Gold.
    pub fn table_header(&self) -> Style {
        Style::default()
            .fg(self.table_header)
            .add_modifier(Modifier::BOLD)
    }

    /// Table Border style — Dim HUD.
    pub fn table_border(&self) -> Style {
        Style::default().fg(self.table_border)
    }

    /// Flowchart Node style — Cyan Glow.
    pub fn flowchart_node(&self) -> Style {
        Style::default()
            .fg(self.flowchart_node)
            .add_modifier(Modifier::BOLD)
    }

    /// Flowchart Connector style — Dim HUD.
    pub fn flowchart_arrow(&self) -> Style {
        Style::default().fg(self.dim)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}
