use std::collections::{HashSet, VecDeque};
use std::time::SystemTime;
use std::path::PathBuf;

pub struct FileModification {
    pub path: String,
    pub timestamp: SystemTime,
    pub size: u64,
    pub added: usize,
    pub deleted: usize,
    pub diff: String,
    pub is_binary: bool,
}

pub struct AppStats {
    pub modified: usize,
    pub lines_added: usize,
    pub lines_deleted: usize,
}

pub enum Event {
    FileChanged(FileModification),
    Error(String),
    Log(String),
    Key(crossterm::event::KeyCode, crossterm::event::KeyModifiers),
    Tick,
}

pub struct App {
    pub modifications: VecDeque<FileModification>,
    pub selected_index: usize,
    pub diff_scroll: (u16, u16),
    pub ignore_list: HashSet<String>,
    pub globset: globset::GlobSet,
    pub logs: VecDeque<String>,
    pub help_visible: bool,
    pub ignore_menu_visible: bool,
    pub ignore_menu_selected: usize,
    pub ignore_menu_options: Vec<String>,
    pub ignore_input_visible: bool,
    pub ignore_input_text: String,
    pub should_quit: bool,
    pub anim_frame: usize,
}

impl App {
    pub fn new() -> App {
        App {
            modifications: VecDeque::new(),
            selected_index: 0,
            diff_scroll: (0, 0),
            ignore_list: HashSet::new(),
            globset: globset::GlobSetBuilder::new().build().unwrap(),
            logs: VecDeque::new(),
            help_visible: false,
            ignore_menu_visible: false,
            ignore_menu_selected: 0,
            ignore_menu_options: Vec::new(),
            ignore_input_visible: false,
            ignore_input_text: String::new(),
            should_quit: false,
            anim_frame: 0,
        }
    }

    pub fn stats(&self) -> AppStats {
        let mut modified = 0;
        let mut lines_added = 0;
        let mut lines_deleted = 0;

        for m in &self.modifications {
            if !self.is_ignored(&m.path) {
                modified += 1;
                lines_added += m.added;
                lines_deleted += m.deleted;
            }
        }

        AppStats {
            modified,
            lines_added,
            lines_deleted,
        }
    }

    pub fn add_log(&mut self, log: String) {
        let timestamp = chrono::Local::now().format("[%H:%M:%S]").to_string();
        self.logs.push_back(format!("{} {}", timestamp, log));
        if self.logs.len() > 100 {
            self.logs.pop_front();
        }
    }

    pub fn handle_file_changed(&mut self, mut modif: FileModification) {
        if self.is_ignored(&modif.path) {
            return;
        }
        let added = modif.added;
        let deleted = modif.deleted;
        let is_binary = modif.is_binary;

        // Add or update modification
        if let Some(existing) = self.modifications.iter_mut().find(|m| m.path == modif.path) {
            existing.timestamp = modif.timestamp;
            existing.size = modif.size;
            existing.added = added;
            existing.deleted = deleted;
            existing.diff = std::mem::take(&mut modif.diff);
            existing.is_binary = is_binary;
            
            // Move to front
            let path_clone = modif.path.clone();
            if let Some(idx) = self.modifications.iter().position(|m| m.path == path_clone) {
                let m = self.modifications.remove(idx).unwrap();
                self.modifications.push_front(m);
            }
        } else {
            self.modifications.push_front(modif);
        }

        if self.modifications.len() > 50 {
            self.modifications.pop_back();
        }
    }

    pub fn select_next(&mut self) {
        let visible_count = self.modifications.iter().filter(|m| !self.is_ignored(&m.path)).count();
        if visible_count > 0 && self.selected_index < visible_count - 1 {
            self.selected_index += 1;
            self.diff_scroll = (0, 0);
        }
    }

