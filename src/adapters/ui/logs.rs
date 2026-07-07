// Copyright (c) 2026 Nyxia. All rights reserved.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use super::{Component, Palette};
use crate::app::{MonitorDomain, TerminalUiState};

pub struct LogsComponent;

impl Component for LogsComponent {
    type State = TerminalUiState;
    type Context = MonitorDomain;

    fn draw(&self, f: &mut Frame<'_>, area: Rect, state: &mut Self::State, _ctx: &Self::Context) {
        let text: Vec<Line<'_>> = state.logs.iter().map(|l| Line::from(l.as_str())).collect();

        let title_parts = vec![
            Span::styled(" ◈ ", Style::default().fg(Color::Rgb(241, 196, 15))),
            Span::styled(
                "LOG",
                Style::default().fg(Palette::TEXT_BRIGHT).add_modifier(Modifier::BOLD),
            ),
        ];

        let p = Paragraph::new(text)
            .block(
                Block::default()
                    .title(Line::from(title_parts))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Palette::BORDER_DARK)),
            )
            .wrap(Wrap { trim: false });

        f.render_widget(p, area);
    }
}

//use ratatui::text::Span;
use ratatui::style::Color;
