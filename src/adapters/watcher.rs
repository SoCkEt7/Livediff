// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use notify::{EventKind, RecursiveMode, Watcher};
use std::path::PathBuf;
use tokio::sync::mpsc;

use crate::app::Event;
use crate::domain::interfaces::FileSystemPort;

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

        let root_path_clone = root_path.clone();
        let ignore_engine_clone = self.config.ignore_engine.clone();

        let paths = tokio::task::spawn_blocking(move || {
            let mut paths = Vec::new();
            let engine = match ignore_engine_clone.read() {
                Ok(e) => e,
                Err(_) => return paths,
            };

            let mut builder = ignore::WalkBuilder::new(&root_path_clone);
            builder.hidden(!engine.all); // ignores hidden files if all is false
            builder.parents(!engine.no_ignore_parent);
            builder.ignore(!engine.no_ignore);
            builder.git_ignore(!engine.no_ignore_vcs);
            builder.git_exclude(!engine.no_ignore_vcs);
            builder.git_global(!engine.no_ignore_vcs);

            let walker = builder.build();
            for entry in walker.flatten() {
                let path = entry.path().to_path_buf();
                if path == root_path_clone {
                    continue;
                }
                let relative_path = path.strip_prefix(&root_path_clone).unwrap_or(&path);
                let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);

                let is_ignored = engine.is_ignored(&path, relative_path, is_dir);
                if !is_ignored && !is_dir {
                    paths.push(path);
                }
            }
            paths
        })
        .await
        .unwrap_or_default();

        let total_files = paths.len();
        let _ = tx.send(Event::Log(format!("Found {} files to check.", total_files))).await;

        let mut join_set = tokio::task::JoinSet::new();
        for path in paths {
            let old_content = session.get_cached(&path).unwrap_or_default();
            let max_size = self.config.max_size;

            join_set.spawn(async move {
                let fs = crate::adapters::fs_adapter::TokioFileSystem;
                let meta = match fs.metadata(&path).await {
                    Ok(m) => m,
                    Err(_) => return None,
                };
                if !meta.is_file || meta.len > max_size {
                    return None;
                }

                let bytes = match fs.read_file(&path).await {
                    Ok(b) => b,
                    Err(_) => return None,
                };

                let (is_binary, new_content) = match String::from_utf8(bytes) {
                    Ok(s) => (false, s),
                    Err(_) => (true, "[Binary File]".to_string()),
                };

                if new_content == old_content && !old_content.is_empty() {
                    return Some((path, new_content, None));
                }

                let diff_engine = crate::domain::diff_engine::DiffEngine::new();
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
                    diff_engine.compute_diff(&old_content, &new_content)
                };

                let relative_path = path.to_string_lossy().to_string();
                let modif = crate::domain::entities::FileModification {
                    path: relative_path,
                    timestamp: meta.modified,
                    size: meta.len,
                    added: diff_result.added,
                    deleted: diff_result.deleted,
                    diff_lines: diff_result.lines,
                    is_binary,
                };

                Some((path, new_content, Some(modif)))
            });
        }

        while let Some(res) = join_set.join_next().await {
            if let Ok(Some((path, new_content, opt_modif))) = res {
                session.insert_cached(path, new_content);
                if let Some(mut modif) = opt_modif {
                    let path_buf = std::path::PathBuf::from(&modif.path);
                    let relative_path = path_buf.strip_prefix(&root_path).unwrap_or(&path_buf);
                    modif.path = relative_path.to_string_lossy().to_string();

                    let _ = tx
                        .send(Event::FileChanged {
                            modification: modif,
                            total_files: session.file_count(),
                        })
                        .await;
                }
            }
        }

        let _ = tx.send(Event::TotalFilesUpdated(session.file_count())).await;
        let _ = tx
            .send(Event::Log(format!(
                "Initial scan complete. Monitoring {} files.",
                session.file_count()
            )))
            .await;

        while let Some(event) = notify_rx.recv().await {
            if matches!(event.kind, EventKind::Remove(_)) {
                for path in event.paths {
                    session.remove_file(&path);
                    let relative_path = path.strip_prefix(&root_path).unwrap_or(&path);
                    let path_str = relative_path.to_string_lossy().to_string();
                    let _ = tx
                        .send(Event::FileDeleted {
                            path: path_str,
                            total_files: session.file_count(),
                        })
                        .await;
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
                        let _ = tx
                            .send(Event::FileChanged {
                                modification: modif,
                                total_files: session.file_count(),
                            })
                            .await;
                    }
                }
            }
        }

        Ok(())
    }
}
