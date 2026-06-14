// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use super::{Component, Palette};
use crate::app::{MonitorDomain, TerminalUiState};

pub struct HeaderComponent;

impl Component for HeaderComponent {
    type State = TerminalUiState;
    type Context = MonitorDomain;

    fn draw(&self, f: &mut Frame<'_>, area: Rect, state: &mut Self::State, ctx: &Self::Context) {
        let anim_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        let spinner = anim_chars[state.anim_frame % anim_chars.len()];

        let (status_text, status_color) = if ctx.modifications.is_empty() {
            (format!(" {} STANDBY ", spinner), Color::Rgb(243, 156, 18)) // Orange/Yellow
        } else {
            (format!(" {} LIVE ", spinner), Color::Rgb(46, 204, 113)) // Bright Green
        };

        let cwd = std::env::current_dir()
            .map(|p| p.file_name().unwrap_or_default().to_string_lossy().into_owned())
            .unwrap_or_else(|_| "Unknown".to_string());

        let phase = (state.anim_frame as f32 * 0.08) % 1.0;
        let mut left_spans = tui_shimmer::shimmer_spans_with_style_at_phase(
            " ◈ LIVEDIFF ",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Rgb(10, 10, 15))
                .bg(Palette::PRIMARY),
            phase,
        );
        left_spans.push(Span::styled(
            "",
            Style::default().fg(Palette::PRIMARY).bg(Color::Rgb(40, 40, 55)),
        ));
        left_spans.push(Span::styled(
            format!("  {} ", cwd),
            Style::default().fg(Palette::TEXT_BRIGHT).bg(Color::Rgb(40, 40, 55)),
        ));
        left_spans
            .push(Span::styled("", Style::default().fg(Color::Rgb(40, 40, 55)).bg(status_color)));

        let mut status_spans = tui_shimmer::shimmer_spans_with_style_at_phase(
            &status_text,
            Style::default()
                .fg(Color::Rgb(10, 10, 15))
                .bg(status_color)
                .add_modifier(Modifier::BOLD),
            phase,
        );
        left_spans.append(&mut status_spans);
        left_spans.push(Span::styled("", Style::default().fg(status_color)));

        let left_content = Line::from(left_spans);

        let ram_str = &state.ram_usage;
        let ignore_count =
            if let Ok(engine) = ctx.ignore_engine.read() { engine.ignore_list.len() } else { 0 };

        let right_content = Line::from(vec![
            Span::styled("", Style::default().fg(Color::Rgb(55, 55, 75))),
            Span::styled(
                format!("  {} ", ram_str),
                Style::default()
                    .fg(Color::Rgb(46, 204, 113))
                    .bg(Color::Rgb(55, 55, 75))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "",
                Style::default().fg(Color::Rgb(40, 40, 55)).bg(Color::Rgb(55, 55, 75)),
            ),
            Span::styled(
                format!("  {} Files ", ctx.total_files),
                Style::default().fg(Palette::TEXT_BRIGHT).bg(Color::Rgb(40, 40, 55)),
            ),
            Span::styled(
                "",
                Style::default().fg(Color::Rgb(55, 55, 75)).bg(Color::Rgb(40, 40, 55)),
            ),
            Span::styled(
                format!(" {} Ignored ", ignore_count),
                Style::default().fg(Palette::TEXT_BRIGHT).bg(Color::Rgb(55, 55, 75)),
            ),
            Span::styled(
                "",
                Style::default().fg(Color::Rgb(40, 40, 55)).bg(Color::Rgb(55, 55, 75)),
            ),
            Span::styled(
                format!("  {}ms ", state.tick_rate_ms),
                Style::default()
                    .fg(Palette::PRIMARY)
                    .bg(Color::Rgb(40, 40, 55))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("", Style::default().fg(Palette::BORDER_DARK).bg(Color::Rgb(40, 40, 55))),
            Span::styled(
                " ? Help ",
                Style::default()
                    .fg(Color::Rgb(10, 10, 15))
                    .bg(Palette::BORDER_DARK)
                    .add_modifier(Modifier::BOLD),
            ),
        ]);

        let layout_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(area);

        f.render_widget(
            Paragraph::new(left_content).alignment(ratatui::layout::Alignment::Left),
            layout_chunks[0],
        );
        f.render_widget(
            Paragraph::new(right_content).alignment(ratatui::layout::Alignment::Right),
            layout_chunks[1],
        );
    }
}
