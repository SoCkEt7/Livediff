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
        let primary = state.current_theme.primary();
        let border_dark = state.current_theme.border_dark();
        let accent = state.current_theme.accent();

        let mut spans = vec![
            Span::styled(" ◈ ", Style::default().fg(primary)),
            Span::styled(
                format!("v{} ", env!("CARGO_PKG_VERSION")),
                Style::default().add_modifier(Modifier::BOLD).fg(Palette::TEXT_BRIGHT),
            ),
            Span::styled("│ ", Style::default().fg(border_dark)),
        ];

        // Mode badge
        if state.command_palette_visible {
            spans.push(Span::styled(
                " COMMAND PALETTE ",
                Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled(" │ ", Style::default().fg(border_dark)));
            spans.push(Span::styled("Type: ", Style::default().fg(primary)));
            spans.push(Span::styled("Search command  ", Style::default().fg(Palette::TEXT_BRIGHT)));
            spans.push(Span::styled("Enter: ", Style::default().fg(primary)));
            spans.push(Span::styled("Execute  ", Style::default().fg(Palette::TEXT_BRIGHT)));
            spans.push(Span::styled("Esc: ", Style::default().fg(primary)));
            spans.push(Span::styled("Close", Style::default().fg(Palette::TEXT_BRIGHT)));
        } else if state.diff_search_active {
            spans.push(Span::styled(
                " DIFF SEARCH ",
                Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled(" │ ", Style::default().fg(border_dark)));
            spans.push(Span::styled("Enter / n: ", Style::default().fg(primary)));
            spans.push(Span::styled("Next  ", Style::default().fg(Palette::TEXT_BRIGHT)));
            spans.push(Span::styled("N / Shift+Enter: ", Style::default().fg(primary)));
            spans.push(Span::styled("Prev  ", Style::default().fg(Palette::TEXT_BRIGHT)));
            spans.push(Span::styled("Esc: ", Style::default().fg(primary)));
            spans.push(Span::styled("Clear & Exit", Style::default().fg(Palette::TEXT_BRIGHT)));
        } else if state.filter_active {
            spans.push(Span::styled(
                " FILE FILTER ",
                Style::default().fg(Color::Black).bg(accent).add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled(" │ ", Style::default().fg(border_dark)));
            spans.push(Span::styled("Type: ", Style::default().fg(primary)));
            spans.push(Span::styled("Filter files  ", Style::default().fg(Palette::TEXT_BRIGHT)));
            spans.push(Span::styled("Enter / Esc: ", Style::default().fg(primary)));
            spans.push(Span::styled("Confirm", Style::default().fg(Palette::TEXT_BRIGHT)));
        } else {
            let phase = (state.anim_frame as f32 * 0.08) % 1.0;
            let git_text = if state.respect_vcs_ignore { " GIT " } else { " !GIT " };
            let git_color = if state.respect_vcs_ignore { Color::Green } else { Color::Red };

            let mut git_spans = tui_shimmer::shimmer_spans_with_style_at_phase(
                git_text,
                Style::default().fg(git_color).add_modifier(Modifier::BOLD),
                phase,
            );
            spans.append(&mut git_spans);

            let w = area.width;
            let mut shortcuts: Vec<(&str, &str)> = Vec::new();

            if w >= 120 {
                shortcuts.push((": / ^P", "Cmds"));
                shortcuts.push(("v", "Split"));
                shortcuts.push(("n/p", "Hunk"));
                shortcuts.push(("f", "Fold"));
                shortcuts.push(("^F", "Find"));
                shortcuts.push(("o", "Syms"));
                shortcuts.push(("/", "Filter"));
                shortcuts.push(("y", "Yank"));
                shortcuts.push(("t", "Theme"));
                shortcuts.push(("?", "Help"));
                shortcuts.push(("q", "Quit"));
            } else if w >= 90 {
                shortcuts.push((": ", "Cmds"));
                shortcuts.push(("v", "Split"));
                shortcuts.push(("n/p", "Hunk"));
                shortcuts.push(("^F", "Find"));
                shortcuts.push(("o", "Syms"));
                shortcuts.push(("/", "Filter"));
                shortcuts.push(("?", "Help"));
                shortcuts.push(("q", "Quit"));
            } else if w >= 60 {
                shortcuts.push((": ", "Cmds"));
                shortcuts.push(("v", "Split"));
                shortcuts.push(("?", "Help"));
                shortcuts.push(("q", "Quit"));
            } else {
                shortcuts.push(("?", "Help"));
                shortcuts.push(("q", "Quit"));
            }

            for (key, label) in shortcuts {
                spans.push(Span::styled("│ ", Style::default().fg(border_dark)));
                spans.push(Span::styled(
                    format!("{} ", key),
                    Style::default()
                        .fg(if key.contains(':') { Color::Yellow } else { primary })
                        .add_modifier(Modifier::BOLD),
                ));
                spans.push(Span::styled(
                    format!("{} ", label),
                    Style::default().fg(Palette::TEXT_BRIGHT),
                ));
            }

            if w >= 80 {
                spans.push(Span::styled("│ ", Style::default().fg(border_dark)));
                spans.push(Span::styled(
                    "© 2026 Antonin Nivoche ",
                    Style::default().fg(Palette::TEXT_MUTED),
                ));
                spans.push(Span::styled(" ", Style::default().fg(Palette::TEXT_BRIGHT)));
                spans.push(Span::styled(" ", Style::default().fg(Color::Rgb(0, 119, 181))));
            } else if w >= 45 {
                spans.push(Span::styled("│ ", Style::default().fg(border_dark)));
                spans.push(Span::styled(
                    "© 2026 Antonin Nvh ",
                    Style::default().fg(Palette::TEXT_MUTED),
                ));
                spans.push(Span::styled(" ", Style::default().fg(Palette::TEXT_BRIGHT)));
                spans.push(Span::styled(" ", Style::default().fg(Color::Rgb(0, 119, 181))));
            }
        }

        let p = Paragraph::new(Line::from(spans)).style(Style::default().bg(Palette::BG_DARK));
        f.render_widget(p, area);
    }
}
