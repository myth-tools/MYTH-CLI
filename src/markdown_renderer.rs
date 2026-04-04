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
    pub in_table: bool,
    pub table_rows: Vec<Vec<String>>,
    pub last_line_was_live_printed: bool,
    pub in_link_label: bool,
    pub in_link_url: bool,
    pub escaped: bool,
    pub in_highlight: bool,
    pub in_subscript: bool,
    pub in_superscript: bool,
    pub blockquote_level: i32,
    pub in_underline: bool,
    pub in_insert: bool,
    pub in_comment: bool,
    pub space_count: usize, // For EOL line breaks
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
            in_table: false,
            table_rows: Vec::new(),
            last_line_was_live_printed: false,
            in_link_label: false,
            in_link_url: false,
            escaped: false,
            in_highlight: false,
            in_subscript: false,
            in_superscript: false,
            blockquote_level: 0,
            in_underline: false,
            in_insert: false,
            in_comment: false,
            space_count: 0,
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

        let pipe_count = state.line_buffer.matches('|').count();
        let gap_count = state
            .line_buffer
            .split("  ")
            .filter(|s| !s.trim().is_empty())
            .count();
        // Relax heuristic: 3 segments generally OR 2 segments if we are already in a table
        let is_heuristic = gap_count >= 3 || (state.in_table && gap_count >= 2);
        let has_gap = state.line_buffer.contains("  ");

        if state.in_table && pipe_count < 2 && !(is_heuristic && has_gap) {
            flush_table(state, w);
        }

        if !state.pending_markdown.is_empty() {
            flush_pending_markdown(state, w);
        }

        if !state.line_buffer.is_empty() {
            let p_count = state.line_buffer.matches('|').count();
            let g_count = state
                .line_buffer
                .split("  ")
                .filter(|s| !s.trim().is_empty())
                .count();
            let has_gap = state.line_buffer.contains("  ");
            let is_heuristic = g_count >= 3 || (state.in_table && g_count >= 2);

            if p_count >= 2 || (is_heuristic && has_gap) {
                state.in_table = true;
                let cells: Vec<String> = if p_count >= 2 {
                    state
                        .line_buffer
                        .split('|')
                        .filter(|s| !s.trim().is_empty())
                        .map(|s| s.trim().to_string())
                        .collect()
                } else {
                    state
                        .line_buffer
                        .split("  ")
                        .filter(|s| !s.trim().is_empty())
                        .map(|s| s.trim().to_string())
                        .collect()
                };

                if !cells
                    .iter()
                    .all(|c| c.chars().all(|ch| ch == '-' || ch == ':'))
                {
                    state.table_rows.push(cells);
                }
                // When we are in a table, we don't clear at_line_start yet, but we stop live-printing next lines?
                // Actually, the simplest is: if in_table, WE STILL LIVE PRINT but we track it.
            } else {
                if state.in_table {
                    flush_table(state, w);
                }
            }
        } else if state.in_table {
            flush_table(state, w);
        }

        if !state.pending_markdown.is_empty() {
            flush_pending_markdown(state, w);
        }

        state.line_buffer.clear();
        state.last_line_was_live_printed = false;

        if state.header_level > 0
            || state.in_bold
            || state.in_italic
            || state.in_inline_code
            || state.in_highlight
            || state.in_link_label
            || state.in_link_url
            || state.in_underline
            || state.in_insert
            || state.in_comment
        {
            write!(w, "\x1b[0m").ok();
        }
        if state.space_count >= 2 {
            writeln!(w).ok();
        }
        writeln!(w).ok();
        state.at_line_start = true;
        state.space_count = 0;
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
        state.in_link_label = false;
        state.in_link_url = false;
        state.escaped = false;
        state.in_highlight = false;
        state.in_subscript = false;
        state.in_superscript = false;
        state.in_blockquote = false;
        state.blockquote_level = 0;
        state.in_underline = false;
        state.in_insert = false;
        state.in_comment = false;
        state.in_inline_code = false;
        // Reset states that shouldn't persist at raw state across lines?
        // Actually, markdown usually persists bold/italic across lines in a block.
        // But for terminal simplicity, we reset highlight if it bleeds too much.
    } else {
        if c == ' ' {
            state.space_count += 1;
        } else {
            state.space_count = 0;
        }

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
                state.blockquote_level += 1;
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
            } else if state.blockquote_level > 0 {
                for _ in 0..state.blockquote_level {
                    write!(w, "{} ", ui::CyberTheme::secondary("┃").bold()).ok();
                }
                write!(w, "\x1b[2m").ok();
                state.in_blockquote = true;
                state.at_line_start = false;
                if c == ' ' {
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

        if state.escaped {
            write!(w, "{}", c).ok();
            state.escaped = false;
            return;
        }

        if c == '\\' && !state.in_code_block {
            state.escaped = true;
            return;
        }

        if c == '=' && !state.in_code_block {
            state.pending_markdown.push(c);
            return;
        }
        if c == '[' && state.line_buffer.is_empty() && !state.in_code_block {
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

        if (c == '|' || ((c == '-' || c == ':') && !state.at_line_start)) && !state.in_code_block {
            state.line_buffer.push(c);
            write!(w, "{}", c).ok(); // LIVE PRINT
            state.last_line_was_live_printed = true;
            return;
        }

        if (c == '├'
            || c == '└'
            || c == '│'
            || c == '─'
            || c == '┌'
            || c == '┐'
            || c == '┘'
            || c == '┬'
            || c == '┴'
            || c == '┼')
            && !state.in_code_block
        {
            write!(w, "\x1b[2m{}\x1b[0m", c).ok();
            return;
        }

        if !state.in_code_block && c == '>' && (state.last_char == '-' || state.last_char == '=') {
            write!(w, "\x08\x1b[2m{}{}\x1b[22m", state.last_char, c).ok();
            state.last_char = c;
            return;
        }

        let is_markdown_char = c == '*'
            || c == '_'
            || c == '`'
            || c == '~'
            || c == '^'
            || c == '!'
            || c == '?'
            || c == '='
            || c == '$';

        if is_markdown_char && (!state.in_code_block || c == '`') {
            if state.pending_markdown.is_empty() || state.pending_markdown.chars().all(|pc| pc == c)
            {
                state.pending_markdown.push(c);

                // Advanced GFM: !! (insert), ?? (comment), ^^ (underline), $$ (math block)
                if state.pending_markdown == "!!"
                    || state.pending_markdown == "??"
                    || state.pending_markdown == "^^"
                    || state.pending_markdown == "=="
                    || state.pending_markdown == "~~"
                    || state.pending_markdown == "$$"
                {
                    flush_pending_markdown(state, w);
                    return;
                }

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
                    return;
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

        if c == '[' && !state.in_code_block {
            state.in_link_label = true;
            write!(w, "{}", ui::CyberTheme::primary("[")).ok();
            write!(w, "\x1b[4m").ok(); // Underline
            return;
        }
        if c == ']' && state.in_link_label && !state.in_code_block {
            state.in_link_label = false;
            write!(w, "\x1b[24m{}", ui::CyberTheme::primary("]")).ok();
            return;
        }
        if c == '(' && !state.in_code_block && state.last_char == ']' {
            state.in_link_url = true;
            write!(w, "\x1b[2m(").ok();
            return;
        }
        if c == ')' && state.in_link_url && !state.in_code_block {
            state.in_link_url = false;
            write!(w, ")\x1b[0m").ok();
            if state.in_bold {
                write!(w, "\x1b[1m").ok();
            }
            if state.in_italic {
                write!(w, "\x1b[3m").ok();
            }
            return;
        }

        if c == '<' && !state.in_code_block {
            write!(w, "\x1b[2m<").ok();
            return;
        }
        if c == '>' && !state.in_code_block && !state.at_line_start {
            write!(w, ">\x1b[0m").ok();
            return;
        }

        write!(w, "{}", c).ok();
        state.line_buffer.push(c); // Always buffer the line for heuristic checks on \n
        state.last_line_was_live_printed = true;
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

fn header_color_code(level: i32) -> &'static str {
    match level {
        1 => "\x1b[1;35m",
        2 => "\x1b[1;36m",
        3 => "\x1b[1;32m",
        _ => "\x1b[1;33m",
    }
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
        "==" => {
            state.in_highlight = !state.in_highlight;
            if state.in_highlight {
                write!(w, "\x1b[48;2;255;215;0m\x1b[38;2;5;5;8m\x1b[1m").ok(); // Tactical Gold BG, Obsidian FG, Bold
            } else {
                write!(w, "\x1b[0m").ok();
                if state.header_level > 0 {
                    write!(w, "\x1b[1m{}", header_color_code(state.header_level)).ok();
                }
                if state.in_blockquote {
                    write!(w, "\x1b[2m").ok();
                }
            }
        }
        "!!" => {
            state.in_insert = !state.in_insert;
            if state.in_insert {
                write!(w, "\x1b[4m").ok();
            } else {
                write!(w, "\x1b[24m").ok();
            }
        }
        "??" => {
            state.in_comment = !state.in_comment;
            if state.in_comment {
                write!(w, "\x1b[2;3m").ok();
            } else {
                write!(w, "\x1b[22;23m").ok();
            }
        }
        "^^" => {
            state.in_underline = !state.in_underline;
            if state.in_underline {
                write!(w, "\x1b[4m").ok();
            } else {
                write!(w, "\x1b[24m").ok();
            }
        }
        "$" | "$$" => {
            write!(w, "\x1b[3;32m").ok(); // Green Italic for Math
        }
        "\\(" | "\\[" => {
            write!(w, "\x1b[3;32m").ok();
        }
        "\\)" | "\\]" => {
            write!(w, "\x1b[0m").ok(); // Close math
        }
        "~" => {
            state.in_subscript = !state.in_subscript;
            if state.in_subscript {
                write!(w, "\x1b[2m").ok();
            } else {
                write!(w, "\x1b[22m").ok();
            }
        }
        "^" => {
            state.in_superscript = !state.in_superscript;
            if state.in_superscript {
                write!(w, "\x1b[1;2m").ok();
            } else {
                write!(w, "\x1b[22m").ok();
            }
        }
        "~~" => {
            write!(w, "\x1b[9m").ok();
        }
        "`" => {
            if !state.in_code_block {
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
            } else {
                write!(w, "`").ok();
            }
        }
        "``" => {
            write!(w, "``").ok();
        }
        other => {
            write!(w, "{}", other).ok();
        }
    }
    state.pending_markdown.clear();
}

pub fn flush_table(state: &mut RenderVizState, w: &mut impl std::io::Write) {
    if state.table_rows.is_empty() {
        state.in_table = false;
        return;
    }

    // Morph Phase: Erase raw live-printed lines
    // We add 1 if the current line was also a table row and we just saw the \n
    let lines_to_erase = state.table_rows.len();
    for _ in 0..lines_to_erase {
        write!(w, "\x1b[1F\x1b[2K").ok();
    }
    write!(w, "\r").ok();

    let mut col_widths = Vec::new();
    for row in &state.table_rows {
        for (i, cell) in row.iter().enumerate() {
            let display_len = cell.chars().count();
            let effective_len = display_len.min(40); // Cap at 40 chars for robustness
            if i >= col_widths.len() {
                col_widths.push(effective_len);
            } else if effective_len > col_widths[i] {
                col_widths[i] = effective_len;
            }
        }
    }

    // Header Top Border
    write!(w, "  \x1b[2m┌").ok();
    for (i, width) in col_widths.iter().enumerate() {
        write!(w, "{}", "─".repeat(width + 2)).ok();
        if i < col_widths.len() - 1 {
            write!(w, "┬").ok();
        }
    }
    writeln!(w, "┐\x1b[0m").ok();

    for (r, row) in state.table_rows.iter().enumerate() {
        // Prepare wrapped lines for each cell in this row
        let mut wrapped_cells: Vec<Vec<String>> = Vec::new();
        let mut max_lines = 1;
        for (i, width) in col_widths.iter().enumerate() {
            let cell_text = row.get(i).cloned().unwrap_or_default();
            let mut lines = Vec::new();
            let mut chars = cell_text.chars();
            loop {
                let chunk: String = chars.by_ref().take(*width).collect();
                if chunk.is_empty() {
                    break;
                }
                lines.push(chunk);
            }
            if lines.is_empty() {
                lines.push(String::new());
            }
            if lines.len() > max_lines {
                max_lines = lines.len();
            }
            wrapped_cells.push(lines);
        }

        // Render each logical line of this table row
        for line_idx in 0..max_lines {
            write!(w, "  \x1b[2m│\x1b[0m").ok();
            for (i, width) in col_widths.iter().enumerate() {
                let cell_part = wrapped_cells[i].get(line_idx).cloned().unwrap_or_default();
                let padding = width - cell_part.chars().count();

                if r == 0 {
                    // Header row: Bold Magenta
                    write!(w, " \x1b[1;35m{}\x1b[0m{} ", cell_part, " ".repeat(padding)).ok();
                } else if r % 2 == 0 {
                    // Alternating rows: Dim for readability
                    write!(w, " \x1b[2m{}\x1b[0m{} ", cell_part, " ".repeat(padding)).ok();
                } else {
                    write!(w, " {} {} ", cell_part, " ".repeat(padding)).ok();
                }
                write!(w, "\x1b[2m│\x1b[0m").ok();
            }
            writeln!(w).ok();
        }

        // Row separator (after header only for now for ultra-premium look)
        if r == 0 {
            write!(w, "  \x1b[2m├").ok();
            for (i, width) in col_widths.iter().enumerate() {
                write!(w, "{}", "─".repeat(width + 2)).ok();
                if i < col_widths.len() - 1 {
                    write!(w, "┼").ok();
                }
            }
            writeln!(w, "┤\x1b[0m").ok();
        }
    }

    // Table Bottom Border
    write!(w, "  \x1b[2m└").ok();
    for (i, width) in col_widths.iter().enumerate() {
        write!(w, "{}", "─".repeat(width + 2)).ok();
        if i < col_widths.len() - 1 {
            write!(w, "┴").ok();
        }
    }
    writeln!(w, "┘\x1b[0m").ok();

    state.table_rows.clear();
    state.in_table = false;
}

pub fn flush_final(state: &mut RenderVizState, w: &mut impl std::io::Write) {
    if state.in_table {
        flush_table(state, w);
    }
    if !state.pending_markdown.is_empty() {
        flush_pending_markdown(state, w);
    }
    if !state.line_buffer.is_empty() {
        write!(w, "{}", state.line_buffer).ok();
        state.line_buffer.clear();
    }
    write!(w, "\x1b[0m").ok(); // Final reset
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
