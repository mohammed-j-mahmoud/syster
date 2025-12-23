use super::LspServer;
use std::collections::HashMap;
use std::path::Path;
use syster::semantic::symbol_table::Symbol;
use tower_lsp::lsp_types::{DocumentSymbol, SymbolKind};

impl LspServer {
    /// Get all symbols in a document for the outline view
    pub fn get_document_symbols(&self, file_path: &Path) -> Vec<DocumentSymbol> {
        let mut flat_symbols = Vec::new();

        // Collect all symbols from this file
        for (_, symbol) in self.workspace.symbol_table().all_symbols() {
            // Only include symbols defined in this file
            if symbol.source_file() != Some(file_path.to_str().unwrap_or("")) {
                continue;
            }

            if let Some(span) = symbol.span() {
                let range = super::helpers::span_to_lsp_range(&span);
                let selection_range = range;

                let symbol_kind = match symbol {
                    Symbol::Package { .. } => SymbolKind::NAMESPACE,
                    Symbol::Classifier { .. } | Symbol::Definition { .. } => SymbolKind::CLASS,
                    Symbol::Feature { .. } | Symbol::Usage { .. } => SymbolKind::PROPERTY,
                    Symbol::Alias { .. } => SymbolKind::VARIABLE,
                };

                let doc_symbol = DocumentSymbol {
                    name: symbol.name().to_string(),
                    detail: Some(symbol.qualified_name().to_string()),
                    kind: symbol_kind,
                    range,
                    selection_range,
                    children: Some(Vec::new()),
                    tags: None,
                    #[allow(deprecated)]
                    deprecated: None,
                };

                flat_symbols.push((symbol.qualified_name().to_string(), doc_symbol));
            }
        }

        // Build hierarchy from qualified names
        self.build_symbol_hierarchy(flat_symbols)
    }

    /// Build a hierarchical structure from flat symbols using qualified names
    fn build_symbol_hierarchy(
        &self,
        flat_symbols: Vec<(String, DocumentSymbol)>,
    ) -> Vec<DocumentSymbol> {
        let mut symbol_map: HashMap<String, DocumentSymbol> = HashMap::new();
        let mut root_symbols = Vec::new();

        // First, add all symbols to the map
        for (qualified_name, symbol) in flat_symbols {
            symbol_map.insert(qualified_name, symbol);
        }

        // Sort by qualified name to ensure parents are processed before children
        let mut sorted_names: Vec<String> = symbol_map.keys().cloned().collect();
        sorted_names.sort();

        // Build the hierarchy
        for qualified_name in sorted_names {
            if let Some(symbol) = symbol_map.remove(&qualified_name) {
                // Find parent by removing the last segment of qualified name
                if let Some(last_separator) = qualified_name.rfind("::") {
                    let parent_name = &qualified_name[..last_separator];
                    
                    // Try to add as child to parent
                    if let Some(parent) = symbol_map.get_mut(parent_name) {
                        if let Some(ref mut children) = parent.children {
                            children.push(symbol);
                        }
                        // Re-insert parent
                        let parent = symbol_map.remove(parent_name).unwrap();
                        symbol_map.insert(parent_name.to_string(), parent);
                    } else {
                        // Parent not found, add to root
                        root_symbols.push(symbol);
                    }
                } else {
                    // No parent (top-level symbol)
                    root_symbols.push(symbol);
                }
            }
        }

        // Add any remaining symbols that weren't added as children
        for (_, symbol) in symbol_map {
            root_symbols.push(symbol);
        }

        root_symbols
    }
}
