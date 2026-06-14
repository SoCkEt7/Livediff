// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use notify::{EventKind, RecursiveMode, Watcher};
use std::path::PathBuf;
use tokio::sync::mpsc;

use crate::app::Event;

#[derive(Clone)]
pub struct WatcherConfig {
    pub root_path: PathBuf,
    pub max_size: u64,
    pub ignore_engine:
        std::sync::Arc<std::sync::RwLock<crate::domain::ignore_engine::IgnoreEngine>>,
}

pub struct FileMonitor {
    config: WatcherConfig,
}

impl FileMonitor {
    pub fn new(config: WatcherConfig) -> Self {
        Self { config }
    }

    pub fn start(self) -> mpsc::Receiver<Event> {
        let (tx, rx) = mpsc::channel(100);
        tokio::spawn(async move {
            if let Err(e) = self.run(tx).await {
                tracing::error!("Watcher error: {:?}", e);
            }
        });
        rx
    }

    async fn run(self, tx: mpsc::Sender<Event>) -> color_eyre::Result<()> {
        let (notify_tx, mut notify_rx) = mpsc::unbounded_channel();

        let mut watcher = notify::RecommendedWatcher::new(
            move |res| {
                if let Ok(event) = res {
                    let _ = notify_tx.send(event);
                }
            },
            notify::Config::default(),
        )?;

        let root_path =
            self.config.root_path.canonicalize().unwrap_or(self.config.root_path.clone());

        watcher.watch(&root_path, RecursiveMode::Recursive)?;
        let _ = tx
            .send(Event::Log(format!("Watcher ready - monitoring {}...", root_path.display())))
            .await;

        let fs_adapter = crate::adapters::fs_adapter::TokioFileSystem;
        let mut session = crate::domain::watcher_session::WatcherSession::new(
            fs_adapter,
            root_path.clone(),
            self.config.max_size,
        );

        // Initial Scan
        let _ = tx.send(Event::Log("Performing initial scan...".to_string())).await;
        let mut walk_stack = vec![root_path.clone()];
        while let Some(dir) = walk_stack.pop() {
            if let Ok(mut entries) = tokio::fs::read_dir(&dir).await {
                while let Some(entry) = entries.next_entry().await.unwrap_or(None) {
                    let path = entry.path();
                    let relative_path = path.strip_prefix(&root_path).unwrap_or(&path);

                    let is_dir = entry.file_type().await.map(|t| t.is_dir()).unwrap_or(false);
                    let is_ignored = if let Ok(engine) = self.config.ignore_engine.read() {
                        engine.is_ignored(&path, relative_path, is_dir)
                    } else {
                        false
                    };

                    if is_ignored {
                        continue;
                    }

                    let meta = match entry.metadata().await {
                        Ok(m) => m,
                        Err(_) => continue,
                    };

                    if meta.is_dir() {
                        walk_stack.push(path);
                        continue;
                    }

                    if meta.is_file()
                        && let Some(modif) = session.process_raw_event(&path).await
                    {
                        let _ = tx.send(Event::FileChanged(modif)).await;
                    }
                }
            }
        }
        let _ = tx.send(Event::Log("Initial scan complete.".to_string())).await;

        while let Some(event) = notify_rx.recv().await {
            if matches!(event.kind, EventKind::Remove(_)) {
                for path in event.paths {
                    let relative_path = path.strip_prefix(&root_path).unwrap_or(&path);
                    let path_str = relative_path.to_string_lossy().to_string();
                    let _ = tx.send(Event::FileDeleted(path_str)).await;
                }
            } else if matches!(
                event.kind,
                EventKind::Modify(_) | EventKind::Create(_) | EventKind::Any
            ) {
                for path in event.paths {
                    let path = tokio::fs::canonicalize(&path).await.unwrap_or(path.clone());
                    let relative_path = path.strip_prefix(&root_path).unwrap_or(&path);

                    let is_dir =
                        tokio::fs::metadata(&path).await.map(|m| m.is_dir()).unwrap_or(false);
                    let is_ignored = if let Ok(engine) = self.config.ignore_engine.read() {
                        engine.is_ignored(&path, relative_path, is_dir)
                    } else {
                        false
                    };

                    if is_ignored {
                        continue;
                    }

                    if let Some(modif) = session.process_raw_event(&path).await {
                        let _ = tx.send(Event::FileChanged(modif)).await;
                    }
                }
            }
        }

        Ok(())
    }
}
