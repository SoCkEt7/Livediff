// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
};
use std::path::Path;

pub mod diff_view;
pub mod file_list;
pub mod footer;
pub mod header;
pub mod logs;
pub mod popups;
pub mod stats;

use crate::app::{MonitorDomain, TerminalUiState};

pub trait Component {
    type State;
    type Context;

    fn draw(&self, f: &mut Frame<'_>, area: Rect, state: &mut Self::State, ctx: &Self::Context);
}

// Global UI Palette - Sleek Dark Theme (Btop-spec)
pub struct Palette;
impl Palette {
    pub const BG_DARK: Color = Color::Rgb(10, 10, 15);
    pub const BG_PANEL: Color = Color::Rgb(22, 22, 29);
    pub const BORDER_DARK: Color = Color::Rgb(45, 45, 58);
    pub const BORDER_FOCUS: Color = Color::Rgb(80, 80, 110);

    pub const PRIMARY: Color = Color::Rgb(0, 220, 220); // Cyan
    pub const ACCENT: Color = Color::Rgb(220, 0, 220); // Magenta
    pub const TEXT_MUTED: Color = Color::Rgb(120, 120, 140);
    pub const TEXT_BRIGHT: Color = Color::Rgb(240, 240, 250);

    // Gradients
    pub const GRADIENT_START: (u8, u8, u8) = (46, 204, 113); // Green
    pub const GRADIENT_MID: (u8, u8, u8) = (241, 196, 15); // Yellow
    pub const GRADIENT_END: (u8, u8, u8) = (231, 76, 60); // Red
}

pub struct FileType {
    pub label: &'static str,
    pub icon: &'static str,
    pub color: Color,
}

pub fn get_file_type(path_str: &str) -> FileType {
    let path = Path::new(path_str);
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();

    match ext.as_str() {
        "rs" => FileType { label: "RUST", icon: "", color: Color::Rgb(230, 80, 80) },
        "toml" => FileType { label: "TOML", icon: "", color: Color::Rgb(80, 200, 120) },
        "json" | "yaml" | "yml" => {
            FileType { label: "CONF", icon: "", color: Color::Rgb(230, 180, 80) }
        }
        "md" => FileType { label: "MD", icon: "", color: Color::Rgb(240, 240, 240) },
        "html" => FileType { label: "HTML", icon: "", color: Color::Rgb(220, 100, 80) },
        "css" | "scss" => FileType { label: "CSS", icon: "", color: Color::Rgb(80, 150, 230) },
        "js" | "ts" | "jsx" | "tsx" => {
            FileType { label: "JS", icon: "", color: Color::Rgb(230, 210, 80) }
        }
        _ => FileType { label: "FILE", icon: "", color: Color::Rgb(180, 180, 180) },
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

pub fn interpolate_rgb(val: f32, start: (u8, u8, u8), end: (u8, u8, u8)) -> Color {
    let clamped = val.clamp(0.0, 1.0);
    let r = (start.0 as f32 + clamped * (end.0 as f32 - start.0 as f32)) as u8;
    let g = (start.1 as f32 + clamped * (end.1 as f32 - start.1 as f32)) as u8;
    let b = (start.2 as f32 + clamped * (end.2 as f32 - start.2 as f32)) as u8;
    Color::Rgb(r, g, b)
}

pub fn get_value_color(val: f32) -> Color {
    if val < 0.5 {
        interpolate_rgb(val * 2.0, Palette::GRADIENT_START, Palette::GRADIENT_MID)
    } else {
        interpolate_rgb((val - 0.5) * 2.0, Palette::GRADIENT_MID, Palette::GRADIENT_END)
    }
}

pub fn draw(f: &mut Frame<'_>, ui_state: &mut TerminalUiState, domain: &MonitorDomain) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Title/Header
            Constraint::Length(3), // Stats
            Constraint::Min(10),   // Main content (FileList + DiffView)
            Constraint::Length(5), // Logs
            Constraint::Length(1), // Footer
        ])
        .split(f.area());

    ui_state.stats_rect = chunks[1];
    ui_state.footer_rect = chunks[4];

    // 1. Draw Header
    header::HeaderComponent.draw(f, chunks[0], ui_state, domain);

    // 2. Draw Stats
    stats::StatsComponent.draw(f, chunks[1], ui_state, domain);

    // 3. Draw Main area (File List + Diff View)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(chunks[2]);

    ui_state.file_list_rect = main_chunks[0];
    ui_state.diff_view_rect = main_chunks[1];

    file_list::FileListComponent.draw(f, main_chunks[0], ui_state, domain);
    diff_view::DiffComponent.draw(f, main_chunks[1], ui_state, domain);

    // 4. Draw Logs
    logs::LogsComponent.draw(f, chunks[3], ui_state, domain);

    // 5. Draw Footer
    footer::FooterComponent.draw(f, chunks[4], ui_state, domain);

    // 6. Draw Popups if active
    if ui_state.ignore_input_visible {
        popups::PopupComponent::IgnoreInput.draw(f, f.area(), ui_state, domain);
    } else if ui_state.ignore_menu_visible {
        popups::PopupComponent::IgnoreMenu.draw(f, f.area(), ui_state, domain);
    } else if ui_state.help_visible {
        popups::PopupComponent::Help.draw(f, f.area(), ui_state, domain);
    }
}
