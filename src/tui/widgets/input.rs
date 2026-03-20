//! Input widget — command input box.

use crate::tui::theme::Theme;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

/// Command input box with cursor.
pub struct InputWidget {
    content: String,
    cursor_position: usize,
    history: Vec<String>,
    history_index: Option<usize>,
    draft: String,
}

impl InputWidget {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor_position: 0,
            history: Vec::new(),
            history_index: None,
            draft: String::new(),
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

    /// Autocomplete the current input with the ghost suggestion.
    pub fn autocomplete(&mut self, ctx: &crate::core::commands::CommandContext) -> bool {
        if let Some(suggestion) = crate::core::commands::get_ghost_suggestion(&self.content, ctx) {
            self.content.push_str(&suggestion);
            self.cursor_position = self.content.len();
            return true;
        }
        false
    }

    /// Get current content.
    pub fn content(&self) -> &str {
        &self.content
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

        // Elite Dynamic Title with Tactical HUD
        let title = if let Some(meta) = &metadata {
            format!(
                " ⌨ Tactical HUD: {} // {} ",
                meta.usage,
                meta.description.to_uppercase()
            )
        } else if is_valid {
            format!(" ⌨ Tactical Protocol: {} ", verb.to_uppercase())
        } else if ghost.is_some() {
            " ⌨ Command Autocomplete Available [TAB] ".to_string()
        } else {
            " ⌨ Command Matrix ".to_string()
        };

        let active_border = Style::default()
            .fg(theme.primary)
            .add_modifier(Modifier::BOLD);
        let default_border = Style::default().fg(theme.dim);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(if is_valid {
                active_border
            } else {
                default_border
            })
            .title(title)
            .title_style(if is_valid || metadata.is_some() {
                active_border
            } else {
                default_border
            });

        let inner = block.inner(area);
        block.render(area, buf);

        let prompt_text = format!("{}> ", prompt_name.to_lowercase());

        // Multi-layered semantic input rendering
        let mut spans = vec![Span::styled(
            prompt_text,
            if is_valid {
                active_border
            } else {
                default_border
            },
        )];

        // Semantic Tokenizer for Highlight
        let semantics = crate::core::commands::tokenize_semantics(&self.content);
        let mut current_pos = 0;

        for (i, token_sem) in semantics.iter().enumerate() {
            let token = &token_sem.text;
            // Add space before tokens after the first one
            if i > 0 {
                // Find space between current token and previous
                let space_start = self.content[current_pos..].find(token).unwrap_or(0);
                if space_start > 0 {
                    spans.push(Span::raw(
                        &self.content[current_pos..current_pos + space_start],
                    ));
                    current_pos += space_start;
                }
            } else {
                // Handle leading spaces
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
                    .add_modifier(Modifier::ITALIC), // Magenta
                crate::core::commands::TokenRole::FlagValue => Style::default().fg(theme.text),
                crate::core::commands::TokenRole::Target => Style::default().fg(theme.text),
                crate::core::commands::TokenRole::Other => Style::default().fg(theme.text),
            };

            spans.push(Span::styled(token, style));
            current_pos += token.len();
        }

        // Add remaining characters (spaces at end)
        if current_pos < self.content.len() {
            spans.push(Span::raw(&self.content[current_pos..]));
        }

        // Ghost Suggestion
        if let Some(suggestion) = ghost {
            spans.push(Span::styled(
                suggestion,
                Style::default()
                    .fg(Color::Rgb(100, 100, 120))
                    .add_modifier(Modifier::ITALIC),
            ));
        }

        let paragraph = Paragraph::new(Line::from(spans));
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
