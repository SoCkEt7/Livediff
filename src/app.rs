// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use std::collections::VecDeque;
use std::sync::OnceLock;
use std::time::{Duration, Instant, SystemTime};

use syntect::easy::HighlightLines;
use syntect::highlighting::{FontStyle, ThemeSet};
use syntect::parsing::SyntaxSet;
use tui_overlay::OverlayState;

pub use crate::domain::entities::{AppStats, DomainEvent, FileModification};

pub enum Event {
    FileChanged { modification: FileModification, total_files: usize },
    FileDeleted { path: String, total_files: usize },
    TotalFilesUpdated(usize),
    Error(String),
    Log(String),
    Key(crossterm::event::KeyCode, crossterm::event::KeyModifiers),
    Mouse(crossterm::event::MouseEvent),
    Tick,
}

pub struct Toast {
    pub message: String,
    pub kind: ToastKind,
    pub expires: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastKind {
    Info,
    Success,
    Error,
}

// The pure domain state for tracking file changes and ignore lists
pub struct MonitorDomain {
    pub modifications: VecDeque<FileModification>,
    pub ignore_engine:
        std::sync::Arc<std::sync::RwLock<crate::domain::ignore_engine::IgnoreEngine>>,
    pub events: VecDeque<DomainEvent>,
    pub events_count: usize,
    pub total_files: usize,
}

impl Default for MonitorDomain {
    fn default() -> Self {
        Self::new(std::sync::Arc::new(std::sync::RwLock::new(
            crate::domain::ignore_engine::IgnoreEngine::new(false, false, false, false, &[]),
        )))
    }
}

impl MonitorDomain {
    pub fn new(
        ignore_engine: std::sync::Arc<
            std::sync::RwLock<crate::domain::ignore_engine::IgnoreEngine>,
        >,
    ) -> Self {
        Self {
            modifications: VecDeque::new(),
            ignore_engine,
            events: VecDeque::new(),
            events_count: 0,
            total_files: 0,
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

        AppStats { modified, lines_added, lines_deleted }
    }

    pub fn handle_file_changed(&mut self, mut modif: FileModification) -> bool {
        self.events_count += 1;
        if self.is_ignored(&modif.path) {
            return false;
        }
        let path = modif.path.clone();
        let added = modif.added;
        let deleted = modif.deleted;
        let is_binary = modif.is_binary;

        // Add or update modification
        if let Some(existing) = self.modifications.iter_mut().find(|m| m.path == path) {
            existing.timestamp = modif.timestamp;
            existing.size = modif.size;
            existing.added = added;
            existing.deleted = deleted;
            existing.diff_lines = std::mem::take(&mut modif.diff_lines);
            existing.is_binary = is_binary;

            // Move to front
            if let Some(idx) = self.modifications.iter().position(|m| m.path == path) {
                let m = self.modifications.remove(idx).unwrap();
                self.modifications.push_front(m);
            }
        } else {
            self.modifications.push_front(modif);
        }

        if self.modifications.len() > 1000 {
            self.modifications.pop_back();
        }

        self.events.push_back(DomainEvent::FileChanged { path, added, deleted });

        true
    }

    pub fn handle_file_deleted(&mut self, path: &str) -> bool {
        self.events_count += 1;
        let mut removed = false;
        if let Some(idx) = self.modifications.iter().position(|m| m.path == path) {
            self.modifications.remove(idx);
            removed = true;
        }
        removed
    }

    pub fn is_ignored(&self, path: &str) -> bool {
        if let Ok(engine) = self.ignore_engine.read() {
            engine.is_ignored(std::path::Path::new(path), std::path::Path::new(path), false)
        } else {
            false
        }
    }
}

pub enum PopupKind {
    Menu,
    Help,
    IgnoreMenu,
    IgnoreInput,
    Editor,
    SavePrompt,
    ActiveIgnores,
    Settings,
}

// The visual controller managing display coordinates, cursors, and menus
pub struct TerminalUiState {
    pub selected_index: usize,
    pub diff_scroll: (u16, u16),
    pub help_visible: bool,
    pub ignore_menu_visible: bool,
    pub ignore_menu_selected: usize,
    pub ignore_menu_options: Vec<String>,
    pub active_ignores_visible: bool,
    pub active_ignores_selected: usize,
    pub active_ignores_list: Vec<String>,
    pub respect_vcs_ignore: bool,
    pub ignore_gitignore_files: bool,
    pub settings_visible: bool,
    pub settings_selected: usize,
    pub notifications: VecDeque<Toast>,
    pub notification_overlay_state: OverlayState,
    pub ignore_input_visible: bool,
    pub ignore_input_text: String,
    pub should_quit: bool,
    pub anim_frame: usize,
    pub file_list_rect: ratatui::layout::Rect,
    pub diff_view_rect: ratatui::layout::Rect,
    pub popup_rect: ratatui::layout::Rect,
    pub save_popup_rect: ratatui::layout::Rect,
    pub stats_rect: ratatui::layout::Rect,
    pub footer_rect: ratatui::layout::Rect,
    pub header_rect: ratatui::layout::Rect,
    pub logs_rect: ratatui::layout::Rect,
    pub logs: VecDeque<String>,
    pub highlighted_diff:
        Vec<(crate::domain::diff_engine::LineChangeType, Vec<(ratatui::style::Style, String)>)>,
    pub last_selected_path: Option<String>,
    pub last_selected_timestamp: Option<SystemTime>,
    pub ram_usage: String,
    pub tick_rate_ms: u64,
    pub event_history: Vec<u64>,
    pub last_events_count: usize,
    pub ignore_cursor_idx: usize,
    pub menu_visible: bool,
    pub menu_selected: usize,
    pub file_list_width_pct: u16,
    pub is_dragging_divider: bool,
    pub editor_visible: bool,
    pub editor_instance: Option<Box<ratatui_code_editor::editor::Editor>>,
    pub editor_file_path: Option<String>,
    pub editor_save_prompt: bool,
    pub editor_has_changes: bool,
    pub overlay_state: OverlayState,
    pub save_overlay_state: OverlayState,
}

impl Default for TerminalUiState {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalUiState {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            diff_scroll: (0, 0),
            help_visible: false,
            ignore_menu_visible: false,
            ignore_menu_selected: 0,
            ignore_menu_options: Vec::new(),
            active_ignores_visible: false,
            active_ignores_selected: 0,
            active_ignores_list: Vec::new(),
            respect_vcs_ignore: true,
            ignore_gitignore_files: false,
            settings_visible: false,
            settings_selected: 0,
            notifications: VecDeque::new(),
            notification_overlay_state: OverlayState::new(),
            ignore_input_visible: false,
            ignore_input_text: String::new(),
            should_quit: false,
            anim_frame: 0,
            file_list_rect: ratatui::layout::Rect::default(),
            diff_view_rect: ratatui::layout::Rect::default(),
            popup_rect: ratatui::layout::Rect::default(),
            save_popup_rect: ratatui::layout::Rect::default(),
            stats_rect: ratatui::layout::Rect::default(),
            footer_rect: ratatui::layout::Rect::default(),
            header_rect: ratatui::layout::Rect::default(),
            logs_rect: ratatui::layout::Rect::default(),
            logs: VecDeque::new(),
            highlighted_diff: Vec::new(),
            last_selected_path: None,
            last_selected_timestamp: None,
            ram_usage: "0 KB".to_string(),
            tick_rate_ms: 500,
            event_history: vec![0; 40],
            last_events_count: 0,
            ignore_cursor_idx: 0,
            menu_visible: false,
            menu_selected: 0,
            file_list_width_pct: 40,
            is_dragging_divider: false,
            editor_visible: false,
            editor_instance: None,
            editor_file_path: None,
            editor_save_prompt: false,
            editor_has_changes: false,
            overlay_state: OverlayState::new(),
            save_overlay_state: OverlayState::new(),
        }
    }

