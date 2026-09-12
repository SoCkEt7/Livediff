// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::collections::HashSet;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct IgnoreEngine {
    pub root_path: Option<std::path::PathBuf>,
    pub ignore_list: HashSet<String>,
    pub globset: GlobSet,
    pub gitignore: Gitignore,
    pub no_ignore: bool,
    pub all: bool,
    pub no_ignore_parent: bool,
    pub no_ignore_vcs: bool,
    pub respect_vcs: bool,
    pub ignore_vcs_files: bool,
}

impl IgnoreEngine {
    pub fn new(
        no_ignore: bool,
        all: bool,
        no_ignore_parent: bool,
        no_ignore_vcs: bool,
        ignore_patterns: &[String],
    ) -> Self {
        let mut engine = Self {
            root_path: None,
            ignore_list: HashSet::new(),
            globset: GlobSetBuilder::new().build().unwrap(),
            gitignore: GitignoreBuilder::new("").build().unwrap(),
            no_ignore,
            all,
            no_ignore_parent,
            no_ignore_vcs,
            respect_vcs: !no_ignore,
            ignore_vcs_files: false,
        };

        for pattern in ignore_patterns {
            engine.ignore_list.insert(pattern.clone());
        }
        engine.rebuild_globset();
        engine
    }

    pub fn rebuild_globset(&mut self) {
        let mut builder = GlobSetBuilder::new();
        for pattern in &self.ignore_list {
            let mut glob_str = pattern.clone();
            if glob_str.ends_with('/') {
                glob_str.push_str("**");
            }
            if let Ok(glob) = Glob::new(&glob_str) {
                builder.add(glob);
            }
            if !glob_str.contains("**")
                && let Ok(glob) = Glob::new(&format!("{}/**", glob_str))
            {
                builder.add(glob);
            }
        }
        if let Ok(gs) = builder.build() {
            self.globset = gs;
        }
    }

    pub fn load_vcs_ignores(&mut self, root_path: &Path) -> Vec<String> {
        let mut warnings = Vec::new();
        let canonical_root = root_path.canonicalize().unwrap_or_else(|_| root_path.to_path_buf());
        self.root_path = Some(canonical_root.clone());

        if self.no_ignore || self.all {
            return warnings;
        }

        let mut ignore_builder = GitignoreBuilder::new(&canonical_root);
        let mut ignore_names = vec![".livediffignore", ".ignore", ".rgignore"];
        if !self.no_ignore_vcs {
            ignore_names.push(".gitignore");
        }

        let mut found_git = false;
        for ancestor in canonical_root.ancestors() {
            if self.no_ignore_parent && ancestor != canonical_root.as_path() {
                break;
            }
            for ignore_name in &ignore_names {
                let ignore_path = ancestor.join(ignore_name);
                if ignore_path.exists()
                    && let Some(err) = ignore_builder.add(&ignore_path)
                {
                    warnings.push(format!(
                        "Warning: Failed to load {}: {}",
                        ignore_path.display(),
                        err
                    ));
                }
            }
            if ancestor.join(".git").is_dir() {
                found_git = true;
                break;
            }
        }

        if !found_git {
            let mut local_ignore_builder = GitignoreBuilder::new(&canonical_root);
            for ignore_name in &ignore_names {
                let local_ignore_path = canonical_root.join(ignore_name);
                if local_ignore_path.exists() {
                    let _ = local_ignore_builder.add(&local_ignore_path);
                }
            }
            if let Ok(gi) = local_ignore_builder.build() {
                self.gitignore = gi;
            }
        } else {
            if let Ok(gi) = ignore_builder.build() {
                self.gitignore = gi;
            }
        }

        warnings
    }

