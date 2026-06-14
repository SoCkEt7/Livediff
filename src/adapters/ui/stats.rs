// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use super::{Component, Palette, get_value_color};
use crate::app::{MonitorDomain, TerminalUiState};

pub struct StatsComponent;

impl Component for StatsComponent {
    type State = TerminalUiState;
    type Context = MonitorDomain;

    fn draw(&self, f: &mut Frame<'_>, area: Rect, state: &mut Self::State, ctx: &Self::Context) {
        let stats = ctx.stats();

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(18),
                Constraint::Length(18),
                Constraint::Length(18),
                Constraint::Length(18),
                Constraint::Min(10),
            ])
            .split(area);

        // Normalize values to 0..1 to pick dynamic colors from our gradient
        let files_ratio = (stats.modified as f32 / 20.0).min(1.0);
        let added_ratio = (stats.lines_added as f32 / 500.0).min(1.0);
        let deleted_ratio = (stats.lines_deleted as f32 / 500.0).min(1.0);
        let events_ratio = (ctx.events_count as f32 / 100.0).min(1.0);

        let items = [
            ("FILES", format!("{}", stats.modified), get_value_color(files_ratio)),
            ("ADDED", format!("+{}", stats.lines_added), get_value_color(added_ratio)),
            ("DELETED", format!("-{}", stats.lines_deleted), get_value_color(deleted_ratio)),
            ("EVENTS", format!("{}", ctx.events_count), get_value_color(events_ratio)),
        ];

        for (i, (label, value, color)) in items.iter().enumerate() {
            let p = Paragraph::new(Line::from(vec![
                Span::styled(format!(" {} ", label), Style::default().fg(Palette::TEXT_MUTED)),
                Span::styled(value, Style::default().fg(*color).add_modifier(Modifier::BOLD)),
            ]))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(Style::default().fg(Palette::BORDER_DARK)),
            );
            f.render_widget(p, chunks[i]);
        }

        // Draw real-time sparkline graph inside chunks[4]
        let max_val = *state.event_history.iter().max().unwrap_or(&0);
        let mut sparkline_str = String::new();

        for chunk in state.event_history.chunks(2) {
            let mut code = 0u32;
            let get_dots = |val: u64, max: u64| -> u8 {
                if max == 0 || val == 0 {
                    0
                } else {
                    ((val as f32 / max as f32) * 4.0).round() as u8
                }
            };

            if let Some(&left) = chunk.first() {
                let dots = get_dots(left, max_val);
                if dots >= 1 {
                    code |= 0x40;
                } // Dot 7
                if dots >= 2 {
                    code |= 0x04;
                } // Dot 3
                if dots >= 3 {
                    code |= 0x02;
                } // Dot 2
                if dots >= 4 {
                    code |= 0x01;
                } // Dot 1
            }
            if let Some(&right) = chunk.get(1) {
                let dots = get_dots(right, max_val);
                if dots >= 1 {
                    code |= 0x80;
                } // Dot 8
                if dots >= 2 {
                    code |= 0x20;
                } // Dot 6
                if dots >= 3 {
                    code |= 0x10;
                } // Dot 5
                if dots >= 4 {
                    code |= 0x08;
                } // Dot 4
            }

            // Baseline ⠤ if no events
            if code == 0 {
                code = 0x40 | 0x80;
            }

            sparkline_str.push(char::from_u32(0x2800 + code).unwrap_or(' '));
        }

        let sparkline_paragraph = Paragraph::new(Line::from(vec![
            Span::styled(" ACTIVITY GRAPH ❱ ", Style::default().fg(Palette::TEXT_MUTED)),
            Span::styled(
                sparkline_str,
                Style::default().fg(Palette::PRIMARY).add_modifier(Modifier::BOLD),
            ),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Style::default().fg(Palette::BORDER_DARK)),
        );
        f.render_widget(sparkline_paragraph, chunks[4]);
    }
}
