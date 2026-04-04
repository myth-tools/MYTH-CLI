//! Tree widget — recon findings tree view with depth coloring.

use crate::tui::theme::Theme;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};

/// A node in the recon tree.
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub label: String,
    pub icon: String,
    pub depth: usize,
    pub expanded: bool,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(label: &str, icon: &str) -> Self {
        Self {
            label: label.to_string(),
            icon: icon.to_string(),
            depth: 0,
            expanded: true,
            children: Vec::new(),
        }
    }

    pub fn with_child(mut self, child: TreeNode) -> Self {
        let mut child = child;
        child.depth = self.depth + 1;
        self.children.push(child);
        self
    }

    pub fn add_child(&mut self, child: TreeNode) {
        let mut child = child;
        child.depth = self.depth + 1;
        self.children.push(child);
    }

    /// Flatten to a displayable list with connector lines.
    fn flatten(&self, is_last: bool) -> Vec<(String, usize)> {
        let mut items = Vec::new();

        // Build prefix with proper tree connectors
        let connector = if self.depth == 0 {
            String::new()
        } else if is_last {
            format!("{}└─ ", "│  ".repeat(self.depth.saturating_sub(1)))
        } else {
            format!("{}├─ ", "│  ".repeat(self.depth.saturating_sub(1)))
        };

        let expand_icon = if !self.children.is_empty() {
            if self.expanded {
                "▾ "
            } else {
                "▸ "
            }
        } else {
            ""
        };

        items.push((
            format!("{}{}{} {}", connector, expand_icon, self.icon, self.label),
            self.depth,
        ));

        if self.expanded {
            let child_count = self.children.len();
            for (i, child) in self.children.iter().enumerate() {
                items.extend(child.flatten(i == child_count - 1));
            }
        }

        items
    }
}

/// Recon findings tree panel.
pub struct TreeWidget {
    root_nodes: Vec<TreeNode>,
    state: ListState,
}

impl TreeWidget {
    pub fn new() -> Self {
        Self {
            root_nodes: Vec::new(),
            state: ListState::default(),
        }
    }

    /// Set the tree from root nodes.
    pub fn set_nodes(&mut self, nodes: Vec<TreeNode>) {
        self.root_nodes = nodes;
    }

    /// Add a root-level node.
    pub fn add_root(&mut self, node: TreeNode) {
        self.root_nodes.push(node);
    }

    /// Extract the clean label of the selected node for clipboard copying
    pub fn selected_path(&self) -> Option<String> {
        if let Some(idx) = self.state.selected() {
            let flat = self.flatten_all();
            if let Some((label, _)) = flat.get(idx) {
                let clean = label
                    .replace("├─", "")
                    .replace("│", "")
                    .replace("└─", "")
                    .replace("▾", "")
                    .replace("▸", "")
                    .trim()
                    .to_string();
                return Some(clean);
            }
        }
        None
    }

    /// Select next item.
    pub fn next(&mut self) {
        let total = self.flatten_all().len();
        if total == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => (i + 1).min(total.saturating_sub(1)),
            None => 0,
        };
        self.state.select(Some(i));
    }

    /// Scroll aliases for mouse wheel routing
    pub fn scroll_down(&mut self) {
        self.next();
    }

    pub fn scroll_up(&mut self) {
        self.previous();
    }

    /// Select previous item.
    pub fn previous(&mut self) {
        let total = self.flatten_all().len();
        if total == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => i.saturating_sub(1),
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn flatten_all(&self) -> Vec<(String, usize)> {
        let mut items = Vec::new();
        let root_count = self.root_nodes.len();
        for (i, node) in self.root_nodes.iter().enumerate() {
            items.extend(node.flatten(i == root_count - 1));
        }
        items
    }

    /// Toggle expansion of the node at the given flattened index.
    pub fn toggle_node_at(&mut self, index: usize) {
        let mut current_idx = 0;
        for node in &mut self.root_nodes {
            if Self::toggle_recursive(node, index, &mut current_idx) {
                break;
            }
        }
    }

    fn toggle_recursive(node: &mut TreeNode, target_idx: usize, current_idx: &mut usize) -> bool {
        if *current_idx == target_idx {
            node.expanded = !node.expanded;
            return true;
        }
        *current_idx += 1;
        if node.expanded {
            for child in &mut node.children {
                if Self::toggle_recursive(child, target_idx, current_idx) {
                    return true;
                }
            }
        }
        false
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer, theme: &Theme, focused: bool) {
        let title_text = if focused {
            " ▶ ACTIVE :: RECON_MAP ".to_string()
        } else {
            " ◈ RECON_MAP ".to_string()
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
            .title(Span::styled(title_text, title_style));

        let inner = block.inner(area);
        block.render(area, buf);

        let flat_items = self.flatten_all();
        let list_items: Vec<ListItem> = flat_items
            .iter()
            .map(|(label, depth)| {
                // Depth-based coloring
                let style = match depth {
                    0 => Style::default()
                        .fg(theme.primary)
                        .add_modifier(Modifier::BOLD),
                    1 => Style::default().fg(theme.secondary),
                    2 => Style::default().fg(theme.warning),
                    _ => Style::default().fg(theme.dim),
                };
                ListItem::new(Line::from(Span::styled(label.as_str(), style)))
            })
            .collect();

        let list = List::new(list_items)
            .highlight_style(
                Style::default()
                    .bg(theme.surface_bright)
                    .fg(theme.text)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▸ ");

        ratatui::widgets::StatefulWidget::render(list, inner, buf, &mut self.state);
    }
}

impl Default for TreeWidget {
    fn default() -> Self {
        Self::new()
    }
}
