// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::{IncrementalIgnore, WalkBuilder};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tracing::warn;

/// Ignore-file names that are specific to livediff/ripgrep. `.ignore` is handled
/// natively by the `ignore` crate via `WalkBuilder::ignore`.
pub const CUSTOM_IGNORE_FILES: [&str; 2] = [".livediffignore", ".rgignore"];

pub struct IgnoreEngine {
    pub root_path: Option<PathBuf>,
    pub ignore_list: HashSet<String>,
    pub globset: GlobSet,
    /// Hierarchical ignore matcher. Unlike a flat `Gitignore`, this resolves
    /// per-directory ignore files at any depth, so a `.gitignore` in a
    /// subdirectory applies to that subtree with its patterns anchored to its
    /// own directory. Matchers are loaded lazily and cached per directory.
    ///
    /// `IncrementalIgnore::matched` needs `&mut self`, but callers hold only a
    /// read lock on the engine, so the matcher carries its own `Mutex`.
    matcher: Mutex<Option<IncrementalIgnore>>,
    pub no_ignore: bool,
    pub all: bool,
    pub no_ignore_parent: bool,
    pub no_ignore_vcs: bool,
    pub respect_vcs: bool,
    pub ignore_vcs_files: bool,
}

impl std::fmt::Debug for IgnoreEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IgnoreEngine")
            .field("root_path", &self.root_path)
            .field("ignore_list", &self.ignore_list)
            .field("no_ignore", &self.no_ignore)
            .field("all", &self.all)
            .field("no_ignore_parent", &self.no_ignore_parent)
            .field("no_ignore_vcs", &self.no_ignore_vcs)
            .field("respect_vcs", &self.respect_vcs)
            .field("ignore_vcs_files", &self.ignore_vcs_files)
            .finish_non_exhaustive()
    }
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
            matcher: Mutex::new(None),
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

    /// The single source of truth for how ignore files are discovered.
    ///
    /// Both the initial directory scan and the live per-event checks are built
    /// from this, so a file cannot be skipped at startup yet reported on change.
    pub fn walk_builder(&self, root: &Path) -> WalkBuilder {
        let mut builder = WalkBuilder::new(root);
        let use_ignores = !self.no_ignore;

        builder.hidden(!self.all);
        // livediff monitors plain directories too, not just git checkouts, so
        // ignore files are honoured even when there is no repository around.
        builder.require_git(false);
        builder.parents(use_ignores && !self.no_ignore_parent);
        builder.ignore(use_ignores);
        builder.git_ignore(use_ignores && !self.no_ignore_vcs);
        builder.git_exclude(use_ignores && !self.no_ignore_vcs);
        builder.git_global(use_ignores && !self.no_ignore_vcs);

        if use_ignores {
            for name in CUSTOM_IGNORE_FILES {
                builder.add_custom_ignore_filename(name);
            }
        }
        builder
    }

    pub fn load_vcs_ignores(&mut self, root_path: &Path) -> Vec<String> {
        let canonical_root = root_path.canonicalize().unwrap_or_else(|_| root_path.to_path_buf());
        self.root_path = Some(canonical_root.clone());

        if self.no_ignore {
            *self.matcher.lock().unwrap_or_else(|e| e.into_inner()) = None;
            return Vec::new();
        }

        // `build_matchers` yields one matcher per configured root; we only ever
        // configure the watch root.
        let matcher = self.walk_builder(&canonical_root).build_matchers().into_iter().next();
        if matcher.is_none() {
            return vec![format!(
                "Warning: could not build ignore matcher for {}",
                canonical_root.display()
            )];
        }
        *self.matcher.lock().unwrap_or_else(|e| e.into_inner()) = matcher;

        Vec::new()
    }

    pub fn is_ignored(&self, path: &Path, relative_path: &Path, is_dir: bool) -> bool {
        // Quick first-pass ignore for common build dirs, for performance and to
        // avoid watching our own artifacts. `--no-ignore` opts out of it.
        if !self.no_ignore {
            let path_str = relative_path.to_string_lossy();
            if path_str.contains(".git/")
                || path_str.contains("node_modules/")
                || path_str.contains("target/")
                || path_str.contains("build/")
            {
                return true;
            }
        }

        // Explicit user toggle, independent of the ignore-file configuration.
        if self.ignore_vcs_files && relative_path.to_string_lossy().ends_with(".gitignore") {
            return true;
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

        // 2. Check hierarchical ignore files (.gitignore, .livediffignore, ...)
        //    at every level between the watch root and the file itself.
        if self.respect_vcs {
            let mut guard = match self.matcher.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            if let Some(matcher) = guard.as_mut() {
                // `matched` expects a path relative to the matcher root. Try the
                // absolute path first, then the caller's relative path; anything
                // outside the root normalizes to `None` and is left alone.
                let target = matcher.normalize(path).or_else(|| matcher.normalize(relative_path));

                if let Some(target) = target {
                    let (matched, err) = matcher.matched_with_errors(&target, is_dir);
                    if let Some(err) = err {
                        warn!("ignore file error while matching {}: {}", target.display(), err);
                    }
                    if matched.is_ignore() {
                        return true;
                    }
                    if matched.is_whitelist() {
                        return false;
                    }
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

    fn engine_at(root: &Path) -> IgnoreEngine {
        let mut engine = IgnoreEngine::new(false, false, false, false, &[]);
        engine.load_vcs_ignores(root);
        engine
    }

    #[test]
    fn test_gitignored_dir_contents_are_ignored() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join(".gitignore"), "ignored_dir/\n").unwrap();

        let engine = engine_at(tmp.path());

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

        let engine = engine_at(tmp.path());

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

    /// A `.gitignore` in a subdirectory must apply to that subtree. Previously
    /// only the watch root's ignore files were loaded, so nested rules were
    /// silently dropped on live file events.
    #[test]
    fn test_nested_gitignore_is_respected() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join(".git")).unwrap();
        std::fs::create_dir_all(root.join("src")).unwrap();
        std::fs::write(root.join(".gitignore"), "root_only.log\n").unwrap();
        std::fs::write(root.join("src/.gitignore"), "*.log\n").unwrap();

        let engine = engine_at(root);

        let nested = root.join("src/debug.log");
        assert!(
            engine.is_ignored(&nested, Path::new("src/debug.log"), false),
            "nested src/.gitignore rule '*.log' must apply"
        );

        let kept = root.join("src/main.rs");
        assert!(!engine.is_ignored(&kept, Path::new("src/main.rs"), false));

        let root_rule = root.join("root_only.log");
        assert!(engine.is_ignored(&root_rule, Path::new("root_only.log"), false));
    }

    /// A nested negation must be able to re-include a file the root ignored.
    #[test]
    fn test_nested_negation_whitelists_file() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join(".git")).unwrap();
        std::fs::create_dir_all(root.join("src")).unwrap();
        std::fs::write(root.join(".gitignore"), "*.log\n").unwrap();
        std::fs::write(root.join("src/.gitignore"), "!keep.log\n").unwrap();

        let engine = engine_at(root);

        let kept = root.join("src/keep.log");
        assert!(
            !engine.is_ignored(&kept, Path::new("src/keep.log"), false),
            "nested '!keep.log' must re-include the file"
        );

        let dropped = root.join("src/other.log");
        assert!(engine.is_ignored(&dropped, Path::new("src/other.log"), false));
    }

    /// An anchored pattern in a parent `.gitignore` is relative to the file that
    /// declares it, not to the watch root. Re-anchoring it caused livediff to
    /// over-ignore when monitoring a subdirectory of a repository.
    #[test]
    fn test_parent_anchored_pattern_is_not_reanchored() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join(".git")).unwrap();
        std::fs::create_dir_all(root.join("src/docs/generated")).unwrap();
        std::fs::create_dir_all(root.join("docs/generated")).unwrap();
        std::fs::write(root.join(".gitignore"), "docs/generated/\n").unwrap();

        let watch_root = root.join("src");
        let engine = engine_at(&watch_root);

        // `docs/generated/` is anchored at the repository root, so a same-named
        // directory under the watch root is a different path and stays visible.
        let nested = watch_root.join("docs/generated/x.txt");
        assert!(
            !engine.is_ignored(&nested, Path::new("docs/generated/x.txt"), false),
            "parent pattern must not be re-anchored at the watch root"
        );
    }

    /// The initial scan and the live event path must agree on every file.
    #[test]
    fn test_scan_and_event_paths_agree() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join(".git")).unwrap();
        std::fs::create_dir_all(root.join("src/deep")).unwrap();
        std::fs::write(root.join(".gitignore"), "*.tmp\n").unwrap();
        std::fs::write(root.join("src/.gitignore"), "*.log\n").unwrap();
        std::fs::write(root.join("src/deep/.gitignore"), "!important.log\n").unwrap();
        for f in ["a.tmp", "src/b.log", "src/deep/important.log", "src/deep/c.rs"] {
            std::fs::write(root.join(f), "x").unwrap();
        }

        let engine = engine_at(root);
        let canonical = root.canonicalize().unwrap();

        let mut disagreements = Vec::new();
        for entry in engine.walk_builder(&canonical).build().flatten() {
            let path = entry.path();
            if path == canonical {
                continue;
            }
            let rel = path.strip_prefix(&canonical).unwrap_or(path);
            let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
            // The walker already excluded ignored files, so anything it yields
            // must also survive the per-event check.
            if engine.is_ignored(path, rel, is_dir) {
                disagreements.push(rel.to_string_lossy().to_string());
            }
        }
        assert!(
            disagreements.is_empty(),
            "scan yielded files the event path would drop: {disagreements:?}"
        );
    }
}
