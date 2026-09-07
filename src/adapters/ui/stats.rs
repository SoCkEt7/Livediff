// Copyright (c) 2026 Nyxia. All rights reserved.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use super::{Component, Palette, get_value_color};
use crate::app::{MonitorDomain, TerminalUiState};

pub struct StatsComponent;

impl Component for StatsComponent {
    type State = TerminalUiState;
    type Context = MonitorDomain;

    fn draw(&self, f: &mut Frame<'_>, area: Rect, state: &mut Self::State, ctx: &Self::Context) {
        let stats = ctx.stats();

        let files_ratio = (stats.modified as f32 / 20.0).min(1.0);
        let added_ratio = (stats.lines_added as f32 / 500.0).min(1.0);
        let deleted_ratio = (stats.lines_deleted as f32 / 500.0).min(1.0);
        let events_ratio = (ctx.events_count as f32 / 100.0).min(1.0);

        // Ratio gauge computation
        let total_diff_lines = stats.lines_added + stats.lines_deleted;
        let (add_blocks, del_blocks) = if total_diff_lines == 0 {
            (5, 5)
        } else {
            let add_ratio = stats.lines_added as f32 / total_diff_lines as f32;
            let add_count = (add_ratio * 10.0).round() as usize;
            (add_count.min(10), (10usize.saturating_sub(add_count)))
        };

        let border_dark = state.current_theme.border_dark();
        let primary_color = state.current_theme.primary();

        // Compact single-line stats with inline sparkline and ratio gauge
        let mut spans = vec![
            Span::styled("▎", Style::default().fg(border_dark)),
            Span::styled("  ", Style::default().fg(get_value_color(files_ratio))),
            Span::styled(
                format!("{}", stats.modified),
                Style::default().fg(get_value_color(files_ratio)).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" files ", Style::default().fg(Palette::TEXT_MUTED)),
            Span::styled("│", Style::default().fg(border_dark)),
            Span::styled(" +", Style::default().fg(get_value_color(added_ratio))),
            Span::styled(
                format!("{}", stats.lines_added),
                Style::default().fg(get_value_color(added_ratio)).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" / ", Style::default().fg(border_dark)),
            Span::styled("-", Style::default().fg(get_value_color(deleted_ratio))),
            Span::styled(
                format!("{}", stats.lines_deleted),
                Style::default().fg(get_value_color(deleted_ratio)).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" [", Style::default().fg(border_dark)),
            Span::styled("█".repeat(add_blocks), Style::default().fg(Color::Rgb(46, 204, 113))),
            Span::styled("░".repeat(del_blocks), Style::default().fg(Color::Rgb(231, 76, 60))),
            Span::styled("] ", Style::default().fg(border_dark)),
            Span::styled("│", Style::default().fg(border_dark)),
            Span::styled(" ⚡ ", Style::default().fg(get_value_color(events_ratio))),
            Span::styled(
                format!("{}", ctx.events_count),
                Style::default().fg(get_value_color(events_ratio)).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" events ", Style::default().fg(Palette::TEXT_MUTED)),
            Span::styled("│", Style::default().fg(border_dark)),
        ];

        // Inline sparkline
        let max_val = *state.event_history.iter().max().unwrap_or(&0);
        let mut sparkline_str = String::new();
        for chunk in state.event_history.chunks(2) {
            let mut code = 0u32;
            if let Some(&left) = chunk.first() {
                let dots = if max_val == 0 {
                    0
                } else {
                    ((left as f32 / max_val as f32) * 4.0).round() as u8
                };
                if dots >= 1 {
                    code |= 0x40;
                }
                if dots >= 2 {
                    code |= 0x04;
                }
                if dots >= 3 {
                    code |= 0x02;
                }
                if dots >= 4 {
                    code |= 0x01;
                }
            }
            if let Some(&right) = chunk.get(1) {
                let dots = if max_val == 0 {
                    0
                } else {
                    ((right as f32 / max_val as f32) * 4.0).round() as u8
                };
                if dots >= 1 {
                    code |= 0x80;
                }
                if dots >= 2 {
                    code |= 0x20;
                }
                if dots >= 3 {
                    code |= 0x10;
                }
                if dots >= 4 {
                    code |= 0x08;
                }
            }
            if code == 0 {
                code = 0x40 | 0x80;
            }
            sparkline_str.push(char::from_u32(0x2800 + code).unwrap_or(' '));
        }

        spans.push(Span::styled(sparkline_str, Style::default().fg(primary_color)));
        spans.push(Span::styled(" ▎", Style::default().fg(border_dark)));

        let p = Paragraph::new(Line::from(spans));
        f.render_widget(p, area);
    }
}
