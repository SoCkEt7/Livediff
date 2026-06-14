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
            .map(|m| {
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

                let mut line_spans =
                    vec![Span::styled(format!(" {} ", ft.icon), Style::default().fg(ft.color))];

                if m.is_binary {
                    line_spans.push(Span::styled(
                        "BIN    ",
                        Style::default().fg(Color::Rgb(220, 0, 220)),
                    ));
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
                line_spans.push(Span::styled(
                    format!("+{} ", m.added),
                    Style::default().fg(Color::Rgb(46, 204, 113)),
                ));
                line_spans.push(Span::styled(
                    format!("-{} ", m.deleted),
                    Style::default().fg(Color::Rgb(231, 76, 60)),
                ));
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

        let title_line = Line::from(vec![Span::styled(
            " ◈ RECENT CHANGES ",
            Style::default().fg(Palette::PRIMARY).add_modifier(Modifier::BOLD),
        )]);

        let list = List::new(items)
            .block(
                Block::default()
                    .title(title_line)
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
