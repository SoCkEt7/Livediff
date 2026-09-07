// Copyright (c) 2026 Nyxia. All rights reserved.

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

    fn draw(&self, f: &mut Frame<'_>, area: Rect, state: &mut Self::State, _ctx: &Self::Context) {
        let primary = state.current_theme.primary();
        let border_dark = state.current_theme.border_dark();

        let mut spans = vec![
            Span::styled(" ◈ ", Style::default().fg(primary)),
            Span::styled(
                format!("v{} ", env!("CARGO_PKG_VERSION")),
                Style::default().add_modifier(Modifier::BOLD).fg(Palette::TEXT_BRIGHT),
            ),
            Span::styled("│ ", Style::default().fg(border_dark)),
            Span::styled(
                format!("{} ", state.current_theme.name()),
                Style::default().fg(Palette::TEXT_MUTED),
            ),
            Span::styled("│ ", Style::default().fg(border_dark)),
        ];

        let phase = (state.anim_frame as f32 * 0.08) % 1.0;
        let git_text = if state.respect_vcs_ignore { " GIT " } else { " !GIT " };
        let git_color = if state.respect_vcs_ignore { Color::Green } else { Color::Red };

        let mut git_spans = tui_shimmer::shimmer_spans_with_style_at_phase(
            git_text,
            Style::default().fg(git_color).add_modifier(Modifier::BOLD),
            phase,
        );
        spans.append(&mut git_spans);

        spans.extend(vec![
            Span::styled("│ ", Style::default().fg(border_dark)),
            Span::styled("v/Tab ", Style::default().fg(primary)),
            Span::styled("View ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled("│ ", Style::default().fg(border_dark)),
            Span::styled("/ ", Style::default().fg(primary)),
            Span::styled("Filter ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled("│ ", Style::default().fg(border_dark)),
            Span::styled("y ", Style::default().fg(primary)),
            Span::styled("Yank ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled("│ ", Style::default().fg(border_dark)),
            Span::styled("s ", Style::default().fg(primary)),
            Span::styled("Patch ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled("│ ", Style::default().fg(border_dark)),
            Span::styled("t ", Style::default().fg(primary)),
            Span::styled("Theme ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled("│ ", Style::default().fg(border_dark)),
            Span::styled("W ", Style::default().fg(primary)),
            Span::styled("Wrap ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled("│ ", Style::default().fg(border_dark)),
            Span::styled("w ", Style::default().fg(primary)),
            Span::styled("WS ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled("│ ", Style::default().fg(border_dark)),
            Span::styled("i/c ", Style::default().fg(primary)),
            Span::styled("Ign/Clr ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled("│ ", Style::default().fg(border_dark)),
            Span::styled("? ", Style::default().fg(primary)),
            Span::styled("Help ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled("│ ", Style::default().fg(border_dark)),
            Span::styled("q ", Style::default().fg(primary)),
            Span::styled("Quit ", Style::default().fg(Palette::TEXT_BRIGHT)),
        ]);

        let p = Paragraph::new(Line::from(spans)).style(Style::default().bg(Palette::BG_DARK));
        f.render_widget(p, area);
    }
}
