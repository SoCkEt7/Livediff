use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use std::path::Path;

use crate::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Title
            Constraint::Length(3), // Stats
            Constraint::Min(10),   // Main content
            Constraint::Length(5), // Logs
            Constraint::Length(1), // Footer
        ])
        .split(f.area());

    draw_title(f, app, chunks[0]);
    draw_stats(f, app, chunks[1]);
    
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);
        
    draw_file_list(f, app, main_chunks[0]);
    draw_diff_view(f, app, main_chunks[1]);
    
    draw_logs(f, app, chunks[3]);
    draw_footer(f, chunks[4]);

    if app.ignore_input_visible {
        draw_ignore_input(f, app);
    } else if app.ignore_menu_visible {
        draw_ignore_menu(f, app);
    } else if app.help_visible {
        draw_help(f);
    }
}

struct FileType {
    label: &'static str,
    icon: &'static str,
    color: Color,
}

fn get_file_type(path_str: &str) -> FileType {
    let ext = Path::new(path_str)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    match ext {
        "js" | "jsx" => FileType { label: "JS", icon: "", color: Color::Yellow },
        "ts" | "tsx" => FileType { label: "TS", icon: "", color: Color::Blue },
        "php" => FileType { label: "PHP", icon: "", color: Color::Magenta },
        "twig" => FileType { label: "TWIG", icon: "", color: Color::Green },
        "css" | "scss" => FileType { label: "CSS", icon: "", color: Color::Blue },
        "html" => FileType { label: "HTML", icon: "", color: Color::Red },
        "json" | "yaml" | "yml" => FileType { label: "CONF", icon: "", color: Color::Red },
        "md" => FileType { label: "MD", icon: "", color: Color::White },
        "rs" => FileType { label: "RUST", icon: "", color: Color::Red },
        "toml" => FileType { label: "TOML", icon: "", color: Color::Green },
        _ => FileType { label: "FILE", icon: "", color: Color::White },
    }
}

fn draw_title(f: &mut Frame, app: &App, area: Rect) {
    let anim_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let spinner = anim_chars[app.anim_frame % anim_chars.len()];
    
    let (status_text, status_color) = if app.modifications.is_empty() {
        (format!(" {} STANDBY ", spinner), Color::Yellow)
    } else {
        (format!(" {} LIVE ", spinner), Color::Green)
    };

    let cwd = std::env::current_dir()
        .map(|p| p.file_name().unwrap_or_default().to_string_lossy().into_owned())
        .unwrap_or_else(|_| "Unknown".to_string());

    let left_content = Line::from(vec![
        Span::styled(" ◈ CODELENS ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Black).bg(Color::Cyan)),
        Span::styled("", Style::default().fg(Color::Cyan).bg(Color::Rgb(40, 40, 60))),
        Span::styled(format!("  {} ", cwd), Style::default().fg(Color::White).bg(Color::Rgb(40, 40, 60))),
        Span::styled("", Style::default().fg(Color::Rgb(40, 40, 60)).bg(status_color)),
        Span::styled(status_text, Style::default().fg(Color::Black).bg(status_color).add_modifier(Modifier::BOLD)),
        Span::styled("", Style::default().fg(status_color)),
    ]);

    let right_content = Line::from(vec![
        Span::styled("", Style::default().fg(Color::Rgb(40, 40, 60))),
        Span::styled(format!(" {} Ignored ", app.ignore_list.len()), Style::default().fg(Color::White).bg(Color::Rgb(40, 40, 60))),
        Span::styled("", Style::default().fg(Color::DarkGray).bg(Color::Rgb(40, 40, 60))),
        Span::styled(" ? Help ", Style::default().fg(Color::Black).bg(Color::DarkGray).add_modifier(Modifier::BOLD)),
    ]);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    f.render_widget(Paragraph::new(left_content).alignment(ratatui::layout::Alignment::Left), chunks[0]);
    f.render_widget(Paragraph::new(right_content).alignment(ratatui::layout::Alignment::Right), chunks[1]);
}

fn draw_stats(f: &mut Frame, app: &App, area: Rect) {
    let stats = app.stats();
    
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Min(10),
        ])
        .split(area);

    let items = vec![
        ("FILES", format!("{}", stats.modified), Color::Cyan),
        ("ADDED", format!("+{}", stats.lines_added), Color::Green),
        ("DELETED", format!("-{}", stats.lines_deleted), Color::Red),
    ];

    for (i, (label, value, color)) in items.iter().enumerate() {
        let p = Paragraph::new(Line::from(vec![
            Span::styled(format!(" {} ", label), Style::default().fg(Color::DarkGray)),
            Span::styled(value, Style::default().fg(*color).add_modifier(Modifier::BOLD)),
        ]))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(Color::Rgb(40, 40, 60))));
        f.render_widget(p, chunks[i]);
    }
}

