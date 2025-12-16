use super::AnalysisContext;
use crate::semantic::RelationshipGraph;
use crate::semantic::processors::{NoOpValidator, RelationshipValidator};
use crate::semantic::symbol_table::SymbolTable;
use crate::semantic::types::SemanticResult;
use std::sync::Arc;

/// Main semantic analyzer that orchestrates analysis passes
pub struct SemanticAnalyzer {
    pub(super) symbol_table: SymbolTable,
    pub(super) relationship_graph: RelationshipGraph,
    pub(super) validator: Arc<dyn RelationshipValidator>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            relationship_graph: RelationshipGraph::new(),
            validator: Arc::new(NoOpValidator),
        }
    }

    pub fn with_symbol_table(symbol_table: SymbolTable) -> Self {
        Self {
            symbol_table,
            relationship_graph: RelationshipGraph::new(),
            validator: Arc::new(NoOpValidator),
        }
    }

    pub fn with_symbol_table_and_relationships(
        symbol_table: SymbolTable,
        relationship_graph: RelationshipGraph,
    ) -> Self {
        Self {
            symbol_table,
            relationship_graph,
            validator: Arc::new(NoOpValidator),
        }
    }

    pub fn with_validator(
        symbol_table: SymbolTable,
        relationship_graph: RelationshipGraph,
        validator: Arc<dyn RelationshipValidator>,
    ) -> Self {
        Self {
            symbol_table,
            relationship_graph,
            validator,
        }
    }

    pub fn symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }

    pub fn symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }

    pub fn relationship_graph(&self) -> &RelationshipGraph {
        &self.relationship_graph
    }

    pub fn relationship_graph_mut(&mut self) -> &mut RelationshipGraph {
        &mut self.relationship_graph
    }

    /// Run all analysis passes on the symbol table
    /// # Errors
    ///
    /// Returns a `Vec<SemanticError>` if any semantic errors are detected during:
    /// - Symbol table structure validation
    /// - Type reference validation
    /// - Relationship validation (specialization, redefinition, etc.)
    pub fn analyze(&self) -> SemanticResult<()> {
        let mut context = AnalysisContext::new(&self.symbol_table, &self.relationship_graph);

        // Pass 1: Validate type references
        self.validate_types(&mut context);

        // Pass 2: Validate relationships (specialization, redefinition, etc.)
        self.validate_relationships(&mut context);

        context.into_result(())
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
