//! Chat widget — displays agent messages, tool output, and conversation.
//! Performance-hardened with per-entry line caching and viewport-aware rendering.

use crate::tui::theme::Theme;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

/// Maximum number of chat entries before the oldest are drained.
const MAX_ENTRIES: usize = 500;

/// Maximum character length for a single content line before truncation.
const MAX_LINE_DISPLAY_CHARS: usize = 500;

/// Maximum character length for a single entry's content to prevent memory pressure.
const MAX_ENTRY_CONTENT_CHARS: usize = 1_000_000; // 1MB per entry max

/// A single chat entry.
#[derive(Debug, Clone)]
pub struct ChatEntry {
    pub role: String,
    pub content: String,
    pub timestamp: String,
    pub cached_lines: Vec<ratatui::text::Line<'static>>,
    /// Number of characters visible (for typewriter effect)
    pub visible_chars: usize,
    /// Whether the entry is currently "Streaming" its typewriter effect
    pub is_streaming: bool,
}

/// Chat/output panel widget.
pub struct ChatWidget {
    entries: Vec<ChatEntry>,
    pub scroll_offset: usize,
    /// Target scroll offset for interpolation
    pub target_scroll_offset: f64,
    /// Interpolated scroll offset for smooth movement
    pub current_scroll_offset: f64,
    /// When true, scroll is pinned to the bottom (auto-follows new content).
    auto_scroll: bool,
    /// Total rendered line count (cached to avoid recomputation).
    pub total_rendered_lines: usize,
    /// Persistent cache of the last set of visual lines (for hit-testing).
    pub last_visual_lines: Vec<Line<'static>>,
}

