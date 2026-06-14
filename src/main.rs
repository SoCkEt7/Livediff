// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

pub mod adapters;
pub mod app;
pub mod domain;
pub mod infrastructure;
pub mod use_cases;

use crate::app::Event;
use clap::Parser;
use color_eyre::Result;
use tokio::sync::mpsc;
use tracing::info;

use crate::infrastructure::terminal::{init_terminal, restore_terminal};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize Panic/Error handler for TUI (restores terminal on panic)
    color_eyre::install()?;

    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal();
        original_hook(panic_info);
    }));

    // 2. Parse CLI arguments
    let cli = adapters::cli::Cli::parse();

    // 3. Initialize background logging
    infrastructure::logging::init_logging()?;

    info!("Starting Livediff monitoring at path: {:?}", cli.path);

    // Initialize state split
    let mut ignore_engine = domain::ignore_engine::IgnoreEngine::new(
        cli.no_ignore,
        cli.show_hidden,
        cli.no_ignore_parent,
        cli.no_ignore_vcs,
        &cli.ignore,
    );
    let canonical_path = cli.path.canonicalize().unwrap_or(cli.path.clone());
    let warnings = ignore_engine.load_vcs_ignores(&canonical_path);

    let ignore_engine_arc = std::sync::Arc::new(std::sync::RwLock::new(ignore_engine));

    let mut domain = app::MonitorDomain::new(ignore_engine_arc.clone());
    let mut ui_state = app::TerminalUiState::new();
    let process_file_change = use_cases::process_file_change::ProcessFileChangeUseCase::new();

    for warning in warnings {
        ui_state.add_log(warning);
    }

    // Initialize Terminal
    let mut terminal = init_terminal()?;

    // MPSC Channel for combining TUI events (Key, Mouse, Tick, Watcher)
    let (event_tx, mut event_rx) = mpsc::channel(100);

    // Spawn Watcher
    let watcher_config = adapters::watcher::WatcherConfig {
        root_path: cli.path.clone(),
        max_size: 1024 * 1024,
        ignore_engine: ignore_engine_arc.clone(),
    };

    // Wire watcher event stream receiver to forward into unified channel
    let mut watcher_rx = adapters::watcher::FileMonitor::new(watcher_config).start();
    let watcher_tx = event_tx.clone();
    tokio::spawn(async move {
        while let Some(ev) = watcher_rx.recv().await {
            let _ = watcher_tx.send(ev).await;
        }
    });

    // Initialize RAM usage status on startup
    ui_state.update_ram_usage();

    let tick_rate_ms = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(150));

    // Spawn Input/Tick Event Task on dedicated OS thread to avoid blocking Tokio runtime
    let input_tx = event_tx.clone();
    let thread_tick_rate = tick_rate_ms.clone();
    std::thread::spawn(move || {
        let mut last_tick = std::time::Instant::now();
        loop {
            let tick_rate = std::time::Duration::from_millis(
                thread_tick_rate.load(std::sync::atomic::Ordering::Relaxed),
            );
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| std::time::Duration::from_secs(0));
            if crossterm::event::poll(timeout).unwrap_or(false) {
                match crossterm::event::read() {
                    Ok(crossterm::event::Event::Key(key)) => {
                        let _ = input_tx.blocking_send(Event::Key(key.code, key.modifiers));
                    }
                    Ok(crossterm::event::Event::Mouse(mouse)) => {
                        let _ = input_tx.blocking_send(Event::Mouse(mouse));
                    }
                    _ => {}
                }
            }
            if last_tick.elapsed() >= tick_rate {
                if input_tx.blocking_send(Event::Tick).is_err() {
                    break;
                }
                last_tick = std::time::Instant::now();
            }
        }
    });

    // Main event loop
    terminal.draw(|f| adapters::ui::draw(f, &mut ui_state, &domain))?;

    while let Some(event) = event_rx.recv().await {
        match event {
            Event::FileChanged { modification, total_files } => {
                domain.total_files = total_files;
                let selected_path = {
                    let visible_mods: Vec<_> = domain
                        .modifications
                        .iter()
                        .filter(|m| !domain.is_ignored(&m.path))
                        .collect();
                    visible_mods.get(ui_state.selected_index).map(|m| m.path.clone())
                };
                let changed = process_file_change.execute(&mut domain, modification);
                if changed {
                    let is_current = if let Some(ref path) = selected_path {
                        if let Some(first_m) = domain.modifications.front() {
                            first_m.path == *path
                        } else {
                            false
                        }
                    } else {
                        true
                    };
                    if is_current {
                        ui_state.reset_diff_scroll_to_first_change(&domain);
                    }
                }
            }
            Event::FileDeleted { path, total_files } => {
                domain.total_files = total_files;
                let removed = domain.handle_file_deleted(&path);
                if removed {
                    ui_state.add_log(format!("File deleted: {}", path));
                    let visible_mods: Vec<_> = domain
                        .modifications
                        .iter()
                        .filter(|m| !domain.is_ignored(&m.path))
                        .collect();
                    if visible_mods.is_empty() {
                        ui_state.selected_index = 0;
                    } else if ui_state.selected_index >= visible_mods.len() {
                        ui_state.selected_index = visible_mods.len() - 1;
                    }
                    ui_state.reset_diff_scroll_to_first_change(&domain);
                }
            }
            Event::TotalFilesUpdated(total_files) => {
                domain.total_files = total_files;
            }
            Event::Error(err) => {
                ui_state.add_log(format!("Error: {}", err));
            }
            Event::Log(log) => {
                ui_state.add_log(log);
            }
            Event::Tick => {
                ui_state.anim_frame = ui_state.anim_frame.wrapping_add(1);
                ui_state.update_ram_usage();
                ui_state.update_event_history(domain.events_count);
                ui_state.update_notifications();
            }
            Event::Mouse(mouse) => {
                if ui_state.editor_visible {
                    if ui_state.editor_save_prompt {
                        if let crossterm::event::MouseEventKind::Down(
                            crossterm::event::MouseButton::Left,
                        ) = mouse.kind
                        {
                            let pr = ui_state.save_popup_rect;
                            let clicked_inside_save = mouse.column >= pr.x
                                && mouse.column < pr.x + pr.width
                                && mouse.row >= pr.y
                                && mouse.row < pr.y + pr.height;
                            if !clicked_inside_save {
                                ui_state.hide_save_prompt();
                            }
                        }
                    } else {
                        let pr = ui_state.popup_rect;
                        let clicked_inside = mouse.column >= pr.x
                            && mouse.column < pr.x + pr.width
                            && mouse.row >= pr.y
                            && mouse.row < pr.y + pr.height;

                        if !clicked_inside {
                            if let crossterm::event::MouseEventKind::Down(
                                crossterm::event::MouseButton::Left,
                            ) = mouse.kind
                            {
                                if ui_state.editor_has_changes {
                                    ui_state.show_popup(app::PopupKind::SavePrompt);
                                } else {
                                    ui_state.hide_all_popups();
                                    ui_state.editor_instance = None;
                                    ui_state.editor_file_path = None;
                                }
                            }
                        } else if let Some(ref mut editor) = ui_state.editor_instance {
                            let block = ratatui::widgets::Block::default()
                                .borders(ratatui::widgets::Borders::ALL)
                                .border_type(ratatui::widgets::BorderType::Rounded);
                            let editor_rect = block.inner(pr);
                            let _ = editor.mouse(mouse, &editor_rect);
                        }
                    }
                } else {
                    match mouse.kind {
                        crossterm::event::MouseEventKind::Down(
                            crossterm::event::MouseButton::Left,
                        ) => {
                            let is_popup_visible = ui_state.overlay_state.is_open();

                            if is_popup_visible {
                                let pr = ui_state.popup_rect;
                                let clicked_inside = mouse.column >= pr.x
                                    && mouse.column < pr.x + pr.width
                                    && mouse.row >= pr.y
                                    && mouse.row < pr.y + pr.height;

                                if !clicked_inside {
                                    ui_state.hide_all_popups();
                                } else if ui_state.menu_visible {
                                    let clicked_row = mouse.row as i32 - (pr.y as i32 + 2);
                                    if (0..8).contains(&clicked_row) {
                                        ui_state.menu_selected = clicked_row as usize;
                                        ui_state.hide_all_popups();
                                        match ui_state.menu_selected {
                                            0 => ui_state.show_popup(app::PopupKind::Help),
                                            1 => ui_state.show_popup(app::PopupKind::Settings),
                                            2 => ui_state.toggle_ignore_menu(&domain),
                                            3 => ui_state.toggle_active_ignores(&domain),
                                            4 => {
                                                ui_state.show_popup(app::PopupKind::IgnoreInput);
                                                ui_state.ignore_input_text.clear();
                                                ui_state.ignore_cursor_idx = 0;
                                            }
                                            5 => ui_state.clear_all(&mut domain),
                                            6 => {} // Close menu (already hidden)
                                            7 => ui_state.should_quit = true,
                                            _ => {}
                                        }
                                    }
                                } else if ui_state.ignore_menu_visible {
                                    let clicked_row = mouse.row as i32 - (pr.y as i32 + 1);
                                    if clicked_row >= 0
                                        && (clicked_row as usize)
                                            < ui_state.ignore_menu_options.len()
                                    {
                                        ui_state.ignore_menu_selected = clicked_row as usize;
                                        ui_state.ignore_menu_apply(&mut domain);
                                    }
                                } else if ui_state.active_ignores_visible {
                                    let clicked_row = mouse.row as i32 - (pr.y as i32 + 1);
                                    if clicked_row >= 0
                                        && (clicked_row as usize)
                                            < ui_state.active_ignores_list.len()
                                    {
                                        ui_state.active_ignores_selected = clicked_row as usize;
                                        ui_state.remove_active_ignore(&domain);
                                    }
                                } else if ui_state.settings_visible {
                                    let clicked_row = mouse.row as i32 - (pr.y as i32 + 1);
                                    if (0..3).contains(&clicked_row) {
                                        ui_state.settings_selected = clicked_row as usize;
                                        match ui_state.settings_selected {
                                            0 => {
                                                if let Ok(mut engine) = domain.ignore_engine.write()
                                                {
                                                    engine.toggle_vcs_respect();
                                                    ui_state.respect_vcs_ignore =
                                                        engine.respect_vcs;
                                                    ui_state.add_notification(
                                                        format!(
                                                            "Respect .gitignore: {}",
                                                            if ui_state.respect_vcs_ignore {
                                                                "ON"
                                                            } else {
                                                                "OFF"
                                                            }
                                                        ),
                                                        app::ToastKind::Info,
                                                    );
                                                }
                                            }
                                            1 => {
                                                if let Ok(mut engine) = domain.ignore_engine.write()
                                                {
                                                    engine.ignore_vcs_files =
                                                        !engine.ignore_vcs_files;
                                                    ui_state.ignore_gitignore_files =
                                                        engine.ignore_vcs_files;
                                                    ui_state.add_notification(
                                                        format!(
                                                            "Hide .gitignore: {}",
                                                            if ui_state.ignore_gitignore_files {
                                                                "ON"
                                                            } else {
                                                                "OFF"
                                                            }
                                                        ),
                                                        app::ToastKind::Info,
                                                    );
                                                }
                                            }
                                            2 => ui_state.hide_all_popups(),
                                            _ => {}
                                        }
                                    }
                                }
                            } else {
                                let list_rect = ui_state.file_list_rect;
                                let stats_rect = ui_state.stats_rect;
                                let footer_rect = ui_state.footer_rect;

                                let divider_col = list_rect.x + list_rect.width;
                                let is_near_divider = mouse.column == divider_col
                                    || mouse.column == divider_col.saturating_sub(1)
                                    || mouse.column == divider_col.saturating_add(1);

                                if is_near_divider {
                                    ui_state.is_dragging_divider = true;
                                    let main_chunks_width =
                                        list_rect.width + ui_state.diff_view_rect.width;
                                    let click_offset = mouse.column.saturating_sub(list_rect.x);
                                    if main_chunks_width > 0 {
                                        let pct = (click_offset as f32 / main_chunks_width as f32
                                            * 100.0)
                                            as u16;
                                        ui_state.file_list_width_pct = pct.clamp(15, 85);
                                    }
                                } else {
                                    if mouse.column >= list_rect.x
                                        && mouse.column < list_rect.x + list_rect.width
                                        && mouse.row > list_rect.y
                                        && mouse.row < list_rect.y + list_rect.height
                                    {
                                        let clicked_row = mouse.row - (list_rect.y + 1);
                                        let visible_count = domain
                                            .modifications
                                            .iter()
                                            .filter(|m| !domain.is_ignored(&m.path))
                                            .count();
                                        if (clicked_row as usize) < visible_count {
                                            ui_state.selected_index = clicked_row as usize;
                                            ui_state.reset_diff_scroll_to_first_change(&domain);
                                        }
                                    } else if mouse.row >= stats_rect.y
                                        && mouse.row < stats_rect.y + stats_rect.height
                                    {
                                        let rel_x = mouse.column as i32 - stats_rect.x as i32;
                                        if (0..18).contains(&rel_x) {
                                            ui_state.selected_index = 0;
                                            ui_state.reset_diff_scroll_to_first_change(&domain);
                                        } else if (18..36).contains(&rel_x) {
                                            let stats = domain.stats();
                                            ui_state.add_log(format!(
                                                "Total additions: +{} lines",
                                                stats.lines_added
                                            ));
                                        } else if (36..54).contains(&rel_x) {
                                            let stats = domain.stats();
                                            ui_state.add_log(format!(
                                                "Total deletions: -{} lines",
                                                stats.lines_deleted
                                            ));
                                        } else if (54..72).contains(&rel_x) {
                                            ui_state.clear_all(&mut domain);
                                        } else if rel_x >= 72 {
                                            let current = tick_rate_ms
                                                .load(std::sync::atomic::Ordering::Relaxed);
                                            let new_rate =
                                                if current <= 50 { 500 } else { current - 50 };
                                            tick_rate_ms.store(
                                                new_rate,
                                                std::sync::atomic::Ordering::Relaxed,
                                            );
                                            ui_state.tick_rate_ms = new_rate;
                                            ui_state.add_log(format!(
                                                "Speed cycled (tick rate: {}ms)",
                                                new_rate
                                            ));
                                        }
                                    } else if mouse.column >= ui_state.header_rect.x
                                        && mouse.column
                                            < ui_state.header_rect.x + ui_state.header_rect.width
                                        && mouse.row >= ui_state.header_rect.y
                                        && mouse.row
                                            < ui_state.header_rect.y + ui_state.header_rect.height
                                    {
                                        let rel_x =
                                            mouse.column as i32 - ui_state.header_rect.x as i32;
                                        let from_right = ui_state.header_rect.width as i32 - rel_x;
                                        if (0..10).contains(&from_right) {
                                            ui_state.show_popup(app::PopupKind::Menu);
                                            ui_state.menu_selected = 0;
                                        } else if (11..25).contains(&from_right) {
                                            let current = tick_rate_ms
                                                .load(std::sync::atomic::Ordering::Relaxed);
                                            let new_rate =
                                                if current <= 50 { 500 } else { current - 50 };
                                            tick_rate_ms.store(
                                                new_rate,
                                                std::sync::atomic::Ordering::Relaxed,
                                            );
                                            ui_state.tick_rate_ms = new_rate;
                                            ui_state.add_log(format!(
                                                "Speed cycled (tick rate: {}ms)",
                                                new_rate
                                            ));
                                        } else if (26..38).contains(&from_right) {
                                            ui_state.toggle_ignore_menu(&domain);
                                        } else if (39..50).contains(&from_right) {
                                            ui_state.selected_index = 0;
                                            ui_state.reset_diff_scroll_to_first_change(&domain);
                                        }
                                    } else if mouse.column >= ui_state.logs_rect.x
                                        && mouse.column
                                            < ui_state.logs_rect.x + ui_state.logs_rect.width
                                        && mouse.row >= ui_state.logs_rect.y
                                        && mouse.row
                                            < ui_state.logs_rect.y + ui_state.logs_rect.height
                                    {
                                        ui_state.logs.clear();
                                        ui_state.add_log("Logs cleared.".to_string());
                                    } else if mouse.row == footer_rect.y {
                                        let x = mouse.column;
                                        if (47..=54).contains(&x) {
                                            ui_state.toggle_ignore_menu(&domain);
                                        } else if (55..=62).contains(&x) {
                                            ui_state.clear_all(&mut domain);
                                        } else if (63..=72).contains(&x) {
                                            let current = tick_rate_ms
                                                .load(std::sync::atomic::Ordering::Relaxed);
                                            let new_rate =
                                                if current <= 50 { 500 } else { current - 50 };
                                            tick_rate_ms.store(
                                                new_rate,
                                                std::sync::atomic::Ordering::Relaxed,
                                            );
                                            ui_state.tick_rate_ms = new_rate;
                                            ui_state.add_log(format!(
                                                "Speed cycled (tick rate: {}ms)",
                                                new_rate
                                            ));
                                        } else if (73..=80).contains(&x) {
                                            ui_state.show_popup(app::PopupKind::Menu);
                                            ui_state.menu_selected = 0;
                                        } else if (81..=88).contains(&x) {
                                            ui_state.should_quit = true;
                                        }
                                    }
                                }
                            }
                        }
                        crossterm::event::MouseEventKind::Drag(
                            crossterm::event::MouseButton::Left,
                        ) => {
                            if ui_state.is_dragging_divider {
                                let list_rect = ui_state.file_list_rect;
                                let main_chunks_width =
                                    list_rect.width + ui_state.diff_view_rect.width;
                                let click_offset = mouse.column.saturating_sub(list_rect.x);
                                if main_chunks_width > 0 {
                                    let pct = (click_offset as f32 / main_chunks_width as f32
                                        * 100.0)
                                        as u16;
                                    ui_state.file_list_width_pct = pct.clamp(15, 85);
                                }
                            }
                        }
                        crossterm::event::MouseEventKind::Up(
                            crossterm::event::MouseButton::Left,
                        ) => {
                            ui_state.is_dragging_divider = false;
                        }
                        crossterm::event::MouseEventKind::ScrollUp => {
                            let diff_rect = ui_state.diff_view_rect;
                            if mouse.column >= diff_rect.x
                                && mouse.column < diff_rect.x + diff_rect.width
                                && mouse.row >= diff_rect.y
                                && mouse.row < diff_rect.y + diff_rect.height
                            {
                                ui_state.scroll_up();
                            } else {
                                ui_state.select_previous(&domain);
                            }
                        }
                        crossterm::event::MouseEventKind::ScrollDown => {
                            let diff_rect = ui_state.diff_view_rect;
                            if mouse.column >= diff_rect.x
                                && mouse.column < diff_rect.x + diff_rect.width
                                && mouse.row >= diff_rect.y
                                && mouse.row < diff_rect.y + diff_rect.height
                            {
                                ui_state.scroll_down();
                            } else {
                                ui_state.select_next(&domain);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::Key(code, modifiers) => {
                // Quit conditions: Ctrl+C (always)
                if code == crossterm::event::KeyCode::Char('c')
                    && modifiers.contains(crossterm::event::KeyModifiers::CONTROL)
                {
                    ui_state.should_quit = true;
                } else if ui_state.editor_visible {
                    if ui_state.editor_save_prompt {
                        match code {
                            crossterm::event::KeyCode::Char('y')
                            | crossterm::event::KeyCode::Char('Y') => {
                                if let (Some(editor), Some(path)) =
                                    (&ui_state.editor_instance, &ui_state.editor_file_path)
                                {
                                    let path_clone = path.clone();
                                    let content = editor.code_ref().get_content();
                                    let full_path = canonical_path.join(&path_clone);
                                    if let Err(e) = std::fs::write(&full_path, content) {
                                        ui_state.add_log(format!("Failed to save file: {}", e));
                                        ui_state.add_notification(
                                            format!("Save failed: {}", e),
                                            app::ToastKind::Error,
                                        );
                                    } else {
                                        ui_state.add_log(format!("Saved {}", path_clone));
                                        ui_state.add_notification(
                                            format!("Saved {}", path_clone),
                                            app::ToastKind::Success,
                                        );
                                        ui_state.editor_has_changes = false;
                                    }
                                }
                                ui_state.hide_all_popups();
                                ui_state.hide_save_prompt();
                                ui_state.editor_instance = None;
                                ui_state.editor_file_path = None;
                            }
                            crossterm::event::KeyCode::Char('n')
                            | crossterm::event::KeyCode::Char('N') => {
                                ui_state.hide_all_popups();
                                ui_state.hide_save_prompt();
                                ui_state.editor_instance = None;
                                ui_state.editor_file_path = None;
                            }
                            crossterm::event::KeyCode::Char('c')
                            | crossterm::event::KeyCode::Char('C')
                            | crossterm::event::KeyCode::Esc => {
                                ui_state.hide_save_prompt();
                            }
                            _ => {}
                        }
                    } else {
                        match code {
                            crossterm::event::KeyCode::Esc => {
                                if ui_state.editor_has_changes {
                                    ui_state.show_popup(app::PopupKind::SavePrompt);
                                } else {
                                    ui_state.hide_all_popups();
                                    ui_state.editor_instance = None;
                                    ui_state.editor_file_path = None;
                                }
                            }
                            crossterm::event::KeyCode::Char('s')
                                if modifiers.contains(crossterm::event::KeyModifiers::CONTROL) =>
                            {
                                if let (Some(editor), Some(path)) =
                                    (&ui_state.editor_instance, &ui_state.editor_file_path)
                                {
                                    let path_clone = path.clone();
                                    let content = editor.code_ref().get_content();
                                    let full_path = canonical_path.join(&path_clone);
                                    if let Err(e) = std::fs::write(&full_path, content) {
                                        ui_state.add_log(format!("Failed to save file: {}", e));
                                        ui_state.add_notification(
                                            format!("Save failed: {}", e),
                                            app::ToastKind::Error,
                                        );
                                    } else {
                                        ui_state.add_log(format!("Saved {}", path_clone));
                                        ui_state.add_notification(
                                            format!("Saved {}", path_clone),
                                            app::ToastKind::Success,
                                        );
                                        ui_state.editor_has_changes = false;
                                    }
                                }
                            }
                            _ => {
                                if let Some(ref mut editor) = ui_state.editor_instance {
                                    let block = ratatui::widgets::Block::default()
                                        .borders(ratatui::widgets::Borders::ALL)
                                        .border_type(ratatui::widgets::BorderType::Rounded);
                                    let editor_rect = block.inner(ui_state.popup_rect);

                                    let key_event =
                                        crossterm::event::KeyEvent::new(code, modifiers);
                                    if let Err(e) = editor.input(key_event, &editor_rect) {
                                        ui_state.add_log(format!("Editor error: {}", e));
                                    } else {
                                        if let Some(ref path) = ui_state.editor_file_path {
                                            let full_path = canonical_path.join(path);
                                            if let Ok(orig) = std::fs::read_to_string(full_path) {
                                                let current = editor.code_ref().get_content();
                                                ui_state.editor_has_changes = current != orig;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if code == crossterm::event::KeyCode::Esc {
                    if ui_state.overlay_state.is_open() {
                        ui_state.hide_all_popups();
                    } else {
                        ui_state.show_popup(app::PopupKind::Menu);
                        ui_state.menu_selected = 0;
                    }
                } else if code == crossterm::event::KeyCode::Char('q') {
                    if ui_state.overlay_state.is_open() {
                        if ui_state.ignore_input_visible {
                            ui_state.ignore_input_char('q');
                        } else {
                            ui_state.hide_all_popups();
                        }
                    } else {
                        ui_state.should_quit = true;
                    }
                } else if ui_state.menu_visible {
                    match code {
                        crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                            if ui_state.menu_selected > 0 {
                                ui_state.menu_selected -= 1;
                            } else {
                                ui_state.menu_selected = 7;
                            }
                        }
                        crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                            if ui_state.menu_selected < 7 {
                                ui_state.menu_selected += 1;
                            } else {
                                ui_state.menu_selected = 0;
                            }
                        }
                        crossterm::event::KeyCode::Enter => {
                            ui_state.hide_all_popups();
                            match ui_state.menu_selected {
                                0 => ui_state.show_popup(app::PopupKind::Help),
                                1 => ui_state.show_popup(app::PopupKind::Settings),
                                2 => ui_state.toggle_ignore_menu(&domain),
                                3 => ui_state.toggle_active_ignores(&domain),
                                4 => {
                                    ui_state.show_popup(app::PopupKind::IgnoreInput);
                                    ui_state.ignore_input_text.clear();
                                    ui_state.ignore_cursor_idx = 0;
                                }
                                5 => ui_state.clear_all(&mut domain),
                                6 => {} // Close Menu (already hidden)
                                7 => ui_state.should_quit = true,
                                _ => {}
                            }
                        }
                        crossterm::event::KeyCode::Char('m') => {
                            ui_state.hide_all_popups();
                        }
                        _ => {}
                    }
                } else if ui_state.ignore_input_visible {
                    match code {
                        crossterm::event::KeyCode::Enter => {
                            ui_state.ignore_input_apply(&mut domain);
                        }

                        crossterm::event::KeyCode::Backspace => {
                            ui_state.ignore_input_backspace();
                        }
                        crossterm::event::KeyCode::Left => {
                            ui_state.ignore_input_left();
                        }
                        crossterm::event::KeyCode::Right => {
                            ui_state.ignore_input_right();
                        }
                        crossterm::event::KeyCode::Char(c) => {
                            ui_state.ignore_input_char(c);
                        }
                        _ => {}
                    }
                } else if ui_state.ignore_menu_visible {
                    match code {
                        crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                            ui_state.ignore_menu_up();
                        }
                        crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                            ui_state.ignore_menu_down();
                        }
                        crossterm::event::KeyCode::Enter => {
                            ui_state.ignore_menu_apply(&mut domain);
                        }
                        _ => {}
                    }
                } else if ui_state.active_ignores_visible {
                    match code {
                        crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                            ui_state.active_ignores_up();
                        }
                        crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                            ui_state.active_ignores_down();
                        }
                        crossterm::event::KeyCode::Enter => {
                            ui_state.remove_active_ignore(&domain);
                        }
                        crossterm::event::KeyCode::Char('x')
                        | crossterm::event::KeyCode::Char('X') => {
                            ui_state.clear_active_ignores(&domain);
                        }
                        _ => {}
                    }
                } else if ui_state.settings_visible {
                    match code {
                        crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                            if ui_state.settings_selected > 0 {
                                ui_state.settings_selected -= 1;
                            } else {
                                ui_state.settings_selected = 2;
                            }
                        }
                        crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                            if ui_state.settings_selected < 2 {
                                ui_state.settings_selected += 1;
                            } else {
                                ui_state.settings_selected = 0;
                            }
                        }
                        crossterm::event::KeyCode::Enter => match ui_state.settings_selected {
                            0 => {
                                if let Ok(mut engine) = domain.ignore_engine.write() {
                                    engine.toggle_vcs_respect();
                                    ui_state.respect_vcs_ignore = engine.respect_vcs;
                                    ui_state.add_notification(
                                        format!(
                                            "Respect .gitignore: {}",
                                            if ui_state.respect_vcs_ignore { "ON" } else { "OFF" }
                                        ),
                                        app::ToastKind::Info,
                                    );
                                }
                            }
                            1 => {
                                if let Ok(mut engine) = domain.ignore_engine.write() {
                                    engine.ignore_vcs_files = !engine.ignore_vcs_files;
                                    ui_state.ignore_gitignore_files = engine.ignore_vcs_files;
                                    ui_state.add_notification(
                                        format!(
                                            "Hide .gitignore: {}",
                                            if ui_state.ignore_gitignore_files {
                                                "ON"
                                            } else {
                                                "OFF"
                                            }
                                        ),
                                        app::ToastKind::Info,
                                    );
                                }
                            }
                            2 => ui_state.hide_all_popups(),
                            _ => {}
                        },
                        _ => {}
                    }
                } else if ui_state.help_visible {
                    if code == crossterm::event::KeyCode::Char('?') {
                        ui_state.hide_all_popups();
                    }
                } else {
                    match code {
                        crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                            ui_state.select_previous(&domain);
                        }
                        crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                            ui_state.select_next(&domain);
                        }
                        crossterm::event::KeyCode::PageUp => {
                            ui_state.scroll_up();
                        }
                        crossterm::event::KeyCode::PageDown => {
                            ui_state.scroll_down();
                        }
                        crossterm::event::KeyCode::Left | crossterm::event::KeyCode::Char('h') => {
                            ui_state.scroll_left();
                        }
                        crossterm::event::KeyCode::Right | crossterm::event::KeyCode::Char('l') => {
                            ui_state.scroll_right();
                        }
                        crossterm::event::KeyCode::Char('i') => {
                            ui_state.toggle_ignore_menu(&domain);
                        }
                        crossterm::event::KeyCode::Char('c') => {
                            ui_state.clear_all(&mut domain);
                        }
                        crossterm::event::KeyCode::Char('r')
                        | crossterm::event::KeyCode::Char('R') => {
                            if let Ok(mut engine) = ignore_engine_arc.write() {
                                engine.ignore_list.clear();
                                let warnings = engine.load_vcs_ignores(&canonical_path);
                                ui_state.add_log("Reloaded ignore configuration.".to_string());
                                ui_state.add_notification(
                                    "Reloaded ignore cfg".to_string(),
                                    app::ToastKind::Success,
                                );
                                for warning in warnings {
                                    ui_state.add_log(warning);
                                }
                            }
                        }
                        crossterm::event::KeyCode::Char('+')
                        | crossterm::event::KeyCode::Char('=') => {
                            let current = tick_rate_ms.load(std::sync::atomic::Ordering::Relaxed);
                            let new_rate = if current <= 50 { 50 } else { current - 50 };
                            tick_rate_ms.store(new_rate, std::sync::atomic::Ordering::Relaxed);
                            ui_state.tick_rate_ms = new_rate;
                            ui_state
                                .add_log(format!("Speed increased (tick rate: {}ms)", new_rate));
                            ui_state.add_notification(
                                format!("Tick rate: {}ms", new_rate),
                                app::ToastKind::Info,
                            );
                        }
                        crossterm::event::KeyCode::Char('-')
                        | crossterm::event::KeyCode::Char('_') => {
                            let current = tick_rate_ms.load(std::sync::atomic::Ordering::Relaxed);
                            let new_rate = (current + 50).min(2000);
                            tick_rate_ms.store(new_rate, std::sync::atomic::Ordering::Relaxed);
                            ui_state.tick_rate_ms = new_rate;
                            ui_state
                                .add_log(format!("Speed decreased (tick rate: {}ms)", new_rate));
                            ui_state.add_notification(
                                format!("Tick rate: {}ms", new_rate),
                                app::ToastKind::Info,
                            );
                        }
                        crossterm::event::KeyCode::Char('?') => {
                            ui_state.show_popup(app::PopupKind::Help);
                        }
                        crossterm::event::KeyCode::Char('e') => {
                            let selected_path = {
                                let visible_mods: Vec<_> = domain
                                    .modifications
                                    .iter()
                                    .filter(|m| !domain.is_ignored(&m.path))
                                    .collect();
                                visible_mods.get(ui_state.selected_index).map(|m| m.path.clone())
                            };
                            if let Some(path) = selected_path {
                                let full_path = canonical_path.join(&path);
                                match std::fs::read_to_string(&full_path) {
                                    Ok(content) => {
                                        let extension = std::path::Path::new(&path)
                                            .extension()
                                            .and_then(|ext| ext.to_str())
                                            .unwrap_or("txt");

                                        match ratatui_code_editor::editor::Editor::new(
                                            extension,
                                            &content,
                                            ratatui_code_editor::theme::vesper(),
                                        ) {
                                            Ok(editor) => {
                                                ui_state.editor_instance = Some(Box::new(editor));
                                                ui_state.editor_file_path = Some(path.clone());
                                                ui_state.show_popup(app::PopupKind::Editor);
                                                ui_state.editor_has_changes = false;
                                                ui_state.hide_save_prompt();
                                                ui_state
                                                    .add_log(format!("Opened {} in editor", path));
                                            }
                                            Err(err) => {
                                                ui_state.add_log(format!(
                                                    "Failed to start editor: {}",
                                                    err
                                                ));
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        ui_state.add_log(format!("Failed to read file: {}", err));
                                    }
                                }
                            }
                        }
                        crossterm::event::KeyCode::Char('m') => {
                            ui_state.show_popup(app::PopupKind::Menu);
                            ui_state.menu_selected = 0;
                        }
                        _ => {}
                    }
                }
            }
        }

        while let Some(dom_ev) = domain.events.pop_front() {
            match dom_ev {
                app::DomainEvent::FileChanged { path, added, deleted } => {
                    ui_state.add_log(format!("File changed: {} (+{}, -{})", path, added, deleted));
                }
                app::DomainEvent::IgnoreAdded { pattern } => {
                    ui_state.add_log(format!("Ignore pattern added: {}", pattern));
                }
                app::DomainEvent::HistoryCleared => {
                    ui_state.add_log("History cleared".to_string());
                }
            }
        }

        if ui_state.should_quit {
            break;
        }

        terminal.draw(|f| adapters::ui::draw(f, &mut ui_state, &domain))?;
    }

    // Restore terminal
    restore_terminal()?;

    Ok(())
}
