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
    CommandPalette,
    IgnoreMenu,
    IgnoreInput,
    CodeEditor,
    ActiveIgnores,
    Settings,
    SymbolInspector,
}

impl Component for PopupComponent {
    type State = TerminalUiState;
    type Context = MonitorDomain;

    fn draw(&self, f: &mut Frame<'_>, area: Rect, state: &mut Self::State, _ctx: &Self::Context) {
        let (width, height) = match self {
            PopupComponent::GeneralMenu => (Constraint::Length(42), Constraint::Length(16)),
            PopupComponent::Help => (Constraint::Percentage(75), Constraint::Percentage(75)),
            PopupComponent::CommandPalette => (Constraint::Percentage(65), Constraint::Length(16)),
            PopupComponent::IgnoreMenu => (Constraint::Length(45), Constraint::Length(17)),
            PopupComponent::IgnoreInput => (Constraint::Length(60), Constraint::Length(8)),
            PopupComponent::CodeEditor => (Constraint::Percentage(80), Constraint::Percentage(80)),
            PopupComponent::ActiveIgnores => (Constraint::Length(50), Constraint::Length(20)),
            PopupComponent::Settings => (Constraint::Length(45), Constraint::Length(12)),
            PopupComponent::SymbolInspector => (Constraint::Length(60), Constraint::Length(22)),
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
                PopupComponent::Help => draw_help(f, popup_area, state),
                PopupComponent::CommandPalette => draw_command_palette(f, popup_area, state),
                PopupComponent::IgnoreMenu => draw_ignore_menu(f, popup_area, state),
                PopupComponent::IgnoreInput => draw_ignore_input(f, popup_area, state),
                PopupComponent::CodeEditor => draw_code_editor(f, popup_area, state),
                PopupComponent::ActiveIgnores => draw_active_ignores(f, popup_area, state),
                PopupComponent::Settings => draw_settings(f, popup_area, state),
                PopupComponent::SymbolInspector => draw_symbol_inspector(f, popup_area, state),
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
                Span::styled(
                    "[YES]",
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("[NO ]", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            },
        ])),
        ListItem::new(Line::from(vec![
            Span::raw(" 2. Hide .gitignore files: "),
            if state.ignore_gitignore_files {
                Span::styled(
                    "[YES]",
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                )
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
                    " ☰ LIVEDIFF NAVIGATION MENU ",
                    Style::default().add_modifier(Modifier::BOLD).fg(Palette::PRIMARY),
                ))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Double)
                .border_style(Style::default().fg(Palette::PRIMARY)),
        )
        .style(Style::default().bg(Palette::BG_DARK));

    f.render_widget(list, area);
}

