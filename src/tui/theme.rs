//! TUI Theme — premium cyber-HUD color palette and styles.

use ratatui::style::{Color, Modifier, Style};

/// MYTH color palette — elite neon cyber-HUD aesthetic.
pub struct Theme {
    // ═══ Core Palette ═══
    pub primary: Color,        // Electric Cyan
    pub secondary: Color,      // Matrix Green
    pub accent: Color,         // Ultraviolet
    pub background: Color,     // True Black
    pub surface: Color,        // Dark Navy
    pub surface_bright: Color, // Bright Navy (focus states)
    pub text: Color,           // Cool White
    pub dim: Color,            // Steel Gray
    // ═══ Semantic ═══
    pub success: Color, // Neon Green
    pub warning: Color, // Amber
    pub error: Color,   // Hot Pink
    // ═══ Extended ═══
    pub glow: Color,           // Glow/Pulse base
    pub border_focus: Color,   // Focused panel border
    pub gradient_start: Color, // Progress bar start
    pub gradient_end: Color,   // Progress bar end
    pub table_header: Color,   // Table header
    pub table_border: Color,   // Table border
    pub flowchart_node: Color, // Flowchart nodes
    pub tactical: Color,       // Premium Operative/Chief Indigo
    // ═══ Typography ═══
    pub font: String,         // Dynamic unicode font mapper
    pub simulated_mode: bool, // Whether to use Unicode simulation
}

impl Theme {
    /// Premium Neon Cyber-HUD theme.
    pub fn dark(simulated_mode: bool) -> Self {
        Self {
            primary: Color::Rgb(0, 212, 255),        // #00d4ff — Electric Cyan
            secondary: Color::Rgb(0, 255, 136),      // #00ff88 — Matrix Green
            accent: Color::Rgb(191, 90, 242),        // #bf5af2 — Ultraviolet
            background: Color::Rgb(10, 14, 20),      // #0a0e14 — True Black
            surface: Color::Rgb(26, 30, 46),         // #1a1e2e — Dark Navy
            surface_bright: Color::Rgb(36, 42, 62),  // #242a3e — Bright Navy
            text: Color::Rgb(228, 232, 240),         // #e4e8f0 — Cool White
            dim: Color::Rgb(61, 69, 96),             // #3d4560 — Steel Gray
            success: Color::Rgb(0, 255, 136),        // #00ff88 — Neon Green
            warning: Color::Rgb(255, 184, 0),        // #ffb800 — Amber
            error: Color::Rgb(255, 59, 92),          // #ff3b5c — Hot Pink
            glow: Color::Rgb(0, 212, 255),           // #00d4ff — Cyan glow
            border_focus: Color::Rgb(0, 212, 255),   // #00d4ff — Focused border
            gradient_start: Color::Rgb(0, 212, 255), // Cyan
            gradient_end: Color::Rgb(0, 255, 136),   // Green
            table_header: Color::Rgb(0, 212, 255),   // Electric Cyan
            table_border: Color::Rgb(42, 48, 80),    // #2a3050 — Night Blue
            flowchart_node: Color::Rgb(0, 212, 255), // Electric Cyan
            tactical: Color::Rgb(130, 100, 255),     // Premium Electric Indigo
            font: "jet-brains-mono".to_string(),
            simulated_mode,
        }
    }

    // ═══════════════════════════════════════════
    //  Core Styles
    // ═══════════════════════════════════════════

