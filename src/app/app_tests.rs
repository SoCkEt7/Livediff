// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use super::*;
use crate::domain::ignore_engine::IgnoreEngine;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

#[test]
fn test_monitor_domain_ignores() {
    let engine = Arc::new(RwLock::new(IgnoreEngine::new(
        false,
        false,
        false,
        false,
        &["target/".to_string(), "*.tmp".to_string()],
    )));
    let domain = MonitorDomain::new(engine);

    assert!(domain.is_ignored("target/debug/build"));
    assert!(domain.is_ignored("src/main.tmp"));
    assert!(!domain.is_ignored("src/main.rs"));
}

#[test]
fn test_monitor_domain_history_limit() {
    let engine = Arc::new(RwLock::new(IgnoreEngine::new(false, false, false, false, &[])));
    let mut domain = MonitorDomain::new(engine);
    for i in 0..1200 {
        let modif = FileModification {
            path: format!("file_{}.rs", i),
            timestamp: SystemTime::now(),
            size: 100,
            added: 1,
            deleted: 0,
            diff_lines: vec![],
            is_binary: false,
        };
        domain.handle_file_changed(modif);
    }
    // History limit is 1000, so older modifications should be popped
    assert_eq!(domain.modifications.len(), 1000);
    assert_eq!(domain.modifications.front().unwrap().path, "file_1199.rs");
}

#[test]
fn test_update_highlighting() {
    let engine = Arc::new(RwLock::new(IgnoreEngine::new(false, false, false, false, &[])));
    let mut domain = MonitorDomain::new(engine);
    let modif = FileModification {
        path: "test.rs".to_string(),
        timestamp: SystemTime::now(),
        size: 100,
        added: 1,
        deleted: 0,
        diff_lines: vec![crate::domain::diff_engine::DiffLine {
            change_type: crate::domain::diff_engine::LineChangeType::Insert,
            content: "fn main() {".to_string(),
            old_lineno: None,
            new_lineno: Some(1),
        }],
        is_binary: false,
    };
    domain.handle_file_changed(modif);

    let mut ui_state = TerminalUiState::default();
    ui_state.update_highlighting(&domain);

    assert!(!ui_state.highlighted_diff.is_empty());
    let (diff_line, spans) = &ui_state.highlighted_diff[0];
    assert!(matches!(diff_line.change_type, crate::domain::diff_engine::LineChangeType::Insert));
    assert!(!spans.is_empty());
    let full_content: String = spans.iter().map(|(_, text)| text.as_str()).collect();
    assert!(full_content.contains("fn"));
}

#[test]
fn test_toml_highlighting() {
    let engine = Arc::new(RwLock::new(IgnoreEngine::new(false, false, false, false, &[])));
    let mut domain = MonitorDomain::new(engine);
    let modif = FileModification {
        path: "rustfmt.toml".to_string(),
        timestamp: SystemTime::now(),
        size: 100,
        added: 1,
        deleted: 0,
        diff_lines: vec![
            crate::domain::diff_engine::DiffLine {
                change_type: crate::domain::diff_engine::LineChangeType::Insert,
                content: "[package] # comment".to_string(),
                old_lineno: None,
                new_lineno: Some(1),
            },
            crate::domain::diff_engine::DiffLine {
                change_type: crate::domain::diff_engine::LineChangeType::Insert,
                content: "name = \"livediff\"".to_string(),
                old_lineno: None,
                new_lineno: Some(2),
            },
        ],
        is_binary: false,
    };
    domain.handle_file_changed(modif);

    let mut ui_state = TerminalUiState::default();
    ui_state.update_highlighting(&domain);

    assert_eq!(ui_state.highlighted_diff.len(), 2);

    // First line: [package] # comment
    let (_, spans1) = &ui_state.highlighted_diff[0];
    assert_eq!(spans1.len(), 2); // section header + comment
    assert_eq!(spans1[0].1, "[package] ");
    assert_eq!(spans1[1].1, "# comment");

    // Second line: name = "livediff"
    let (_, spans2) = &ui_state.highlighted_diff[1];
    assert!(spans2.len() >= 3); // key + equals + value
    assert_eq!(spans2[0].1, "name ");
    assert_eq!(spans2[1].1, "=");
    assert!(spans2[2].1.contains("livediff"));
}