fn draw_help(f: &mut Frame<'_>, area: Rect, state: &TerminalUiState) {
    let tabs_titles = [
        " 1. Navigation & View ",
        " 2. Diff & Hunks ",
        " 3. Search & Symbols ",
        " 4. Actions & Config ",
    ];

    let mut tab_spans = Vec::new();
    for (i, title) in tabs_titles.iter().enumerate() {
        if i == state.help_tab {
            tab_spans.push(Span::styled(
                *title,
                Style::default().fg(Color::Black).bg(Palette::PRIMARY).add_modifier(Modifier::BOLD),
            ));
        } else {
            tab_spans.push(Span::styled(
                *title,
                Style::default().fg(Palette::TEXT_MUTED).bg(Palette::BG_DARK),
            ));
        }
        if i < tabs_titles.len() - 1 {
            tab_spans.push(Span::raw(" "));
        }
    }

    let block = Block::default()
        .title(Span::styled(
            " ◈ LIVEDIFF HELP & KEYBOARD GUIDE ◈ ",
            Style::default().add_modifier(Modifier::BOLD).fg(Palette::PRIMARY),
        ))
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .border_style(Style::default().fg(Palette::PRIMARY))
        .style(Style::default().bg(Palette::BG_DARK));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Tab bar
            Constraint::Min(8),    // Content
            Constraint::Length(2), // Footer
        ])
        .split(inner_area);

    let tab_bar = Paragraph::new(Line::from(tab_spans));
    f.render_widget(tab_bar, chunks[0]);

    let content = match state.help_tab {
        0 => vec![
            Line::from(Span::styled(
                "▸ FILE LIST NAVIGATION & LAYOUT",
                Style::default().fg(Palette::PRIMARY).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "  Up / Down, k / j ",
                    Style::default().fg(Color::Rgb(241, 196, 15)).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Select file in tracked recent modifications list",
                    Style::default().fg(Palette::TEXT_BRIGHT),
                ),
            ]),
            Line::from(vec![
                Span::styled("  g / G            ", Style::default().fg(Color::Rgb(241, 196, 15))),
                Span::styled(
                    "Jump to top / bottom of current list or buffer",
                    Style::default().fg(Palette::TEXT_MUTED),
                ),
            ]),
            Line::from(vec![
                Span::styled("  PgUp / PgDn      ", Style::default().fg(Color::Rgb(241, 196, 15))),
                Span::styled(
                    "Scroll diff preview vertically by page",
                    Style::default().fg(Palette::TEXT_MUTED),
                ),
            ]),
            Line::from(vec![
                Span::styled("  h / l, Left/Right", Style::default().fg(Color::Rgb(241, 196, 15))),
                Span::styled(
                    "Scroll diff preview horizontally (long lines)",
                    Style::default().fg(Palette::TEXT_MUTED),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "  v / Tab          ",
                    Style::default().fg(Color::Rgb(241, 196, 15)).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Toggle Unified vs Side-by-Side (Split) diff view mode",
                    Style::default().fg(Palette::TEXT_BRIGHT),
                ),
            ]),
            Line::from(vec![
                Span::styled("  Mouse Drag       ", Style::default().fg(Color::Rgb(241, 196, 15))),
                Span::styled(
                    "Drag center divider between file list and diff view",
                    Style::default().fg(Palette::TEXT_MUTED),
                ),
            ]),
        ],
        1 => vec![
            Line::from(Span::styled(
                "▸ DIFF ENGINE, HUNKS & FOLDING",
                Style::default().fg(Palette::PRIMARY).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "  n / p, ] / [     ",
                    Style::default().fg(Color::Rgb(241, 196, 15)).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Jump to next / previous diff hunk with HUD counter [Hunk i/N]",
                    Style::default().fg(Palette::TEXT_BRIGHT),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "  f / F            ",
                    Style::default().fg(Color::Rgb(241, 196, 15)).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Toggle Smart Context Folding (focus only on modified hunks)",
                    Style::default().fg(Palette::TEXT_BRIGHT),
                ),
            ]),
            Line::from(vec![
                Span::styled("  W                ", Style::default().fg(Color::Rgb(241, 196, 15))),
                Span::styled(
                    "Toggle soft line-wrapping for long changed lines",
                    Style::default().fg(Palette::TEXT_MUTED),
                ),
            ]),
            Line::from(vec![
                Span::styled("  w                ", Style::default().fg(Color::Rgb(241, 196, 15))),
                Span::styled(
                    "Toggle ignore whitespace variations (indentation / spaces)",
                    Style::default().fg(Palette::TEXT_MUTED),
                ),
            ]),
        ],
        2 => vec![
            Line::from(Span::styled(
                "▸ IN-DIFF SEARCH & CODE INSPECTION",
                Style::default().fg(Palette::PRIMARY).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "  Ctrl+F           ",
                    Style::default().fg(Color::Rgb(241, 196, 15)).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Open real-time in-diff search bar (Enter: next, N: prev, Esc: clear)",
                    Style::default().fg(Palette::TEXT_BRIGHT),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "  o / O            ",
                    Style::default().fg(Color::Rgb(241, 196, 15)).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Open AST symbol outline inspector (Rust, Python, TS/JS, Go, C/C++)",
                    Style::default().fg(Palette::TEXT_BRIGHT),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "  /                ",
                    Style::default().fg(Color::Rgb(241, 196, 15)).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Filter and fuzzy match tracked files list by substring",
                    Style::default().fg(Palette::TEXT_BRIGHT),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "  Ctrl+P / :       ",
                    Style::default().fg(Color::Rgb(241, 196, 15)).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Open Command Palette for instant keyboard execution",
                    Style::default().fg(Palette::TEXT_BRIGHT),
                ),
            ]),
            Line::from(vec![
                Span::styled("  e                ", Style::default().fg(Color::Rgb(241, 196, 15))),
                Span::styled(
                    "Open active file in embedded modal code editor",
                    Style::default().fg(Palette::TEXT_MUTED),
                ),
            ]),
        ],
        _ => vec![
            Line::from(Span::styled(
                "▸ ACTIONS, THEMES & CONFIGURATION",
                Style::default().fg(Palette::PRIMARY).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "  y / Y            ",
                    Style::default().fg(Color::Rgb(241, 196, 15)).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Yank unified git patch directly to system clipboard",
                    Style::default().fg(Palette::TEXT_BRIGHT),
                ),
            ]),
            Line::from(vec![
                Span::styled("  s / S            ", Style::default().fg(Color::Rgb(241, 196, 15))),
                Span::styled(
                    "Export snapshot as timestamped .patch file",
                    Style::default().fg(Palette::TEXT_MUTED),
                ),
            ]),
            Line::from(vec![
                Span::styled("  t / T            ", Style::default().fg(Color::Rgb(241, 196, 15))),
                Span::styled(
                    "Cycle theme palette (Cyberpunk, Catppuccin, Tokyo Night, Nord, Gruvbox)",
                    Style::default().fg(Palette::TEXT_MUTED),
                ),
            ]),
            Line::from(vec![
                Span::styled("  i / I            ", Style::default().fg(Color::Rgb(241, 196, 15))),
                Span::styled(
                    "Open Ignore Options menu & custom pattern builder",
                    Style::default().fg(Palette::TEXT_MUTED),
                ),
            ]),
            Line::from(vec![
                Span::styled("  c / C            ", Style::default().fg(Color::Rgb(241, 196, 15))),
                Span::styled(
                    "Clear tracked modifications and history",
                    Style::default().fg(Palette::TEXT_MUTED),
                ),
            ]),
            Line::from(vec![
                Span::styled("  r / R            ", Style::default().fg(Color::Rgb(241, 196, 15))),
                Span::styled(
                    "Reload VCS ignore configurations (.gitignore, .livediffignore)",
                    Style::default().fg(Palette::TEXT_MUTED),
                ),
            ]),
            Line::from(vec![
                Span::styled("  + / -            ", Style::default().fg(Color::Rgb(241, 196, 15))),
                Span::styled(
                    "Increase / decrease filesystem polling speed",
                    Style::default().fg(Palette::TEXT_MUTED),
                ),
            ]),
            Line::from(vec![
                Span::styled("  q / Q            ", Style::default().fg(Color::Rgb(241, 196, 15))),
                Span::styled("Quit Livediff", Style::default().fg(Palette::TEXT_MUTED)),
            ]),
        ],
    };

    let p = Paragraph::new(content).style(Style::default().bg(Palette::BG_DARK));
    f.render_widget(p, chunks[1]);

    let footer_line = Line::from(vec![
        Span::styled(
            " [Tab / ← →] ",
            Style::default().fg(Palette::PRIMARY).add_modifier(Modifier::BOLD),
        ),
        Span::styled("Switch Tab  ", Style::default().fg(Palette::TEXT_MUTED)),
        Span::styled("·  ", Style::default().fg(Palette::BORDER_DARK)),
        Span::styled(
            " [ESC / ?] ",
            Style::default().fg(Palette::PRIMARY).add_modifier(Modifier::BOLD),
        ),
        Span::styled("Close  ", Style::default().fg(Palette::TEXT_MUTED)),
        Span::styled("·  ", Style::default().fg(Palette::BORDER_DARK)),
        Span::styled(
            format!("Livediff v{}  ", env!("CARGO_PKG_VERSION")),
            Style::default().fg(Palette::TEXT_MUTED),
        ),
        Span::styled("·  ", Style::default().fg(Palette::BORDER_DARK)),
        Span::styled(
            "© 2026 Antonin Nivoche (@SoCkEt7)  ",
            Style::default().fg(Palette::TEXT_BRIGHT).add_modifier(Modifier::BOLD),
        ),
        Span::styled("·  MIT / Apache-2.0", Style::default().fg(Palette::TEXT_MUTED)),
    ]);
    let footer_p = Paragraph::new(footer_line)
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().bg(Palette::BG_DARK));
    f.render_widget(footer_p, chunks[2]);
}

