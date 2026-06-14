use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tokio::sync::mpsc;
use notify::{Watcher, RecursiveMode, EventKind};
use similar::{ChangeTag, TextDiff};
use ignore::gitignore::{GitignoreBuilder};
use ignore::Match;

use crate::app::{Event, FileModification};

#[derive(Clone, Debug)]
pub struct WatcherConfig {
    pub root_path: PathBuf,
    pub all: bool,
    pub no_ignore: bool,
    pub max_size: u64,
}

pub async fn run_watcher(tx: mpsc::Sender<Event>, config: WatcherConfig) -> anyhow::Result<()> {
    let (notify_tx, mut notify_rx) = mpsc::unbounded_channel();

    let mut watcher = notify::RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                let _ = notify_tx.send(event);
            }
        },
        notify::Config::default()
    )?;

    let root_path = config.root_path.canonicalize().unwrap_or(config.root_path.clone());
    
    watcher.watch(&root_path, RecursiveMode::Recursive)?;
    let _ = tx.send(Event::Log(format!("Watcher ready - monitoring {}...", root_path.display()))).await;

    // Load ignore files
    let gitignore = if !config.no_ignore && !config.all {
        let mut ignore_builder = GitignoreBuilder::new(&root_path);
        let ignore_names = [".gitignore", ".ignore", ".rgignore"];
        
        // Search upwards for ignore files (standard git behavior)
        let mut found_git = false;
        for ancestor in root_path.ancestors() {
            for ignore_name in &ignore_names {
                let ignore_path = ancestor.join(ignore_name);
                if ignore_path.exists() {
                    if let Some(err) = ignore_builder.add(&ignore_path) {
                        let _ = tx.send(Event::Log(format!("Warning: Failed to load {}: {}", ignore_path.display(), err))).await;
                    }
                }
            }
            
            // Stop at .git directory or if we reached root
            if ancestor.join(".git").is_dir() {
                found_git = true;
                break;
            }
        }
        
        // If not in a git repo, we only use the local ignore files to avoid over-reaching parent ignores
        if !found_git {
            let mut local_ignore_builder = GitignoreBuilder::new(&root_path);
            for ignore_name in &ignore_names {
                let local_ignore_path = root_path.join(ignore_name);
                if local_ignore_path.exists() {
                    let _ = local_ignore_builder.add(&local_ignore_path);
                }
            }
            local_ignore_builder.build().unwrap_or_else(|_| GitignoreBuilder::new(&root_path).build().unwrap())
        } else {
            ignore_builder.build().unwrap_or_else(|_| GitignoreBuilder::new(&root_path).build().unwrap())
        }
    } else {
        GitignoreBuilder::new(&root_path).build().unwrap()
    };

    let mut file_cache: HashMap<PathBuf, String> = HashMap::new();

    // Initial Scan
    let _ = tx.send(Event::Log("Performing initial scan...".to_string())).await;
    let mut walk_stack = vec![root_path.clone()];
    while let Some(dir) = walk_stack.pop() {
        if let Ok(mut entries) = tokio::fs::read_dir(&dir).await {
            while let Some(entry) = entries.next_entry().await.unwrap_or(None) {
                let path = entry.path();
                let relative_path = path.strip_prefix(&root_path).unwrap_or(&path);
                
                if should_filter(&path, relative_path, &gitignore, &config) {
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

                if meta.is_file() {
                    process_file_change(&path, &root_path, &mut file_cache, &tx, &config).await;
                }
            }
        }
    }
    let _ = tx.send(Event::Log("Initial scan complete.".to_string())).await;

    while let Some(event) = notify_rx.recv().await {
        if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_) | EventKind::Any) {
            for path in event.paths {
                // Canonicalize event path to match root_path
                let path = path.canonicalize().unwrap_or(path);
                let relative_path = path.strip_prefix(&root_path).unwrap_or(&path);

                if should_filter(&path, relative_path, &gitignore, &config) {
                    continue;
                }

                process_file_change(&path, &root_path, &mut file_cache, &tx, &config).await;
            }
        }
    }

    Ok(())
}

fn should_filter(path: &Path, relative_path: &Path, gitignore: &ignore::gitignore::Gitignore, config: &WatcherConfig) -> bool {
    // Quick first-pass ignore for dot directories and common build dirs
    // for performance and to avoid watching our own artifacts.
    if !config.all {
        let path_str = relative_path.to_string_lossy();
        if path_str.contains(".git/") 
            || path_str.contains("node_modules/")
            || path_str.contains("target/")
            || path_str.contains("build/")
        {
            return true;
        }

        // Gitignore check
        if !config.no_ignore {
            if let Match::Ignore(_) = gitignore.matched(relative_path, path.is_dir()) {
                return true;
            }
        }
    }
    false
}

async fn process_file_change(
    path: &PathBuf, 
    root_path: &PathBuf,
    file_cache: &mut HashMap<PathBuf, String>,
    tx: &mpsc::Sender<Event>,
    config: &WatcherConfig
) {
    // Metadata check (skip directories, large files, and check existence)
    let meta = match tokio::fs::metadata(path).await {
        Ok(m) => m,
        Err(_) => return,
    };

    if !meta.is_file() || meta.len() > config.max_size {
        return;
    }

    let is_binary;
    let new_content = match tokio::fs::read(path).await {
        Ok(bytes) => {
            match String::from_utf8(bytes) {
                Ok(s) => {
                    is_binary = false;
                    s
                },
                Err(_) => {
                    is_binary = true;
                    "[Binary File]".to_string()
                }
            }
        },
        Err(_) => return,
    };

    let old_content = file_cache.get(path).cloned().unwrap_or_default();
    
    if new_content == old_content && !old_content.is_empty() {
        return;
    }

    let mut added = 0;
    let mut deleted = 0;
    let mut colored_diff = String::new();

    if is_binary {
        colored_diff = "  (Binary file content hidden)".to_string();
        added = 0;
        deleted = 0;
    } else {
        let diff = TextDiff::from_lines(&old_content, &new_content);
        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Delete => {
                    deleted += 1;
                    colored_diff.push_str(&format!("- {}", change.value()));
                }
                ChangeTag::Insert => {
                    added += 1;
                    colored_diff.push_str(&format!("+ {}", change.value()));
                }
                ChangeTag::Equal => {
                    colored_diff.push_str(&format!("  {}", change.value()));
                }
            }
        }
    }

    file_cache.insert(path.clone(), new_content);

    let timestamp = meta.modified().unwrap_or_else(|_| SystemTime::now());
    let relative_path = path.strip_prefix(root_path).unwrap_or(path);
    let display_path = relative_path.to_string_lossy().to_string();

    let _ = tx.send(Event::FileChanged(FileModification {
        path: display_path,
        timestamp,
        size: meta.len(),
        added,
        deleted,
        diff: colored_diff,
        is_binary,
    })).await;
}
