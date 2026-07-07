// Git status integration for livediff
// Queries git for branch info and file status via CLI (zero dependency)

use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Default)]
pub struct GitInfo {
    pub branch: String,
    pub ahead: usize,
    pub behind: usize,
    pub dirty: bool,
    pub file_statuses: HashMap<String, GitFileStatus>,
    pub is_git_repo: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitFileStatus {
    /// Staged (index has changes)
    Staged,
    /// Modified in working tree
    Modified,
    /// Untracked
    Untracked,
    /// Deleted
    Deleted,
    /// Renamed
    Renamed,
    /// Tracked and unchanged
    Clean,
}

impl GitInfo {
    pub fn refresh(root: &Path) -> Self {
        let is_git_repo = Command::new("git")
            .args(["rev-parse", "--is-inside-work-tree"])
            .current_dir(root)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if !is_git_repo {
            return GitInfo { is_git_repo: false, ..Default::default() };
        }

        let branch = Self::get_branch(root);
        let (ahead, behind) = Self::get_ahead_behind(root);
        let dirty = Self::is_dirty(root);
        let file_statuses = Self::get_file_statuses(root);

        GitInfo { branch, ahead, behind, dirty, file_statuses, is_git_repo: true }
    }

    fn get_branch(root: &Path) -> String {
        Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .current_dir(root)
            .output()
            .ok()
            .and_then(|o| {
                if o.status.success() {
                    Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
                } else {
                    // Maybe detached HEAD
                    Command::new("git")
                        .args(["rev-parse", "--short", "HEAD"])
                        .current_dir(root)
                        .output()
                        .ok()
                        .and_then(|o| {
                            if o.status.success() {
                                Some(format!(
                                    "detached@{}",
                                    String::from_utf8_lossy(&o.stdout).trim()
                                ))
                            } else {
                                None
                            }
                        })
                }
            })
            .unwrap_or_else(|| "unknown".to_string())
    }

    fn get_ahead_behind(root: &Path) -> (usize, usize) {
        let output = Command::new("git")
            .args(["rev-list", "--left-right", "--count", "HEAD...@{upstream}"])
            .current_dir(root)
            .output();

        match output {
            Ok(o) if o.status.success() => {
                let s = String::from_utf8_lossy(&o.stdout);
                let parts: Vec<&str> = s.trim().split('\t').collect();
                if parts.len() == 2 {
                    let ahead = parts[0].parse().unwrap_or(0);
                    let behind = parts[1].parse().unwrap_or(0);
                    (ahead, behind)
                } else {
                    (0, 0)
                }
            }
            _ => (0, 0), // No upstream or no remote
        }
    }

    fn is_dirty(root: &Path) -> bool {
        Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(root)
            .output()
            .map(|o| !o.stdout.is_empty())
            .unwrap_or(false)
    }

    #[allow(clippy::collapsible_if)]
    fn get_file_statuses(root: &Path) -> HashMap<String, GitFileStatus> {
        let mut map = HashMap::new();

        // Porcelain v1: XY <file>
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(root)
            .output();

        if let Ok(o) = output {
            if o.status.success() {
                for line in String::from_utf8_lossy(&o.stdout).lines() {
                    if line.len() < 4 {
                        continue;
                    }
                    let (xy, file) = line.split_at(2);
                    let file = file.trim();
                    let xy = xy.trim();

                    match xy {
                        "M" | "MM" => {
                            map.insert(file.to_string(), GitFileStatus::Modified);
                        }
                        "A" | "AM" | "AD" => {
                            map.insert(file.to_string(), GitFileStatus::Staged);
                        }
                        "D" | "DM" => {
                            map.insert(file.to_string(), GitFileStatus::Deleted);
                        }
                        "??" => {
                            map.insert(file.to_string(), GitFileStatus::Untracked);
                        }
                        "R" | "RM" => {
                            map.insert(file.to_string(), GitFileStatus::Renamed);
                        }
                        _ if xy.contains('M') => {
                            map.insert(file.to_string(), GitFileStatus::Modified);
                        }
                        _ if xy.contains('A') => {
                            map.insert(file.to_string(), GitFileStatus::Staged);
                        }
                        _ if xy.contains('D') => {
                            map.insert(file.to_string(), GitFileStatus::Deleted);
                        }
                        _ => {}
                    }
                }
            }
        }
        map
    }

    pub fn get_status_for(&self, path: &str) -> Option<GitFileStatus> {
        // Try exact match first, then suffix match for relative paths
        if let Some(status) = self.file_statuses.get(path) {
            return Some(*status);
        }
        // Try matching by path suffix (for relative vs absolute)
        self.file_statuses.iter().find_map(|(k, v)| {
            if k.ends_with(path) || path.ends_with(k) {
                Some(*v)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_parsing() {
        // Test porcelain parsing via a temp git repo
        let dir = std::env::temp_dir().join("livediff-git-test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        // Init git repo
        Command::new("git").args(["init"]).current_dir(&dir).output().unwrap();
        Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(&dir)
            .output()
            .unwrap();
        Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(&dir)
            .output()
            .unwrap();

        // Create and commit a file
        std::fs::write(dir.join("test.txt"), "hello").unwrap();
        Command::new("git").args(["add", "."]).current_dir(&dir).output().unwrap();
        Command::new("git")
            .args(["commit", "-m", "initial"])
            .current_dir(&dir)
            .output()
            .unwrap();

        // Modify
        std::fs::write(dir.join("test.txt"), "world").unwrap();
        // Create untracked
        std::fs::write(dir.join("new.txt"), "new").unwrap();

        let info = GitInfo::refresh(&dir);
        assert_eq!(info.branch, "main");
        assert!(info.dirty);
        assert_eq!(info.get_status_for("test.txt"), Some(GitFileStatus::Modified));
        assert_eq!(info.get_status_for("new.txt"), Some(GitFileStatus::Untracked));

        let _ = std::fs::remove_dir_all(&dir);
    }
}