#[test]
fn test_code_editor_integration() {
    use ratatui_code_editor::editor::Editor;
    use ratatui_code_editor::theme::vesper;
    let mut editor = Editor::new("rust", "fn main() {}", vesper()).unwrap();

    let key = crossterm::event::KeyEvent::new(
        crossterm::event::KeyCode::Char('a'),
        crossterm::event::KeyModifiers::empty(),
    );
    editor.input(key, &ratatui::layout::Rect::new(0, 0, 80, 24)).unwrap();

    let text = editor.code_ref().get_content();
    assert!(text.contains('a'));

    // Check if Editor implements Widget
    let rect = ratatui::layout::Rect::new(0, 0, 80, 24);
    let mut buf = ratatui::buffer::Buffer::empty(rect);
    ratatui::widgets::Widget::render(&editor, rect, &mut buf);
}

#[test]
fn test_view_mode_and_filtering() {
    let engine = Arc::new(RwLock::new(IgnoreEngine::new(false, false, false, false, &[])));
    let mut domain = MonitorDomain::new(engine);

    let modif1 = FileModification {
        path: "src/adapters/ui/diff_view.rs".to_string(),
        timestamp: SystemTime::now(),
        size: 200,
        added: 5,
        deleted: 2,
        diff_lines: vec![],
        is_binary: false,
    };
    let modif2 = FileModification {
        path: "src/domain/diff_engine.rs".to_string(),
        timestamp: SystemTime::now(),
        size: 300,
        added: 10,
        deleted: 0,
        diff_lines: vec![],
        is_binary: false,
    };
    domain.handle_file_changed(modif1);
    domain.handle_file_changed(modif2);

    let mut ui_state = TerminalUiState::default();
    assert_eq!(ui_state.view_mode, DiffViewMode::Unified);
    ui_state.toggle_view_mode();
    assert_eq!(ui_state.view_mode, DiffViewMode::Split);

    // Initial visible
    let visible = ui_state.get_visible_modifications(&domain);
    assert_eq!(visible.len(), 2);

    // Filter by "engine"
    ui_state.filter_query = "engine".to_string();
    let visible_filtered = ui_state.get_visible_modifications(&domain);
    assert_eq!(visible_filtered.len(), 1);
    assert_eq!(visible_filtered[0].path, "src/domain/diff_engine.rs");

    // Clear filter
    ui_state.filter_clear(&domain);
    assert_eq!(ui_state.get_visible_modifications(&domain).len(), 2);
}

#[test]
fn test_export_patch_success() {
    let engine = Arc::new(RwLock::new(IgnoreEngine::new(false, false, false, false, &[])));
    let mut domain = MonitorDomain::new(engine);

    let modif = FileModification {
        path: "sample.txt".to_string(),
        timestamp: SystemTime::now(),
        size: 50,
        added: 1,
        deleted: 1,
        diff_lines: vec![
            crate::domain::diff_engine::DiffLine {
                change_type: crate::domain::diff_engine::LineChangeType::Delete,
                content: "old line\n".to_string(),
                old_lineno: Some(1),
                new_lineno: None,
            },
            crate::domain::diff_engine::DiffLine {
                change_type: crate::domain::diff_engine::LineChangeType::Insert,
                content: "new line\n".to_string(),
                old_lineno: None,
                new_lineno: Some(1),
            },
        ],
        is_binary: false,
    };
    domain.handle_file_changed(modif);

    let mut ui_state = TerminalUiState::default();
    let temp_dir = std::env::temp_dir();
    let patch_res = ui_state.export_current_patch(&domain, &temp_dir);
    assert!(patch_res.is_ok());
    let patch_path = patch_res.unwrap();
    assert!(patch_path.exists());
    let patch_content = std::fs::read_to_string(&patch_path).unwrap();
    assert!(patch_content.contains("--- a/sample.txt"));
    assert!(patch_content.contains("+++ b/sample.txt"));
    assert!(patch_content.contains("-old line"));
    assert!(patch_content.contains("+new line"));
    let _ = std::fs::remove_file(patch_path);
}

