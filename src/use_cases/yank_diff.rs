// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use crate::domain::entities::FileModification;
use crate::use_cases::export_patch::ExportPatchUseCase;

pub struct YankDiffUseCase;

impl Default for YankDiffUseCase {
    fn default() -> Self {
        Self::new()
    }
}

impl YankDiffUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, modification: &FileModification) -> Result<usize, String> {
        let patch_text = ExportPatchUseCase::new().format_patch(modification);
        let char_count = patch_text.len();

        let mut clipboard =
            arboard::Clipboard::new().map_err(|e| format!("Clipboard unavailable: {}", e))?;

        clipboard
            .set_text(patch_text)
            .map_err(|e| format!("Failed to copy to clipboard: {}", e))?;

        Ok(char_count)
    }
}