fn draw_command_palette(f: &mut Frame<'_>, area: Rect, state: &TerminalUiState) {
    let filtered_cmds = crate::domain::command_palette::CommandPaletteEngine::filter_commands(
        &state.command_palette_query,
    );

    let block = Block::default()
        .title(Span::styled(
            " ◈ COMMAND PALETTE (Type to search, ↑/↓ Select, ENTER Execute, ESC Close) ◈ ",
            Style::default().add_modifier(Modifier::BOLD).fg(Palette::PRIMARY),
        ))
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .border_style(Style::default().fg(Palette::PRIMARY))
        .style(Style::default().bg(Palette::BG_DARK));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search input box
            Constraint::Min(5),    // Commands list
        ])
        .split(inner_area);

    // Prompt input bar
    let input_line = Line::from(vec![
        Span::styled(" ❱ ", Style::default().fg(Palette::PRIMARY).add_modifier(Modifier::BOLD)),
        Span::styled(
            &state.command_palette_query,
            Style::default().fg(Palette::TEXT_BRIGHT).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "█",
            Style::default().fg(Palette::PRIMARY).add_modifier(Modifier::RAPID_BLINK),
        ),
        Span::raw(" "),
        Span::styled(
            format!("({} commands)", filtered_cmds.len()),
            Style::default().fg(Palette::TEXT_MUTED),
        ),
    ]);
    let input_block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(Palette::BORDER_DARK));
    f.render_widget(Paragraph::new(input_line).block(input_block), chunks[0]);

    if filtered_cmds.is_empty() {
        let empty_msg = vec![
            Line::from(""),
            Line::from(Span::styled(
                "  No matching command found.",
                Style::default().fg(Palette::TEXT_MUTED),
            )),
        ];
        f.render_widget(Paragraph::new(empty_msg), chunks[1]);
        return;
    }

    let items: Vec<ListItem<'_>> = filtered_cmds
        .iter()
        .enumerate()
        .map(|(i, cmd)| {
            let is_selected = i == state.command_palette_selected;
            let style = if is_selected {
                Style::default().fg(Color::Black).bg(Palette::PRIMARY).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Palette::TEXT_BRIGHT)
            };

            let category_style = if is_selected {
                Style::default().fg(Color::Rgb(40, 40, 40))
            } else {
                Style::default().fg(Palette::ACCENT)
            };

            let shortcut_style = if is_selected {
                Style::default().fg(Color::Rgb(20, 20, 20)).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Yellow)
            };

            let line = Line::from(vec![
                Span::styled(format!(" {:<10} ", cmd.category), category_style),
                Span::styled(format!("{:<42} ", cmd.title), style),
                Span::styled(format!(" {:>12} ", cmd.shortcut), shortcut_style),
            ]);
            ListItem::new(line).style(style)
        })
        .collect();

    let list = List::new(items).style(Style::default().bg(Palette::BG_DARK));
    f.render_widget(list, chunks[1]);
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
            // plain
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

