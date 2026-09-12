// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use crate::domain::interfaces::{AstParserPort, SymbolInfo};

#[derive(Clone, Copy, Debug, Default)]
pub struct SemanticSymbolAdapter;

impl SemanticSymbolAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl AstParserPort for SemanticSymbolAdapter {
    fn extract_symbols(&self, content: &str, extension: &str) -> Vec<SymbolInfo> {
        let mut symbols = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        match extension {
            "rs" => extract_rust_symbols(&lines, &mut symbols),
            "py" => extract_python_symbols(&lines, &mut symbols),
            "ts" | "tsx" | "js" | "jsx" => extract_js_ts_symbols(&lines, &mut symbols),
            "go" => extract_go_symbols(&lines, &mut symbols),
            "c" | "cpp" | "h" | "hpp" => extract_c_cpp_symbols(&lines, &mut symbols),
            _ => extract_generic_symbols(&lines, &mut symbols),
        }

        symbols
    }
}

fn extract_rust_symbols(lines: &[&str], symbols: &mut Vec<SymbolInfo>) {
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
            i += 1;
            continue;
        }

        let mut kind_and_name = None;
        if trimmed.starts_with("fn ")
            || trimmed.starts_with("pub fn ")
            || trimmed.starts_with("pub(crate) fn ")
            || trimmed.starts_with("async fn ")
            || trimmed.starts_with("pub async fn ")
        {
            let sig = extract_signature(trimmed, "fn ");
            kind_and_name = Some(("fn", sig));
        } else if trimmed.starts_with("struct ") || trimmed.starts_with("pub struct ") {
            let sig = extract_signature(trimmed, "struct ");
            kind_and_name = Some(("struct", sig));
        } else if trimmed.starts_with("enum ") || trimmed.starts_with("pub enum ") {
            let sig = extract_signature(trimmed, "enum ");
            kind_and_name = Some(("enum", sig));
        } else if trimmed.starts_with("trait ") || trimmed.starts_with("pub trait ") {
            let sig = extract_signature(trimmed, "trait ");
            kind_and_name = Some(("trait", sig));
        } else if trimmed.starts_with("impl ") || trimmed.starts_with("impl<") {
            let sig = extract_signature(trimmed, "impl");
            kind_and_name = Some(("impl", sig));
        } else if trimmed.starts_with("mod ") || trimmed.starts_with("pub mod ") {
            let sig = extract_signature(trimmed, "mod ");
            kind_and_name = Some(("mod", sig));
        }

        if let Some((kind, name)) = kind_and_name {
            let start_line = i + 1;
            let end_line = find_block_end(lines, i);
            symbols.push(SymbolInfo { kind: kind.to_string(), name, start_line, end_line });
        }
        i += 1;
    }
}

fn extract_python_symbols(lines: &[&str], symbols: &mut Vec<SymbolInfo>) {
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("def ") || trimmed.starts_with("async def ") {
            let name = extract_signature(trimmed, "def ");
            let indent = get_indent(line);
            let end_line = find_python_block_end(lines, i, indent);
            symbols.push(SymbolInfo { kind: "def".to_string(), name, start_line: i + 1, end_line });
        } else if trimmed.starts_with("class ") {
            let name = extract_signature(trimmed, "class ");
            let indent = get_indent(line);
            let end_line = find_python_block_end(lines, i, indent);
            symbols.push(SymbolInfo {
                kind: "class".to_string(),
                name,
                start_line: i + 1,
                end_line,
            });
        }
    }
}

fn extract_js_ts_symbols(lines: &[&str], symbols: &mut Vec<SymbolInfo>) {
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("function ")
            || trimmed.starts_with("export function ")
            || trimmed.starts_with("export default function ")
            || trimmed.starts_with("async function ")
        {
            let name = extract_signature(trimmed, "function ");
            symbols.push(SymbolInfo {
                kind: "function".to_string(),
                name,
                start_line: i + 1,
                end_line: find_block_end(lines, i),
            });
        } else if trimmed.starts_with("class ") || trimmed.starts_with("export class ") {
            let name = extract_signature(trimmed, "class ");
            symbols.push(SymbolInfo {
                kind: "class".to_string(),
                name,
                start_line: i + 1,
                end_line: find_block_end(lines, i),
            });
        } else if trimmed.starts_with("interface ") || trimmed.starts_with("export interface ") {
            let name = extract_signature(trimmed, "interface ");
            symbols.push(SymbolInfo {
                kind: "interface".to_string(),
                name,
                start_line: i + 1,
                end_line: find_block_end(lines, i),
            });
        } else if (trimmed.starts_with("const ") || trimmed.starts_with("export const "))
            && trimmed.contains("=>")
        {
            let name = extract_signature(trimmed, "const ");
            symbols.push(SymbolInfo {
                kind: "const".to_string(),
                name,
                start_line: i + 1,
                end_line: find_block_end(lines, i),
            });
        }
    }
}

