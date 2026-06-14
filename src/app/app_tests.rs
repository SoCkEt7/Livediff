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
        }],
        is_binary: false,
    };
    domain.handle_file_changed(modif);

    let mut ui_state = TerminalUiState::default();
    ui_state.update_highlighting(&domain);

    assert!(!ui_state.highlighted_diff.is_empty());
    let (change_type, spans) = &ui_state.highlighted_diff[0];
    assert!(matches!(change_type, crate::domain::diff_engine::LineChangeType::Insert));
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
            },
            crate::domain::diff_engine::DiffLine {
                change_type: crate::domain::diff_engine::LineChangeType::Insert,
                content: "name = \"livediff\"".to_string(),
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