impl ChatWidget {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            scroll_offset: 0,
            target_scroll_offset: 0.0,
            current_scroll_offset: 0.0,
            auto_scroll: true,
            total_rendered_lines: 0,
            last_visual_lines: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, role: &str, content: &str) {
        self.entries.push(ChatEntry {
            role: role.to_string(),
            content: content.to_string(),
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
            cached_lines: Vec::new(),
            visible_chars: 0,
            is_streaming: true,
        });
        // Enforce max entry cap
        if self.entries.len() > MAX_ENTRIES {
            let drain_count = self.entries.len() - MAX_ENTRIES;
            self.entries.drain(..drain_count);
            self.total_rendered_lines = 0;
        }
        if self.auto_scroll {
            self.pin_scroll_to_bottom();
        }
    }

    pub fn append_chunk(&mut self, chunk: &str) {
        if let Some(last) = self.entries.last_mut() {
            last.content.push_str(chunk);
            last.is_streaming = true; // Keep streaming while content is arriving
                                      // Invalidate only this entry's cache
            let old_line_count = last.cached_lines.len();
            last.cached_lines.clear();
            // Adjust total line count (subtract old, will be re-added on next render)
            self.total_rendered_lines = self.total_rendered_lines.saturating_sub(old_line_count);
        } else {
            self.add_entry("system", chunk);
        }
        if self.auto_scroll {
            self.pin_scroll_to_bottom();
        }
    }

    pub fn append_tool_stream(&mut self, _tool: &str, line: &str) {
        let is_tool = self
            .entries
            .last()
            .map(|e| e.role == "tool")
            .unwrap_or(false);
        if is_tool {
            if let Some(last) = self.entries.last_mut() {
                // Safety check: Avoid unbounded memory growth for massive tool outputs
                if last.content.len() > MAX_ENTRY_CONTENT_CHARS {
                    // Prepend a truncation notice and keep only the tail
                    let keep_len = MAX_ENTRY_CONTENT_CHARS / 2;
                    let tail = last.content[last.content.len() - keep_len..].to_string();
                    last.content = format!("... [OUTPUT TRUNCATED FOR PERFORMANCE] ...\n{}", tail);
                }

                last.content.push('\n');
                last.content.push_str(line);
                last.is_streaming = true;
                let old_line_count = last.cached_lines.len();
                last.cached_lines.clear();
                self.total_rendered_lines =
                    self.total_rendered_lines.saturating_sub(old_line_count);
            }
        } else {
            self.add_entry("tool", line);
        }
        if self.auto_scroll {
            self.pin_scroll_to_bottom();
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.scroll_offset = 0;
        self.auto_scroll = true;
        self.total_rendered_lines = 0;
    }

    /// Compute total rendered line count (uses cache when available).
    fn compute_total_lines(&self) -> usize {
        let mut total = 0;
        for entry in &self.entries {
            if !entry.cached_lines.is_empty() {
                total += entry.cached_lines.len();
            } else {
                // Estimate: prefix + content lines + separator
                total += 2;
                for content_line in entry.content.lines() {
                    total += 1;
                    if content_line.starts_with("```") {
                        total += 1;
                    }
                }
            }
        }
        total
    }

    pub fn scroll_up_by(&mut self, amount: usize) {
        self.target_scroll_offset = (self.target_scroll_offset - amount as f64).max(0.0);
        self.auto_scroll = false;
    }

    pub fn scroll_down_by(&mut self, amount: usize, viewport_height: usize) {
        let total = if self.total_rendered_lines > 0 {
            self.total_rendered_lines
        } else {
            self.compute_total_lines()
        };
        let max_scroll = total.saturating_sub(viewport_height);
        self.target_scroll_offset =
            (self.target_scroll_offset + amount as f64).min(max_scroll as f64);
        if self.target_scroll_offset >= max_scroll as f64 {
            self.auto_scroll = true;
        }
    }

    pub fn scroll_up(&mut self) {
        self.scroll_up_by(5);
    }

    pub fn scroll_down(&mut self, _theme: &Theme, viewport_height: usize) {
        self.scroll_down_by(5, viewport_height);
    }

    pub fn scroll_page_up(&mut self, viewport_height: usize) {
        self.scroll_up_by(viewport_height.saturating_sub(2));
    }

    pub fn scroll_page_down(&mut self, viewport_height: usize) {
        self.scroll_down_by(viewport_height.saturating_sub(2), viewport_height);
    }

    pub fn jump_to_bottom(&mut self) {
        self.scroll_offset = self.total_rendered_lines; // Will be capped in render
        self.auto_scroll = true;
    }

    /// Update scroll offset based on scrollbar drag ratio (0.0 to 1.0)
    pub fn handle_scrollbar_drag(&mut self, ratio: f64) {
        let max_scroll = self.total_rendered_lines; // render() will handle the viewport_h subtraction
        self.scroll_offset = (ratio * max_scroll as f64) as usize;
        self.auto_scroll = false;
    }

    fn pin_scroll_to_bottom(&mut self) {
        let total = if self.total_rendered_lines > 0 {
            self.total_rendered_lines
        } else {
            self.compute_total_lines()
        };
        self.target_scroll_offset = total.saturating_sub(1) as f64;
    }

    /// Get the total line count for external viewport calculations.
    pub fn viewport_height_hint(&self) -> usize {
        if self.total_rendered_lines > 0 {
            self.total_rendered_lines
        } else {
            self.compute_total_lines()
        }
    }

    pub fn render(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        theme: &Theme,
        focused: bool,
        search_query: Option<&str>,
    ) {
        // ─── Scroll position indicator ───
        let total = if self.total_rendered_lines > 0 {
            self.total_rendered_lines
        } else {
            self.compute_total_lines()
        };

        let scroll_indicator = if total > 0 {
            let viewport_h = area.height.saturating_sub(2) as usize;
            let visible_end = (self.scroll_offset + viewport_h).min(total);
            format!(" [{}/{}] ", visible_end, total)
        } else {
            String::new()
        };

        // Tactical Title Elevation
        let title_text = if focused {
            " ▶ ACTIVE :: TACTICAL_INTEL ".to_string()
        } else {
            " ◈ TACTICAL_INTEL ".to_string()
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
            .title(Span::styled(title_text, title_style))
            .title_bottom(
                Line::from(vec![Span::styled(scroll_indicator, theme.dim_style())])
                    .alignment(Alignment::Right),
            );

        let inner = block.inner(area);
        block.render(area, buf);

        // ─── Visual Scrollbar ───
        if total > inner.height as usize {
            let scrollbar_area = Rect {
                x: area.right().saturating_sub(1),
                y: area.y + 1,
                width: 1,
                height: area.height.saturating_sub(2),
            };

            let total_f = total as f32;
            let v_h_f = inner.height as f32;
            let offset_f = self.scroll_offset as f32;

            let bar_height = ((v_h_f / total_f) * v_h_f).max(1.0) as u16;
            let bar_pos = ((offset_f / total_f) * v_h_f) as u16;

            let sb_style = theme.scrollbar_style(focused);
            for i in 0..scrollbar_area.height {
                let symbol = if i >= bar_pos && i < bar_pos + bar_height {
                    "┃"
                } else {
                    "│"
                };
                if let Some(cell) = buf.cell_mut((scrollbar_area.x, scrollbar_area.y + i)) {
                    cell.set_symbol(symbol).set_style(sb_style);
                }
            }
        }

        // ─── Build Lines ───
        let mut lines: Vec<Line> = Vec::with_capacity(self.entries.len() * 3);

        for entry in &mut self.entries {
            // Use cache if available, but skip if currently streaming (typewriter active)
            if !entry.cached_lines.is_empty() && !entry.is_streaming {
                lines.extend(entry.cached_lines.iter().cloned());
                continue;
            }

            // Render and cache
            let content_display = if entry.is_streaming {
                let limit = entry.visible_chars.min(entry.content.len());
                // Safe boundary check
                let mut safe_limit = limit;
                while safe_limit > 0 && !entry.content.is_char_boundary(safe_limit) {
                    safe_limit -= 1;
                }
                &entry.content[..safe_limit]
            } else {
                &entry.content
            };
            // Render and cache
            let (role_style, prefix, align) = match entry.role.as_str() {
                "agent" | "assistant" => (
                    Style::default()
                        .fg(theme.primary)
                        .add_modifier(Modifier::BOLD),
                    "⬡ MYTH",
                    Alignment::Left,
                ),
                "user" => (theme.tactical_style(), "▸ YOU", Alignment::Right),
                "tool" => (
                    Style::default()
                        .fg(theme.warning)
                        .add_modifier(Modifier::BOLD),
                    "⚡ EXEC",
                    Alignment::Left,
                ),
                "system" => (
                    Style::default()
                        .fg(theme.surface_bright)
                        .add_modifier(Modifier::BOLD),
                    "◈ SYS",
                    Alignment::Left,
                ),
                "error" => (
                    Style::default()
                        .fg(theme.error)
                        .add_modifier(Modifier::BOLD),
                    "✗ FAIL",
                    Alignment::Left,
                ),
                _ => (Style::default().fg(theme.text), "◾ ???", Alignment::Left),
            };

            let mut entry_lines = Vec::new();

            // Role badge + timestamp
            let mut header_spans = vec![];
            let timestamp_str = entry.timestamp.clone();

            if align == Alignment::Right {
                header_spans.push(Span::styled("─── ", theme.border_style()));
                header_spans.push(Span::styled(prefix.to_string(), role_style));
                header_spans.push(Span::styled(" ──[ ", theme.dim_style()));
                header_spans.push(Span::styled(
                    timestamp_str,
                    Style::default().fg(theme.text).add_modifier(Modifier::DIM),
                ));
                header_spans.push(Span::styled(" ]──╮", theme.dim_style()));
            } else {
                header_spans.push(Span::styled("╭──[ ", theme.dim_style()));
                header_spans.push(Span::styled(
                    timestamp_str,
                    Style::default().fg(theme.text).add_modifier(Modifier::DIM),
                ));
                header_spans.push(Span::styled(" ]── ", theme.dim_style()));
                header_spans.push(Span::styled(prefix.to_string(), role_style));
                header_spans.push(Span::styled(" ───", theme.border_style()));
            }

            entry_lines.push(Line::from(header_spans).alignment(align));

            let mut in_code_block = false;
            for content_line in content_display.lines() {
                let safe_slice = if content_line.len() > MAX_LINE_DISPLAY_CHARS {
                    let mut safe_idx = MAX_LINE_DISPLAY_CHARS;
                    while safe_idx > 0 && !content_line.is_char_boundary(safe_idx) {
                        safe_idx -= 1;
                    }
                    &content_line[..safe_idx]
                } else {
                    content_line
                };
                let display_line = safe_slice;

                if display_line.starts_with("```") {
                    in_code_block = !in_code_block;
                    if in_code_block {
                        let lang = display_line.trim_start_matches('`').trim();
                        let mut header_spans = vec![Span::styled(
                            if align == Alignment::Right {
                                "     "
                            } else {
                                " ┌─"
                            },
                            Style::default().fg(theme.dim),
                        )];
                        if !lang.is_empty() {
                            header_spans.push(Span::styled(
                                format!("── {} ", lang),
                                Style::default()
                                    .fg(theme.dim)
                                    .add_modifier(Modifier::ITALIC),
                            ));
                        }
                        if align != Alignment::Right {
                            header_spans
                                .push(Span::styled("─".repeat(20), Style::default().fg(theme.dim)));
                        }
                        entry_lines.push(Line::from(header_spans).alignment(align));
                    } else {
                        entry_lines.push(
                            Line::from(vec![Span::styled(
                                if align == Alignment::Right {
                                    " "
                                } else {
                                    " └─────────────────────────────────"
                                },
                                Style::default().fg(theme.dim),
                            )])
                            .alignment(align),
                        );
                    }
                    continue;
                }

                if in_code_block {
                    entry_lines.push(
                        Line::from(vec![
                            Span::styled(
                                if align == Alignment::Right {
                                    "   "
                                } else {
                                    " │ "
                                },
                                Style::default().fg(theme.dim),
                            ),
                            Span::styled(display_line.to_string(), theme.markdown_code()),
                        ])
                        .alignment(align),
                    );
                } else {
                    // Detect tables
                    let pipe_count = display_line.matches('|').count();
                    let space_gap_count =
                        display_line.split("  ").filter(|s| !s.is_empty()).count();
                    let is_table =
                        pipe_count >= 2 || (space_gap_count >= 2 && display_line.contains("  "));
                    let is_separator = pipe_count >= 2
                        && display_line
                            .chars()
                            .all(|c| c == '|' || c == '-' || c == ':' || c == ' ');

                    if is_table {
                        let mut spans = vec![Span::styled("   ", theme.border_style())];
                        let parts: Vec<&str> = display_line.split('|').collect();

                        for (i, part) in parts.iter().enumerate() {
                            if i == 0 || i == parts.len() - 1 {
                                continue;
                            }

                            spans.push(Span::styled("│", theme.table_border()));

                            let style = if is_separator {
                                theme.table_border()
                            } else if entry_lines
                                .iter()
                                .all(|l| !l.spans.iter().any(|s| s.content == "│"))
                            {
                                theme.table_header()
                            } else {
                                theme.text_style()
                            };

                            let mut cell_content = part.trim().to_string();
                            if is_separator {
                                cell_content = " ──── ".to_string();
                            } else if cell_content.len() > 30 {
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
                        entry_lines.push(Line::from(spans).alignment(align));
                    } else if display_line.contains("->") || display_line.contains("=>") {
                        // Flowchart aesthetics
                        let mut spans = vec![Span::styled("   ", theme.border_style())];

                        let mut current = display_line;
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
                        entry_lines.push(Line::from(spans).alignment(align));
                    } else {
                        entry_lines.push(Self::parse_line_owned(display_line, theme, align));
                    }
                }
            }

            // ─── TABLE RE-PROCESSING PASS ───
            let mut final_entry_lines = Vec::new();
            let mut i = 0;
            while i < entry_lines.len() {
                let current_line = &entry_lines[i];
                let is_tb_pipe = current_line.spans.iter().any(|s| s.content == "│");
                let line_text: String = current_line
                    .spans
                    .iter()
                    .map(|s| s.content.as_ref())
                    .collect();
                let is_tb_heuristic = line_text.contains("  ")
                    && line_text
                        .split("  ")
                        .filter(|s| !s.trim().is_empty())
                        .count()
                        >= 2;

                if is_tb_pipe || is_tb_heuristic {
                    let mut table_block = Vec::new();
                    let block_align = current_line.alignment.unwrap_or(Alignment::Left);
                    while i < entry_lines.len() {
                        let line = &entry_lines[i];
                        let l_text: String =
                            line.spans.iter().map(|s| s.content.as_ref()).collect();
                        let l_pipe = line.spans.iter().any(|s| s.content == "│");
                        let l_heuristic = l_text.contains("  ")
                            && l_text.split("  ").filter(|s| !s.trim().is_empty()).count() >= 2;

                        if l_pipe || l_heuristic {
                            let cells: Vec<String> = if l_pipe {
                                line.spans
                                    .iter()
                                    .filter(|s| {
                                        s.style != theme.table_border()
                                            && s.content != "   "
                                            && s.content != "│"
                                    })
                                    .map(|s| s.content.trim().to_string())
                                    .collect()
                            } else {
                                l_text
                                    .split("  ")
                                    .filter(|s| !s.trim().is_empty())
                                    .map(|s| s.trim().to_string())
                                    .collect()
                            };

                            if !cells.is_empty() && !cells.iter().all(|c| c == "────") {
                                table_block.push(cells);
                            }
                            i += 1;
                        } else {
                            break;
                        }
                    }

                    if !table_block.is_empty() {
                        Self::render_table_block(
                            &table_block,
                            &mut final_entry_lines,
                            theme,
                            block_align,
                        );
                    }
                } else {
                    final_entry_lines.push(current_line.clone());
                    i += 1;
                }
            }

            entry_lines = final_entry_lines;

            if in_code_block {
                entry_lines.push(Line::from(vec![
                    Span::styled("   ", theme.border_style()),
                    Span::styled(
                        " └─────────────────────────────────",
                        Style::default().fg(theme.dim),
                    ),
                ]));
            }

            // Cache ONLY if finished streaming to avoid redundant re-renders next frame
            if !entry.is_streaming {
                entry.cached_lines = entry_lines.clone();
            }
            lines.extend(entry_lines);
        }

        // Update total rendered line count
        let mut exact_visual_lines: Vec<Line> = Vec::new();
        let base_wrap_width = (inner.width as usize).saturating_sub(1); // Leave tiny margin

        if base_wrap_width > 0 {
            for logical_line in lines {
                let align = logical_line.alignment.unwrap_or(Alignment::Left);
                let wrap_width = if align == Alignment::Right {
                    (inner.width as usize).saturating_sub(4)
                } else {
                    (inner.width as usize).saturating_sub(1)
                };

                let mut current_line_spans = Vec::new();
                let mut current_width = 0;

                for span in logical_line.spans {
                    let mut content: &str = span.content.as_ref();
                    while !content.is_empty() {
                        let remaining_space = wrap_width.saturating_sub(current_width);
                        if remaining_space == 0 {
                            if align == Alignment::Right {
                                current_line_spans.push(Span::styled("   ", Style::default()));
                            }
                            exact_visual_lines
                                .push(Line::from(current_line_spans).alignment(align));
                            current_line_spans = Vec::new();
                            current_width = 0;
                            continue;
                        }

                        let char_count = content.chars().count();
                        if char_count <= remaining_space {
                            current_line_spans.push(Span::styled(content.to_string(), span.style));
                            current_width += char_count;
                            break;
                        }

                        // Text exceeds remaining space. Find best break point.
                        let limit_byte_idx = content
                            .char_indices()
                            .nth(remaining_space)
                            .map(|(i, _)| i)
                            .unwrap_or(content.len());
                        let chunk = &content[..limit_byte_idx];

                        // Look for last space to wrap cleanly
                        if let Some(last_space_byte_idx) = chunk.rfind(' ') {
                            // Break at space
                            let (head, tail) = content.split_at(last_space_byte_idx);
                            current_line_spans.push(Span::styled(head.to_string(), span.style));
                            if align == Alignment::Right {
                                current_line_spans.push(Span::styled("   ", Style::default()));
                            }
                            exact_visual_lines
                                .push(Line::from(current_line_spans).alignment(align));
                            current_line_spans = Vec::new();
                            current_width = 0;

                            // To prevent trailing spaces from breaking the next line, we skip the exact space we broke on
                            content = &tail[1..];
                        } else if current_width > 0 {
                            // No space in current chunk, but we have text already.
                            // Move the whole word-start to the next line.
                            if align == Alignment::Right {
                                current_line_spans.push(Span::styled("   ", Style::default()));
                            }
                            exact_visual_lines
                                .push(Line::from(current_line_spans).alignment(align));
                            current_line_spans = Vec::new();
                            current_width = 0;
                            // content stays same, will be processed on fresh line
                        } else {
                            // Word itself is wider than terminal. Force break it.
                            let (head, tail) = content.split_at(limit_byte_idx);
                            current_line_spans.push(Span::styled(head.to_string(), span.style));
                            if align == Alignment::Right {
                                current_line_spans.push(Span::styled("   ", Style::default()));
                            }
                            exact_visual_lines
                                .push(Line::from(current_line_spans).alignment(align));
                            current_line_spans = Vec::new();
                            current_width = 0;
                            content = tail;
                        }
                    }
                }
                if !current_line_spans.is_empty() {
                    if align == Alignment::Right {
                        current_line_spans.push(Span::styled("   ", Style::default()));
                    }
                    exact_visual_lines.push(Line::from(current_line_spans).alignment(align));
                }
            }
        } else {
            exact_visual_lines = lines;
        }

        let mut final_visual_lines: Vec<Line> = Vec::with_capacity(exact_visual_lines.len());
        for line in exact_visual_lines {
            if let Some(query) = search_query {
                if !query.is_empty() {
                    let mut new_spans = Vec::new();
                    for span in line.spans {
                        if span.content.contains(query) {
                            let mut current = span.content.as_ref();
                            while let Some(idx) = current.find(query) {
                                if idx > 0 {
                                    new_spans
                                        .push(Span::styled(current[..idx].to_string(), span.style));
                                }
                                new_spans.push(Span::styled(
                                    query.to_string(),
                                    theme.warning_style().add_modifier(Modifier::REVERSED),
                                ));
                                current = &current[idx + query.len()..];
                            }
                            if !current.is_empty() {
                                new_spans.push(Span::styled(current.to_string(), span.style));
                            }
                        } else {
                            new_spans.push(span);
                        }
                    }
                    let align = line.alignment.unwrap_or(Alignment::Left);
                    final_visual_lines.push(Line::from(new_spans).alignment(align));
                    continue;
                }
            }
            final_visual_lines.push(line);
        }

        self.total_rendered_lines = final_visual_lines.len();
        self.last_visual_lines = final_visual_lines.clone();

        // ─── Neural Link: Smooth Interpolation ───
        let diff = self.target_scroll_offset - self.current_scroll_offset;
        if diff.abs() > 0.1 {
            // "Industry-Grade" Lerp: Approach target by 25% each frame for fluid 60FPS feel
            self.current_scroll_offset += diff * 0.25;
            self.scroll_offset = self.current_scroll_offset.round() as usize;
        } else {
            self.current_scroll_offset = self.target_scroll_offset;
            self.scroll_offset = self.target_scroll_offset as usize;
        }

        if self.auto_scroll {
            let viewport_h = inner.height as usize;
            let max_scroll = self.total_rendered_lines.saturating_sub(viewport_h);
            self.target_scroll_offset = max_scroll as f64;
        } else {
            let viewport_h = inner.height as usize;
            let max_scroll = self.total_rendered_lines.saturating_sub(viewport_h);
            self.target_scroll_offset = self.target_scroll_offset.clamp(0.0, max_scroll as f64);
        }

        let paragraph = Paragraph::new(final_visual_lines)
            .scroll((self.scroll_offset.min(u16::MAX as usize) as u16, 0));

        paragraph.render(inner, buf);
    }

    /// Advance visual states (Lerp scroll, Typewriter increments)
    pub fn tick(&mut self, is_thinking: bool) -> bool {
        let mut progress_made = false;

        // 1. Advance Typewriter for the latest message
        if let Some(last) = self.entries.last_mut() {
            if last.is_streaming {
                // atmospheric speed: lightning fast for ultimate robustness
                let speed = if is_thinking { 25 } else { 300 };
                last.visible_chars += speed;
                if last.visible_chars >= last.content.len() {
                    last.is_streaming = false;
                }
                progress_made = true;
            }
        }

        // 2. Linear Interpolation for Scroll
        let diff = self.target_scroll_offset - self.current_scroll_offset;
        if diff.abs() > 0.05 {
            self.current_scroll_offset += diff * 0.15;
            self.scroll_offset = self.current_scroll_offset.round() as usize;
            progress_made = true;
        } else {
            self.current_scroll_offset = self.target_scroll_offset;
            self.scroll_offset = self.target_scroll_offset as usize;
        }

        progress_made
    }

    /// Detect if a click hits an interactive command.
    pub fn get_command_at(&self, rel_col: u16, rel_row: u16, inner_area: Rect) -> Option<String> {
        let viewport_row = rel_row as usize;
        let actual_line_idx = self.scroll_offset + viewport_row;

        if let Some(line) = self.last_visual_lines.get(actual_line_idx) {
            let mut current_x = 0;
            // Handle alignment offsetting
            let line_width: usize = line.spans.iter().map(|s| s.content.chars().count()).sum();
            if line.alignment == Some(Alignment::Right) {
                current_x = inner_area.width.saturating_sub(line_width as u16);
            }

            for span in &line.spans {
                let span_len = span.content.chars().count() as u16;
                if rel_col >= current_x && rel_col < current_x + span_len {
                    let text = span.content.trim();
                    // Detect if it looks like a command
                    if (text.starts_with('/') && text.len() > 1 && !text.contains(' '))
                        || (text.starts_with('`') && text.ends_with('`') && text.len() > 2)
                    {
                        return Some(text.trim_matches('`').to_string());
                    }
                }
                current_x += span_len;
            }
        }
        None
    }

    /// Render a collected table block with proper alignment and box borders.
    fn render_table_block(
        table_block: &[Vec<String>],
        output: &mut Vec<Line<'static>>,
        theme: &Theme,
        align: Alignment,
    ) {
        // Compute column widths
        let mut widths = Vec::new();
        for row in table_block {
            for (idx, cell) in row.iter().enumerate() {
                let len = cell.chars().count().min(40);
                if idx >= widths.len() {
                    widths.push(len);
                } else if len > widths[idx] {
                    widths[idx] = len;
                }
            }
        }

        // Top border
        let mut top = vec![
            Span::styled(
                if align == Alignment::Right {
                    "     "
                } else {
                    "   "
                },
                Style::default(),
            ),
            Span::styled("┌─", theme.table_border()),
        ];
        for (idx, w) in widths.iter().enumerate() {
            top.push(Span::styled("─".repeat(*w), theme.table_border()));
            if idx < widths.len() - 1 {
                top.push(Span::styled("─┬─", theme.table_border()));
            }
        }
        top.push(Span::styled("─┐", theme.table_border()));
        output.push(Line::from(top).alignment(align));

        for (r_idx, row) in table_block.iter().enumerate() {
            // Wrap cells
            let mut wrapped_cells: Vec<Vec<String>> = Vec::new();
            let mut max_lines = 1;
            for (idx, w) in widths.iter().enumerate() {
                let cell_text = row.get(idx).cloned().unwrap_or_default();
                let mut l_lines = Vec::new();
                let mut chars = cell_text.chars();
                loop {
                    let chunk: String = chars.by_ref().take(*w).collect();
                    if chunk.is_empty() {
                        break;
                    }
                    l_lines.push(chunk);
                }
                if l_lines.is_empty() {
                    l_lines.push(String::new());
                }
                if l_lines.len() > max_lines {
                    max_lines = l_lines.len();
                }
                wrapped_cells.push(l_lines);
            }

            // Render row sub-lines
            for line_idx in 0..max_lines {
                let mut row_spans = vec![
                    Span::styled(
                        if align == Alignment::Right {
                            "     "
                        } else {
                            "   "
                        },
                        Style::default(),
                    ),
                    Span::styled("│ ", theme.table_border()),
                ];
                for (idx, w) in widths.iter().enumerate() {
                    let content = wrapped_cells[idx]
                        .get(line_idx)
                        .cloned()
                        .unwrap_or_default();
                    let pad = w.saturating_sub(content.chars().count());
                    let style = if r_idx == 0 {
                        theme.table_header()
                    } else if r_idx % 2 == 0 {
                        theme.dim_style()
                    } else {
                        theme.text_style()
                    };
                    row_spans.push(Span::styled(content, style));
                    row_spans.push(Span::styled(" ".repeat(pad), style));
                    if idx < widths.len() - 1 {
                        row_spans.push(Span::styled(" │ ", theme.table_border()));
                    }
                }
                row_spans.push(Span::styled(" │", theme.table_border()));
                output.push(Line::from(row_spans).alignment(align));
            }

            // Header separator
            if r_idx == 0 {
                let mut sep = vec![
                    Span::styled(
                        if align == Alignment::Right {
                            "     "
                        } else {
                            "   "
                        },
                        Style::default(),
                    ),
                    Span::styled("├─", theme.table_border()),
                ];
                for (idx, w) in widths.iter().enumerate() {
                    sep.push(Span::styled("─".repeat(*w), theme.table_border()));
                    if idx < widths.len() - 1 {
                        sep.push(Span::styled("─┼─", theme.table_border()));
                    }
                }
                sep.push(Span::styled("─┤", theme.table_border()));
                output.push(Line::from(sep).alignment(align));
            }
        }

        // Bottom border
        let mut bot = vec![
            Span::styled(
                if align == Alignment::Right {
                    "     "
                } else {
                    "   "
                },
                Style::default(),
            ),
            Span::styled("└─", theme.table_border()),
        ];
        for (idx, w) in widths.iter().enumerate() {
            bot.push(Span::styled("─".repeat(*w), theme.table_border()));
            if idx < widths.len() - 1 {
                bot.push(Span::styled("─┴─", theme.table_border()));
            }
        }
        bot.push(Span::styled("─┘", theme.table_border()));
        output.push(Line::from(bot).alignment(align));
    }
    fn parse_line_owned(text: &str, theme: &Theme, align: Alignment) -> Line<'static> {
        // Apply dynamic unicode font mapping
        let mapped_text = crate::ui::apply_font(text, &theme.font, theme.simulated_mode);

        let line = Self::parse_line_internal(&mapped_text, theme, align);
        let owned_spans: Vec<Span<'static>> = line
            .spans
            .into_iter()
            .map(|s| Span::styled(s.content.into_owned(), s.style))
            .collect();
        Line::from(owned_spans).alignment(align)
    }

    fn parse_line_internal<'a>(text: &'a str, theme: &Theme, align: Alignment) -> Line<'a> {
        let mut spans = vec![Span::styled(
            if align == Alignment::Right {
                "     "
            } else {
                "   "
            },
            theme.border_style(),
        )];

        // Headers
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

        // Empty lines
        if text.trim().is_empty() {
            return Line::from(spans);
        }

        // Interactive prompts
        if let Some(stripped) = text.trim_start().strip_prefix("[?] ") {
            spans.push(Span::styled(
                " ❓ ",
                theme.warning_style().add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled(stripped.to_string(), theme.warning_style()));
            return Line::from(spans);
        }

        // One-liner summaries
        if let Some(stripped) = text.trim_start().strip_prefix("(⚡) ") {
            spans.push(Span::styled(
                " ⚡ ",
                theme.primary_style().add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled(stripped.to_string(), theme.markdown_bold()));
            return Line::from(spans);
        }

        // Horizontal rules
        if text.trim() == "---" || text.trim() == "***" || text.trim() == "___" {
            spans.push(Span::styled(
                "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
                theme.markdown_hr(),
            ));
            return Line::from(spans);
        }

        // Blockquotes (recursive/nested)
        let mut text_to_parse = text;
        let mut bq_level = 0;
        let mut bq_work = text;
        while bq_work.starts_with('>') {
            bq_level += 1;
            bq_work = &bq_work[1..];
            if bq_work.starts_with(' ') {
                bq_work = &bq_work[1..];
                break;
            }
        }
        if bq_level > 0 {
            text_to_parse = bq_work;
            spans[0] = Span::styled(" ┃ ".repeat(bq_level), theme.markdown_blockquote());
        }

        // Lists
        let text_trim = text_to_parse.trim_start();
        let mut prefix = "";
        let mut task_state: Option<bool> = None;

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
        } else if let Some(stripped) = text_trim.strip_prefix("[^") {
            if let Some(idx) = stripped.find("]: ") {
                let marker = &stripped[..idx];
                spans.push(Span::styled(
                    format!(" [^{}]: ", marker),
                    theme.dim_style().add_modifier(Modifier::BOLD),
                ));
                text_to_parse = &stripped[idx + 3..];
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

        // ─── Inline Formatting State Machine ───
        let mut buffer = String::new();
        let mut is_bold = false;
        let mut is_italic = false;
        let mut is_strike = false;
        let mut is_code = false;
        let mut is_highlight = false;
        let mut is_sub = false;
        let mut is_sup = false;
        let mut is_underline = false;
        let mut is_insert = false;
        let mut is_comment = false;
        let is_footnote = false;
        let mut escaped = false;

        let mut chars = text_to_parse.chars().peekable();

        let flush_buffer = |buf: &mut String,
                            spans: &mut Vec<Span<'a>>,
                            b: bool,
                            i: bool,
                            s: bool,
                            c: bool,
                            h: bool,
                            sub: bool,
                            sup: bool,
                            u: bool,
                            ins: bool,
                            comm: bool,
                            ft: bool,
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
                    if h {
                        style = style.patch(t.markdown_highlight());
                    }
                    if sub || sup {
                        style = style.patch(t.dim_style());
                    }
                    if ft {
                        style = style.patch(t.dim_style().add_modifier(Modifier::ITALIC));
                    }
                    if u || ins {
                        style = style.patch(Style::default().add_modifier(Modifier::UNDERLINED));
                    }
                    if comm {
                        style = style.patch(t.dim_style().add_modifier(Modifier::ITALIC));
                    }
                    if text.starts_with("> ") && !b && !c {
                        style = style.patch(t.markdown_blockquote());
                    }
                }
                spans.push(Span::styled(buf.clone(), style));
                buf.clear();
            }
        };

        while let Some(c) = chars.next() {
            if escaped {
                buffer.push(c);
                escaped = false;
                continue;
            }

            match c {
                '\\' if !is_code => {
                    escaped = true;
                }
                '`' => {
                    flush_buffer(
                        &mut buffer,
                        &mut spans,
                        is_bold,
                        is_italic,
                        is_strike,
                        is_code,
                        is_highlight,
                        is_sub,
                        is_sup,
                        is_underline,
                        is_insert,
                        is_comment,
                        is_footnote,
                        theme,
                    );
                    is_code = !is_code;
                }
                '*' if !is_code => {
                    if chars.peek() == Some(&'*') {
                        chars.next();
                        if chars.peek() == Some(&'*') {
                            chars.next();
                            flush_buffer(
                                &mut buffer,
                                &mut spans,
                                is_bold,
                                is_italic,
                                is_strike,
                                is_code,
                                is_highlight,
                                is_sub,
                                is_sup,
                                is_underline,
                                is_insert,
                                is_comment,
                                is_footnote,
                                theme,
                            );
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
                                is_highlight,
                                is_sub,
                                is_sup,
                                is_underline,
                                is_insert,
                                is_comment,
                                is_footnote,
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
                            is_highlight,
                            is_sub,
                            is_sup,
                            is_underline,
                            is_insert,
                            is_comment,
                            is_footnote,
                            theme,
                        );
                        is_italic = !is_italic;
                    }
                }
                '_' if !is_code => {
                    if chars.peek() == Some(&'_') {
                        chars.next();
                        flush_buffer(
                            &mut buffer,
                            &mut spans,
                            is_bold,
                            is_italic,
                            is_strike,
                            is_code,
                            is_highlight,
                            is_sub,
                            is_sup,
                            is_underline,
                            is_insert,
                            is_comment,
                            is_footnote,
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
                            is_highlight,
                            is_sub,
                            is_sup,
                            is_underline,
                            is_insert,
                            is_comment,
                            is_footnote,
                            theme,
                        );
                        is_italic = !is_italic;
                    }
                }
                '~' if !is_code => {
                    if chars.peek() == Some(&'~') {
                        chars.next();
                        flush_buffer(
                            &mut buffer,
                            &mut spans,
                            is_bold,
                            is_italic,
                            is_strike,
                            is_code,
                            is_highlight,
                            is_sub,
                            is_sup,
                            is_underline,
                            is_insert,
                            is_comment,
                            is_footnote,
                            theme,
                        );
                        is_strike = !is_strike;
                    } else {
                        flush_buffer(
                            &mut buffer,
                            &mut spans,
                            is_bold,
                            is_italic,
                            is_strike,
                            is_code,
                            is_highlight,
                            is_sub,
                            is_sup,
                            is_underline,
                            is_insert,
                            is_comment,
                            is_footnote,
                            theme,
                        );
                        is_sub = !is_sub;
                    }
                }
                '^' if !is_code => {
                    if chars.peek() == Some(&'^') {
                        chars.next();
                        flush_buffer(
                            &mut buffer,
                            &mut spans,
                            is_bold,
                            is_italic,
                            is_strike,
                            is_code,
                            is_highlight,
                            is_sub,
                            is_sup,
                            is_underline,
                            is_insert,
                            is_comment,
                            is_footnote,
                            theme,
                        );
                        is_underline = !is_underline;
                    } else {
                        flush_buffer(
                            &mut buffer,
                            &mut spans,
                            is_bold,
                            is_italic,
                            is_strike,
                            is_code,
                            is_highlight,
                            is_sub,
                            is_sup,
                            is_underline,
                            is_insert,
                            is_comment,
                            is_footnote,
                            theme,
                        );
                        is_sup = !is_sup;
                    }
                }
                '=' if !is_code => {
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        flush_buffer(
                            &mut buffer,
                            &mut spans,
                            is_bold,
                            is_italic,
                            is_strike,
                            is_code,
                            is_highlight,
                            is_sub,
                            is_sup,
                            is_underline,
                            is_insert,
                            is_comment,
                            is_footnote,
                            theme,
                        );
                        is_highlight = !is_highlight;
                    } else {
                        buffer.push(c);
                    }
                }
                '!' if !is_code => {
                    if chars.peek() == Some(&'!') {
                        chars.next();
                        flush_buffer(
                            &mut buffer,
                            &mut spans,
                            is_bold,
                            is_italic,
                            is_strike,
                            is_code,
                            is_highlight,
                            is_sub,
                            is_sup,
                            is_underline,
                            is_insert,
                            is_comment,
                            is_footnote,
                            theme,
                        );
                        is_insert = !is_insert;
                    } else if chars.peek() == Some(&'[') {
                        // Image ![alt](url)
                        chars.next();
                        let rem: String = chars.clone().collect();
                        if let Some(c_idx) = rem.find(']') {
                            let label = &rem[..c_idx];
                            let after = &rem[c_idx + 1..];
                            if after.starts_with('(') {
                                if let Some(u_idx) = after.find(')') {
                                    flush_buffer(
                                        &mut buffer,
                                        &mut spans,
                                        is_bold,
                                        is_italic,
                                        is_strike,
                                        is_code,
                                        is_highlight,
                                        is_sub,
                                        is_sup,
                                        is_underline,
                                        is_insert,
                                        is_comment,
                                        is_footnote,
                                        theme,
                                    );
                                    spans.push(Span::styled(
                                        format!(" 🖼️ [{}]", label),
                                        theme.markdown_link(),
                                    ));
                                    for _ in 0..c_idx + u_idx + 2 {
                                        chars.next();
                                    }
                                    continue;
                                }
                            }
                        }
                        buffer.push(c);
                    } else {
                        buffer.push(c);
                    }
                }
                '?' if !is_code => {
                    if chars.peek() == Some(&'?') {
                        chars.next();
                        flush_buffer(
                            &mut buffer,
                            &mut spans,
                            is_bold,
                            is_italic,
                            is_strike,
                            is_code,
                            is_highlight,
                            is_sub,
                            is_sup,
                            is_underline,
                            is_insert,
                            is_comment,
                            is_footnote,
                            theme,
                        );
                        is_comment = !is_comment;
                    } else {
                        buffer.push(c);
                    }
                }
                '$' if !is_code => {
                    flush_buffer(
                        &mut buffer,
                        &mut spans,
                        is_bold,
                        is_italic,
                        is_strike,
                        is_code,
                        is_highlight,
                        is_sub,
                        is_sup,
                        is_underline,
                        is_insert,
                        is_comment,
                        is_footnote,
                        theme,
                    );
                    let style = theme
                        .text_style()
                        .fg(Color::Green)
                        .add_modifier(Modifier::ITALIC);
                    buffer.push(c);
                    if chars.peek() == Some(&'$') {
                        buffer.push(chars.next().unwrap());
                    }
                    spans.push(Span::styled(buffer.clone(), style));
                    buffer.clear();
                }
                '[' if !is_code => {
                    let remaining: String = chars.clone().collect();
                    if let Some(close_idx) = remaining.find(']') {
                        let label = &remaining[..close_idx];
                        if label.starts_with('^') {
                            // Footnote
                            flush_buffer(
                                &mut buffer,
                                &mut spans,
                                is_bold,
                                is_italic,
                                is_strike,
                                is_code,
                                is_highlight,
                                is_sub,
                                is_sup,
                                is_underline,
                                is_insert,
                                is_comment,
                                is_footnote,
                                theme,
                            );
                            spans.push(Span::styled(
                                format!("[{}]", label),
                                theme.dim_style().add_modifier(Modifier::ITALIC),
                            ));
                            for _ in 0..close_idx + 1 {
                                chars.next();
                            }
                            continue;
                        }
                        let after_bracket = &remaining[close_idx + 1..];
                        if after_bracket.starts_with('(') {
                            if let Some(url_close_idx) = after_bracket.find(')') {
                                // Valid link
                                flush_buffer(
                                    &mut buffer,
                                    &mut spans,
                                    is_bold,
                                    is_italic,
                                    is_strike,
                                    is_code,
                                    is_highlight,
                                    is_sub,
                                    is_sup,
                                    is_underline,
                                    is_insert,
                                    is_comment,
                                    is_footnote,
                                    theme,
                                );
                                let label = &remaining[..close_idx];
                                spans.push(Span::styled(
                                    format!("[{}]", label),
                                    theme.markdown_link().add_modifier(Modifier::UNDERLINED),
                                ));
                                for _ in 0..close_idx + url_close_idx + 2 {
                                    chars.next();
                                }
                                continue;
                            }
                        }
                    }
                    buffer.push(c);
                }
                '{' if !is_code => {
                    let rem: String = chars.clone().collect();
                    if rem.starts_with('#') {
                        if let Some(end_idx) = rem.find('}') {
                            flush_buffer(
                                &mut buffer,
                                &mut spans,
                                is_bold,
                                is_italic,
                                is_strike,
                                is_code,
                                is_highlight,
                                is_sub,
                                is_sup,
                                is_underline,
                                is_insert,
                                is_comment,
                                is_footnote,
                                theme,
                            );
                            spans.push(Span::styled(
                                format!("{{{}}}", &rem[..end_idx + 1]),
                                theme.dim_style(),
                            ));
                            for _ in 0..end_idx + 1 {
                                chars.next();
                            }
                            continue;
                        }
                    }
                    buffer.push(c);
                }
                '├' | '└' | '│' | '─' | '┌' | '┐' | '┘' | '┬' | '┴' | '┼' if !is_code =>
                {
                    flush_buffer(
                        &mut buffer,
                        &mut spans,
                        is_bold,
                        is_italic,
                        is_strike,
                        is_code,
                        is_highlight,
                        is_sub,
                        is_sup,
                        is_underline,
                        is_insert,
                        is_comment,
                        is_footnote,
                        theme,
                    );
                    spans.push(Span::styled(c.to_string(), theme.dim_style()));
                }
                '(' if !is_code => {
                    buffer.push('(');
                }
                _ => {
                    buffer.push(c);
                }
            }
        }

        // Flush remaining buffer
        if !buffer.is_empty() {
            let mut style = theme.text_style();
            if is_code {
                style = theme.markdown_code();
            } else {
                if is_bold {
                    style = theme.markdown_bold();
                }
                if is_italic {
                    style = style.patch(theme.markdown_italic());
                }
                if is_strike {
                    style = style.patch(theme.markdown_strikethrough());
                }
                if is_highlight {
                    style = style.patch(Style::default().bg(theme.primary).fg(theme.background));
                }
                if is_sub || is_sup {
                    style = style.patch(theme.dim_style());
                }
                if is_footnote {
                    style = style.patch(theme.dim_style().add_modifier(Modifier::ITALIC));
                }
                if is_underline || is_insert {
                    style = style.patch(Style::default().add_modifier(Modifier::UNDERLINED));
                }
                if is_comment {
                    style = style.patch(theme.dim_style().add_modifier(Modifier::ITALIC));
                }
                if text.starts_with("> ") && !is_bold && !is_code {
                    style = style.patch(theme.markdown_blockquote());
                }
            }
            spans.push(Span::styled(buffer, style));
        }

        Line::from(spans)
    }
}

impl Default for ChatWidget {
    fn default() -> Self {
        Self::new()
    }
}
