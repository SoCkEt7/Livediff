// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use super::{Component, Palette, get_file_type};
use crate::app::{MonitorDomain, TerminalUiState};

pub struct FileListComponent;

impl Component for FileListComponent {
    type State = TerminalUiState;
    type Context = MonitorDomain;

    fn draw(&self, f: &mut Frame<'_>, area: Rect, state: &mut Self::State, ctx: &Self::Context) {
        let now = std::time::SystemTime::now();
        let items: Vec<ListItem<'_>> = ctx
            .modifications
            .iter()
            .filter(|m| !ctx.is_ignored(&m.path))
            .enumerate()
            .map(|(i, m)| {
                let elapsed =
                    now.duration_since(m.timestamp).unwrap_or(std::time::Duration::from_secs(0));
                let time_str = if elapsed.as_secs() < 60 {
                    format!("{}s ago", elapsed.as_secs())
                } else if elapsed.as_secs() < 3600 {
                    format!("{}m ago", elapsed.as_secs() / 60)
                } else {
                    format!("{}h ago", elapsed.as_secs() / 3600)
                };

                let ft = get_file_type(&m.path);
                let is_selected = i == state.selected_index;
                let phase = (state.anim_frame as f32 * 0.08) % 1.0;

                // Dynamic icon color based on change intensity
                let change_intensity = ((m.added + m.deleted) as f32 / 100.0).min(1.0);
                let icon_color =
                    if is_selected { ft.color } else { super::get_value_color(change_intensity) };

                let mut line_spans = if is_selected {
                    tui_shimmer::shimmer_spans_with_style_at_phase(
                        &format!(" {} ", ft.icon),
                        Style::default().fg(icon_color).add_modifier(Modifier::BOLD),
                        phase,
                    )
                } else {
                    vec![Span::styled(format!(" {} ", ft.icon), Style::default().fg(icon_color))]
                };

                if m.is_binary {
                    line_spans
                        .push(Span::styled(" BINARY ", Style::default().fg(Color::Rgb(220, 0, 0))));
                } else {
                    line_spans.push(Span::styled(
                        format!("{:<6} ", ft.label),
                        Style::default().fg(Palette::TEXT_MUTED),
                    ));
                }

                line_spans.push(Span::styled(
                    format!("{:<8} ", time_str),
                    Style::default().fg(Palette::TEXT_MUTED),
                ));

                if is_selected {
                    let mut add_spans = tui_shimmer::shimmer_spans_with_style_at_phase(
                        &format!("+{} ", m.added),
                        Style::default().fg(Color::Rgb(46, 204, 113)).add_modifier(Modifier::BOLD),
                        phase,
                    );
                    line_spans.append(&mut add_spans);

                    let mut del_spans = tui_shimmer::shimmer_spans_with_style_at_phase(
                        &format!("-{} ", m.deleted),
                        Style::default().fg(Color::Rgb(231, 76, 60)).add_modifier(Modifier::BOLD),
                        phase,
                    );
                    line_spans.append(&mut del_spans);
                } else {
                    line_spans.push(Span::styled(
                        format!("+{} ", m.added),
                        Style::default().fg(Color::Rgb(46, 204, 113)),
                    ));
                    line_spans.push(Span::styled(
                        format!("-{} ", m.deleted),
                        Style::default().fg(Color::Rgb(231, 76, 60)),
                    ));
                }

                line_spans.push(Span::raw(&m.path));
                ListItem::new(Line::from(line_spans))
            })
            .collect();

        let border_color =
            if state.ignore_input_visible || state.ignore_menu_visible || state.help_visible {
                Palette::BORDER_DARK
            } else {
                Palette::BORDER_FOCUS
            };

        let phase = (state.anim_frame as f32 * 0.08) % 1.0;
        let title_spans = tui_shimmer::shimmer_spans_with_style_at_phase(
            " ◈ RECENT CHANGES ",
            Style::default().fg(Palette::PRIMARY).add_modifier(Modifier::BOLD).bg(Palette::BG_DARK),
            phase,
        );

        let list = List::new(items)
            .block(
                Block::default()
                    .title(Line::from(title_spans))
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(Style::default().fg(border_color)),
            )
            .highlight_style(
                Style::default().bg(Color::Rgb(30, 30, 45)).add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(" ❱ ");

        let mut list_state = ListState::default();
        list_state.select(Some(state.selected_index));

        f.render_stateful_widget(list, area, &mut list_state);
    }
}
