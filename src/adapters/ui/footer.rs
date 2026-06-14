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

    fn draw(&self, f: &mut Frame<'_>, area: Rect, state: &mut Self::State, _ctx: &Self::Context) {
        let mut spans = vec![
            Span::styled(" ◈ ", Style::default().fg(Palette::PRIMARY)),
            Span::styled(
                format!("v{} ", env!("CARGO_PKG_VERSION")),
                Style::default().add_modifier(Modifier::BOLD).fg(Palette::TEXT_BRIGHT),
            ),
            Span::styled("│ ", Style::default().fg(Palette::BORDER_DARK)),
            Span::styled("© 2026 Antonin Nvh ", Style::default().fg(Palette::TEXT_MUTED)),
            Span::styled(" ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled(" ", Style::default().fg(Color::Rgb(0, 119, 181))), // LinkedIn Blue
            Span::styled("│ ", Style::default().fg(Palette::BORDER_DARK)),
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
            Span::styled("│ ", Style::default().fg(Palette::BORDER_DARK)),
            Span::styled("↑↓←→ ", Style::default().fg(Palette::PRIMARY)),
            Span::styled("Nav ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled("│ ", Style::default().fg(Palette::BORDER_DARK)),
            Span::styled("I/C ", Style::default().fg(Palette::PRIMARY)),
            Span::styled("Ign/Clr ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled("│ ", Style::default().fg(Palette::BORDER_DARK)),
            Span::styled("+/- ", Style::default().fg(Palette::PRIMARY)),
            Span::styled("Speed ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled("│ ", Style::default().fg(Palette::BORDER_DARK)),
            Span::styled("M/? ", Style::default().fg(Palette::PRIMARY)),
            Span::styled("Menu/Help ", Style::default().fg(Palette::TEXT_BRIGHT)),
            Span::styled("│ ", Style::default().fg(Palette::BORDER_DARK)),
            Span::styled("Q ", Style::default().fg(Palette::PRIMARY)),
            Span::styled("Quit ", Style::default().fg(Palette::TEXT_BRIGHT)),
        ]);

        let p = Paragraph::new(Line::from(spans)).style(Style::default().bg(Palette::BG_DARK));
        f.render_widget(p, area);
    }
}
