use crate::semantic::graphs::RelationshipGraph;
use crate::semantic::symbol_table::SymbolTable;
use crate::semantic::types::SemanticError;

pub struct KermlAdapter<'a> {
    pub(super) symbol_table: &'a mut SymbolTable,
    #[allow(dead_code)]
    pub(super) relationship_graph: Option<&'a mut RelationshipGraph>,
    pub(super) current_namespace: Vec<String>,
    pub(super) errors: Vec<SemanticError>,
}

impl<'a> KermlAdapter<'a> {
    pub fn new(symbol_table: &'a mut SymbolTable) -> Self {
        Self {
            symbol_table,
            relationship_graph: None,
            current_namespace: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn with_relationships(
        symbol_table: &'a mut SymbolTable,
        relationship_graph: &'a mut RelationshipGraph,
    ) -> Self {
        Self {
            symbol_table,
            relationship_graph: Some(relationship_graph),
            current_namespace: Vec::new(),
            errors: Vec::new(),
        }
    }
}
