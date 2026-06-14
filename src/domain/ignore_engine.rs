// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::collections::HashSet;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct IgnoreEngine {
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
        if self.no_ignore || self.all {
            return warnings;
        }

        let mut ignore_builder = GitignoreBuilder::new(root_path);
        let mut ignore_names = vec![".ignore", ".rgignore"];
        if !self.no_ignore_vcs {
            ignore_names.push(".gitignore");
        }

        let mut found_git = false;
        for ancestor in root_path.ancestors() {
            if self.no_ignore_parent && ancestor != root_path {
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
            let mut local_ignore_builder = GitignoreBuilder::new(root_path);
            for ignore_name in &ignore_names {
                let local_ignore_path = root_path.join(ignore_name);
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
            match self.gitignore.matched(relative_path, is_dir) {
                ignore::Match::Ignore(_) => return true,
                ignore::Match::None => {}
                ignore::Match::Whitelist(_) => return false,
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
