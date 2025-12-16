//! Main relationship graph that aggregates different graph types

use super::{OneToManyGraph, OneToOneGraph, SymmetricGraph};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct RelationshipGraph {
    one_to_many: HashMap<String, OneToManyGraph>,
    one_to_one: HashMap<String, OneToOneGraph>,
    symmetric: HashMap<String, SymmetricGraph>,
}

impl RelationshipGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_one_to_many(&mut self, relationship_type: &str, source: String, target: String) {
        self.one_to_many
            .entry(relationship_type.to_string())
            .or_default()
            .add(source, target);
    }

    pub fn get_one_to_many(&self, relationship_type: &str, source: &str) -> Option<&[String]> {
        self.one_to_many
            .get(relationship_type)
            .and_then(|g| g.get_targets(source))
    }

    pub fn get_one_to_many_sources(&self, relationship_type: &str, target: &str) -> Vec<&String> {
        self.one_to_many
            .get(relationship_type)
            .map(|g| g.get_sources(target))
            .unwrap_or_default()
    }

    pub fn add_one_to_one(&mut self, relationship_type: &str, source: String, target: String) {
        self.one_to_one
            .entry(relationship_type.to_string())
            .or_default()
            .add(source, target);
    }

    pub fn get_one_to_one(&self, relationship_type: &str, source: &str) -> Option<&String> {
        self.one_to_one
            .get(relationship_type)
            .and_then(|g| g.get_target(source))
    }

    pub fn add_symmetric(&mut self, relationship_type: &str, element1: String, element2: String) {
        self.symmetric
            .entry(relationship_type.to_string())
            .or_default()
            .add(element1, element2);
    }

    pub fn get_symmetric(&self, relationship_type: &str, element: &str) -> Option<&[String]> {
        self.symmetric
            .get(relationship_type)
            .and_then(|g| g.get_related(element))
    }

    pub fn has_transitive_path(&self, relationship_type: &str, from: &str, to: &str) -> bool {
        self.one_to_many
            .get(relationship_type)
            .map(|g| g.has_path(from, to))
            .unwrap_or(false)
    }

    pub fn relationship_types(&self) -> Vec<String> {
        let mut types = Vec::new();
        types.extend(self.one_to_many.keys().cloned());
        types.extend(self.one_to_one.keys().cloned());
        types.extend(self.symmetric.keys().cloned());
        types.sort();
        types.dedup();
        types
    }
}
