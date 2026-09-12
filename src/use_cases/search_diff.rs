// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use crate::domain::diff_engine::DiffLine;
use crate::domain::diff_search::{DiffSearchEngine, SearchOccurrence};

#[derive(Clone, Debug, Default)]
pub struct SearchDiffUseCase;

impl SearchDiffUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, lines: &[DiffLine], query: &str) -> Vec<SearchOccurrence> {
        let engine = DiffSearchEngine::new();
        engine.find_matches(lines, query)
    }

    pub fn next_match_index(&self, current: usize, total: usize) -> usize {
        if total == 0 {
            0
        } else if current + 1 < total {
            current + 1
        } else {
            0
        }
    }

    pub fn prev_match_index(&self, current: usize, total: usize) -> usize {
        if total == 0 {
            0
        } else if current == 0 {
            total - 1
        } else {
            current - 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::diff_engine::LineChangeType;

    #[test]
    fn test_search_diff_navigation() {
        let uc = SearchDiffUseCase::new();
        assert_eq!(uc.next_match_index(0, 3), 1);
        assert_eq!(uc.next_match_index(1, 3), 2);
        assert_eq!(uc.next_match_index(2, 3), 0); // wrap

        assert_eq!(uc.prev_match_index(0, 3), 2); // wrap
        assert_eq!(uc.prev_match_index(2, 3), 1);
    }

    #[test]
    fn test_search_diff_execution() {
        let uc = SearchDiffUseCase::new();
        let lines = vec![DiffLine {
            change_type: LineChangeType::Context,
            content: "let x = 10;".to_string(),
            old_lineno: Some(1),
            new_lineno: Some(1),
        }];
        let matches = uc.execute(&lines, "let");
        assert_eq!(matches.len(), 1);
    }
}