    /// Title style — neon glow.
    pub fn title_style(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    /// Focused title style — highlighted neon.
    pub fn focused_title_style(&self) -> Style {
        Style::default()
            .fg(self.secondary)
            .add_modifier(Modifier::BOLD)
    }

    /// Scrollbar style — subtle telemetry.
    pub fn scrollbar_style(&self, focused: bool) -> Style {
        if focused {
            Style::default().fg(self.primary)
        } else {
            Style::default().fg(self.dim)
        }
    }

    /// Neural text — ultra crisp.
    pub fn text_style(&self) -> Style {
        Style::default().fg(self.text)
    }

    /// Ghost/telemetry — steel gray.
    pub fn dim_style(&self) -> Style {
        Style::default().fg(self.dim)
    }

    /// HUD highlight — inverted high contrast.
    pub fn highlight_style(&self) -> Style {
        Style::default()
            .fg(self.background)
            .bg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    // ═══════════════════════════════════════════
    //  Border Styles
    // ═══════════════════════════════════════════

    /// Standard panel border — dim.
    pub fn border_style(&self) -> Style {
        Style::default().fg(self.dim)
    }

    /// Focused panel border — glowing cyan.
    pub fn focused_border_style(&self) -> Style {
        Style::default()
            .fg(self.border_focus)
            .add_modifier(Modifier::BOLD)
    }

    /// Active scanner/HUD border (legacy compat).
    pub fn active_border_style(&self) -> Style {
        self.focused_border_style()
    }

    /// Unfocused panel border — subtle.
    pub fn unfocused_border_style(&self) -> Style {
        Style::default().fg(self.dim)
    }

    // ═══════════════════════════════════════════
    //  Semantic Styles
    // ═══════════════════════════════════════════

    /// Critical alert.
    pub fn error_style(&self) -> Style {
        Style::default().fg(self.error).add_modifier(Modifier::BOLD)
    }

    /// Success.
    pub fn success_style(&self) -> Style {
        Style::default().fg(self.success)
    }

    /// Warning.
    pub fn warning_style(&self) -> Style {
        Style::default().fg(self.warning)
    }

    /// Primary/Cyan HUD style.
    pub fn primary_style(&self) -> Style {
        Style::default().fg(self.primary)
    }

    /// Secondary/Green HUD style.
    pub fn secondary_style(&self) -> Style {
        Style::default().fg(self.secondary)
    }

    /// Accent/Violet HUD style.
    pub fn accent_style(&self) -> Style {
        Style::default().fg(self.accent)
    }

    /// Tactical/Indigo HUD style.
    pub fn tactical_style(&self) -> Style {
        Style::default()
            .fg(self.tactical)
            .add_modifier(Modifier::BOLD)
    }

    /// Glow/pulse base style.
    pub fn glow_style(&self) -> Style {
        Style::default().fg(self.glow).add_modifier(Modifier::BOLD)
    }

    /// Status badge style.
    pub fn badge_style(&self, fg: Color, bg: Color) -> Style {
        Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD)
    }

    // ═══════════════════════════════════════════
    //  Bar Styles
    // ═══════════════════════════════════════════

    /// HUD telemetry bar.
    pub fn status_bar_style(&self) -> Style {
        Style::default().bg(self.surface).fg(self.text)
    }

    // ═══════════════════════════════════════════
    //  Markdown Styles
    // ═══════════════════════════════════════════

    pub fn markdown_bold(&self) -> Style {
        Style::default().fg(self.text).add_modifier(Modifier::BOLD)
    }

    pub fn markdown_italic(&self) -> Style {
        Style::default()
            .fg(self.text)
            .add_modifier(Modifier::ITALIC)
    }

    pub fn markdown_code(&self) -> Style {
        Style::default().fg(self.primary).bg(self.surface)
    }

    pub fn markdown_lead(&self) -> Style {
        Style::default()
            .fg(self.secondary)
            .add_modifier(Modifier::BOLD)
    }

    pub fn markdown_strikethrough(&self) -> Style {
        Style::default()
            .fg(self.dim)
            .add_modifier(Modifier::CROSSED_OUT)
    }

    /// Highlight — Cyan on dark.
    pub fn markdown_highlight(&self) -> Style {
        Style::default()
            .fg(self.background)
            .bg(self.secondary)
            .add_modifier(Modifier::BOLD)
    }

    pub fn markdown_blockquote(&self) -> Style {
        Style::default().fg(self.dim).add_modifier(Modifier::ITALIC)
    }

    pub fn markdown_hr(&self) -> Style {
        Style::default().fg(self.dim)
    }

    pub fn markdown_link(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::UNDERLINED)
    }

    /// H1 — Ultraviolet bold.
    pub fn markdown_h1(&self) -> Style {
        Style::default()
            .fg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    /// H2 — Electric Cyan bold.
    pub fn markdown_h2(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    /// H3 — Matrix Green bold.
    pub fn markdown_h3(&self) -> Style {
        Style::default()
            .fg(self.secondary)
            .add_modifier(Modifier::BOLD)
    }

    /// Task list: completed.
    pub fn markdown_task_done(&self) -> Style {
        Style::default()
            .fg(self.success)
            .add_modifier(Modifier::CROSSED_OUT)
    }

    /// Task list: pending.
    pub fn markdown_task_pending(&self) -> Style {
        Style::default().fg(self.dim)
    }

    /// Bold + Italic combined.
    pub fn markdown_bold_italic(&self) -> Style {
        Style::default()
            .fg(self.text)
            .add_modifier(Modifier::BOLD | Modifier::ITALIC)
    }

    // ═══════════════════════════════════════════
    //  Table Styles
    // ═══════════════════════════════════════════

    pub fn table_header(&self) -> Style {
        Style::default()
            .fg(self.table_header)
            .add_modifier(Modifier::BOLD)
    }

    pub fn table_border(&self) -> Style {
        Style::default().fg(self.table_border)
    }

    // ═══════════════════════════════════════════
    //  Flowchart Styles
    // ═══════════════════════════════════════════

    pub fn flowchart_node(&self) -> Style {
        Style::default()
            .fg(self.flowchart_node)
            .add_modifier(Modifier::BOLD)
    }

    pub fn flowchart_arrow(&self) -> Style {
        Style::default().fg(self.dim)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark(false)
    }
}
