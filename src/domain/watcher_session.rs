// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use crate::domain::diff_engine::DiffEngine;
use crate::domain::entities::FileModification;
use crate::domain::interfaces::FileSystemPort;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct WatcherSession<F: FileSystemPort> {
    fs: F,
    diff_engine: DiffEngine,
    file_cache: HashMap<PathBuf, String>,
    max_size: u64,
    root_path: PathBuf,
}

impl<F: FileSystemPort> WatcherSession<F> {
    pub fn new(fs: F, root_path: PathBuf, max_size: u64) -> Self {
        Self { fs, diff_engine: DiffEngine::new(), file_cache: HashMap::new(), max_size, root_path }
    }

    pub fn remove_file(&mut self, path: &std::path::Path) {
        self.file_cache.remove(path);
    }

    pub fn file_count(&self) -> usize {
        self.file_cache.len()
    }

    pub fn get_cached(&self, path: &PathBuf) -> Option<String> {
        self.file_cache.get(path).cloned()
    }

    pub fn insert_cached(&mut self, path: PathBuf, content: String) {
        self.file_cache.insert(path, content);
    }

    pub async fn process_raw_event(&mut self, path: &PathBuf) -> Option<FileModification> {
        let meta = self.fs.metadata(path).await.ok()?;
        if !meta.is_file || meta.len > self.max_size {
            return None;
        }

        let bytes = self.fs.read_file(path).await.ok()?;
        let (is_binary, new_content) = match String::from_utf8(bytes) {
            Ok(s) => (false, s),
            Err(_) => (true, "[Binary File]".to_string()),
        };

        let old_content = self.file_cache.get(path).cloned().unwrap_or_default();
        if new_content == old_content && !old_content.is_empty() {
            return None;
        }

        let diff_result = if is_binary {
            crate::domain::diff_engine::DiffResult {
                lines: vec![crate::domain::diff_engine::DiffLine {
                    change_type: crate::domain::diff_engine::LineChangeType::Context,
                    content: "(Binary file content hidden)".to_string(),
                }],
                added: 0,
                deleted: 0,
            }
        } else {
            self.diff_engine.compute_diff(&old_content, &new_content)
        };

        self.file_cache.insert(path.clone(), new_content);

        let relative_path = path.strip_prefix(&self.root_path).unwrap_or(path);

        Some(FileModification {
            path: relative_path.to_string_lossy().to_string(),
            timestamp: meta.modified,
            size: meta.len,
            added: diff_result.added,
            deleted: diff_result.deleted,
            diff_lines: diff_result.lines,
            is_binary,
        })
    }
}
