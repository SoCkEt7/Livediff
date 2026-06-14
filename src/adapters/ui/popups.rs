// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use tui_overlay::{Anchor, Backdrop, Overlay};

use super::{Component, Palette};
use crate::app::{MonitorDomain, TerminalUiState};

pub enum PopupComponent {
    GeneralMenu,
    Help,
    IgnoreMenu,
    IgnoreInput,
    CodeEditor,
    ActiveIgnores,
    Settings,
}

impl Component for PopupComponent {
    type State = TerminalUiState;
    type Context = MonitorDomain;

    fn draw(&self, f: &mut Frame<'_>, area: Rect, state: &mut Self::State, _ctx: &Self::Context) {
        let (width, height) = match self {
            PopupComponent::GeneralMenu => (Constraint::Length(42), Constraint::Length(16)),
            PopupComponent::Help => (Constraint::Percentage(60), Constraint::Percentage(65)),
            PopupComponent::IgnoreMenu => (Constraint::Length(45), Constraint::Length(17)),
            PopupComponent::IgnoreInput => (Constraint::Length(60), Constraint::Length(8)),
            PopupComponent::CodeEditor => (Constraint::Percentage(80), Constraint::Percentage(80)),
            PopupComponent::ActiveIgnores => (Constraint::Length(50), Constraint::Length(20)),
            PopupComponent::Settings => (Constraint::Length(45), Constraint::Length(12)),
        };

        let overlay = Overlay::new()
            .anchor(Anchor::Center)
            .width(width)
            .height(height)
            .backdrop(Backdrop::new(Color::Rgb(0, 0, 0)));

        f.render_stateful_widget(overlay, area, &mut state.overlay_state);

        if let Some(popup_area) = state.overlay_state.inner_area() {
            state.popup_rect = popup_area;

            match self {
                PopupComponent::GeneralMenu => draw_general_menu(f, popup_area, state),
                PopupComponent::Help => draw_help(f, popup_area),
                PopupComponent::IgnoreMenu => draw_ignore_menu(f, popup_area, state),
                PopupComponent::IgnoreInput => draw_ignore_input(f, popup_area, state),
                PopupComponent::CodeEditor => draw_code_editor(f, popup_area, state),
                PopupComponent::ActiveIgnores => draw_active_ignores(f, popup_area, state),
                PopupComponent::Settings => draw_settings(f, popup_area, state),
            }
        }

        // 7. Draw Notifications (Non-modal overlay)
        if !state.notifications.is_empty() {
            let overlay = Overlay::new()
                .anchor(Anchor::TopRight)
                .width(Constraint::Length(35))
                .height(Constraint::Length((state.notifications.len() * 3) as u16 + 1))
                .offset(1, 1);

            f.render_stateful_widget(overlay, area, &mut state.notification_overlay_state);

            if let Some(notif_area) = state.notification_overlay_state.inner_area() {
                let chunks = Layout::default()
                    .direction(ratatui::layout::Direction::Vertical)
                    .constraints(
                        state
                            .notifications
                            .iter()
                            .map(|_| Constraint::Length(3))
                            .collect::<Vec<_>>(),
                    )
                    .split(notif_area);

                for (i, toast) in state.notifications.iter().enumerate() {
                    let (icon, color) = match toast.kind {
                        crate::app::ToastKind::Info => ("  ", Palette::PRIMARY),
                        crate::app::ToastKind::Success => ("  ", Color::Green),
                        crate::app::ToastKind::Error => ("  ", Color::Red),
                    };

                    let block = Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(color))
                        .style(Style::default().bg(Palette::BG_DARK));

                    let inner = block.inner(chunks[i]);
                    f.render_widget(block, chunks[i]);

                    let text = Line::from(vec![
                        Span::styled(icon, Style::default().fg(color).add_modifier(Modifier::BOLD)),
                        Span::styled(&toast.message, Style::default().fg(Palette::TEXT_BRIGHT)),
                    ]);
                    f.render_widget(Paragraph::new(text), inner);
                }
            }
        }
    }
}

