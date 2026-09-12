// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandAction {
    ToggleViewMode,
    JumpNextHunk,
    JumpPrevHunk,
    ToggleFold,
    SearchInDiff,
    OpenSymbols,
    FilterFiles,
    YankPatch,
    ExportPatch,
    CycleTheme,
    ToggleWrap,
    ToggleWhitespace,
    ManageIgnores,
    ClearTracked,
    ReloadIgnores,
    OpenEditor,
    HelpMenu,
    SettingsMenu,
    Quit,
}

#[derive(Debug, Clone)]
pub struct CommandItem {
    pub id: &'static str,
    pub title: &'static str,
    pub category: &'static str,
    pub shortcut: &'static str,
    pub action: CommandAction,
}

pub struct CommandPaletteEngine;

impl CommandPaletteEngine {
    pub fn all_commands() -> Vec<CommandItem> {
        vec![
            CommandItem {
                id: "view.toggle_split",
                title: "Toggle Unified / Split View Mode",
                category: "View",
                shortcut: "v / Tab",
                action: CommandAction::ToggleViewMode,
            },
            CommandItem {
                id: "diff.next_hunk",
                title: "Jump to Next Diff Hunk",
                category: "Diff",
                shortcut: "n / ]",
                action: CommandAction::JumpNextHunk,
            },
            CommandItem {
                id: "diff.prev_hunk",
                title: "Jump to Previous Diff Hunk",
                category: "Diff",
                shortcut: "p / [",
                action: CommandAction::JumpPrevHunk,
            },
            CommandItem {
                id: "diff.toggle_fold",
                title: "Toggle Context Folding (Compact / Full)",
                category: "Diff",
                shortcut: "f",
                action: CommandAction::ToggleFold,
            },
            CommandItem {
                id: "diff.search",
                title: "Search Text / Regex Inside Diff",
                category: "Search",
                shortcut: "Ctrl+F",
                action: CommandAction::SearchInDiff,
            },
            CommandItem {
                id: "code.symbols",
                title: "Inspect File AST Symbol Outline",
                category: "Search",
                shortcut: "o",
                action: CommandAction::OpenSymbols,
            },
            CommandItem {
                id: "files.filter",
                title: "Filter File List by Substring",
                category: "Files",
                shortcut: "/",
                action: CommandAction::FilterFiles,
            },
            CommandItem {
                id: "patch.yank",
                title: "Yank Unified Git Patch to Clipboard",
                category: "Actions",
                shortcut: "y",
                action: CommandAction::YankPatch,
            },
            CommandItem {
                id: "patch.export",
                title: "Export Snapshot to .patch File",
                category: "Actions",
                shortcut: "s",
                action: CommandAction::ExportPatch,
            },
            CommandItem {
                id: "theme.cycle",
                title: "Cycle Palette Color Theme",
                category: "Settings",
                shortcut: "t",
                action: CommandAction::CycleTheme,
            },
            CommandItem {
                id: "view.toggle_wrap",
                title: "Toggle Diff Soft Line Wrapping",
                category: "View",
                shortcut: "W",
                action: CommandAction::ToggleWrap,
            },
            CommandItem {
                id: "diff.toggle_ws",
                title: "Toggle Ignore Whitespace Changes",
                category: "Diff",
                shortcut: "w",
                action: CommandAction::ToggleWhitespace,
            },
            CommandItem {
                id: "files.edit",
                title: "Open Active File in Embedded Code Editor",
                category: "Files",
                shortcut: "e",
                action: CommandAction::OpenEditor,
            },
            CommandItem {
                id: "ignore.menu",
                title: "Manage Ignore Patterns & Suggestions",
                category: "Ignore",
                shortcut: "i",
                action: CommandAction::ManageIgnores,
            },
            CommandItem {
                id: "ignore.reload",
                title: "Reload VCS Ignore Rules (.gitignore)",
                category: "Ignore",
                shortcut: "r",
                action: CommandAction::ReloadIgnores,
            },
            CommandItem {
                id: "diff.clear",
                title: "Clear Tracked Modifications History",
                category: "Actions",
                shortcut: "c",
                action: CommandAction::ClearTracked,
            },
            CommandItem {
                id: "app.settings",
                title: "Open Global Settings Menu",
                category: "Settings",
                shortcut: "m -> 2",
                action: CommandAction::SettingsMenu,
            },
            CommandItem {
                id: "app.help",
                title: "Show Keyboard Shortcuts & Help Menu",
                category: "Help",
                shortcut: "?",
                action: CommandAction::HelpMenu,
            },
            CommandItem {
                id: "app.quit",
                title: "Quit Livediff Terminal Monitor",
                category: "App",
                shortcut: "q",
                action: CommandAction::Quit,
            },
        ]
    }

    pub fn filter_commands(query: &str) -> Vec<CommandItem> {
        let all = Self::all_commands();
        if query.trim().is_empty() {
            return all;
        }

        let q = query.to_lowercase();
        all.into_iter()
            .filter(|cmd| {
                cmd.title.to_lowercase().contains(&q)
                    || cmd.category.to_lowercase().contains(&q)
                    || cmd.shortcut.to_lowercase().contains(&q)
                    || cmd.id.to_lowercase().contains(&q)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_commands_contain_core_actions() {
        let cmds = CommandPaletteEngine::all_commands();
        assert!(cmds.len() >= 15);
    }

    #[test]
    fn test_filter_commands_matching() {
        let res = CommandPaletteEngine::filter_commands("hunk");
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].action, CommandAction::JumpNextHunk);
        assert_eq!(res[1].action, CommandAction::JumpPrevHunk);

        let res_theme = CommandPaletteEngine::filter_commands("theme");
        assert_eq!(res_theme.len(), 1);
        assert_eq!(res_theme[0].action, CommandAction::CycleTheme);
    }
}
