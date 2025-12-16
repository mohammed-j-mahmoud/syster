//! Symmetric (bidirectional) graph for mutual relationships

use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct SymmetricGraph {
    relationships: HashMap<String, Vec<String>>,
}

impl SymmetricGraph {
    pub fn new() -> Self {
        Self {
            relationships: HashMap::new(),
        }
    }

    pub fn add(&mut self, element1: String, element2: String) {
        self.relationships
            .entry(element1.clone())
            .or_default()
            .push(element2.clone());
        self.relationships
            .entry(element2)
            .or_default()
            .push(element1);
    }

    pub fn get_related(&self, element: &str) -> Option<&[String]> {
        self.relationships.get(element).map(|v| v.as_slice())
    }

    pub fn are_related(&self, element1: &str, element2: &str) -> bool {
        self.relationships
            .get(element1)
            .is_some_and(|related| related.iter().any(|e| e == element2))
    }
}