fn draw_settings(f: &mut Frame<'_>, area: Rect, state: &TerminalUiState) {
    let items: Vec<_> = vec![
        ListItem::new(Line::from(vec![
            Span::raw(" 1. Respect .gitignore:   "),
            if state.respect_vcs_ignore {
                Span::styled("[YES]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            } else {
                Span::styled("[NO ]", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            },
        ])),
        ListItem::new(Line::from(vec![
            Span::raw(" 2. Hide .gitignore files: "),
            if state.ignore_gitignore_files {
                Span::styled("[YES]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            } else {
                Span::styled("[NO ]", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            },
        ])),
        ListItem::new(Line::from(" 3. Close Settings")),
    ]
    .into_iter()
    .enumerate()
    .map(|(i, item)| {
        let style = if i == state.settings_selected {
            Style::default()
                .fg(Palette::TEXT_BRIGHT)
                .bg(Palette::PRIMARY)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Palette::TEXT_MUTED)
        };
        item.style(style)
    })
    .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(Span::styled(
                    " GLOBAL SETTINGS (ENTER: Toggle, ESC: Close) ",
                    Style::default().add_modifier(Modifier::BOLD).fg(Palette::PRIMARY),
                ))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Double)
                .border_style(Style::default().fg(Palette::PRIMARY)),
        )
        .style(Style::default().bg(Palette::BG_DARK));

    f.render_widget(list, area);
}

fn draw_active_ignores(f: &mut Frame<'_>, area: Rect, state: &TerminalUiState) {
    if state.active_ignores_list.is_empty() {
        let text = vec![
            Line::from(""),
            Line::from(Span::styled(
                "No custom ignore patterns active.",
                Style::default().fg(Palette::TEXT_MUTED),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Add patterns via 'Ignore Suggestions' or",
                Style::default().fg(Palette::TEXT_MUTED),
            )),
            Line::from(Span::styled(
                "'Custom Ignore Pattern'.",
                Style::default().fg(Palette::TEXT_MUTED),
            )),
        ];
        let p = Paragraph::new(text).alignment(ratatui::layout::Alignment::Center).block(
            Block::default()
                .title(Span::styled(
                    " MANAGE ACTIVE IGNORES (ESC: Close) ",
                    Style::default().add_modifier(Modifier::BOLD).fg(Palette::ACCENT),
                ))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Double)
                .border_style(Style::default().fg(Palette::ACCENT)),
        );
        f.render_widget(p, area);
        return;
    }

    let items: Vec<_> = state
        .active_ignores_list
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let style = if i == state.active_ignores_selected {
                Style::default()
                    .fg(Palette::TEXT_BRIGHT)
                    .bg(Palette::ACCENT)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Palette::TEXT_MUTED)
            };
            ListItem::new(Line::from(format!("  {}  ", opt))).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(Span::styled(
                    " MANAGE ACTIVE IGNORES (ENTER: Remove, X: Clear All, ESC: Close) ",
                    Style::default().add_modifier(Modifier::BOLD).fg(Palette::ACCENT),
                ))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Double)
                .border_style(Style::default().fg(Palette::ACCENT)),
        )
        .style(Style::default().fg(Palette::TEXT_BRIGHT).bg(Palette::BG_DARK));

    f.render_widget(list, area);
}

