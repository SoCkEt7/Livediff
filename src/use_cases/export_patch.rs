// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use crate::domain::diff_engine::LineChangeType;
use crate::domain::entities::FileModification;
use std::path::{Path, PathBuf};

pub struct ExportPatchUseCase;

impl Default for ExportPatchUseCase {
    fn default() -> Self {
        Self::new()
    }
}

impl ExportPatchUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn format_patch(&self, modification: &FileModification) -> String {
        let mut patch_content = String::new();
        patch_content.push_str(&format!("--- a/{}\n", modification.path));
        patch_content.push_str(&format!("+++ b/{}\n", modification.path));

        for line in &modification.diff_lines {
            match line.change_type {
                LineChangeType::Insert => {
                    patch_content.push('+');
                    patch_content.push_str(&line.content);
                }
                LineChangeType::Delete => {
                    patch_content.push('-');
                    patch_content.push_str(&line.content);
                }
                LineChangeType::Context => {
                    patch_content.push(' ');
                    patch_content.push_str(&line.content);
                }
                LineChangeType::Header => {
                    patch_content.push_str(&line.content);
                }
            }
            if !line.content.ends_with('\n') {
                patch_content.push('\n');
            }
        }

        patch_content
    }

    pub fn execute(
        &self,
        modification: &FileModification,
        destination_dir: &Path,
    ) -> Result<PathBuf, String> {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let safe_filename = modification.path.replace(['/', '\\'], "_");
        let patch_filename = format!("{}_{}.patch", safe_filename, timestamp);
        let patch_path = destination_dir.join(&patch_filename);

        let patch_content = self.format_patch(modification);

        std::fs::write(&patch_path, patch_content)
            .map_err(|e| format!("Failed to write patch: {}", e))?;

        Ok(patch_path)
    }
}
