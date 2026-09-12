// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

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

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HunkRange {
    pub old_start: usize,
    pub old_lines: usize,
    pub new_start: usize,
    pub new_lines: usize,
    pub start_line_idx: usize,
    pub end_line_idx: usize,
    pub symbol_context: Option<String>,
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

    pub fn detect_hunks(&self, lines: &[DiffLine], context_lines: usize) -> Vec<HunkRange> {
        if lines.is_empty() {
            return Vec::new();
        }

        let change_indices: Vec<usize> = lines
            .iter()
            .enumerate()
            .filter(|(_, line)| {
                matches!(line.change_type, LineChangeType::Insert | LineChangeType::Delete)
            })
            .map(|(idx, _)| idx)
            .collect();

        if change_indices.is_empty() {
            return Vec::new();
        }

        // Group changes that are separated by <= 2 * context_lines
        let mut groups: Vec<Vec<usize>> = Vec::new();
        let mut current_group = vec![change_indices[0]];

        for &idx in &change_indices[1..] {
            let prev = *current_group.last().unwrap();
            if idx.saturating_sub(prev) <= 2 * context_lines + 1 {
                current_group.push(idx);
            } else {
                groups.push(current_group);
                current_group = vec![idx];
            }
        }
        groups.push(current_group);

        let mut hunks = Vec::new();
        for group in groups {
            let first_change = group[0];
            let last_change = *group.last().unwrap();

            let start_line_idx = first_change.saturating_sub(context_lines);
            let end_line_idx = (last_change + context_lines).min(lines.len() - 1);

            let slice = &lines[start_line_idx..=end_line_idx];

            let old_start = slice.iter().find_map(|l| l.old_lineno).unwrap_or(1);
            let new_start = slice.iter().find_map(|l| l.new_lineno).unwrap_or(1);

            let old_lines = slice
                .iter()
                .filter(|l| {
                    matches!(l.change_type, LineChangeType::Context | LineChangeType::Delete)
                })
                .count();
            let new_lines = slice
                .iter()
                .filter(|l| {
                    matches!(l.change_type, LineChangeType::Context | LineChangeType::Insert)
                })
                .count();

            hunks.push(HunkRange {
                old_start,
                old_lines,
                new_start,
                new_lines,
                start_line_idx,
                end_line_idx,
                symbol_context: None,
            });
        }

        hunks
    }

    pub fn compute_folded_lines(&self, lines: &[DiffLine], hunks: &[HunkRange]) -> Vec<DiffLine> {
        if hunks.is_empty() || lines.is_empty() {
            return lines.to_vec();
        }

        let mut folded = Vec::new();
        for (i, hunk) in hunks.iter().enumerate() {
            let symbol_suffix =
                hunk.symbol_context.as_deref().map(|s| format!("  {}", s)).unwrap_or_default();

            let header = format!(
                "@@ -{},{} +{},{} @@ [Hunk {}/{}{}]",
                hunk.old_start,
                hunk.old_lines,
                hunk.new_start,
                hunk.new_lines,
                i + 1,
                hunks.len(),
                symbol_suffix
            );
            folded.push(DiffLine {
                change_type: LineChangeType::Header,
                content: header,
                old_lineno: None,
                new_lineno: None,
            });

            for line in &lines[hunk.start_line_idx..=hunk.end_line_idx] {
                folded.push(line.clone());
            }
        }

        folded
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

    #[test]
    fn test_detect_hunks_and_folding() {
        let engine = DiffEngine::new();
        // 20 lines with changes at line 2 and line 18
        let mut old_lines = Vec::new();
        let mut new_lines = Vec::new();
        for i in 1..=20 {
            old_lines.push(format!("line {}\n", i));
            if i == 2 {
                new_lines.push("line 2 modified\n".to_string());
            } else if i == 18 {
                new_lines.push("line 18 modified\n".to_string());
            } else {
                new_lines.push(format!("line {}\n", i));
            }
        }

        let old_text = old_lines.concat();
        let new_text = new_lines.concat();
        let diff = engine.compute_diff(&old_text, &new_text);

        let hunks = engine.detect_hunks(&diff.lines, 2);
        assert_eq!(hunks.len(), 2);

        assert_eq!(hunks[0].old_start, 1);
        assert_eq!(hunks[1].old_start, 16);

        let folded = engine.compute_folded_lines(&diff.lines, &hunks);
        // Folded should have 2 headers + slice lines
        let header_count =
            folded.iter().filter(|l| l.change_type == LineChangeType::Header).count();
        assert_eq!(header_count, 2);
        assert!(folded.len() < diff.lines.len() + 2);
    }
}