fn draw_general_menu(f: &mut Frame<'_>, area: Rect, state: &TerminalUiState) {
    let menu_options = [
        "1. Help & Controls",
        "2. Global Settings",
        "3. Ignore Suggestions",
        "4. Manage Active Ignores",
        "5. Custom Ignore Pattern",
        "6. Clear Tracked Changes",
        "7. Close Menu",
        "8. Quit Livediff",
    ];

    let items: Vec<_> = menu_options
        .iter()
        .enumerate()
        .map(|(i, &opt)| {
            let style = if i == state.menu_selected {
                Style::default()
                    .fg(Palette::TEXT_BRIGHT)
                    .bg(Palette::PRIMARY)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Palette::TEXT_MUTED)
            };
            ListItem::new(Line::from(format!("  {}  ", opt))).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(Span::styled(
                    " ☰ LIVEDIF NAVIGATION MENU ",
                    Style::default().add_modifier(Modifier::BOLD).fg(Palette::PRIMARY),
                ))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Double)
                .border_style(Style::default().fg(Palette::PRIMARY)),
        )
        .style(Style::default().bg(Palette::BG_DARK));

    f.render_widget(list, area);
}

fn draw_help(f: &mut Frame<'_>, area: Rect) {
    let help_content = vec![
        Line::from(vec![Span::styled(
            " ◈ Livediff Help Menu ",
            Style::default().add_modifier(Modifier::BOLD).fg(Palette::PRIMARY),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Up/Down, k/j   ", Style::default().fg(Color::Rgb(241, 196, 15))),
            Span::raw(" Select file in recent changes"),
        ]),
        Line::from(vec![
            Span::styled("  e              ", Style::default().fg(Color::Rgb(241, 196, 15))),
            Span::raw(" Open selected file in editor"),
        ]),
        Line::from(vec![
            Span::styled("  Left/Right, h/l ", Style::default().fg(Color::Rgb(241, 196, 15))),
            Span::raw(" Scroll diff preview horizontally"),
        ]),
        Line::from(vec![
            Span::styled("  PgUp/PgDn      ", Style::default().fg(Color::Rgb(241, 196, 15))),
            Span::raw(" Scroll diff preview vertically"),
        ]),
        Line::from(vec![
            Span::styled("  I / i          ", Style::default().fg(Color::Rgb(241, 196, 15))),
            Span::raw(" Open Ignore Options (Add/Remove)"),
        ]),
        Line::from(vec![
            Span::styled("  C / c          ", Style::default().fg(Color::Rgb(241, 196, 15))),
            Span::raw(" Clear all tracked changes and logs"),
        ]),
        Line::from(vec![
            Span::styled("  R / r          ", Style::default().fg(Color::Rgb(241, 196, 15))),
            Span::raw(" Reload ignore configuration files"),
        ]),
        Line::from(vec![
            Span::styled("  + / -          ", Style::default().fg(Color::Rgb(241, 196, 15))),
            Span::raw(" Increase / decrease update frequency"),
        ]),
        Line::from(vec![
            Span::styled("  ?              ", Style::default().fg(Color::Rgb(241, 196, 15))),
            Span::raw(" Toggle help menu"),
        ]),
        Line::from(vec![
            Span::styled("  Q / q          ", Style::default().fg(Color::Rgb(241, 196, 15))),
            Span::raw(" Quit Livediff"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            " Press ESC or ? to close help menu ",
            Style::default().fg(Palette::TEXT_MUTED),
        )]),
    ];

    let p = Paragraph::new(help_content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Double)
                .border_style(Style::default().fg(Palette::PRIMARY)),
        )
        .style(Style::default().bg(Palette::BG_DARK));

    f.render_widget(p, area);
}

fn draw_ignore_menu(f: &mut Frame<'_>, area: Rect, state: &TerminalUiState) {
    let items: Vec<_> = state
        .ignore_menu_options
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let style = if i == state.ignore_menu_selected {
                Style::default()
                    .fg(Palette::TEXT_BRIGHT)
                    .bg(Palette::PRIMARY)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Palette::TEXT_MUTED)
            };
            ListItem::new(Line::from(opt.clone())).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(Span::styled(
                    " ACTIVE IGNORE PATTERNS ",
                    Style::default().add_modifier(Modifier::BOLD).fg(Palette::ACCENT),
                ))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Double)
                .border_style(Style::default().fg(Palette::ACCENT)),
        )
        .style(Style::default().fg(Palette::TEXT_BRIGHT).bg(Palette::BG_DARK));

    f.render_widget(list, area);
}