    pub fn show_popup(&mut self, kind: PopupKind) {
        match kind {
            PopupKind::Menu => {
                self.menu_visible = true;
                self.overlay_state.open();
            }
            PopupKind::Help => {
                self.help_visible = true;
                self.overlay_state.open();
            }
            PopupKind::IgnoreMenu => {
                self.ignore_menu_visible = true;
                self.overlay_state.open();
            }
            PopupKind::IgnoreInput => {
                self.ignore_input_visible = true;
                self.overlay_state.open();
            }
            PopupKind::Editor => {
                self.editor_visible = true;
                self.overlay_state.open();
            }
            PopupKind::SavePrompt => {
                self.editor_save_prompt = true;
                self.save_overlay_state.open();
            }
            PopupKind::ActiveIgnores => {
                self.active_ignores_visible = true;
                self.overlay_state.open();
            }
            PopupKind::Settings => {
                self.settings_visible = true;
                self.overlay_state.open();
            }
        }
    }

    pub fn hide_all_popups(&mut self) {
        self.menu_visible = false;
        self.help_visible = false;
        self.ignore_menu_visible = false;
        self.active_ignores_visible = false;
        self.settings_visible = false;
        self.ignore_input_visible = false;
        self.editor_visible = false;
        self.editor_save_prompt = false;
        self.overlay_state.close();
        self.save_overlay_state.close();
    }

