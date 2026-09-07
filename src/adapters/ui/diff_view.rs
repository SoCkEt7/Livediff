// Copyright (c) 2026 Nyxia. All rights reserved.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

use super::{Component, Palette, get_file_type};
use crate::app::{DiffViewMode, MonitorDomain, TerminalUiState};

pub struct DiffComponent;

impl Component for DiffComponent {
    type State = TerminalUiState;
    type Context = MonitorDomain;

    fn draw(&self, f: &mut Frame<'_>, area: Rect, state: &mut Self::State, ctx: &Self::Context) {
        let visible_mods = state.get_visible_modifications(ctx);

        if visible_mods.is_empty() {
            let empty_text = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled(" ◈ ", Style::default().fg(Palette::PRIMARY)),
                    Span::styled(
                        "No file changes tracked",
                        Style::default().fg(Palette::TEXT_MUTED).add_modifier(Modifier::ITALIC),
                    ),
                ]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    "  Waiting for filesystem modifications... (Edit or generate files to inspect live diffs)",
                    Style::default().fg(Palette::TEXT_MUTED),
                )]),
            ];
            let p = Paragraph::new(empty_text).block(
                Block::default()
                    .title(Span::styled(
                        " ◈ DIFF PREVIEW ",
                        Style::default().fg(Palette::TEXT_BRIGHT).add_modifier(Modifier::BOLD),
                    ))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Palette::BORDER_DARK)),
            );
            f.render_widget(p, area);
            return;
        }

        let Some(m) = visible_mods.get(state.selected_index) else {
            return;
        };

        state.update_highlighting(ctx);

        let ft = get_file_type(&m.path);
        let header_bg = if m.is_binary { Color::Rgb(220, 0, 220) } else { ft.color };
        let header_label = if m.is_binary { " BINARY " } else { ft.label };

        let mode_badge = match state.view_mode {
            DiffViewMode::Unified => " UNIFIED [v] ",
            DiffViewMode::Split => " SPLIT [v] ",
        };

        let ws_badge = if state.ignore_whitespace { " [IGN-WS] " } else { "" };
        let wrap_badge = if state.wrap_lines { " [WRAP] " } else { "" };

        let size_str = if m.size < 1024 {
            format!("{}B", m.size)
        } else {
            format!("{:.1}K", m.size as f64 / 1024.0)
        };

        let primary_color = state.current_theme.primary();
        let accent_color = state.current_theme.accent();
        let border_focus = state.current_theme.border_focus();

        let title_parts = vec![
            Span::styled(" ◈ ", Style::default().fg(accent_color)),
            Span::styled(
                "DIFF ",
                Style::default().fg(Palette::TEXT_BRIGHT).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                mode_badge,
                Style::default().fg(primary_color).add_modifier(Modifier::BOLD),
            ),
            if !ws_badge.is_empty() {
                Span::styled(
                    ws_badge,
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                )
            } else {
                Span::raw("")
            },
            if !wrap_badge.is_empty() {
                Span::styled(
                    wrap_badge,
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                )
            } else {
                Span::raw("")
            },
            Span::styled(
                format!(" +{} -{} ", m.added, m.deleted),
                Style::default().fg(Color::Rgb(46, 204, 113)),
            ),
        ];

        let block = Block::default()
            .title(Line::from(title_parts))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_focus));

        let inner_area = block.inner(area);
        f.render_widget(block, area);

        if inner_area.height < 3 || inner_area.width < 10 {
            return;
        }

        // Split top header info and diff area
        let vert_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(1)])
            .split(inner_area);

        // Header bar
        let header_line = Line::from(vec![
            Span::styled(
                format!(" {} ", header_label),
                Style::default().fg(Color::Rgb(10, 10, 15)).bg(header_bg),
            ),
            Span::raw(" "),
            Span::styled(&m.path, Style::default().add_modifier(Modifier::BOLD).fg(primary_color)),
            Span::raw("  ·  "),
            Span::styled(size_str, Style::default().fg(Palette::TEXT_MUTED)),
            Span::raw("  ·  "),
            Span::styled(
                chrono::DateTime::<chrono::Local>::from(m.timestamp).format("%H:%M:%S").to_string(),
                Style::default().fg(Palette::TEXT_MUTED),
            ),
        ]);
        f.render_widget(Paragraph::new(header_line), vert_chunks[0]);

        if m.is_binary {
            let bin_text = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  (Binary file content hidden)",
                    Style::default().fg(Palette::TEXT_MUTED),
                )),
            ];
            f.render_widget(Paragraph::new(bin_text), vert_chunks[1]);
            return;
        }

        match state.view_mode {
            DiffViewMode::Unified => {
                let mut text = Text::default();
                for (diff_line, spans) in &state.highlighted_diff {
                    let (prefix_char, prefix_style, bg_color) = match diff_line.change_type {
                        crate::domain::diff_engine::LineChangeType::Insert => (
                            "+",
                            Style::default().fg(Color::Rgb(46, 204, 113)),
                            Some(Color::Rgb(22, 50, 22)),
                        ),
                        crate::domain::diff_engine::LineChangeType::Delete => (
                            "-",
                            Style::default().fg(Color::Rgb(231, 76, 60)),
                            Some(Color::Rgb(50, 22, 22)),
                        ),
                        crate::domain::diff_engine::LineChangeType::Header => (
                            "@",
                            Style::default().fg(Palette::ACCENT).add_modifier(Modifier::BOLD),
                            None,
                        ),
                        crate::domain::diff_engine::LineChangeType::Context => {
                            (" ", Style::default().fg(Palette::TEXT_MUTED), None)
                        }
                    };

                    let old_str = diff_line
                        .old_lineno
                        .map(|n| format!("{:>4}", n))
                        .unwrap_or_else(|| "    ".to_string());
                    let new_str = diff_line
                        .new_lineno
                        .map(|n| format!("{:>4}", n))
                        .unwrap_or_else(|| "    ".to_string());

                    let gutter = format!("{} {} {} ", old_str, new_str, prefix_char);
                    let mut line_spans = vec![Span::styled(gutter, prefix_style)];

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

                if text.lines.is_empty() {
                    text.lines.push(Line::from(Span::styled(
                        "  (no content changes to display)",
                        Style::default().fg(Palette::TEXT_MUTED),
                    )));
                }

                let mut p = Paragraph::new(text).scroll(state.diff_scroll);
                if state.wrap_lines {
                    p = p.wrap(ratatui::widgets::Wrap { trim: false });
                }
                f.render_widget(p, vert_chunks[1]);
            }
            DiffViewMode::Split => {
                let col_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(vert_chunks[1]);

                let mut left_text = Text::default();
                let mut right_text = Text::default();

                for row in &state.highlighted_split_diff {
                    // Left Column (Old)
                    let left_lineno_str = row
                        .old_lineno
                        .map(|n| format!("{:>4} - ", n))
                        .unwrap_or_else(|| "       ".to_string());

                    let left_style = match row.old_change {
                        Some(crate::domain::diff_engine::LineChangeType::Delete) => {
                            Style::default().fg(Color::Rgb(231, 76, 60))
                        }
                        _ => Style::default().fg(Palette::TEXT_MUTED),
                    };

                    let left_bg = match row.old_change {
                        Some(crate::domain::diff_engine::LineChangeType::Delete) => {
                            Some(Color::Rgb(50, 22, 22))
                        }
                        _ => None,
                    };

                    let mut left_spans = vec![Span::styled(left_lineno_str, left_style)];
                    if let Some(ref spans) = row.old_spans {
                        for (style, text) in spans {
                            let mut final_style = *style;
                            if let Some(bg) = left_bg {
                                final_style = final_style.bg(bg);
                            }
                            let clean = text.trim_end_matches(['\r', '\n']);
                            if !clean.is_empty() {
                                left_spans.push(Span::styled(clean.to_string(), final_style));
                            }
                        }
                    }
                    left_text.lines.push(Line::from(left_spans));

                    // Right Column (New)
                    let right_lineno_str = row
                        .new_lineno
                        .map(|n| format!("{:>4} + ", n))
                        .unwrap_or_else(|| "       ".to_string());

                    let right_style = match row.new_change {
                        Some(crate::domain::diff_engine::LineChangeType::Insert) => {
                            Style::default().fg(Color::Rgb(46, 204, 113))
                        }
                        _ => Style::default().fg(Palette::TEXT_MUTED),
                    };

                    let right_bg = match row.new_change {
                        Some(crate::domain::diff_engine::LineChangeType::Insert) => {
                            Some(Color::Rgb(22, 50, 22))
                        }
                        _ => None,
                    };

                    let mut right_spans = vec![Span::styled(right_lineno_str, right_style)];
                    if let Some(ref spans) = row.new_spans {
                        for (style, text) in spans {
                            let mut final_style = *style;
                            if let Some(bg) = right_bg {
                                final_style = final_style.bg(bg);
                            }
                            let clean = text.trim_end_matches(['\r', '\n']);
                            if !clean.is_empty() {
                                right_spans.push(Span::styled(clean.to_string(), final_style));
                            }
                        }
                    }
                    right_text.lines.push(Line::from(right_spans));
                }

                let mut left_p = Paragraph::new(left_text)
                    .block(
                        Block::default()
                            .title(Span::styled(
                                " OLD ",
                                Style::default()
                                    .fg(Color::Rgb(231, 76, 60))
                                    .add_modifier(Modifier::BOLD),
                            ))
                            .borders(Borders::RIGHT)
                            .border_style(Style::default().fg(state.current_theme.border_dark())),
                    )
                    .scroll(state.diff_scroll);
                if state.wrap_lines {
                    left_p = left_p.wrap(ratatui::widgets::Wrap { trim: false });
                }
                f.render_widget(left_p, col_chunks[0]);

                let mut right_p = Paragraph::new(right_text)
                    .block(Block::default().title(Span::styled(
                        " NEW ",
                        Style::default().fg(Color::Rgb(46, 204, 113)).add_modifier(Modifier::BOLD),
                    )))
                    .scroll(state.diff_scroll);
                if state.wrap_lines {
                    right_p = right_p.wrap(ratatui::widgets::Wrap { trim: false });
                }
                f.render_widget(right_p, col_chunks[1]);
            }
        }
    }
}