fn draw_ignore_input(f: &mut Frame<'_>, area: Rect, state: &TerminalUiState) {
    let text = &state.ignore_input_text;
    let cursor_idx = state.ignore_cursor_idx.min(text.len());
    let left = &text[..cursor_idx];
    let right = &text[cursor_idx..];

    let content = vec![
        Line::from(vec![Span::styled(
            " Type a glob pattern (e.g. *.log, tests/): ",
            Style::default().fg(Palette::TEXT_MUTED),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled(" ❱ ", Style::default().fg(Palette::PRIMARY).add_modifier(Modifier::BOLD)),
            Span::styled(left, Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled(
                "█",
                Style::default().fg(Color::Rgb(150, 150, 150)).add_modifier(Modifier::RAPID_BLINK),
            ),
            Span::styled(right, Style::default().fg(Palette::TEXT_BRIGHT)),
        ]),
    ];

    let p = Paragraph::new(content)
        .block(
            Block::default()
                .title(Span::styled(
                    " CUSTOM IGNORE PATTERN ",
                    Style::default().add_modifier(Modifier::BOLD).fg(Palette::ACCENT),
                ))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Double)
                .border_style(Style::default().fg(Palette::ACCENT)),
        )
        .style(Style::default().bg(Palette::BG_DARK));

    f.render_widget(p, area);
}

fn draw_code_editor(f: &mut Frame<'_>, area: Rect, state: &mut TerminalUiState) {
    if let Some(ref mut editor) = state.editor_instance {
        let file_name = state.editor_file_path.as_deref().unwrap_or("Untitled");
        let border_style = if state.editor_has_changes {
            Style::default().fg(Palette::ACCENT)
        } else {
            Style::default().fg(Palette::PRIMARY)
        };

        let title_text = format!(
            " EDIT: {} {} ",
            file_name,
            if state.editor_has_changes { "[Modified]" } else { "" }
        );

        let block = Block::default()
            .title(Span::styled(title_text, border_style.add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(border_style)
            .style(Style::default().bg(Palette::BG_DARK));

        let inner_area = block.inner(area);
        f.render_widget(block, area);

        // Render the editor widget
        f.render_widget(&**editor, inner_area);
    }

    if state.editor_save_prompt {
        let overlay = Overlay::new()
            .anchor(Anchor::Center)
            .width(Constraint::Length(35))
            .height(Constraint::Length(10))
            .backdrop(Backdrop::new(Color::Rgb(0, 0, 0)));

        f.render_stateful_widget(overlay, area, &mut state.save_overlay_state);

        if let Some(save_area) = state.save_overlay_state.inner_area() {
            state.save_popup_rect = save_area;
            let block = Block::default()
                .title(Span::styled(
                    " SAVE CHANGES? ",
                    Style::default().add_modifier(Modifier::BOLD).fg(Palette::ACCENT),
                ))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Double)
                .border_style(Style::default().fg(Palette::ACCENT))
                .style(Style::default().bg(Palette::BG_DARK));

            f.render_widget(block, save_area);

            let text = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "You have unsaved changes.",
                    Style::default().fg(Palette::TEXT_BRIGHT),
                )),
                Line::from("Do you want to save them before closing?"),
                Line::from(""),
                Line::from(vec![
                    Span::styled(
                        " [Y] ",
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("Save & Close   "),
                    Span::styled(
                        " [N] ",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("Discard   "),
                    Span::styled(
                        " [C] ",
                        Style::default().fg(Palette::TEXT_MUTED).add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("Cancel"),
                ]),
            ];

            let paragraph = Paragraph::new(text).alignment(ratatui::layout::Alignment::Center);
            let inner_save_area = Layout::default()
                .margin(1)
                .constraints([Constraint::Percentage(100)])
                .split(save_area)[0];
            f.render_widget(paragraph, inner_save_area);
        }
    }
}
