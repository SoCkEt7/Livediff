// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SymbolInfo {
    pub kind: String,
    pub name: String,
    pub start_line: usize,
    pub end_line: usize,
}

pub trait AstParserPort: Send + Sync {
    fn extract_symbols(&self, content: &str, extension: &str) -> Vec<SymbolInfo>;
    fn find_enclosing_symbol<'a>(
        &self,
        symbols: &'a [SymbolInfo],
        line_no: usize,
    ) -> Option<&'a SymbolInfo> {
        // Return the deepest (tightest range) enclosing symbol
        symbols
            .iter()
            .filter(|s| s.start_line <= line_no && line_no <= s.end_line)
            .min_by_key(|s| s.end_line.saturating_sub(s.start_line))
    }
}