fn draw_file_list(f: &mut Frame, app: &mut App, area: Rect) {
    let now = std::time::SystemTime::now();
    let items: Vec<ListItem> = app.modifications.iter().filter(|m| !app.is_ignored(&m.path)).map(|m| {
        let elapsed = now.duration_since(m.timestamp).unwrap_or(std::time::Duration::from_secs(0));
        let time_str = if elapsed.as_secs() < 60 {
            format!("{}s ago", elapsed.as_secs())
        } else if elapsed.as_secs() < 3600 {
            format!("{}m ago", elapsed.as_secs() / 60)
        } else {
            format!("{}h ago", elapsed.as_secs() / 3600)
        };
        
        let ft = get_file_type(&m.path);
        
        let mut line_spans = vec![
            Span::styled(format!(" {} ", ft.icon), Style::default().fg(ft.color)),
        ];

        if m.is_binary {
            line_spans.push(Span::styled("BIN    ", Style::default().fg(Color::Magenta)));
        } else {
            line_spans.push(Span::styled(format!("{:<6} ", ft.label), Style::default().fg(Color::DarkGray)));
        }

        line_spans.push(Span::styled(format!("{:<8} ", time_str), Style::default().fg(Color::DarkGray)));
        line_spans.push(Span::styled(format!("+{} ", m.added), Style::default().fg(Color::Green)));
        line_spans.push(Span::styled(format!("-{} ", m.deleted), Style::default().fg(Color::Red)));
        line_spans.push(Span::raw(&m.path));

        let content = Line::from(line_spans);
        ListItem::new(content)
    }).collect();

    let list = List::new(items)
        .block(Block::default()
            .title(Span::styled(" RECENT CHANGES ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)))
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(Color::Rgb(60, 60, 100))))
        .highlight_style(Style::default()
            .bg(Color::Rgb(30, 30, 60))
            .add_modifier(Modifier::BOLD))
        .highlight_symbol(" ❱ ");

    let mut state = ListState::default();
    state.select(Some(app.selected_index));

    f.render_stateful_widget(list, area, &mut state);
}

fn draw_diff_view(f: &mut Frame, app: &App, area: Rect) {
    let visible_mods: Vec<_> = app.modifications.iter().filter(|m| !app.is_ignored(&m.path)).collect();
    
    let mut text = Text::default();
    
    if visible_mods.is_empty() {
        text.lines.push(Line::from(vec![Span::styled(" No active changes detected. ", Style::default().fg(Color::DarkGray))]));
    } else if let Some(m) = visible_mods.get(app.selected_index) {
        let ft = get_file_type(&m.path);
        let header_style = if m.is_binary { Style::default().fg(Color::Black).bg(Color::Magenta) } else { Style::default().fg(Color::Black).bg(ft.color) };
        let header_label = if m.is_binary { " BINARY " } else { ft.label };
        
        text.lines.push(Line::from(vec![
            Span::styled(format!(" {} ", header_label), header_style),
            Span::raw(" "),
            Span::styled(&m.path, Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan))
        ]));
        
        let size_str = if m.size < 1024 { format!("{} B", m.size) } else { format!("{:.2} KB", m.size as f64 / 1024.0) };
        text.lines.push(Line::from(vec![
            Span::styled(format!(" SIZE: {} ", size_str), Style::default().fg(Color::DarkGray)),
            Span::styled(format!(" MODIFIED: {} ", chrono::DateTime::<chrono::Local>::from(m.timestamp).format("%H:%M:%S")), Style::default().fg(Color::DarkGray)),
        ]));
        text.lines.push(Line::from(""));
        
        let mut line_num = 1;
        for line in m.diff.lines() {
            let prefix = if line.starts_with('+') {
                format!("{:>4} + ", line_num)
            } else if line.starts_with('-') {
                format!("{:>4} - ", line_num)
            } else if line.starts_with("@@") {
                line_num = 1; // reset or handle properly, but for simplicity:
                format!("     @@ ")
            } else {
                format!("{:>4}   ", line_num)
            };

            if !line.starts_with('-') && !line.starts_with("@@") {
                line_num += 1;
            }

            if line.starts_with('+') {
                text.lines.push(Line::from(vec![
                    Span::styled(prefix, Style::default().fg(Color::Green)),
                    Span::styled(&line[1..], Style::default().fg(Color::Green))
                ]));
            } else if line.starts_with('-') {
                text.lines.push(Line::from(vec![
                    Span::styled(prefix, Style::default().fg(Color::Red)),
                    Span::styled(&line[1..], Style::default().fg(Color::Red))
                ]));
            } else if line.starts_with("@@") {
                text.lines.push(Line::from(Span::styled(line, Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))));
            } else {
                let content = if line.starts_with(' ') { &line[1..] } else { line };
                text.lines.push(Line::from(vec![
                    Span::styled(prefix, Style::default().fg(Color::DarkGray)),
                    Span::styled(content, Style::default().fg(Color::Gray))
                ]));
            }
        }

        if text.lines.len() <= 3 {
            text.lines.push(Line::from(""));
            text.lines.push(Line::from(vec![Span::styled(" (No content changes to display) ", Style::default().fg(Color::DarkGray))]));
        }
    }

    let p = Paragraph::new(text)
        .block(Block::default()
            .title(Span::styled(" DIFF PREVIEW ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Magenta)))
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(Color::Rgb(60, 60, 100))))
        .scroll(app.diff_scroll);
    
    f.render_widget(p, area);
}

