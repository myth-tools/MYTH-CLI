use crate::ui;
use owo_colors::OwoColorize;

pub struct RenderVizState {
    pub at_line_start: bool,
    pub header_level: i32,
    pub in_blockquote: bool,
    pub in_code_block: bool,
    pub in_code_block_first_line: bool,
    pub in_bold: bool,
    pub in_italic: bool,
    pub hr_count: i32,
    pub pending_markdown: String,
    pub in_inline_code: bool,
    pub number_list_buffer: String,
    pub line_buffer: String,
    pub indent_level: usize,
    pub last_char: char,
}

impl Default for RenderVizState {
    fn default() -> Self {
        Self {
            at_line_start: true,
            header_level: 0,
            in_blockquote: false,
            in_code_block: false,
            in_code_block_first_line: false,
            in_bold: false,
            in_italic: false,
            hr_count: 0,
            pending_markdown: String::new(),
            in_inline_code: false,
            number_list_buffer: String::new(),
            line_buffer: String::new(),
            indent_level: 0,
            last_char: ' ',
        }
    }
}

/// Elite Pure-Viz Markdown Renderer — zero-copy, char-by-char ANSI state machine.
pub fn render_char_pureviz(c: char, state: &mut RenderVizState, w: &mut impl std::io::Write) {
    if c == '\n' {
        if state.in_code_block_first_line {
            let lang = state.line_buffer.trim().to_string();
            if !lang.is_empty() {
                write!(w, "\x1b[2;3m{}\x1b[0m", lang).ok();
            }
            write!(w, "\n\x1b[36m").ok();
            state.in_code_block_first_line = false;
            state.line_buffer.clear();
            state.at_line_start = true;
            state.header_level = 0;
            state.hr_count = 0;
            state.pending_markdown.clear();
            state.number_list_buffer.clear();
            return;
        }

        if !state.pending_markdown.is_empty() {
            flush_pending_markdown(state, w);
        }

        if !state.line_buffer.is_empty() {
            write!(w, "{}", state.line_buffer).ok();
            state.line_buffer.clear();
        }

        if state.header_level > 0 || state.in_bold || state.in_italic || state.in_inline_code {
            write!(w, "\x1b[0m").ok();
        }
        writeln!(w).ok();
        state.at_line_start = true;
        state.header_level = 0;
        if state.hr_count < 3 && state.hr_count > 0 {
            write!(w, "{}", "-".repeat(state.hr_count as usize)).ok();
        }
        state.hr_count = 0;
        state.number_list_buffer.clear();
        state.indent_level = 0;
        if !state.in_code_block {
            state.in_inline_code = false;
        }
        if state.in_code_block {
            write!(w, "\x1b[36m").ok();
        }
        if state.in_blockquote {
            write!(w, "\x1b[2m").ok();
        }
        if state.in_bold {
            write!(w, "\x1b[1m").ok();
        }
        if state.in_italic {
            write!(w, "\x1b[3m").ok();
        }
    } else {
        if state.in_code_block_first_line {
            state.line_buffer.push(c);
            return;
        }

        if state.at_line_start {
            if c == ' ' || c == '\t' {
                state.indent_level += if c == '\t' { 4 } else { 1 };
                if state.hr_count == 0 && state.number_list_buffer.is_empty() {
                    write!(w, "{}", c).ok();
                    return;
                }
            }
            if state.hr_count == 0 && state.number_list_buffer.is_empty() && c == '#' {
                state.header_level += 1;
                return;
            } else if c == '>' {
                state.in_blockquote = true;
                write!(w, "{} ", ui::CyberTheme::secondary("┃").bold()).ok();
                write!(w, "\x1b[2m").ok();
                state.at_line_start = false;
                return;
            } else if c == '-' || c == '*' || c == '_' || c == '━' || c == '═' || c == '=' {
                state.hr_count += 1;
                state.pending_markdown.push(c);
                if state.hr_count == 3 {
                    write!(w, "\r{}\n", ui::CyberTheme::dim("━".repeat(60))).ok();
                    state.pending_markdown.clear();
                }
                return;
            } else if c.is_ascii_digit()
                || ((c == '.' || c == ')') && !state.number_list_buffer.is_empty())
            {
                state.number_list_buffer.push(c);
                return;
            }

            if state.header_level > 0 {
                let color = match state.header_level {
                    1 => "\x1b[1;35m",
                    2 => "\x1b[1;36m",
                    3 => "\x1b[1;32m",
                    _ => "\x1b[1;33m",
                };
                write!(w, "{}", color).ok();
                if c == ' ' {
                    state.at_line_start = false;
                    return;
                }
            } else if (state.hr_count == 1 || state.hr_count == 2) && c == ' ' {
                let bullet = if state.indent_level >= 4 {
                    "◦"
                } else if state.indent_level >= 2 {
                    "▸"
                } else {
                    "•"
                };
                let indent = " ".repeat(state.indent_level);
                write!(
                    w,
                    "\r  {}{} ",
                    indent,
                    ui::CyberTheme::primary(bullet).bold()
                )
                .ok();
                state.pending_markdown.clear();
                state.hr_count = 0;
                state.at_line_start = false;
                return;
            } else if !state.number_list_buffer.is_empty()
                && (state.number_list_buffer.ends_with('.')
                    || state.number_list_buffer.ends_with(')'))
                && c == ' '
            {
                let indent = " ".repeat(state.indent_level);
                write!(
                    w,
                    "\r  {}{} ",
                    indent,
                    ui::CyberTheme::primary(&state.number_list_buffer).bold()
                )
                .ok();
                state.number_list_buffer.clear();
                state.at_line_start = false;
                return;
            } else if state.hr_count > 0 {
                if state.pending_markdown == "**"
                    || state.pending_markdown == "__"
                    || state.pending_markdown == "*"
                    || state.pending_markdown == "_"
                    || state.pending_markdown == "***"
                    || state.pending_markdown == "___"
                {
                    flush_pending_markdown(state, w);
                } else {
                    write!(w, "\r{}", state.pending_markdown).ok();
                    state.pending_markdown.clear();
                }
                state.hr_count = 0;
            } else if !state.number_list_buffer.is_empty() {
                write!(w, "{}", state.number_list_buffer).ok();
                state.number_list_buffer.clear();
            }

            state.at_line_start = false;
        }

        if c == '[' && state.line_buffer.is_empty() && !state.in_code_block {
            state.line_buffer.push(c);
            return;
        }
        if !state.line_buffer.is_empty() && state.line_buffer == "[" {
            state.line_buffer.push(c);
            if state.line_buffer == "[x" || state.line_buffer == "[X" || state.line_buffer == "[ " {
                return;
            }
            write!(w, "{}", state.line_buffer).ok();
            state.line_buffer.clear();
            return;
        }
        if !state.line_buffer.is_empty() && state.line_buffer.len() == 2 && c == ']' {
            if state.line_buffer == "[x" || state.line_buffer == "[X" {
                write!(w, "\x1b[32;1m✅\x1b[0m").ok();
            } else if state.line_buffer == "[ " {
                write!(w, "\x1b[2m☐\x1b[0m").ok();
            }
            state.line_buffer.clear();
            if state.in_bold {
                write!(w, "\x1b[1m").ok();
            }
            if state.in_italic {
                write!(w, "\x1b[3m").ok();
            }
            if state.in_blockquote {
                write!(w, "\x1b[2m").ok();
            }
            return;
        }

        if c == '|' && !state.in_code_block {
            write!(w, "{}", ui::CyberTheme::dim("┃").bold()).ok();
            return;
        } else if (c == '-' || c == ':') && !state.at_line_start && !state.in_code_block {
            write!(w, "\x1b[2m{}\x1b[22m", c).ok();
            return;
        }

        if !state.in_code_block && c == '>' && (state.last_char == '-' || state.last_char == '=') {
            write!(w, "\x08\x1b[2m{}{}\x1b[22m", state.last_char, c).ok();
            state.last_char = c;
            return;
        }

        if (c == '*' || c == '_' || c == '`' || c == '~') && !state.in_code_block {
            if state.pending_markdown.is_empty() || state.pending_markdown.chars().all(|pc| pc == c)
            {
                state.pending_markdown.push(c);

                if state.pending_markdown == "```" {
                    state.in_code_block = !state.in_code_block;
                    state.pending_markdown.clear();
                    if state.in_code_block {
                        state.in_code_block_first_line = true;
                        state.line_buffer.clear();
                    } else {
                        write!(w, "\x1b[0m").ok();
                        if state.in_blockquote {
                            write!(w, "\x1b[2m").ok();
                        }
                    }
                }
                return;
            } else {
                flush_pending_markdown(state, w);
                state.pending_markdown.push(c);
                return;
            }
        } else if !state.pending_markdown.is_empty() {
            flush_pending_markdown(state, w);
        }

        write!(w, "{}", c).ok();
        state.last_char = c;
    }
}

