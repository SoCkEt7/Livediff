// Copyright (c) 2026 Nyxia. All rights reserved.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

use super::{Component, Palette, get_file_type};
use crate::app::{MonitorDomain, TerminalUiState};

pub struct DiffComponent;

impl Component for DiffComponent {
    type State = TerminalUiState;
    type Context = MonitorDomain;

    fn draw(&self, f: &mut Frame<'_>, area: Rect, state: &mut Self::State, ctx: &Self::Context) {
        let visible_mods: Vec<_> =
            ctx.modifications.iter().filter(|m| !ctx.is_ignored(&m.path)).collect();

        if visible_mods.is_empty() {
            return;
        }

        let mut text = Text::default();
        if let Some(m) = visible_mods.get(state.selected_index) {
            let ft = get_file_type(&m.path);
            let header_bg = if m.is_binary { Color::Rgb(220, 0, 220) } else { ft.color };
            let header_label = if m.is_binary { " BINARY " } else { ft.label };

            text.lines.push(Line::from(vec![
                Span::styled(
                    format!(" {} ", header_label),
                    Style::default().fg(Color::Rgb(10, 10, 15)).bg(header_bg),
                ),
                Span::raw(" "),
                Span::styled(
                    &m.path,
                    Style::default().add_modifier(Modifier::BOLD).fg(Palette::PRIMARY),
                ),
            ]));

            let size_str = if m.size < 1024 {
                format!("{}B", m.size)
            } else {
                format!("{:.1}K", m.size as f64 / 1024.0)
            };

            text.lines.push(Line::from(vec![
                Span::styled(size_str, Style::default().fg(Palette::TEXT_MUTED)),
                Span::raw(" · "),
                Span::styled(
                    chrono::DateTime::<chrono::Local>::from(m.timestamp)
                        .format("%H:%M:%S")
                        .to_string(),
                    Style::default().fg(Palette::TEXT_MUTED),
                ),
            ]));

            text.lines.push(Line::from(""));

            state.update_highlighting(ctx);

            let mut line_num = 1;
            for (change_type, spans) in &state.highlighted_diff {
                let bg_color = match change_type {
                    crate::domain::diff_engine::LineChangeType::Insert => {
                        Some(Color::Rgb(22, 50, 22))
                    }
                    crate::domain::diff_engine::LineChangeType::Delete => {
                        Some(Color::Rgb(50, 22, 22))
                    }
                    _ => None,
                };

                let prefix_style = match change_type {
                    crate::domain::diff_engine::LineChangeType::Insert => {
                        Style::default().fg(Color::Rgb(46, 204, 113))
                    }
                    crate::domain::diff_engine::LineChangeType::Delete => {
                        Style::default().fg(Color::Rgb(231, 76, 60))
                    }
                    crate::domain::diff_engine::LineChangeType::Header => {
                        Style::default().fg(Palette::ACCENT).add_modifier(Modifier::BOLD)
                    }
                    crate::domain::diff_engine::LineChangeType::Context => {
                        Style::default().fg(Palette::TEXT_MUTED)
                    }
                };

                match change_type {
                    crate::domain::diff_engine::LineChangeType::Header => {
                        line_num = 1;
                        if let Some((_, text_val)) = spans.first() {
                            text.lines
                                .push(Line::from(Span::styled(text_val.clone(), prefix_style)));
                        }
                    }
                    _ => {
                        let prefix_str = match change_type {
                            crate::domain::diff_engine::LineChangeType::Insert => {
                                let p = format!("{:>3} +", line_num);
                                line_num += 1;
                                p
                            }
                            crate::domain::diff_engine::LineChangeType::Delete => {
                                format!("{:>3} -", line_num)
                            }
                            _ => {
                                let p = format!("{:>3}  ", line_num);
                                line_num += 1;
                                p
                            }
                        };

                        let mut line_spans = vec![Span::styled(prefix_str, prefix_style)];
                        for (span_style, span_text) in spans {
                            let mut final_style = *span_style;
                            if let Some(bg) = bg_color {
                                final_style = final_style.bg(bg);
                            }
                            let clean_text = span_text.trim_end_matches(['\r', '\n']);
                            if !clean_text.is_empty() {
                                line_spans.push(Span::styled(clean_text.to_string(), final_style));
                            }
                        }
                        text.lines.push(Line::from(line_spans));
                    }
                }
            }

            if text.lines.len() <= 3 {
                text.lines.push(Line::from(""));
                text.lines.push(Line::from(vec![Span::styled(
                    " (no content changes to display)",
                    Style::default().fg(Palette::TEXT_MUTED),
                )]));
            }
        }

        let title_parts = vec![
            Span::styled(" ◈ ", Style::default().fg(Palette::ACCENT)),
            Span::styled(
                "DIFF",
                Style::default().fg(Palette::TEXT_BRIGHT).add_modifier(Modifier::BOLD),
            ),
        ];

        let p = Paragraph::new(text)
            .block(
                Block::default()
                    .title(Line::from(title_parts))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Palette::BORDER_FOCUS)),
            )
            .scroll(state.diff_scroll);
        f.render_widget(p, area);
    }
}
