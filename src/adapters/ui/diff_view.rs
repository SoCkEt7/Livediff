// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

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

        let mut text = Text::default();

        if visible_mods.is_empty() {
            text.lines.push(Line::from(vec![Span::styled(
                " No active changes detected. Monitoring directory... ",
                Style::default().fg(Palette::TEXT_MUTED),
            )]));
        } else if let Some(m) = visible_mods.get(state.selected_index) {
            let ft = get_file_type(&m.path);
            let header_style = if m.is_binary {
                Style::default().fg(Color::Rgb(10, 10, 15)).bg(Color::Rgb(220, 0, 220))
            } else {
                Style::default().fg(Color::Rgb(10, 10, 15)).bg(ft.color)
            };
            let header_label = if m.is_binary { " BINARY " } else { ft.label };

            text.lines.push(Line::from(vec![
                Span::styled(format!(" {} ", header_label), header_style),
                Span::raw(" "),
                Span::styled(
                    &m.path,
                    Style::default().add_modifier(Modifier::BOLD).fg(Palette::PRIMARY),
                ),
            ]));

            let size_str = if m.size < 1024 {
                format!("{} B", m.size)
            } else {
                format!("{:.2} KB", m.size as f64 / 1024.0)
            };
            text.lines.push(Line::from(vec![
                Span::styled(
                    format!(" SIZE: {} ", size_str),
                    Style::default().fg(Palette::TEXT_MUTED),
                ),
                Span::styled(
                    format!(
                        " MODIFIED: {} ",
                        chrono::DateTime::<chrono::Local>::from(m.timestamp).format("%H:%M:%S")
                    ),
                    Style::default().fg(Palette::TEXT_MUTED),
                ),
            ]));
            text.lines.push(Line::from(""));

            state.update_highlighting(ctx);

            let mut line_num = 1;
            for (change_type, spans) in &state.highlighted_diff {
                let bg_color = match change_type {
                    crate::domain::diff_engine::LineChangeType::Insert => {
                        Some(Color::Rgb(25, 60, 25))
                    }
                    crate::domain::diff_engine::LineChangeType::Delete => {
                        Some(Color::Rgb(60, 25, 25))
                    }
                    _ => None,
                };

                let mut prefix_style = match change_type {
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
                if let Some(bg) = bg_color {
                    prefix_style = prefix_style.bg(bg);
                }

                match change_type {
                    crate::domain::diff_engine::LineChangeType::Header => {
                        line_num = 1;
                        if let Some((_, text_val)) = spans.first() {
                            text.lines.push(Line::from(Span::styled(text_val, prefix_style)));
                        }
                    }
                    _ => {
                        let prefix_str = match change_type {
                            crate::domain::diff_engine::LineChangeType::Insert => {
                                let p = format!("{:>4} + ", line_num);
                                line_num += 1;
                                p
                            }
                            crate::domain::diff_engine::LineChangeType::Delete => {
                                format!("{:>4} - ", line_num)
                            }
                            _ => {
                                let p = format!("{:>4}   ", line_num);
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
                    " (No content changes to display) ",
                    Style::default().fg(Palette::TEXT_MUTED),
                )]));
            }
        }

        let border_color =
            if state.ignore_input_visible || state.ignore_menu_visible || state.help_visible {
                Palette::BORDER_DARK
            } else {
                Palette::BORDER_FOCUS
            };

        let title_line = Line::from(vec![Span::styled(
            " ◈ DIFF PREVIEW ",
            Style::default().fg(Palette::ACCENT).add_modifier(Modifier::BOLD),
        )]);

        let p = Paragraph::new(text)
            .block(
                Block::default()
                    .title(title_line)
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(Style::default().fg(border_color)),
            )
            .scroll(state.diff_scroll);

        f.render_widget(p, area);
    }
}