#[test]
fn test_theme_cycling() {
    use crate::adapters::ui::theme::ThemeKind;
    let mut ui_state = TerminalUiState::default();
    assert_eq!(ui_state.current_theme, ThemeKind::Cyberpunk);

    ui_state.cycle_theme();
    assert_eq!(ui_state.current_theme, ThemeKind::Catppuccin);

    ui_state.cycle_theme();
    assert_eq!(ui_state.current_theme, ThemeKind::TokyoNight);

    ui_state.cycle_theme();
    assert_eq!(ui_state.current_theme, ThemeKind::Nord);

    ui_state.cycle_theme();
    assert_eq!(ui_state.current_theme, ThemeKind::Gruvbox);

    ui_state.cycle_theme();
    assert_eq!(ui_state.current_theme, ThemeKind::Cyberpunk);
}

#[test]
fn test_wrap_lines_toggle() {
    let mut ui_state = TerminalUiState::default();
    assert!(!ui_state.wrap_lines);

    ui_state.toggle_wrap_lines();
    assert!(ui_state.wrap_lines);

    ui_state.toggle_wrap_lines();
    assert!(!ui_state.wrap_lines);
}

#[test]
fn test_yank_diff_use_case_format() {
    let modif = FileModification {
        path: "test_yank.rs".to_string(),
        timestamp: SystemTime::now(),
        size: 120,
        added: 1,
        deleted: 1,
        diff_lines: vec![
            crate::domain::diff_engine::DiffLine {
                change_type: crate::domain::diff_engine::LineChangeType::Delete,
                content: "let x = 1;\n".to_string(),
                old_lineno: Some(1),
                new_lineno: None,
            },
            crate::domain::diff_engine::DiffLine {
                change_type: crate::domain::diff_engine::LineChangeType::Insert,
                content: "let x = 2;\n".to_string(),
                old_lineno: None,
                new_lineno: Some(1),
            },
        ],
        is_binary: false,
    };

    let patch = crate::use_cases::export_patch::ExportPatchUseCase::new().format_patch(&modif);
    assert!(patch.contains("--- a/test_yank.rs"));
    assert!(patch.contains("+++ b/test_yank.rs"));
    assert!(patch.contains("-let x = 1;"));
    assert!(patch.contains("+let x = 2;"));
}

#[test]
fn test_user_config_serde_and_ui_state() {
    use crate::adapters::ui::theme::ThemeKind;
    use crate::domain::config::{ThemeSetting, UserConfig, ViewModeSetting};

    let config = UserConfig {
        theme: ThemeSetting::TokyoNight,
        view_mode: ViewModeSetting::Split,
        wrap_lines: true,
        ignore_whitespace: true,
        respect_vcs_ignore: false,
        tick_rate_ms: 250,
    };

    let toml_str = toml::to_string(&config).unwrap();
    assert!(toml_str.contains("theme = \"tokyo_night\""));
    assert!(toml_str.contains("view_mode = \"split\""));
    assert!(toml_str.contains("wrap_lines = true"));

    let deserialized: UserConfig = toml::from_str(&toml_str).unwrap();
    assert_eq!(deserialized, config);

    let state = TerminalUiState::from_config(&config);
    assert_eq!(state.current_theme, ThemeKind::TokyoNight);
    assert_eq!(state.view_mode, DiffViewMode::Split);
    assert!(state.wrap_lines);
    assert!(state.ignore_whitespace);
    assert!(!state.respect_vcs_ignore);
    assert_eq!(state.tick_rate_ms, 250);

    let roundtrip_config = state.to_config();
    assert_eq!(roundtrip_config, config);
}