    pub fn is_ignored(&self, path: &Path, relative_path: &Path, is_dir: bool) -> bool {
        // Quick first-pass ignore for dot directories and common build dirs
        // for performance and to avoid watching our own artifacts.
        if !self.all {
            let path_str = relative_path.to_string_lossy();
            if path_str.contains(".git/")
                || path_str.contains("node_modules/")
                || path_str.contains("target/")
                || path_str.contains("build/")
            {
                return true;
            }

            if self.ignore_vcs_files && path_str.ends_with(".gitignore") {
                return true;
            }
        }

        // 1. Check custom glob patterns and runtime ignore list
        let path_str = path.to_string_lossy();
        let rel_path_str = relative_path.to_string_lossy();
        if self.globset.is_match(relative_path)
            || self.ignore_list.contains(&path_str.into_owned())
            || self.ignore_list.contains(&rel_path_str.into_owned())
        {
            return true;
        }

        // 2. Check VCS ignore files (.gitignore, .ignore, etc.)
        if self.respect_vcs && !self.all {
            let target_rel = if let Some(root) = &self.root_path {
                if let Ok(stripped) = relative_path.strip_prefix(root) {
                    stripped
                } else if let Ok(stripped) = path.strip_prefix(root) {
                    stripped
                } else {
                    relative_path
                }
            } else {
                relative_path
            };

            let clean_rel = target_rel.strip_prefix("./").unwrap_or(target_rel);
            let clean_rel = clean_rel.strip_prefix("/").unwrap_or(clean_rel);

            if !clean_rel.has_root() && !clean_rel.is_absolute() {
                match self.gitignore.matched_path_or_any_parents(clean_rel, is_dir) {
                    ignore::Match::Ignore(_) => return true,
                    ignore::Match::None => {}
                    ignore::Match::Whitelist(_) => return false,
                }
            }
        }

        false
    }

    pub fn toggle_vcs_respect(&mut self) {
        self.respect_vcs = !self.respect_vcs;
    }

    pub fn add_ignore(&mut self, pattern: String) {
        self.ignore_list.insert(pattern);
        self.rebuild_globset();
    }

    pub fn remove_ignore(&mut self, pattern: &str) {
        if self.ignore_list.remove(pattern) {
            self.rebuild_globset();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gitignored_dir_contents_are_ignored() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join(".gitignore"), "ignored_dir/\n").unwrap();

        let mut engine = IgnoreEngine::new(false, false, false, false, &[]);
        engine.load_vcs_ignores(tmp.path());

        // Watch events arrive per file, so a directory rule has to match
        // through the parent chain, not just against the exact path.
        let inside = tmp.path().join("ignored_dir/sub/file.txt");
        assert!(engine.is_ignored(&inside, Path::new("ignored_dir/sub/file.txt"), false));

        let kept = tmp.path().join("kept.txt");
        assert!(!engine.is_ignored(&kept, Path::new("kept.txt"), false));
    }

    #[test]
    fn test_livediffignore_file_is_respected() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join(".livediffignore"), "*.secret\ncustom_build/\n").unwrap();

        let mut engine = IgnoreEngine::new(false, false, false, false, &[]);
        engine.load_vcs_ignores(tmp.path());

        let secret = tmp.path().join("keys.secret");
        assert!(engine.is_ignored(&secret, Path::new("keys.secret"), false));

        let inside_dir = tmp.path().join("custom_build/out.bin");
        assert!(engine.is_ignored(&inside_dir, Path::new("custom_build/out.bin"), false));

        let public_file = tmp.path().join("main.rs");
        assert!(!engine.is_ignored(&public_file, Path::new("main.rs"), false));
    }

    #[test]
    fn test_absolute_and_external_paths_do_not_panic() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join(".gitignore"), "target/\n*.tmp\n").unwrap();

        let mut engine = IgnoreEngine::new(false, false, false, false, &[]);
        // Test before loading VCS ignores with absolute path
        assert!(!engine.is_ignored(
            Path::new("/tmp/outside.txt"),
            Path::new("/tmp/outside.txt"),
            false
        ));

        engine.load_vcs_ignores(tmp.path());

        // Absolute path under root (should match target/)
        let inside_target = tmp.path().join("target/debug/app");
        assert!(engine.is_ignored(&inside_target, &inside_target, false));

        // Absolute path outside root (must not panic)
        let outside = Path::new("/etc/hosts");
        assert!(!engine.is_ignored(outside, outside, false));

        // Path with leading slash (must not panic)
        let leading_slash = Path::new("/target/debug/app");
        // Custom check without panic
        let _ = engine.is_ignored(leading_slash, leading_slash, false);
    }
}