/// Advanced Markdown Rendering via Comrak (I-02 Fix).
/// Provides industry-standard GFM parsing for complex reports.
pub fn render_advanced(text: &str) -> String {
    use comrak::{markdown_to_commonmark, ComrakOptions};
    let mut options = ComrakOptions::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.extension.autolink = true;
    options.render.hardbreaks = true;

    markdown_to_commonmark(text, &options)
}

pub fn flush_pending_markdown(state: &mut RenderVizState, w: &mut impl std::io::Write) {
    if state.pending_markdown.is_empty() {
        return;
    }

    match state.pending_markdown.as_str() {
        "***" | "___" => {
            state.in_bold = !state.in_bold;
            state.in_italic = !state.in_italic;
            if state.in_bold && state.in_italic {
                write!(w, "\x1b[1;3m").ok();
            } else {
                write!(w, "\x1b[22;23m").ok();
                if state.in_bold {
                    write!(w, "\x1b[1m").ok();
                }
                if state.in_italic {
                    write!(w, "\x1b[3m").ok();
                }
            }
        }
        "**" | "__" => {
            state.in_bold = !state.in_bold;
            if state.in_bold {
                write!(w, "\x1b[1m").ok();
            } else {
                write!(w, "\x1b[22m").ok();
            }
        }
        "*" | "_" => {
            state.in_italic = !state.in_italic;
            if state.in_italic {
                write!(w, "\x1b[3m").ok();
            } else {
                write!(w, "\x1b[23m").ok();
            }
        }
        "~~" => {
            write!(w, "\x1b[9m").ok();
        }
        "`" => {
            state.in_inline_code = !state.in_inline_code;
            if state.in_inline_code {
                write!(w, "\x1b[40;38;5;220m").ok();
            } else {
                write!(w, "\x1b[0m").ok();
                if state.in_bold {
                    write!(w, "\x1b[1m").ok();
                }
                if state.in_italic {
                    write!(w, "\x1b[3m").ok();
                }
                if state.in_blockquote {
                    write!(w, "\x1b[2m").ok();
                }
            }
        }
        other => {
            write!(w, "{}", other).ok();
        }
    }
    state.pending_markdown.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_char_pureviz_basic() {
        let mut state = RenderVizState::default();
        let mut output = Vec::new(); // Changed to Vec<u8> since it expects impl std::io::Write
                                     // Just feed it a few characters to ensure no panics and some output
        render_char_pureviz('A', &mut state, &mut output);
        render_char_pureviz('B', &mut state, &mut output);
        render_char_pureviz('C', &mut state, &mut output);
        assert!(
            !output.is_empty(),
            "render_char_pureviz should produce output"
        );
    }
}
