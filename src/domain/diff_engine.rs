// Copyright (c) 2026 Nyxia. All rights reserved.

use similar::{ChangeTag, TextDiff};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LineChangeType {
    Insert,
    Delete,
    Context,
    Header,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DiffLine {
    pub change_type: LineChangeType,
    pub content: String,
    pub old_lineno: Option<usize>,
    pub new_lineno: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct SplitDiffRow {
    pub old_lineno: Option<usize>,
    pub old_content: Option<String>,
    pub old_change: Option<LineChangeType>,
    pub new_lineno: Option<usize>,
    pub new_content: Option<String>,
    pub new_change: Option<LineChangeType>,
}

pub struct DiffResult {
    pub lines: Vec<DiffLine>,
    pub added: usize,
    pub deleted: usize,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DiffEngine;

impl DiffEngine {
    pub fn new() -> Self {
        DiffEngine
    }

    pub fn compute_diff(&self, old_content: &str, new_content: &str) -> DiffResult {
        self.compute_diff_with_options(old_content, new_content, false)
    }

    pub fn compute_diff_with_options(
        &self,
        old_content: &str,
        new_content: &str,
        ignore_whitespace: bool,
    ) -> DiffResult {
        let mut added = 0;
        let mut deleted = 0;
        let mut lines = Vec::new();

        let (old_clean, new_clean);
        let (old_ref, new_ref) = if ignore_whitespace {
            old_clean = old_content.lines().map(|l| l.trim_end()).collect::<Vec<_>>().join("\n");
            new_clean = new_content.lines().map(|l| l.trim_end()).collect::<Vec<_>>().join("\n");
            (old_clean.as_str(), new_clean.as_str())
        } else {
            (old_content, new_content)
        };

        let diff = TextDiff::from_lines(old_ref, new_ref);

        for change in diff.iter_all_changes() {
            let old_lineno = change.old_index().map(|i| i + 1);
            let new_lineno = change.new_index().map(|i| i + 1);

            match change.tag() {
                ChangeTag::Delete => {
                    deleted += 1;
                    lines.push(DiffLine {
                        change_type: LineChangeType::Delete,
                        content: change.value().to_string(),
                        old_lineno,
                        new_lineno: None,
                    });
                }
                ChangeTag::Insert => {
                    added += 1;
                    lines.push(DiffLine {
                        change_type: LineChangeType::Insert,
                        content: change.value().to_string(),
                        old_lineno: None,
                        new_lineno,
                    });
                }
                ChangeTag::Equal => {
                    lines.push(DiffLine {
                        change_type: LineChangeType::Context,
                        content: change.value().to_string(),
                        old_lineno,
                        new_lineno,
                    });
                }
            }
        }

        DiffResult { lines, added, deleted }
    }

    pub fn compute_split_rows(&self, lines: &[DiffLine]) -> Vec<SplitDiffRow> {
        let mut rows = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            match lines[i].change_type {
                LineChangeType::Context => {
                    rows.push(SplitDiffRow {
                        old_lineno: lines[i].old_lineno,
                        old_content: Some(lines[i].content.clone()),
                        old_change: Some(LineChangeType::Context),
                        new_lineno: lines[i].new_lineno,
                        new_content: Some(lines[i].content.clone()),
                        new_change: Some(LineChangeType::Context),
                    });
                    i += 1;
                }
                LineChangeType::Delete => {
                    let mut deletes = Vec::new();
                    while i < lines.len() && lines[i].change_type == LineChangeType::Delete {
                        deletes.push(&lines[i]);
                        i += 1;
                    }
                    let mut inserts = Vec::new();
                    while i < lines.len() && lines[i].change_type == LineChangeType::Insert {
                        inserts.push(&lines[i]);
                        i += 1;
                    }

                    let max_count = deletes.len().max(inserts.len());
                    for idx in 0..max_count {
                        let del = deletes.get(idx);
                        let ins = inserts.get(idx);
                        rows.push(SplitDiffRow {
                            old_lineno: del.and_then(|d| d.old_lineno),
                            old_content: del.map(|d| d.content.clone()),
                            old_change: del.map(|_| LineChangeType::Delete),
                            new_lineno: ins.and_then(|ins_line| ins_line.new_lineno),
                            new_content: ins.map(|ins_line| ins_line.content.clone()),
                            new_change: ins.map(|_| LineChangeType::Insert),
                        });
                    }
                }
                LineChangeType::Insert => {
                    rows.push(SplitDiffRow {
                        old_lineno: None,
                        old_content: None,
                        old_change: None,
                        new_lineno: lines[i].new_lineno,
                        new_content: Some(lines[i].content.clone()),
                        new_change: Some(LineChangeType::Insert),
                    });
                    i += 1;
                }
                LineChangeType::Header => {
                    rows.push(SplitDiffRow {
                        old_lineno: None,
                        old_content: Some(lines[i].content.clone()),
                        old_change: Some(LineChangeType::Header),
                        new_lineno: None,
                        new_content: Some(lines[i].content.clone()),
                        new_change: Some(LineChangeType::Header),
                    });
                    i += 1;
                }
            }
        }

        rows
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
        assert_eq!(result.lines[0].old_lineno, Some(1));
        assert_eq!(result.lines[0].new_lineno, Some(1));

        assert!(matches!(result.lines[1].change_type, LineChangeType::Insert));
        assert_eq!(result.lines[1].content, "beautiful\n");
        assert_eq!(result.lines[1].old_lineno, None);
        assert_eq!(result.lines[1].new_lineno, Some(2));

        assert!(matches!(result.lines[2].change_type, LineChangeType::Context));
        assert_eq!(result.lines[2].content, "world\n");
        assert_eq!(result.lines[2].old_lineno, Some(2));
        assert_eq!(result.lines[2].new_lineno, Some(3));
    }

    #[test]
    fn test_compute_split_rows() {
        let engine = DiffEngine::new();
        let old = "a\nb\n";
        let new = "a\nc\n";
        let result = engine.compute_diff(old, new);
        let split_rows = engine.compute_split_rows(&result.lines);

        assert_eq!(split_rows.len(), 2);
        assert_eq!(split_rows[0].old_content.as_deref(), Some("a\n"));
        assert_eq!(split_rows[0].new_content.as_deref(), Some("a\n"));
        assert_eq!(split_rows[1].old_content.as_deref(), Some("b\n"));
        assert_eq!(split_rows[1].new_content.as_deref(), Some("c\n"));
    }
}