fn extract_go_symbols(lines: &[&str], symbols: &mut Vec<SymbolInfo>) {
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("func ") {
            let name = extract_signature(trimmed, "func ");
            symbols.push(SymbolInfo {
                kind: "func".to_string(),
                name,
                start_line: i + 1,
                end_line: find_block_end(lines, i),
            });
        } else if trimmed.starts_with("type ")
            && (trimmed.contains("struct") || trimmed.contains("interface"))
        {
            let name = extract_signature(trimmed, "type ");
            symbols.push(SymbolInfo {
                kind: "type".to_string(),
                name,
                start_line: i + 1,
                end_line: find_block_end(lines, i),
            });
        }
    }
}

fn extract_c_cpp_symbols(lines: &[&str], symbols: &mut Vec<SymbolInfo>) {
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("class ") {
            let name = extract_signature(trimmed, "class ");
            symbols.push(SymbolInfo {
                kind: "class".to_string(),
                name,
                start_line: i + 1,
                end_line: find_block_end(lines, i),
            });
        } else if trimmed.starts_with("struct ") {
            let name = extract_signature(trimmed, "struct ");
            symbols.push(SymbolInfo {
                kind: "struct".to_string(),
                name,
                start_line: i + 1,
                end_line: find_block_end(lines, i),
            });
        }
    }
}

fn extract_generic_symbols(lines: &[&str], symbols: &mut Vec<SymbolInfo>) {
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            symbols.push(SymbolInfo {
                kind: "section".to_string(),
                name: trimmed.to_string(),
                start_line: i + 1,
                end_line: lines.len(),
            });
        }
    }
}

fn extract_signature(line: &str, keyword: &str) -> String {
    let part = if let Some(idx) = line.find(keyword) { &line[idx + keyword.len()..] } else { line };
    let clean = part.split('{').next().unwrap_or(part).trim();
    let clean = clean.split(';').next().unwrap_or(clean).trim();
    let clean = clean.split(':').next().unwrap_or(clean).trim();
    if clean.len() > 50 { format!("{}...", &clean[..47]) } else { clean.to_string() }
}

fn find_block_end(lines: &[&str], start_idx: usize) -> usize {
    let mut brace_count: i32 = 0;
    let mut found_open = false;

    for (idx, line) in lines.iter().enumerate().skip(start_idx) {
        for ch in line.chars() {
            if ch == '{' {
                brace_count += 1;
                found_open = true;
            } else if ch == '}' {
                brace_count -= 1;
                if found_open && brace_count <= 0 {
                    return idx + 1;
                }
            }
        }
    }

    lines.len()
}

fn get_indent(line: &str) -> usize {
    line.chars().take_while(|c| c.is_whitespace()).count()
}

fn find_python_block_end(lines: &[&str], start_idx: usize, base_indent: usize) -> usize {
    for (idx, line) in lines.iter().enumerate().skip(start_idx + 1) {
        if line.trim().is_empty() || line.trim().starts_with('#') {
            continue;
        }
        if get_indent(line) <= base_indent {
            return idx;
        }
    }
    lines.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_symbols_extraction() {
        let code = r#"
pub struct AppState {
    pub count: usize,
}

impl AppState {
    pub fn increment(&mut self) {
        self.count += 1;
    }
}
"#;
        let adapter = SemanticSymbolAdapter::new();
        let symbols = adapter.extract_symbols(code, "rs");

        assert_eq!(symbols.len(), 3);
        assert_eq!(symbols[0].kind, "struct");
        assert_eq!(symbols[0].name, "AppState");

        assert_eq!(symbols[1].kind, "impl");
        assert_eq!(symbols[2].kind, "fn");
        assert!(symbols[2].name.contains("increment"));

        let enclosing = adapter.find_enclosing_symbol(&symbols, 8);
        assert!(enclosing.is_some());
        assert_eq!(enclosing.unwrap().kind, "fn");
    }

    #[test]
    fn test_python_symbols_extraction() {
        let code = r#"
class Worker:
    def process(self):
        print("Working")

def standalone():
    pass
"#;
        let adapter = SemanticSymbolAdapter::new();
        let symbols = adapter.extract_symbols(code, "py");

        assert_eq!(symbols.len(), 3);
        assert_eq!(symbols[0].kind, "class");
        assert_eq!(symbols[1].kind, "def");
        assert_eq!(symbols[2].kind, "def");
    }
}
