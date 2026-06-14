// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use super::{Component, Palette};
use crate::app::{MonitorDomain, TerminalUiState};

pub struct FooterComponent;

impl Component for FooterComponent {
    type State = TerminalUiState;
    type Context = MonitorDomain;

    fn draw(&self, f: &mut Frame<'_>, area: Rect, _state: &mut Self::State, _ctx: &Self::Context) {
        let p = Paragraph::new(Line::from(vec![
            Span::styled(" ◈ ", Style::default().fg(Palette::PRIMARY)),
            Span::styled(
                "Livediff ",
                Style::default().add_modifier(Modifier::BOLD).fg(Palette::TEXT_BRIGHT),
            ),
            Span::styled(
                format!("v{} ", env!("CARGO_PKG_VERSION")),
                Style::default().fg(Palette::TEXT_MUTED),
            ),
            Span::styled("│ ", Style::default().fg(Palette::BORDER_DARK)),
            Span::styled("↑↓ ", Style::default().fg(Palette::PRIMARY)),
            Span::styled("Sel ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled("│ ", Style::default().fg(Palette::BORDER_DARK)),
            Span::styled("PgUp/Dn/←→ ", Style::default().fg(Palette::PRIMARY)),
            Span::styled("Scrl ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled("│ ", Style::default().fg(Palette::BORDER_DARK)),
            Span::styled("I ", Style::default().fg(Palette::PRIMARY)),
            Span::styled("Ign ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled("│ ", Style::default().fg(Palette::BORDER_DARK)),
            Span::styled("C ", Style::default().fg(Palette::PRIMARY)),
            Span::styled("Clr ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled("│ ", Style::default().fg(Palette::BORDER_DARK)),
            Span::styled("+/- ", Style::default().fg(Palette::PRIMARY)),
            Span::styled("Freq ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled("│ ", Style::default().fg(Palette::BORDER_DARK)),
            Span::styled("? ", Style::default().fg(Palette::PRIMARY)),
            Span::styled("Help ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled("│ ", Style::default().fg(Palette::BORDER_DARK)),
            Span::styled("Q ", Style::default().fg(Palette::PRIMARY)),
            Span::styled("Quit ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled("│ ", Style::default().fg(Palette::BORDER_DARK)),
            Span::styled("© 2026 ", Style::default().fg(Palette::TEXT_MUTED)),
            Span::styled("Antonin Nvh", Style::default().fg(Palette::PRIMARY)),
            Span::raw(" ( "),
            Span::styled(
                "socket7",
                Style::default().fg(Color::Rgb(46, 204, 113)).add_modifier(Modifier::UNDERLINED),
            ),
            Span::raw(") |  "),
            Span::styled(
                "antonin-nvh",
                Style::default().fg(Color::Rgb(80, 150, 230)).add_modifier(Modifier::UNDERLINED),
            ),
        ]))
        .alignment(ratatui::layout::Alignment::Left)
        .style(Style::default().fg(Palette::TEXT_MUTED));

        f.render_widget(p, area);
    }
}
