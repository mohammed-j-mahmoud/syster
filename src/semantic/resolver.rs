//! # Name Resolver
//!
//! Resolves symbol names to their definitions in the symbol table, supporting both
//! simple names and qualified (multi-part) names.
//!
//! ## Resolution Algorithm
//!
//! ### Simple Names
//! Simple names (e.g., `Vehicle`) are looked up directly in the symbol table:
//! 1. Check current scope
//! 2. Walk up scope chain to parent scopes
//! 3. Check imported symbols
//!
//! ### Qualified Names
//! Qualified names (e.g., `Automotive::Engine::V8`) are resolved step-by-step:
//! 1. Look up first component (`Automotive`)
//! 2. Enter that symbol's scope
//! 3. Look up next component (`Engine`) within that scope
//! 4. Repeat until all components resolved
//!
//! ## Example
//!
//! ```rust
//! use syster::semantic::{NameResolver, SymbolTable};
//!
//! let symbol_table = SymbolTable::new();
//! let resolver = NameResolver::new(&symbol_table);
//!
//! // Simple lookup
//! let symbol = resolver.resolve("Vehicle");
//!
//! // Qualified lookup
//! let symbol = resolver.resolve_qualified("Automotive::Engine::V8");
//! ```
//!
//! ## Relationship to Import System
//!
//! Name resolution is affected by import statements:
//! - `import Package::*` makes all members of Package visible
//! - `import Package::Member` makes specific member visible
//! - Aliases (`import X as Y`) are resolved during lookup
//!
//! See [Import Resolution](../../docs/SEMANTIC_ANALYSIS.md#import-resolution) for details.

use crate::semantic::symbol_table::{Symbol, SymbolTable};

pub struct NameResolver<'a> {
    symbol_table: &'a SymbolTable,
}

impl<'a> NameResolver<'a> {
    pub fn new(symbol_table: &'a SymbolTable) -> Self {
        Self { symbol_table }
    }

    pub fn resolve(&self, name: &str) -> Option<&Symbol> {
        if name.contains("::") {
            self.resolve_qualified(name)
        } else {
            self.symbol_table.lookup(name)
        }
    }

    pub fn resolve_qualified(&self, qualified_name: &str) -> Option<&Symbol> {
        let parts: Vec<&str> = qualified_name.split("::").collect();
        if parts.is_empty() {
            return None;
        }

        let mut current_symbol = self.symbol_table.lookup(parts[0])?;

        for part in parts.iter().skip(1) {
            match current_symbol {
                Symbol::Package { qualified_name, .. } => {
                    let next_name = format!("{}::{}", qualified_name, part);
                    current_symbol = self.find_in_scope(&next_name)?;
                }
                Symbol::Classifier { qualified_name, .. } => {
                    let next_name = format!("{}::{}", qualified_name, part);
                    current_symbol = self.find_in_scope(&next_name)?;
                }
                _ => return None,
            }
        }

        Some(current_symbol)
    }

    fn find_in_scope(&self, qualified_name: &str) -> Option<&Symbol> {
        for (_, symbol) in self.symbol_table.all_symbols() {
            let symbol_qname = match symbol {
                Symbol::Package { qualified_name, .. } => qualified_name,
                Symbol::Classifier { qualified_name, .. } => qualified_name,
                Symbol::Feature { qualified_name, .. } => qualified_name,
                Symbol::Definition { qualified_name, .. } => qualified_name,
                Symbol::Usage { qualified_name, .. } => qualified_name,
                Symbol::Alias { qualified_name, .. } => qualified_name,
            };
            if symbol_qname == qualified_name {
                return Some(symbol);
            }
        }
        None
    }
}

#[cfg(test)]
#[path = "resolver/tests.rs"]
mod tests;
