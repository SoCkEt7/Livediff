// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use crate::domain::interfaces::{FileMeta, FileSystemPort};
use std::path::Path;

pub struct TokioFileSystem;

impl FileSystemPort for TokioFileSystem {
    async fn read_file(&self, path: &Path) -> std::io::Result<Vec<u8>> {
        tokio::fs::read(path).await
    }
    async fn metadata(&self, path: &Path) -> std::io::Result<FileMeta> {
        let meta = tokio::fs::metadata(path).await?;
        Ok(FileMeta {
            is_file: meta.is_file(),
            len: meta.len(),
            modified: meta.modified().unwrap_or_else(|_| std::time::SystemTime::now()),
        })
    }
}