    pub fn select_previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.diff_scroll = (0, 0);
        }
    }

    pub fn is_ignored(&self, path: &str) -> bool {
        self.globset.is_match(path) || self.ignore_list.contains(path)
    }

    pub fn rebuild_globset(&mut self) {
        let mut builder = globset::GlobSetBuilder::new();
        for pattern in &self.ignore_list {
            let mut glob_str = pattern.clone();
            if glob_str.ends_with('/') {
                glob_str.push_str("**");
            }
            if let Ok(glob) = globset::Glob::new(&glob_str) {
                builder.add(glob);
            }
            if !glob_str.contains("**") {
                if let Ok(glob) = globset::Glob::new(&format!("{}/**", glob_str)) {
                    builder.add(glob);
                }
            }
        }
        if let Ok(gs) = builder.build() {
            self.globset = gs;
        }
    }

    pub fn toggle_ignore_menu(&mut self) {
        if self.ignore_menu_visible {
            self.ignore_menu_visible = false;
        } else {
            let path = {
                let visible: Vec<_> = self.modifications.iter().filter(|m| !self.is_ignored(&m.path)).collect();
                visible.get(self.selected_index).map(|m| m.path.clone())
            };
            if let Some(p) = path {
                self.ignore_menu_options.clear();
                self.ignore_menu_options.push(p.clone()); // exact file
                
                // Add extension if present
                if let Some(ext_idx) = p.rfind('.') {
                    if ext_idx > 0 {
                        self.ignore_menu_options.push(format!("*{}", &p[ext_idx..]));
                    }
                }
                
                // Add directories
                let mut current = p.as_str();
                while let Some(slash_idx) = current.rfind('/') {
                    let dir = &current[..slash_idx];
                    self.ignore_menu_options.push(dir.to_string());
                    current = dir;
                }
                
                self.ignore_menu_options.push("Ignore .*ignore files".to_string());
                self.ignore_menu_options.push("Type custom pattern...".to_string());
                
                self.ignore_menu_selected = 0;
                self.ignore_menu_visible = true;
            } else {
                self.ignore_menu_options.clear();
                self.ignore_menu_options.push("Ignore .*ignore files".to_string());
                self.ignore_menu_options.push("Type custom pattern...".to_string());
                self.ignore_menu_selected = 0;
                self.ignore_menu_visible = true;
            }
        }
    }

    pub fn ignore_menu_up(&mut self) {
        if self.ignore_menu_selected > 0 {
            self.ignore_menu_selected -= 1;
        }
    }

    pub fn ignore_menu_down(&mut self) {
        if self.ignore_menu_selected < self.ignore_menu_options.len().saturating_sub(1) {
            self.ignore_menu_selected += 1;
        }
    }

    pub fn ignore_menu_apply(&mut self) {
        if self.ignore_menu_visible && !self.ignore_menu_options.is_empty() {
            let selected = self.ignore_menu_options[self.ignore_menu_selected].clone();
            if selected == "Type custom pattern..." {
                self.ignore_menu_visible = false;
                self.ignore_input_visible = true;
                self.ignore_input_text.clear();
                return;
            }
            let actual_insert = if selected == "Ignore .*ignore files" {
                ".*ignore".to_string()
            } else {
                selected.clone()
            };

            self.ignore_list.insert(actual_insert.clone());
            self.rebuild_globset();
            self.add_log(format!("Ignored: {}", actual_insert));
            
            self.ignore_menu_visible = false;
            
            // Adjust selected index
            let visible_len = self.modifications.iter().filter(|m| !self.is_ignored(&m.path)).count();
            if self.selected_index > 0 && self.selected_index >= visible_len {
                self.selected_index = visible_len.saturating_sub(1);
            }
        }
    }

    pub fn ignore_input_char(&mut self, c: char) {
        if self.ignore_input_visible {
            self.ignore_input_text.push(c);
        }
    }

    pub fn ignore_input_backspace(&mut self) {
        if self.ignore_input_visible {
            self.ignore_input_text.pop();
        }
    }

    pub fn ignore_input_apply(&mut self) {
        if self.ignore_input_visible {
            if !self.ignore_input_text.is_empty() {
                self.ignore_list.insert(self.ignore_input_text.clone());
                self.rebuild_globset();
                self.add_log(format!("Ignored pattern: {}", self.ignore_input_text));
                
                let visible_len = self.modifications.iter().filter(|m| !self.is_ignored(&m.path)).count();
                if self.selected_index > 0 && self.selected_index >= visible_len {
                    self.selected_index = visible_len.saturating_sub(1);
                }
            }
            self.ignore_input_visible = false;
        }
    }

    pub fn clear_all(&mut self) {
        self.modifications.clear();
        self.ignore_list.clear();
        self.rebuild_globset();
        self.selected_index = 0;
        self.diff_scroll = (0, 0);
        self.add_log("Cleared all changes".to_string());
    }

    pub fn scroll_up(&mut self) {
        self.diff_scroll.0 = self.diff_scroll.0.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.diff_scroll.0 = self.diff_scroll.0.saturating_add(1);
    }

    pub fn scroll_left(&mut self) {
        self.diff_scroll.1 = self.diff_scroll.1.saturating_sub(2);
    }

    pub fn scroll_right(&mut self) {
        self.diff_scroll.1 = self.diff_scroll.1.saturating_add(2);
    }
}
