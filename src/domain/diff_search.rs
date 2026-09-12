// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use crate::domain::diff_engine::DiffLine;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchOccurrence {
    pub line_idx: usize,
    pub col_start: usize,
    pub col_end: usize,
}

#[derive(Clone, Debug, Default)]
pub struct DiffSearchEngine;

impl DiffSearchEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn find_matches(&self, lines: &[DiffLine], query: &str) -> Vec<SearchOccurrence> {
        if query.is_empty() || lines.is_empty() {
            return Vec::new();
        }

        let query_lower = query.to_lowercase();
        let mut occurrences = Vec::new();

        for (line_idx, line) in lines.iter().enumerate() {
            let content_lower = line.content.to_lowercase();
            let mut start = 0;
            while let Some(found_pos) = content_lower[start..].find(&query_lower) {
                let actual_start = start + found_pos;
                let actual_end = actual_start + query.len();
                occurrences.push(SearchOccurrence {
                    line_idx,
                    col_start: actual_start,
                    col_end: actual_end,
                });
                start = actual_start + query.len().max(1);
                if start >= content_lower.len() {
                    break;
                }
            }
        }

        occurrences
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::diff_engine::LineChangeType;

    #[test]
    fn test_find_matches_case_insensitive() {
        let engine = DiffSearchEngine::new();
        let lines = vec![
            DiffLine {
                change_type: LineChangeType::Context,
                content: "fn test_function() {\n".to_string(),
                old_lineno: Some(1),
                new_lineno: Some(1),
            },
            DiffLine {
                change_type: LineChangeType::Insert,
                content: "    println!(\"TESTing function\");\n".to_string(),
                old_lineno: None,
                new_lineno: Some(2),
            },
        ];

        let matches = engine.find_matches(&lines, "test");
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].line_idx, 0);
        assert_eq!(matches[0].col_start, 3);
        assert_eq!(matches[0].col_end, 7);

        assert_eq!(matches[1].line_idx, 1);
        assert_eq!(matches[1].col_start, 14);
        assert_eq!(matches[1].col_end, 18);
    }
}
