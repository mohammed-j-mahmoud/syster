use crate::semantic::RelationshipGraph;
use crate::semantic::resolver::NameResolver;
use crate::semantic::symbol_table::SymbolTable;
use crate::semantic::types::{SemanticError, SemanticResult};

pub struct AnalysisContext<'a> {
    pub symbol_table: &'a SymbolTable,
    pub relationship_graph: &'a RelationshipGraph,
    pub resolver: NameResolver<'a>,
    pub errors: Vec<SemanticError>,
}

impl<'a> AnalysisContext<'a> {
    pub fn new(symbol_table: &'a SymbolTable, relationship_graph: &'a RelationshipGraph) -> Self {
        Self {
            symbol_table,
            relationship_graph,
            resolver: NameResolver::new(symbol_table),
            errors: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: SemanticError) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Converts this context into a `Result`, returning the value if no errors were collected.
    ///
    /// # Errors
    ///
    /// Returns `Err` containing all collected semantic errors if any errors were added to this context.
    pub fn into_result<T>(self, value: T) -> SemanticResult<T> {
        if self.errors.is_empty() {
            Ok(value)
        } else {
            Err(self.errors)
        }
    }
}
