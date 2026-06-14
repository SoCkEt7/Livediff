// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use crate::domain::diff_engine::DiffLine;
use std::time::SystemTime;

pub struct FileModification {
    pub path: String,
    pub timestamp: SystemTime,
    pub size: u64,
    pub added: usize,
    pub deleted: usize,
    pub diff_lines: Vec<DiffLine>,
    pub is_binary: bool,
}

pub struct AppStats {
    pub modified: usize,
    pub lines_added: usize,
    pub lines_deleted: usize,
}

#[derive(Clone, Debug)]
pub enum DomainEvent {
    FileChanged { path: String, added: usize, deleted: usize },
    IgnoreAdded { pattern: String },
    HistoryCleared,
}
