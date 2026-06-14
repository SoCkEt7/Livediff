// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use similar::{ChangeTag, TextDiff};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum LineChangeType {
    Insert,
    Delete,
    Context,
    Header,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DiffLine {
    pub change_type: LineChangeType,
    pub content: String,
}

pub struct DiffResult {
    pub lines: Vec<DiffLine>,
    pub added: usize,
    pub deleted: usize,
}

pub struct DiffEngine;

impl Default for DiffEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl DiffEngine {
    pub fn new() -> Self {
        DiffEngine
    }

    pub fn compute_diff(&self, old_content: &str, new_content: &str) -> DiffResult {
        let mut added = 0;
        let mut deleted = 0;
        let mut lines = Vec::new();

        let diff = TextDiff::from_lines(old_content, new_content);
        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Delete => {
                    deleted += 1;
                    lines.push(DiffLine {
                        change_type: LineChangeType::Delete,
                        content: change.value().to_string(),
                    });
                }
                ChangeTag::Insert => {
                    added += 1;
                    lines.push(DiffLine {
                        change_type: LineChangeType::Insert,
                        content: change.value().to_string(),
                    });
                }
                ChangeTag::Equal => {
                    lines.push(DiffLine {
                        change_type: LineChangeType::Context,
                        content: change.value().to_string(),
                    });
                }
            }
        }

        DiffResult { lines, added, deleted }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_diff() {
        let engine = DiffEngine::new();
        let old = "hello\nworld\n";
        let new = "hello\nbeautiful\nworld\n";
        let result = engine.compute_diff(old, new);

        assert_eq!(result.added, 1);
        assert_eq!(result.deleted, 0);
        assert_eq!(result.lines.len(), 3);

        assert!(matches!(result.lines[0].change_type, LineChangeType::Context));
        assert_eq!(result.lines[0].content, "hello\n");

        assert!(matches!(result.lines[1].change_type, LineChangeType::Insert));
        assert_eq!(result.lines[1].content, "beautiful\n");

        assert!(matches!(result.lines[2].change_type, LineChangeType::Context));
        assert_eq!(result.lines[2].content, "world\n");
    }
}
