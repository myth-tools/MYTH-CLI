//! Tree widget — recon findings tree view.

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

    /// Flatten to a list of displayable items.
    fn flatten(&self) -> Vec<(String, usize)> {
        let mut items = Vec::new();
        let prefix = "  ".repeat(self.depth);
        let expand_icon = if !self.children.is_empty() {
            if self.expanded {
                "▾ "
            } else {
                "▸ "
            }
        } else {
            "  "
        };

        items.push((
            format!("{}{}{} {}", prefix, expand_icon, self.icon, self.label),
            self.depth,
        ));

        if self.expanded {
            for child in &self.children {
                items.extend(child.flatten());
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

    /// Select next item.
    pub fn next(&mut self) {
        let total = self.flatten_all().len();
        if total == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => (i + 1) % total,
            None => 0,
        };
        self.state.select(Some(i));
    }

    /// Select previous item.
    pub fn previous(&mut self) {
        let total = self.flatten_all().len();
        if total == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    total - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn flatten_all(&self) -> Vec<(String, usize)> {
        let mut items = Vec::new();
        for node in &self.root_nodes {
            items.extend(node.flatten());
        }
        items
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme.border_style())
            .title(" 🌳 Recon Tree ")
            .title_style(theme.title_style());

        let inner = block.inner(area);
        block.render(area, buf);

        let flat_items = self.flatten_all();
        let list_items: Vec<ListItem> = flat_items
            .iter()
            .map(|(label, depth)| {
                let style = if *depth == 0 {
                    Style::default()
                        .fg(theme.primary)
                        .add_modifier(Modifier::BOLD)
                } else {
                    theme.text_style()
                };
                ListItem::new(Line::from(Span::styled(label.as_str(), style)))
            })
            .collect();

        let list = List::new(list_items)
            .highlight_style(theme.highlight_style())
            .highlight_symbol("▶ ");

        ratatui::widgets::StatefulWidget::render(list, inner, buf, &mut self.state);
    }
}

impl Default for TreeWidget {
    fn default() -> Self {
        Self::new()
    }
}
