// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
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

        let title_line = Line::from(vec![Span::styled(
            " ◈ ACTIVITY LOG ",
            Style::default().fg(Color::Rgb(241, 196, 15)).add_modifier(Modifier::BOLD),
        )]);

        let p = Paragraph::new(text)
            .block(
                Block::default()
                    .title(title_line)
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(Style::default().fg(Palette::BORDER_DARK)),
            )
            .wrap(Wrap { trim: false });

        f.render_widget(p, area);
    }
}
// Import Span for our title_line mapping
use ratatui::text::Span;
