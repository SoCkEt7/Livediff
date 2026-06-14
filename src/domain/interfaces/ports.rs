// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use std::path::Path;

pub struct FileMeta {
    pub is_file: bool,
    pub len: u64,
    pub modified: std::time::SystemTime,
}

#[allow(async_fn_in_trait)]
pub trait FileSystemPort: Send + Sync {
    async fn read_file(&self, path: &Path) -> std::io::Result<Vec<u8>>;
    async fn metadata(&self, path: &Path) -> std::io::Result<FileMeta>;
}