    pub fn hide_save_prompt(&mut self) {
        self.editor_save_prompt = false;
        self.save_overlay_state.close();
    }

    pub fn add_notification(&mut self, message: String, kind: ToastKind) {
        self.notifications.push_back(Toast {
            message,
            kind,
            expires: Instant::now() + Duration::from_secs(3),
        });
        self.notification_overlay_state.open();
    }

    pub fn update_notifications(&mut self) {
        let now = Instant::now();
        while let Some(toast) = self.notifications.front() {
            if now >= toast.expires {
                self.notifications.pop_front();
            } else {
                break;
            }
        }
        if self.notifications.is_empty() {
            self.notification_overlay_state.close();
        }
    }

    pub fn update_ram_usage(&mut self) {
        if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let value = parts[1];
                        if let Ok(kb) = value.parse::<u64>() {
                            if kb > 1024 {
                                self.ram_usage = format!("{:.1} MB", kb as f64 / 1024.0);
                            } else {
                                self.ram_usage = format!("{} KB", kb);
                            }
                            return;
                        }
                    }
                }
            }
        }
        self.ram_usage = "N/A".to_string();
    }

    pub fn add_log(&mut self, log: String) {
        let timestamp = chrono::Local::now().format("[%H:%M:%S]").to_string();
        self.logs.push_back(format!("{} {}", timestamp, log));
        if self.logs.len() > 100 {
            self.logs.pop_front();
        }
    }

    pub fn reset_diff_scroll_to_first_change(&mut self, domain: &MonitorDomain) {
        self.diff_scroll = (0, 0);
        let visible_mods: Vec<_> =
            domain.modifications.iter().filter(|m| !domain.is_ignored(&m.path)).collect();
        if let Some(m) = visible_mods.get(self.selected_index) {
            let first_change_idx = m.diff_lines.iter().position(|line| {
                matches!(
                    line.change_type,
                    crate::domain::diff_engine::LineChangeType::Insert
                        | crate::domain::diff_engine::LineChangeType::Delete
                )
            });
            if let Some(idx) = first_change_idx {
                self.diff_scroll.0 = idx.saturating_sub(3) as u16;
            }
        }
    }

    pub fn select_next(&mut self, domain: &MonitorDomain) {
        let visible_count =
            domain.modifications.iter().filter(|m| !domain.is_ignored(&m.path)).count();
        if visible_count > 0 && self.selected_index < visible_count - 1 {
            self.selected_index += 1;
            self.reset_diff_scroll_to_first_change(domain);
        }
    }

    pub fn select_previous(&mut self, domain: &MonitorDomain) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.reset_diff_scroll_to_first_change(domain);
        }
    }

    pub fn toggle_ignore_menu(&mut self, domain: &MonitorDomain) {
        if self.ignore_menu_visible {
            self.hide_all_popups();
        } else {
            let path = {
                let visible: Vec<_> =
                    domain.modifications.iter().filter(|m| !domain.is_ignored(&m.path)).collect();
                visible.get(self.selected_index).map(|m| m.path.clone())
            };
            if let Some(p) = path {
                self.ignore_menu_options.clear();
                self.ignore_menu_options.push(p.clone()); // exact file

                // Add extension if present
                if let Some(ext_idx) = p.rfind('.').filter(|&idx| idx > 0) {
                    self.ignore_menu_options.push(format!("*{}", &p[ext_idx..]));
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
                self.ignore_menu_options.push("View/Remove Active Ignores".to_string());
                self.ignore_menu_selected = 0;
                self.show_popup(PopupKind::IgnoreMenu);
                } else {
                self.ignore_menu_options.clear();
                self.ignore_menu_options.push("Ignore .*ignore files".to_string());
                self.ignore_menu_options.push("Type custom pattern...".to_string());
                self.ignore_menu_options.push("View/Remove Active Ignores".to_string());
                self.ignore_menu_selected = 0;
                self.show_popup(PopupKind::IgnoreMenu);
                }
        }
    }

    pub fn toggle_active_ignores(&mut self, domain: &MonitorDomain) {
        if self.active_ignores_visible {
            self.hide_all_popups();
        } else {
            if let Ok(engine) = domain.ignore_engine.read() {
                self.active_ignores_list = engine.ignore_list.iter().cloned().collect();
                self.active_ignores_list.sort();
                self.active_ignores_selected = 0;
                self.show_popup(PopupKind::ActiveIgnores);
            }
        }
    }

    pub fn remove_active_ignore(&mut self, domain: &MonitorDomain) {
        if self.active_ignores_visible && !self.active_ignores_list.is_empty() {
            let pattern = self.active_ignores_list[self.active_ignores_selected].clone();
            if let Ok(mut engine) = domain.ignore_engine.write() {
                engine.remove_ignore(&pattern);
            }
            self.refresh_active_ignores(domain);
        }
    }

    pub fn clear_active_ignores(&mut self, domain: &MonitorDomain) {
        if self.active_ignores_visible {
            if let Ok(mut engine) = domain.ignore_engine.write() {
                engine.ignore_list.clear();
                engine.rebuild_globset();
            }
            self.refresh_active_ignores(domain);
        }
    }

    fn refresh_active_ignores(&mut self, domain: &MonitorDomain) {
        if let Ok(engine) = domain.ignore_engine.read() {
            self.active_ignores_list = engine.ignore_list.iter().cloned().collect();
            self.active_ignores_list.sort();
            if self.active_ignores_selected >= self.active_ignores_list.len() {
                self.active_ignores_selected = self.active_ignores_list.len().saturating_sub(1);
            }
            if self.active_ignores_list.is_empty() {
                self.hide_all_popups();
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

    pub fn active_ignores_up(&mut self) {
        if self.active_ignores_selected > 0 {
            self.active_ignores_selected -= 1;
        }
    }

    pub fn active_ignores_down(&mut self) {
        if self.active_ignores_selected < self.active_ignores_list.len().saturating_sub(1) {
            self.active_ignores_selected += 1;
        }
    }

    pub fn ignore_menu_apply(&mut self, domain: &mut MonitorDomain) {
        if self.ignore_menu_visible && !self.ignore_menu_options.is_empty() {
            let selected = self.ignore_menu_options[self.ignore_menu_selected].clone();
            if selected == "Type custom pattern..." {
                self.hide_all_popups();
                self.ignore_input_text.clear();
                self.ignore_cursor_idx = 0;
                self.show_popup(PopupKind::IgnoreInput);
                return;
            }
            if selected == "View/Remove Active Ignores" {
                self.hide_all_popups();
                self.toggle_active_ignores(domain);
                return;
            }
            let actual_insert =
                if selected == "Ignore .*ignore files" { ".*ignore".to_string() } else { selected };

            if let Ok(mut engine) = domain.ignore_engine.write() {
                engine.add_ignore(actual_insert.clone());
            }
            domain.events.push_back(DomainEvent::IgnoreAdded { pattern: actual_insert });
            self.hide_all_popups();

            // Adjust selected index
            let visible_len =
                domain.modifications.iter().filter(|m| !domain.is_ignored(&m.path)).count();
            if self.selected_index > 0 && self.selected_index >= visible_len {
                self.selected_index = visible_len.saturating_sub(1);
            }
        }
    }

    pub fn ignore_input_char(&mut self, c: char) {
        if self.ignore_input_visible {
            self.ignore_input_text.insert(self.ignore_cursor_idx, c);
            self.ignore_cursor_idx += 1;
        }
    }

    pub fn ignore_input_backspace(&mut self) {
        if self.ignore_input_visible && self.ignore_cursor_idx > 0 {
            self.ignore_cursor_idx -= 1;
            self.ignore_input_text.remove(self.ignore_cursor_idx);
        }
    }

    pub fn ignore_input_left(&mut self) {
        if self.ignore_input_visible && self.ignore_cursor_idx > 0 {
            self.ignore_cursor_idx -= 1;
        }
    }

    pub fn ignore_input_right(&mut self) {
        if self.ignore_input_visible && self.ignore_cursor_idx < self.ignore_input_text.len() {
            self.ignore_cursor_idx += 1;
        }
    }

    pub fn ignore_input_apply(&mut self, domain: &mut MonitorDomain) {
        if self.ignore_input_visible {
            if !self.ignore_input_text.is_empty() {
                if let Ok(mut engine) = domain.ignore_engine.write() {
                    engine.add_ignore(self.ignore_input_text.clone());
                }
                domain.events.push_back(DomainEvent::IgnoreAdded {
                    pattern: self.ignore_input_text.clone(),
                });

                let visible_len =
                    domain.modifications.iter().filter(|m| !domain.is_ignored(&m.path)).count();
                if self.selected_index > 0 && self.selected_index >= visible_len {
                    self.selected_index = visible_len.saturating_sub(1);
                }
            }
            self.hide_all_popups();
        }
    }

    pub fn update_event_history(&mut self, current_events: usize) {
        let delta = current_events.saturating_sub(self.last_events_count) as u64;
        self.last_events_count = current_events;
        self.event_history.remove(0);
        self.event_history.push(delta);
    }

    pub fn clear_all(&mut self, domain: &mut MonitorDomain) {
        domain.modifications.clear();
        if let Ok(mut engine) = domain.ignore_engine.write() {
            engine.ignore_list.clear();
            engine.rebuild_globset();
        }
        domain.events.push_back(DomainEvent::HistoryCleared);
        self.selected_index = 0;
        self.diff_scroll = (0, 0);
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

    pub fn update_highlighting(&mut self, domain: &MonitorDomain) {
        let visible_mods: Vec<_> =
            domain.modifications.iter().filter(|m| !domain.is_ignored(&m.path)).collect();

        let selected_mod = visible_mods.get(self.selected_index);

        let (should_update, mod_to_highlight) =
            match (selected_mod, &self.last_selected_path, self.last_selected_timestamp) {
                (Some(m), Some(last_path), Some(last_ts)) => {
                    (m.path != *last_path || m.timestamp != last_ts, Some(m))
                }
                (Some(m), _, _) => (true, Some(m)),
                (None, _, _) => {
                    (self.last_selected_path.is_some() || !self.highlighted_diff.is_empty(), None)
                }
            };

        if !should_update {
            return;
        }

        self.highlighted_diff.clear();

        let Some(m) = mod_to_highlight else {
            self.last_selected_path = None;
            self.last_selected_timestamp = None;
            return;
        };

        self.last_selected_path = Some(m.path.clone());
        self.last_selected_timestamp = Some(m.timestamp);

        if m.is_binary {
            return;
        }

        let path = std::path::Path::new(&m.path);
        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        let is_toml = extension == "toml";

        let ss = get_syntax_set();
        let syntax =
            ss.find_syntax_by_extension(extension).unwrap_or_else(|| ss.find_syntax_plain_text());

        let ts = get_theme_set();
        let theme = &ts.themes["base16-ocean.dark"];
        let mut highlighter = HighlightLines::new(syntax, theme);

        for line in &m.diff_lines {
            match line.change_type {
                crate::domain::diff_engine::LineChangeType::Header => {
                    self.highlighted_diff.push((
                        line.change_type.clone(),
                        vec![(ratatui::style::Style::default(), line.content.clone())],
                    ));
                }
                _ => {
                    let spans = if is_toml {
                        highlight_toml_line(&line.content)
                    } else {
                        let ranges =
                            highlighter.highlight_line(&line.content, ss).unwrap_or_default();
                        ranges
                            .into_iter()
                            .map(|(style, text)| (map_style(style), text.to_string()))
                            .collect()
                    };
                    self.highlighted_diff.push((line.change_type.clone(), spans));
                }
            }
        }
    }
}

fn highlight_toml_line(line: &str) -> Vec<(ratatui::style::Style, String)> {
    let mut spans = Vec::new();

    // 1. Comment handling
    if let Some(comment_idx) = line.find('#') {
        let (code, comment) = line.split_at(comment_idx);
        if !code.trim().is_empty() {
            spans.extend(highlight_toml_code_line(code));
        }
        spans.push((
            ratatui::style::Style::default().fg(ratatui::style::Color::Rgb(101, 115, 126)), // grey
            comment.to_string(),
        ));
        return spans;
    }

    highlight_toml_code_line(line)
}

fn highlight_toml_code_line(line: &str) -> Vec<(ratatui::style::Style, String)> {
    let mut spans = Vec::new();
    let trimmed = line.trim();

    // 2. Section Header: [section] or [[section]]
    if trimmed.starts_with('[') && trimmed.ends_with(']') {
        spans.push((
            ratatui::style::Style::default()
                .fg(ratatui::style::Color::Rgb(235, 94, 85)) // red/pink for sections
                .add_modifier(ratatui::style::Modifier::BOLD),
            line.to_string(),
        ));
        return spans;
    }

    // 3. Key = Value handling
    if let Some(eq_idx) = line.find('=') {
        let (key_part, val_part) = line.split_at(eq_idx);
        // Key part
        spans.push((
            ratatui::style::Style::default().fg(ratatui::style::Color::Rgb(102, 153, 204)), // blue for keys
            key_part.to_string(),
        ));
        // Equals sign
        spans.push((
            ratatui::style::Style::default().fg(ratatui::style::Color::Rgb(192, 197, 206)), // white/grey
            "=".to_string(),
        ));
        // Value part
        let val_str = &val_part[1..]; // skip the '='
        let trimmed_val = val_str.trim();

        if (trimmed_val.starts_with('"') && trimmed_val.ends_with('"'))
            || (trimmed_val.starts_with('\'') && trimmed_val.ends_with('\''))
        {
            // String value
            spans.push((
                ratatui::style::Style::default().fg(ratatui::style::Color::Rgb(153, 199, 148)), // green for strings
                val_str.to_string(),
            ));
        } else if trimmed_val == "true" || trimmed_val == "false" {
            // Boolean value
            spans.push((
                ratatui::style::Style::default().fg(ratatui::style::Color::Rgb(244, 191, 117)), // orange for booleans
                val_str.to_string(),
            ));
        } else if trimmed_val.chars().all(|c| c.is_numeric() || c == '.' || c == '-' || c == '_') {
            // Numeric value
            spans.push((
                ratatui::style::Style::default().fg(ratatui::style::Color::Rgb(209, 154, 102)), // orange/yellow for numbers
                val_str.to_string(),
            ));
        } else {
            // Default value style
            spans.push((
                ratatui::style::Style::default().fg(ratatui::style::Color::Rgb(192, 197, 206)),
                val_str.to_string(),
            ));
        }
        return spans;
    }

    // Default style
    spans.push((
        ratatui::style::Style::default().fg(ratatui::style::Color::Rgb(192, 197, 206)),
        line.to_string(),
    ));
    spans
}

static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();

fn get_syntax_set() -> &'static SyntaxSet {
    SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines)
}

fn get_theme_set() -> &'static ThemeSet {
    THEME_SET.get_or_init(ThemeSet::load_defaults)
}

fn map_style(syntect_style: syntect::highlighting::Style) -> ratatui::style::Style {
    let fg = syntect_style.foreground;
    let mut ratatui_style =
        ratatui::style::Style::default().fg(ratatui::style::Color::Rgb(fg.r, fg.g, fg.b));

    let font_style = syntect_style.font_style;
    if font_style.contains(FontStyle::BOLD) {
        ratatui_style = ratatui_style.add_modifier(ratatui::style::Modifier::BOLD);
    }
    if font_style.contains(FontStyle::ITALIC) {
        ratatui_style = ratatui_style.add_modifier(ratatui::style::Modifier::ITALIC);
    }
    if font_style.contains(FontStyle::UNDERLINE) {
        ratatui_style = ratatui_style.add_modifier(ratatui::style::Modifier::UNDERLINED);
    }
    ratatui_style
}

#[cfg(test)]
mod app_tests;
