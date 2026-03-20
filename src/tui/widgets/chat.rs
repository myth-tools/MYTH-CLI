//! Chat widget — displays agent messages, tool output, and conversation.

use crate::tui::theme::Theme;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

/// A single chat entry.
#[derive(Debug, Clone)]
pub struct ChatEntry {
    pub role: String,
    pub content: String,
    pub timestamp: String,
    /// Cached rendered lines to avoid re-parsing every frame
    pub cached_lines: Vec<ratatui::text::Line<'static>>,
}

/// Chat/output panel widget.
pub struct ChatWidget {
    entries: Vec<ChatEntry>,
    scroll_offset: u16,
}

impl ChatWidget {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            scroll_offset: 0,
        }
    }

    pub fn add_entry(&mut self, role: &str, content: &str) {
        self.entries.push(ChatEntry {
            role: role.to_string(),
            content: content.to_string(),
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
            cached_lines: Vec::new(),
        });
        // Auto-scroll to bottom
        self.scroll_offset = self.entries.len().saturating_sub(1) as u16;
    }

    pub fn append_chunk(&mut self, chunk: &str) {
        if let Some(last) = self.entries.last_mut() {
            last.content.push_str(chunk);
            // Invalidate cache for the last entry so it gets re-rendered
            last.cached_lines.clear();
        } else {
            // Guard: if no message exists, create one
            self.add_entry("system", chunk);
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.scroll_offset = 0;
    }

    pub fn render_lines_count(&self, _theme: &Theme) -> usize {
        // Estimate rendered lines (accounting for wrapping/line breaks)
        let mut total = 0;
        for entry in &self.entries {
            total += 2; // Prefix line + padding
            for content_line in entry.content.lines() {
                total += 1;
                if content_line.starts_with("```") {
                    total += 1; // Block padding
                }
            }
        }
        total
    }

    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(5);
    }

    pub fn scroll_down(&mut self, theme: &Theme) {
        let total = self.render_lines_count(theme);
        self.scroll_offset = self
            .scroll_offset
            .saturating_add(5)
            .min(total.saturating_sub(1) as u16);
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme.active_border_style())
            .title(" 💬 Output ")
            .title_style(theme.title_style());

        let inner = block.inner(area);
        block.render(area, buf);

        // Pre-allocate lines to avoid frequent resizing
        let mut lines: Vec<Line> = Vec::with_capacity(self.entries.len() * 3);

        for entry in &mut self.entries {
            // Use cache if available
            if !entry.cached_lines.is_empty() {
                lines.extend(entry.cached_lines.iter().cloned());
                continue;
            }

            // Otherwise, render and cache
            let (role_style, prefix) = match entry.role.as_str() {
                "agent" | "assistant" => (
                    theme.primary_style().add_modifier(Modifier::BOLD),
                    "⚙  AGENT",
                ),
                "user" => (
                    theme.secondary_style().add_modifier(Modifier::BOLD),
                    "👤  USER",
                ),
                "tool" => (
                    theme.warning_style().add_modifier(Modifier::BOLD),
                    "⚡  ASSET",
                ),
                "system" => (theme.dim_style(), "◈  SYSTEM"),
                "error" => (theme.error_style(), "⚠  CRITICAL"),
                _ => (theme.text_style(), "◾  UNKNOWN"),
            };

            let mut entry_lines = Vec::new();
            entry_lines.push(Line::from(vec![
                Span::styled(format!("[{}] ", entry.timestamp), theme.dim_style()),
                Span::styled(prefix.to_string(), role_style),
            ]));

            let mut in_code_block = false;
            for content_line in entry.content.lines() {
                if content_line.starts_with("```") {
                    in_code_block = !in_code_block;
                    if in_code_block {
                        let lang = content_line.trim_start_matches('`').trim();
                        let mut header_spans = vec![
                            Span::styled("   ", theme.border_style()),
                            Span::styled(
                                " ┌───────────────────────────────────",
                                theme.dim_style(),
                            ),
                        ];
                        if !lang.is_empty() {
                            header_spans.push(Span::styled(
                                format!(" {}", lang),
                                theme.dim_style().add_modifier(Modifier::ITALIC),
                            ));
                        }
                        entry_lines.push(Line::from(header_spans));
                    } else {
                        entry_lines.push(Line::from(vec![
                            Span::styled("   ", theme.border_style()),
                            Span::styled(
                                " └───────────────────────────────────",
                                theme.dim_style(),
                            ),
                        ]));
                    }
                    continue;
                }

                if in_code_block {
                    entry_lines.push(Line::from(vec![
                        Span::styled("   ", theme.border_style()),
                        Span::styled(" │ ", theme.dim_style()),
                        Span::styled(content_line.to_string(), theme.markdown_code()),
                    ]));
                } else {
                    // Detect high-fidelity tables
                    let is_table =
                        content_line.trim().starts_with('|') && content_line.trim().ends_with('|');
                    let is_separator = is_table
                        && content_line
                            .chars()
                            .all(|c| c == '|' || c == '-' || c == ':' || c == ' ');

                    if is_table {
                        let mut spans = vec![Span::styled("   ", theme.border_style())];
                        let parts: Vec<&str> = content_line.split('|').collect();

                        for (i, part) in parts.iter().enumerate() {
                            if i == 0 || i == parts.len() - 1 {
                                continue;
                            }

                            spans.push(Span::styled("│", theme.table_border()));

                            let style = if is_separator {
                                theme.table_border()
                            } else {
                                // Guess header if it's the first table line in this entry
                                if entry_lines
                                    .iter()
                                    .all(|l| !l.spans.iter().any(|s| s.content == "│"))
                                {
                                    theme.table_header()
                                } else {
                                    theme.text_style()
                                }
                            };

                            // Inject spaces to ensure Ratatui Wrap doesn't hang on long unbreakable cells
                            let mut cell_content = part.trim().to_string();
                            if is_separator {
                                // Replace dashes with a space-padded separator to allow wrapping/truncation without panic
                                cell_content = " ──── ".to_string();
                            } else if cell_content.len() > 30 {
                                // Force break long words/URLs in tables so UI doesn't freeze
                                let mut padded = String::new();
                                for (idx, c) in cell_content.chars().enumerate() {
                                    padded.push(c);
                                    if idx > 0 && idx % 30 == 0 {
                                        padded.push(' ');
                                    }
                                }
                                cell_content = padded;
                            }

                            spans.push(Span::styled(format!(" {} ", cell_content.trim()), style));
                        }
                        spans.push(Span::styled("│", theme.table_border()));
                        entry_lines.push(Line::from(spans));
                    } else if content_line.contains("->") || content_line.contains("=>") {
                        // Detect Flowchart HUD Aesthetics
                        let mut spans = vec![Span::styled("   ", theme.border_style())];

                        let mut current = content_line;
                        while let Some(idx) = current.find("->").or_else(|| current.find("=>")) {
                            let node = &current[..idx];
                            let arrow = &current[idx..idx + 2];

                            if !node.trim().is_empty() {
                                spans.push(Span::styled(node.to_string(), theme.flowchart_node()));
                            }
                            spans.push(Span::styled(arrow.to_string(), theme.flowchart_arrow()));
                            current = &current[idx + 2..];
                        }
                        if !current.trim().is_empty() {
                            spans.push(Span::styled(current.to_string(), theme.flowchart_node()));
                        }
                        entry_lines.push(Line::from(spans));
                    } else {
                        entry_lines.push(Self::parse_line_owned(content_line, theme));
                    }
                }
            }
            if in_code_block {
                entry_lines.push(Line::from(vec![
                    Span::styled("   ", theme.border_style()),
                    Span::styled(" └───────────────────────────────────", theme.dim_style()),
                ]));
            }
            entry_lines.push(Line::from(Span::raw("")));

            // Cache the lines
            entry.cached_lines = entry_lines.clone();
            lines.extend(entry_lines);
        }

        let paragraph = Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .scroll((self.scroll_offset, 0));

        paragraph.render(inner, buf);
    }

    /// Internal parser that returns owned lines for caching.
    fn parse_line_owned(text: &str, theme: &Theme) -> Line<'static> {
        let line = Self::parse_line_internal(text, theme);
        // Safety: We ensure all Spans within the line own their strings.
        // Ratatui Spans use Cow, so we ensure Cow::Owned is used.
        let owned_spans: Vec<Span<'static>> = line
            .spans
            .into_iter()
            .map(|s| Span::styled(s.content.into_owned(), s.style))
            .collect();
        Line::from(owned_spans)
    }

    fn parse_line_internal<'a>(text: &'a str, theme: &Theme) -> Line<'a> {
        let mut spans = vec![Span::styled("   ", theme.border_style())];

        // Handle headers with distinct styles
        if let Some(stripped) = text.strip_prefix("# ") {
            spans.push(Span::styled(stripped.to_uppercase(), theme.markdown_h1()));
            return Line::from(spans);
        }
        if let Some(stripped) = text.strip_prefix("## ") {
            spans.push(Span::styled(stripped.to_uppercase(), theme.markdown_h2()));
            return Line::from(spans);
        }
        if let Some(stripped) = text.strip_prefix("### ") {
            spans.push(Span::styled(stripped.to_string(), theme.markdown_h3()));
            return Line::from(spans);
        }

        // Handle completely empty lines early
        if text.trim().is_empty() {
            return Line::from(spans);
        }

        // Handle Horizontal Rules
        if text.trim() == "---" || text.trim() == "***" || text.trim() == "___" {
            spans.push(Span::styled(
                "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
                theme.markdown_hr(),
            ));
            return Line::from(spans);
        }

        // Handle blockquotes
        let mut text_to_parse = text;
        if let Some(stripped) = text.strip_prefix("> ") {
            text_to_parse = stripped;
            spans[0] = Span::styled(" ┃ ", theme.markdown_blockquote());
        }

        // Handle lists
        let text_trim = text_to_parse.trim_start();

        let mut prefix = "";
        let mut task_state: Option<bool> = None; // Some(true)=done, Some(false)=pending

        if let Some(stripped) = text_trim
            .strip_prefix("- [x] ")
            .or_else(|| text_trim.strip_prefix("- [X] "))
        {
            prefix = " ✅ ";
            text_to_parse = stripped;
            task_state = Some(true);
        } else if let Some(stripped) = text_trim.strip_prefix("- [ ] ") {
            prefix = " ☐  ";
            text_to_parse = stripped;
            task_state = Some(false);
        } else if let Some(stripped) = text_trim
            .strip_prefix("- ")
            .or_else(|| text_trim.strip_prefix("* "))
        {
            prefix = " ◈ ";
            text_to_parse = stripped;
        } else if let Some(idx) = text_trim.find(". ") {
            if text_trim[..idx].chars().all(|c| c.is_ascii_digit()) {
                spans.push(Span::styled(
                    format!(" {} ", &text_trim[..=idx]),
                    theme.title_style(),
                ));
                text_to_parse = &text_trim[idx + 2..];
            }
        } else if let Some(idx) = text_trim.find(") ") {
            if text_trim[..idx].chars().all(|c| c.is_ascii_digit()) {
                spans.push(Span::styled(
                    format!(" {}) ", &text_trim[..idx]),
                    theme.title_style(),
                ));
                text_to_parse = &text_trim[idx + 2..];
            }
        }

        if !prefix.is_empty() && spans.len() == 1 {
            let prefix_style = if task_state == Some(true) {
                theme.markdown_task_done()
            } else if task_state == Some(false) {
                theme.markdown_task_pending()
            } else {
                theme.markdown_lead()
            };
            spans.push(Span::styled(prefix.to_string(), prefix_style));
        }

        // State Machine Parsing
        let mut buffer = String::new();
        let mut is_bold = false;
        let mut is_italic = false;
        let mut is_strike = false;
        let mut is_code = false;

        let mut chars = text_to_parse.chars().peekable();

        let flush_buffer = |buf: &mut String,
                            spans: &mut Vec<Span<'a>>,
                            b: bool,
                            i: bool,
                            s: bool,
                            c: bool,
                            t: &Theme| {
            if !buf.is_empty() {
                let mut style = t.text_style();
                if c {
                    style = t.markdown_code();
                } else {
                    if b {
                        style = t.markdown_bold();
                    }
                    if i {
                        style = style.patch(t.markdown_italic());
                    }
                    if s {
                        style = style.patch(t.markdown_strikethrough());
                    }
                    // if it was blockquote line, dim the whole text
                    if text.starts_with("> ") && !b && !c {
                        style = style.patch(t.markdown_blockquote());
                    }
                }
                spans.push(Span::styled(buf.clone(), style));
                buf.clear();
            }
        };

        while let Some(c) = chars.next() {
            match c {
                '`' => {
                    flush_buffer(
                        &mut buffer,
                        &mut spans,
                        is_bold,
                        is_italic,
                        is_strike,
                        is_code,
                        theme,
                    );
                    is_code = !is_code;
                }
                '*' if !is_code => {
                    if chars.peek() == Some(&'*') {
                        chars.next(); // Consume second '*'
                                      // Check for *** (bold+italic)
                        if chars.peek() == Some(&'*') {
                            chars.next(); // Consume third '*'
                            flush_buffer(
                                &mut buffer,
                                &mut spans,
                                is_bold,
                                is_italic,
                                is_strike,
                                is_code,
                                theme,
                            );
                            // Toggle both
                            is_bold = !is_bold;
                            is_italic = !is_italic;
                        } else {
                            flush_buffer(
                                &mut buffer,
                                &mut spans,
                                is_bold,
                                is_italic,
                                is_strike,
                                is_code,
                                theme,
                            );
                            is_bold = !is_bold;
                        }
                    } else {
                        flush_buffer(
                            &mut buffer,
                            &mut spans,
                            is_bold,
                            is_italic,
                            is_strike,
                            is_code,
                            theme,
                        );
                        is_italic = !is_italic;
                    }
                }
                '_' if !is_code => {
                    if chars.peek() == Some(&'_') {
                        chars.next(); // Consume second '_'
                        flush_buffer(
                            &mut buffer,
                            &mut spans,
                            is_bold,
                            is_italic,
                            is_strike,
                            is_code,
                            theme,
                        );
                        is_bold = !is_bold;
                    } else {
                        flush_buffer(
                            &mut buffer,
                            &mut spans,
                            is_bold,
                            is_italic,
                            is_strike,
                            is_code,
                            theme,
                        );
                        is_italic = !is_italic;
                    }
                }
                '~' if !is_code => {
                    if chars.peek() == Some(&'~') {
                        chars.next(); // Consume second '~'
                        flush_buffer(
                            &mut buffer,
                            &mut spans,
                            is_bold,
                            is_italic,
                            is_strike,
                            is_code,
                            theme,
                        );
                        is_strike = !is_strike;
                    } else {
                        buffer.push(c);
                    }
                }
                '[' if !is_code => {
                    // Try to parse [link](url)
                    let remaining: String = chars.clone().collect();
                    if let Some(close_idx) = remaining.find(']') {
                        let after_bracket = &remaining[close_idx + 1..];
                        if after_bracket.starts_with('(') {
                            if let Some(url_close_idx) = after_bracket.find(')') {
                                // Valid link!
                                flush_buffer(
                                    &mut buffer,
                                    &mut spans,
                                    is_bold,
                                    is_italic,
                                    is_strike,
                                    is_code,
                                    theme,
                                );
                                let label = &remaining[..close_idx];
                                spans.push(Span::styled(label.to_string(), theme.markdown_link()));

                                // Advance the main iterator
                                for _ in 0..close_idx + url_close_idx + 2 {
                                    chars.next();
                                }
                                continue;
                            }
                        }
                    }
                    buffer.push(c);
                }
                _ => {
                    buffer.push(c);
                }
            }
        }
        flush_buffer(
            &mut buffer,
            &mut spans,
            is_bold,
            is_italic,
            is_strike,
            is_code,
            theme,
        );

        Line::from(spans)
    }
}

impl Default for ChatWidget {
    fn default() -> Self {
        Self::new()
    }
}
