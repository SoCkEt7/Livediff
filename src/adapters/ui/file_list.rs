// Copyright (c) 2026 Nyxia. All rights reserved.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use super::{Component, Palette, get_file_type};
use crate::app::{MonitorDomain, TerminalUiState};

pub struct FileListComponent;

impl Component for FileListComponent {
    type State = TerminalUiState;
    type Context = MonitorDomain;

    fn draw(&self, f: &mut Frame<'_>, area: Rect, state: &mut Self::State, ctx: &Self::Context) {
        let now = std::time::SystemTime::now();
        let visible_mods = state.get_visible_modifications(ctx);

        let items: Vec<ListItem<'_>> = visible_mods
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let elapsed =
                    now.duration_since(m.timestamp).unwrap_or(std::time::Duration::from_secs(0));
                let time_str = if elapsed.as_secs() < 60 {
                    format!("{}s", elapsed.as_secs())
                } else if elapsed.as_secs() < 3600 {
                    format!("{}m", elapsed.as_secs() / 60)
                } else {
                    format!("{}h", elapsed.as_secs() / 3600)
                };

                let ft = get_file_type(&m.path);
                let is_selected = i == state.selected_index;
                let change_intensity = ((m.added + m.deleted) as f32 / 100.0).min(1.0);
                let icon_color =
                    if is_selected { ft.color } else { super::get_value_color(change_intensity) };

                let mut spans = vec![
                    Span::styled(format!(" {} ", ft.icon), Style::default().fg(icon_color)),
                    Span::styled(
                        format!("{:<4} ", ft.label),
                        Style::default().fg(Palette::TEXT_MUTED),
                    ),
                    Span::styled(
                        format!("+{:<2}", m.added),
                        Style::default().fg(Color::Rgb(46, 204, 113)),
                    ),
                    Span::styled(
                        format!("-{:<2}", m.deleted),
                        Style::default().fg(Color::Rgb(231, 76, 60)),
                    ),
                    Span::styled(
                        format!(" {} ", time_str),
                        Style::default().fg(Palette::TEXT_MUTED),
                    ),
                ];

                // Git status badge
                if state.git_info.is_git_repo {
                    let git_badge = state.git_info.get_status_for(&m.path);
                    let (badge, badge_color) = match git_badge {
                        Some(crate::domain::git_info::GitFileStatus::Staged) => {
                            ("●", Color::Rgb(52, 152, 219))
                        }
                        Some(crate::domain::git_info::GitFileStatus::Modified) => {
                            ("●", Color::Rgb(230, 126, 34))
                        }
                        Some(crate::domain::git_info::GitFileStatus::Untracked) => {
                            ("○", Color::Rgb(231, 76, 60))
                        }
                        Some(crate::domain::git_info::GitFileStatus::Deleted) => {
                            ("✕", Color::Rgb(231, 76, 60))
                        }
                        Some(crate::domain::git_info::GitFileStatus::Renamed) => {
                            ("◎", Color::Rgb(155, 89, 182))
                        }
                        _ => (" ", Palette::TEXT_MUTED),
                    };
                    spans.push(Span::styled(
                        format!(" {}", badge),
                        Style::default().fg(badge_color),
                    ));
                }

                spans.push(Span::raw(&m.path));

                let style = if is_selected {
                    Style::default().fg(Palette::TEXT_BRIGHT).bg(Color::Rgb(25, 25, 40))
                } else {
                    Style::default().fg(Palette::TEXT_MUTED)
                };

                ListItem::new(Line::from(spans)).style(style)
            })
            .collect();

        let border_color = Palette::BORDER_FOCUS;

        let total_count = ctx.modifications.iter().filter(|m| !ctx.is_ignored(&m.path)).count();
        let filtered_count = visible_mods.len();

        let count_str = if state.filter_query.is_empty() {
            format!("  {} ", total_count)
        } else {
            format!("  {}/{} ", filtered_count, total_count)
        };

        let title_parts = vec![
            Span::styled(" ◈ ", Style::default().fg(Palette::PRIMARY)),
            Span::styled(
                "FILES",
                Style::default().fg(Palette::TEXT_BRIGHT).add_modifier(Modifier::BOLD),
            ),
            Span::styled(count_str, Style::default().fg(Palette::TEXT_MUTED)),
            if state.filter_active {
                Span::styled(
                    " [FILTER] ",
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                )
            } else {
                Span::raw("")
            },
        ];

        let show_filter_bar = state.filter_active || !state.filter_query.is_empty();

        if show_filter_bar && area.height > 6 {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(3)])
                .split(area);

            let list = List::new(items)
                .block(
                    Block::default()
                        .title(Line::from(title_parts))
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(border_color)),
                )
                .highlight_style(
                    Style::default().bg(Color::Rgb(30, 30, 50)).add_modifier(Modifier::BOLD),
                );

            let mut list_state = ListState::default();
            list_state.select(Some(state.selected_index));
            f.render_stateful_widget(list, chunks[0], &mut list_state);

            // Filter bar
            let filter_border =
                if state.filter_active { Palette::ACCENT } else { Palette::BORDER_DARK };

            let filter_spans = vec![
                Span::styled(
                    " / ",
                    Style::default().fg(Palette::PRIMARY).add_modifier(Modifier::BOLD),
                ),
                Span::styled(&state.filter_query, Style::default().fg(Palette::TEXT_BRIGHT)),
                if state.filter_active {
                    Span::styled(
                        "█",
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::RAPID_BLINK),
                    )
                } else {
                    Span::raw("")
                },
                Span::raw(" "),
                Span::styled(
                    if state.filter_active { "(ESC/Enter to confirm)" } else { "(/ to edit)" },
                    Style::default().fg(Palette::TEXT_MUTED),
                ),
            ];

            let filter_paragraph = Paragraph::new(Line::from(filter_spans)).block(
                Block::default()
                    .title(Span::styled(
                        " FILTER ",
                        Style::default().fg(filter_border).add_modifier(Modifier::BOLD),
                    ))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(filter_border)),
            );
            f.render_widget(filter_paragraph, chunks[1]);
        } else {
            let list = List::new(items)
                .block(
                    Block::default()
                        .title(Line::from(title_parts))
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(border_color)),
                )
                .highlight_style(
                    Style::default().bg(Color::Rgb(30, 30, 50)).add_modifier(Modifier::BOLD),
                );

            let mut list_state = ListState::default();
            list_state.select(Some(state.selected_index));
            f.render_stateful_widget(list, area, &mut list_state);
        }
    }
}
