//! Input widget — premium command input with semantic highlighting.

use crate::tui::theme::Theme;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

/// Command input box with cursor and history.
pub struct InputWidget {
    pub content: String,
    pub cursor_position: usize,
    pub history: Vec<String>,
    pub history_index: Option<usize>,
    pub draft: String,
    pub ghost_hint: Option<String>,
}

impl InputWidget {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor_position: 0,
            history: Vec::new(),
            history_index: None,
            draft: String::new(),
            ghost_hint: None,
        }
    }

    /// Insert a character at cursor position.
    pub fn insert_char(&mut self, c: char) {
        self.content.insert(self.cursor_position, c);
        self.cursor_position += c.len_utf8();
        self.history_index = None;
    }

    /// Delete character before cursor (backspace).
    pub fn backspace(&mut self) {
        if self.cursor_position > 0 {
            let prev = self.content[..self.cursor_position]
                .chars()
                .last()
                .map(|c| c.len_utf8())
                .unwrap_or(0);
            self.cursor_position -= prev;
            self.content.remove(self.cursor_position);
        }
    }

    /// Delete character at cursor (delete key).
    pub fn delete(&mut self) {
        if self.cursor_position < self.content.len() {
            self.content.remove(self.cursor_position);
        }
    }

    /// Move cursor left.
    pub fn move_left(&mut self) {
        if self.cursor_position > 0 {
            let prev = self.content[..self.cursor_position]
                .chars()
                .last()
                .map(|c| c.len_utf8())
                .unwrap_or(0);
            self.cursor_position -= prev;
        }
    }

    /// Move cursor right.
    pub fn move_right(&mut self) {
        if self.cursor_position < self.content.len() {
            let next = self.content[self.cursor_position..]
                .chars()
                .next()
                .map(|c| c.len_utf8())
                .unwrap_or(0);
            self.cursor_position += next;
        }
    }

    /// Move cursor to start.
    pub fn home(&mut self) {
        self.cursor_position = 0;
    }

    /// Move cursor to end.
    pub fn end(&mut self) {
        self.cursor_position = self.content.len();
    }

    /// Delete word backward (Ctrl+W).
    pub fn delete_word_backward(&mut self) {
        if self.cursor_position == 0 {
            return;
        }

        let before_cursor = &self.content[..self.cursor_position];
        let mut char_indices: Vec<(usize, char)> = before_cursor.char_indices().collect();

        // 1. Skip trailing spaces immediately before the cursor
        while let Some(&(_idx, c)) = char_indices.last() {
            if c == ' ' {
                char_indices.pop();
            } else {
                break;
            }
        }

        // 2. Skip the actual word characters
        while let Some(&(_idx, c)) = char_indices.last() {
            if c != ' ' {
                char_indices.pop();
            } else {
                break;
            }
        }

        // 3. The new start position is right after the last space we kept
        let start = match char_indices.last() {
            Some(&(idx, c)) => idx + c.len_utf8(),
            None => 0,
        };

        self.content.drain(start..self.cursor_position);
        self.cursor_position = start;
        self.history_index = None;
    }

    /// Clear current line.
    pub fn clear_line(&mut self) {
        self.content.clear();
        self.cursor_position = 0;
        self.history_index = None;
        self.ghost_hint = None;
    }

    /// Update the floating ghost hint based on history and context.
    pub fn update_ghost_hint(&mut self, ctx: &crate::core::commands::CommandContext) {
        if self.content.is_empty() {
            self.ghost_hint = None;
            return;
        }
        self.ghost_hint = crate::core::commands::get_ghost_suggestion(&self.content, ctx);
    }

    /// Perform autocomplete and update content.
    pub fn autocomplete(&mut self, ctx: &crate::core::commands::CommandContext) -> bool {
        let suggestions = crate::core::commands::get_argument_suggestions(&self.content, ctx);
        if !suggestions.is_empty() {
            // Find the common prefix or just pick the first for now (Silicon-Grade precision)
            let suggestion = suggestions[0].clone();

            // Smarter replacement: find where the last token started
            let mut lexer = crate::core::commands::Lexer::new(&self.content);
            let tokens = lexer.tokenize();
            let last_token_start = if self.content.ends_with(' ') {
                self.content.len()
            } else {
                tokens
                    .last()
                    .map(|t| self.content.rfind(t).unwrap_or(self.content.len()))
                    .unwrap_or(self.content.len())
            };

            self.content.truncate(last_token_start);
            self.content.push_str(&suggestion);
            self.cursor_position = self.content.len();
            self.ghost_hint = None;
            true
        } else if let Some(hint) = self.ghost_hint.take() {
            self.content.push_str(&hint);
            self.cursor_position = self.content.len();
            true
        } else {
            false
        }
    }

    /// Submit the current input and return it.
    pub fn submit(&mut self) -> String {
        let content = self.content.clone();
        if !content.is_empty() {
            self.history.push(content.clone());
        }
        self.content.clear();
        self.draft.clear();
        self.cursor_position = 0;
        self.history_index = None;
        content
    }

    /// Navigate history up.
    pub fn history_up(&mut self) {
        if self.history.is_empty() {
            return;
        }

        if self.history_index.is_none() {
            self.draft = self.content.clone();
        }

        let idx = match self.history_index {
            Some(i) => i.saturating_sub(1),
            None => self.history.len() - 1,
        };
        self.history_index = Some(idx);
        self.content = self.history[idx].clone();
        self.cursor_position = self.content.len();
    }

    /// Navigate history down.
    pub fn history_down(&mut self) {
        match self.history_index {
            Some(i) if i + 1 < self.history.len() => {
                self.history_index = Some(i + 1);
                self.content = self.history[i + 1].clone();
                self.cursor_position = self.content.len();
            }
            Some(_) => {
                self.history_index = None;
                self.content = self.draft.clone();
                self.cursor_position = self.content.len();
            }
            None => {}
        }
    }

    /// Get current content.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Set current content programmatically.
    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.cursor_position = self.content.len();
        self.history_index = None;
    }

    /// Set current content programmatically from a string slice (Industry-Grade Utility).
    pub fn set_text(&mut self, text: &str) {
        self.content = text.to_string();
        self.cursor_position = self.content.len();
        self.history_index = None;
    }

    /// Get cursor position.
    pub fn cursor_position(&self) -> usize {
        self.cursor_position
    }

    pub fn render(
        &self,
        area: Rect,
        buf: &mut Buffer,
        theme: &Theme,
        prompt_name: &str,
        ctx: &crate::core::commands::CommandContext,
        focused: bool,
    ) {
        use crate::core::commands::{get_command_metadata, get_ghost_suggestion, is_valid_command};

        let tokens: Vec<&str> = self.content.split_whitespace().collect();
        let verb = if tokens.is_empty() {
            ""
        } else {
            tokens[0].trim_start_matches('/')
        };
        let metadata = get_command_metadata(verb);
        let ghost = get_ghost_suggestion(&self.content, ctx);
        let is_valid = is_valid_command(&self.content);

        // ─── Dynamic Title ───
        let title_text = if focused {
            if let Some(meta) = &metadata {
                format!(
                    " ▶ ACTIVE :: {} // {} ",
                    meta.usage,
                    meta.description.to_uppercase()
                )
            } else if is_valid {
                format!(" ▶ ACTIVE :: PROTOCOL: {} ", verb.to_uppercase())
            } else if ghost.is_some() {
                " ▶ ACTIVE :: TAB_TO_COMPLETE ".to_string()
            } else {
                " ▶ ACTIVE :: NEURAL_INPUT ".to_string()
            }
        } else if let Some(meta) = &metadata {
            format!(" ◈ {} // {} ", meta.usage, meta.description.to_uppercase())
        } else if is_valid {
            format!(" ◈ PROTOCOL: {} ", verb.to_uppercase())
        } else if ghost.is_some() {
            " ◈ TAB_TO_COMPLETE ".to_string()
        } else {
            " ◈ NEURAL_INPUT ".to_string()
        };

        // ─── Border Style Based on State ───
        let border_style = if focused && is_valid {
            theme.focused_border_style()
        } else if focused {
            theme.primary_style().add_modifier(Modifier::BOLD)
        } else {
            theme.unfocused_border_style()
        };

        let title_style = if focused {
            theme.focused_title_style()
        } else if is_valid || metadata.is_some() {
            theme.primary_style().add_modifier(Modifier::BOLD)
        } else {
            theme.dim_style()
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(Span::styled(title_text, title_style));

        let inner = block.inner(area);
        block.render(area, buf);

        // ─── Premium Prompt ───
        let prompt_text = format!("{} ❯ ", prompt_name.to_lowercase());

        let prompt_style = if focused {
            Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.dim)
        };

        let mut spans = vec![Span::styled(prompt_text, prompt_style)];

        // ─── Semantic Tokenizer ───
        let semantics = crate::core::commands::tokenize_semantics(&self.content);
        let mut current_pos = 0;

        for (i, token_sem) in semantics.iter().enumerate() {
            let token = &token_sem.text;
            if i > 0 {
                let space_start = self.content[current_pos..].find(token).unwrap_or(0);
                if space_start > 0 {
                    spans.push(Span::raw(
                        &self.content[current_pos..current_pos + space_start],
                    ));
                    current_pos += space_start;
                }
            } else {
                let token_start = self.content.find(token).unwrap_or(0);
                if token_start > 0 {
                    spans.push(Span::raw(&self.content[..token_start]));
                    current_pos = token_start;
                }
            }

            let style = match token_sem.role {
                crate::core::commands::TokenRole::Command => Style::default()
                    .fg(theme.primary)
                    .add_modifier(Modifier::BOLD),
                crate::core::commands::TokenRole::Flag => Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::ITALIC),
                crate::core::commands::TokenRole::FlagValue => Style::default().fg(theme.text),
                crate::core::commands::TokenRole::Target => Style::default().fg(theme.text),
                crate::core::commands::TokenRole::Other => Style::default().fg(theme.text),
            };

            spans.push(Span::styled(token, style));
            current_pos += token.len();
        }

        // Remaining characters
        if current_pos < self.content.len() {
            spans.push(Span::raw(&self.content[current_pos..]));
        }

        // ─── Ghost Suggestion ───
        if let Some(suggestion) = ghost {
            spans.push(Span::styled(
                suggestion.clone(),
                Style::default()
                    .fg(theme.dim)
                    .add_modifier(Modifier::ITALIC),
            ));

            // Premium Silicon-Grade HUD Metadata (Industry Standard)
            let (meta_text, meta_color) = if self.content.is_empty() || !self.content.contains(' ')
            {
                (" [COMMAND]", theme.primary)
            } else if suggestion.starts_with("--") || suggestion.starts_with('-') {
                (" [FLAG]", theme.accent)
            } else {
                (" [TARGET]", theme.dim)
            };

            spans.push(Span::styled(
                meta_text,
                Style::default()
                    .fg(meta_color)
                    .add_modifier(Modifier::DIM)
                    .add_modifier(Modifier::ITALIC),
            ));
        }

        // ─── Horizontal Auto-Scroll (Unicode Width Safe) ───
        use unicode_width::UnicodeWidthStr;
        let prompt_vis_width = prompt_name.width() + 3; // e.g. "name ❯ "
        let content_until_cursor = &self.content[..self.cursor_position];
        let cursor_vis_x = content_until_cursor.width();

        let visible_width = inner.width as usize;
        let total_cursor_vis_x = prompt_vis_width + cursor_vis_x;

        let scroll_x = if visible_width > 0 && total_cursor_vis_x >= visible_width {
            total_cursor_vis_x - visible_width + 1
        } else {
            0
        };

        let paragraph = Paragraph::new(Line::from(spans)).scroll((0, scroll_x as u16));
        paragraph.render(inner, buf);
    }
}

impl Default for InputWidget {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_draft_preservation() {
        let mut widget = InputWidget::new();
        widget.history = vec!["cmd1".to_string(), "cmd2".to_string()];

        widget.insert_char('h');
        widget.insert_char('e');
        widget.insert_char('l');

        assert_eq!(widget.content(), "hel");

        // Navigate up
        widget.history_up();
        assert_eq!(widget.content(), "cmd2");

        widget.history_up();
        assert_eq!(widget.content(), "cmd1");

        // Navigate back down to draft
        widget.history_down();
        assert_eq!(widget.content(), "cmd2");

        widget.history_down();
        assert_eq!(widget.content(), "hel"); // Draft restored
        assert!(widget.history_index.is_none());
    }
}