fn draw_symbol_inspector(f: &mut Frame<'_>, area: Rect, state: &TerminalUiState) {
    if state.active_file_symbols.is_empty() {
        let text = vec![
            Line::from(""),
            Line::from(Span::styled(
                "No AST symbols detected in this file.",
                Style::default().fg(Palette::TEXT_MUTED),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Supported: Rust, Python, TS/JS, Go, C/C++, TOML/JSON",
                Style::default().fg(Palette::TEXT_MUTED),
            )),
        ];
        let p = Paragraph::new(text).alignment(ratatui::layout::Alignment::Center).block(
            Block::default()
                .title(Span::styled(
                    " AST SYMBOL INSPECTOR (ESC: Close) ",
                    Style::default().add_modifier(Modifier::BOLD).fg(Palette::PRIMARY),
                ))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Double)
                .border_style(Style::default().fg(Palette::PRIMARY)),
        );
        f.render_widget(p, area);
        return;
    }

    let items: Vec<_> = state
        .active_file_symbols
        .iter()
        .enumerate()
        .map(|(i, sym)| {
            let style = if i == state.symbol_inspector_selected {
                Style::default()
                    .fg(Palette::TEXT_BRIGHT)
                    .bg(Palette::PRIMARY)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Palette::TEXT_MUTED)
            };
            let line = Line::from(vec![
                Span::styled(format!(" {:<8} ", sym.kind), Style::default().fg(Color::Yellow)),
                Span::raw(format!("{:<28} ", sym.name)),
                Span::styled(
                    format!("L{}-L{}", sym.start_line, sym.end_line),
                    Style::default().fg(Palette::TEXT_MUTED),
                ),
            ]);
            ListItem::new(line).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(Span::styled(
                    " AST SYMBOLS (ENTER: Jump, ↑/↓: Select, ESC: Close) ",
                    Style::default().add_modifier(Modifier::BOLD).fg(Palette::PRIMARY),
                ))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Double)
                .border_style(Style::default().fg(Palette::PRIMARY)),
        )
        .style(Style::default().fg(Palette::TEXT_BRIGHT).bg(Palette::BG_DARK));

    f.render_widget(list, area);
}