fn draw_logs(f: &mut Frame, app: &App, area: Rect) {
    let text: Vec<Line> = app.logs.iter().map(|l| Line::from(l.as_str())).collect();
    let p = Paragraph::new(text)
        .block(Block::default()
            .title(Span::styled(" ACTIVITY LOG ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)))
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(Color::Rgb(60, 60, 100))))
        .wrap(Wrap { trim: false });
    f.render_widget(p, area);
}

fn draw_footer(f: &mut Frame, area: Rect) {
    let p = Paragraph::new(Line::from(vec![
        Span::styled(" ◈ ", Style::default().fg(Color::Cyan)),
        Span::styled("CodeLens ", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled(format!("v{} ", env!("CARGO_PKG_VERSION")), Style::default().fg(Color::DarkGray)),
        Span::raw("│ "),
        Span::styled("↑↓", Style::default().fg(Color::Yellow)),
        Span::raw(" Sel │ "),
        Span::styled("PgUp/Dn/←→", Style::default().fg(Color::Yellow)),
        Span::raw(" Scrl │ "),
        Span::styled("I", Style::default().fg(Color::Yellow)),
        Span::raw(" Ign │ "),
        Span::styled("C", Style::default().fg(Color::Yellow)),
        Span::raw(" Clr │ "),
        Span::styled("?", Style::default().fg(Color::Yellow)),
        Span::raw(" Help │ "),
        Span::styled("Q", Style::default().fg(Color::Yellow)),
        Span::raw(" Quit │ "),
        Span::raw("© 2026 "),
        Span::styled("Antonin Nivoche", Style::default().fg(Color::Cyan)),
        Span::raw(" | "),
        Span::styled("https://github.com/SoCkEt7", Style::default().fg(Color::Green).add_modifier(Modifier::UNDERLINED)),
        Span::raw(" | "),
        Span::styled("https://olive.click", Style::default().fg(Color::Blue).add_modifier(Modifier::UNDERLINED)),
    ]))
    .alignment(ratatui::layout::Alignment::Left)
    .style(Style::default().fg(Color::DarkGray));
    f.render_widget(p, area);
}

fn draw_help(f: &mut Frame) {
    let area = centered_rect(60, 65, f.area());
    let help_content = vec![
        Line::from(vec![Span::styled(" ⌨  SHORTCUTS ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan))]),
        Line::from(""),
        Line::from(vec![Span::styled("  ↑ / k      ", Style::default().fg(Color::Yellow)), Span::raw("- Select previous file")]),
        Line::from(vec![Span::styled("  ↓ / j      ", Style::default().fg(Color::Yellow)), Span::raw("- Select next file")]),
        Line::from(vec![Span::styled("  PgUp / PgDn", Style::default().fg(Color::Yellow)), Span::raw("- Scroll diff vertically")]),
        Line::from(vec![Span::styled("  ← / → / h/l", Style::default().fg(Color::Yellow)), Span::raw("- Scroll diff horizontally")]),
        Line::from(vec![Span::styled("  i          ", Style::default().fg(Color::Yellow)), Span::raw("- Ignore menu (file/dir/ext)")]),
        Line::from(vec![Span::styled("  c          ", Style::default().fg(Color::Yellow)), Span::raw("- Clear history")]),
        Line::from(vec![Span::styled("  ?          ", Style::default().fg(Color::Yellow)), Span::raw("- Toggle this menu")]),
        Line::from(vec![Span::styled("  q / Ctrl+C ", Style::default().fg(Color::Yellow)), Span::raw("- Quit")]),
        Line::from(""),
        Line::from(vec![Span::styled(" ◈  FEATURES ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Magenta))]),
        Line::from(""),
        Line::from("  • Real-time async monitoring"),
        Line::from("  • Smart .gitignore filtering"),
        Line::from("  • 1MB file size limit safety"),
        Line::from("  • Inline diff visualization"),
    ];

    let p = Paragraph::new(help_content)
        .block(Block::default()
            .title(Span::styled(" HELP ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Green)))
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Double)
            .border_style(Style::default().fg(Color::Green)))
        .style(Style::default().fg(Color::White).bg(Color::Rgb(10, 10, 30)));
    
    f.render_widget(ratatui::widgets::Clear, area); // clear background
    f.render_widget(p, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn draw_ignore_menu(f: &mut Frame, app: &App) {
    let area = centered_rect(40, 40, f.area());
    
    let mut items = vec![];
    for (i, opt) in app.ignore_menu_options.iter().enumerate() {
        let prefix = if i == app.ignore_menu_selected { " ❱ " } else { "   " };
        let style = if i == app.ignore_menu_selected {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        items.push(ListItem::new(Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(opt, style),
        ])));
    }

    let list = List::new(items)
        .block(Block::default()
            .title(Span::styled(" IGNORE OPTIONS ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)))
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Double)
            .border_style(Style::default().fg(Color::Yellow)))
        .style(Style::default().fg(Color::White).bg(Color::Rgb(10, 10, 30)));
    
    f.render_widget(ratatui::widgets::Clear, area);
    f.render_widget(list, area);
}

fn draw_ignore_input(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 20, f.area());
    
    let content = vec![
        Line::from(vec![Span::styled(" Type a glob pattern (e.g. *.log, tests/): ", Style::default().fg(Color::DarkGray))]),
        Line::from(""),
        Line::from(vec![
            Span::styled(" ❱ ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(&app.ignore_input_text, Style::default().fg(Color::White)),
            Span::styled("█", Style::default().fg(Color::Gray).add_modifier(Modifier::RAPID_BLINK)),
        ]),
    ];

    let p = Paragraph::new(content)
        .block(Block::default()
            .title(Span::styled(" CUSTOM IGNORE PATTERN ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Magenta)))
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Double)
            .border_style(Style::default().fg(Color::Magenta)))
        .style(Style::default().bg(Color::Rgb(10, 10, 30)));

    f.render_widget(ratatui::widgets::Clear, area);
    f.render_widget(p, area);
}
