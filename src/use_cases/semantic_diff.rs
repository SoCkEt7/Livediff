// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use crate::domain::diff_engine::HunkRange;
use crate::domain::interfaces::{AstParserPort, SymbolInfo};

#[derive(Clone, Debug, Default)]
pub struct EnrichDiffWithSymbolsUseCase;

impl EnrichDiffWithSymbolsUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(
        &self,
        parser: &dyn AstParserPort,
        hunks: &mut [HunkRange],
        file_content: &str,
        extension: &str,
    ) -> Vec<SymbolInfo> {
        let symbols = parser.extract_symbols(file_content, extension);
        for hunk in hunks.iter_mut() {
            if let Some(sym) = parser.find_enclosing_symbol(&symbols, hunk.new_start) {
                hunk.symbol_context = Some(format!("{} {}", sym.kind, sym.name));
            }
        }
        symbols
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::semantic_symbol_adapter::SemanticSymbolAdapter;

    #[test]
    fn test_enrich_diff_with_symbols() {
        let code = r#"
fn hello_world() {
    println!("hello");
}
"#;
        let mut hunks = vec![HunkRange {
            old_start: 1,
            old_lines: 3,
            new_start: 2,
            new_lines: 4,
            start_line_idx: 0,
            end_line_idx: 3,
            symbol_context: None,
        }];

        let parser = SemanticSymbolAdapter::new();
        let use_case = EnrichDiffWithSymbolsUseCase::new();
        let symbols = use_case.execute(&parser, &mut hunks, code, "rs");

        assert!(!symbols.is_empty());
        assert!(hunks[0].symbol_context.is_some());
        assert!(hunks[0].symbol_context.as_ref().unwrap().contains("hello_world"));
    }
}
