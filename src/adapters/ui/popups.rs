// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};

use super::{Component, Palette, centered_rect};
use crate::app::{MonitorDomain, TerminalUiState};

pub enum PopupComponent {
    Help,
    IgnoreMenu,
    IgnoreInput,
}

impl Component for PopupComponent {
    type State = TerminalUiState;
    type Context = MonitorDomain;

    fn draw(&self, f: &mut Frame<'_>, area: Rect, state: &mut Self::State, _ctx: &Self::Context) {
        let popup_area = match self {
            PopupComponent::Help => centered_rect(60, 65, area),
            PopupComponent::IgnoreMenu => centered_rect(40, 40, area),
            PopupComponent::IgnoreInput => centered_rect(50, 20, area),
        };
        state.popup_rect = popup_area;

        match self {
            PopupComponent::Help => draw_help(f, popup_area),
            PopupComponent::IgnoreMenu => draw_ignore_menu(f, popup_area, state),
            PopupComponent::IgnoreInput => draw_ignore_input(f, popup_area, state),
        }
    }
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
            Span::styled("  Left/Right, h/l ", Style::default().fg(Color::Rgb(241, 196, 15))),
            Span::raw(" Scroll diff preview horizontally"),
        ]),
        Line::from(vec![
            Span::styled("  PgUp/PgDn      ", Style::default().fg(Color::Rgb(241, 196, 15))),
            Span::raw(" Scroll diff preview vertically"),
        ]),
        Line::from(vec![
            Span::styled("  I / i          ", Style::default().fg(Color::Rgb(241, 196, 15))),
            Span::raw(" Open Ignore Options menu"),
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
            " Mouse Controls: ",
            Style::default().add_modifier(Modifier::UNDERLINED).fg(Palette::PRIMARY),
        )]),
        Line::from(vec![
            Span::styled("  Left Click     ", Style::default().fg(Color::Rgb(241, 196, 15))),
            Span::raw(" Select clicked file in the list"),
        ]),
        Line::from(vec![
            Span::styled("  Scroll Wheel   ", Style::default().fg(Color::Rgb(241, 196, 15))),
            Span::raw(" Scroll active list or diff preview"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw(" Designed for high performance and low resource usage. "),
            Span::styled("© 2026 Antonin Nvh", Style::default().fg(Palette::PRIMARY)),
        ]),
    ];

    let p = Paragraph::new(help_content)
        .block(
            Block::default()
                .title(Span::styled(
                    " HELP ",
                    Style::default().add_modifier(Modifier::BOLD).fg(Color::Rgb(46, 204, 113)),
                ))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Double)
                .border_style(Style::default().fg(Color::Rgb(46, 204, 113))),
        )
        .style(Style::default().fg(Palette::TEXT_BRIGHT).bg(Palette::BG_DARK));

    f.render_widget(Clear, area); // clear background
    f.render_widget(p, area);
}

fn draw_ignore_menu(f: &mut Frame<'_>, area: Rect, state: &TerminalUiState) {
    let mut items = vec![];
    for (i, opt) in state.ignore_menu_options.iter().enumerate() {
        let prefix = if i == state.ignore_menu_selected { " ❱ " } else { "   " };
        let style = if i == state.ignore_menu_selected {
            Style::default().fg(Palette::PRIMARY).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Palette::TEXT_BRIGHT)
        };
        items.push(ListItem::new(Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(opt, style),
        ])));
    }

    let list = List::new(items)
        .block(
            Block::default()
                .title(Span::styled(
                    " IGNORE OPTIONS ",
                    Style::default().add_modifier(Modifier::BOLD).fg(Color::Rgb(241, 196, 15)),
                ))
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Double)
                .border_style(Style::default().fg(Color::Rgb(241, 196, 15))),
        )
        .style(Style::default().fg(Palette::TEXT_BRIGHT).bg(Palette::BG_DARK));

    f.render_widget(Clear, area);
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

    f.render_widget(Clear, area);
    f.render_widget(p, area);
}
