// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use crate::domain::diff_engine::HunkRange;

#[derive(Clone, Copy, Debug, Default)]
pub struct NavigateHunksUseCase;

impl NavigateHunksUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn next_hunk_index(&self, current: usize, total: usize) -> usize {
        if total == 0 {
            0
        } else if current + 1 < total {
            current + 1
        } else {
            total - 1
        }
    }

    pub fn prev_hunk_index(&self, current: usize, total: usize) -> usize {
        if total == 0 { 0 } else { current.saturating_sub(1) }
    }

    pub fn calculate_scroll_offset(
        &self,
        hunk_idx: usize,
        hunks: &[HunkRange],
        folded: bool,
    ) -> usize {
        if hunks.is_empty() || hunk_idx >= hunks.len() {
            return 0;
        }

        if folded {
            let mut offset = 0;
            for hunk in hunks.iter().take(hunk_idx) {
                let hunk_line_count = (hunk.end_line_idx - hunk.start_line_idx + 1) + 1;
                offset += hunk_line_count;
            }
            offset
        } else {
            hunks[hunk_idx].start_line_idx
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hunk_navigation_bounds() {
        let uc = NavigateHunksUseCase::new();
        assert_eq!(uc.next_hunk_index(0, 3), 1);
        assert_eq!(uc.next_hunk_index(1, 3), 2);
        assert_eq!(uc.next_hunk_index(2, 3), 2);

        assert_eq!(uc.prev_hunk_index(2, 3), 1);
        assert_eq!(uc.prev_hunk_index(1, 3), 0);
        assert_eq!(uc.prev_hunk_index(0, 3), 0);
    }

    #[test]
    fn test_scroll_offset_calculation() {
        let uc = NavigateHunksUseCase::new();
        let hunks = vec![
            HunkRange {
                old_start: 1,
                old_lines: 5,
                new_start: 1,
                new_lines: 6,
                start_line_idx: 0,
                end_line_idx: 5,
                symbol_context: None,
            },
            HunkRange {
                old_start: 20,
                old_lines: 4,
                new_start: 21,
                new_lines: 5,
                start_line_idx: 20,
                end_line_idx: 24,
                symbol_context: None,
            },
        ];

        // Unfolded mode
        assert_eq!(uc.calculate_scroll_offset(0, &hunks, false), 0);
        assert_eq!(uc.calculate_scroll_offset(1, &hunks, false), 20);

        // Folded mode
        assert_eq!(uc.calculate_scroll_offset(0, &hunks, true), 0);
        // Hunk 0 has (5-0+1) + 1 = 7 lines (header + 6 diff lines)
        assert_eq!(uc.calculate_scroll_offset(1, &hunks, true), 7);
    }
}
