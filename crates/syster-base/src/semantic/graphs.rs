//! Relationship graphs for tracking semantic relationships between symbols

mod dependency_graph;
mod one_to_many;
mod one_to_one;
mod relationship_graph;
mod symmetric;

pub use dependency_graph::DependencyGraph;
pub use one_to_many::OneToManyGraph;
pub use one_to_one::OneToOneGraph;
pub use relationship_graph::RelationshipGraph;
pub use symmetric::SymmetricGraph;

#[cfg(test)]
#[path = "graphs/tests.rs"]
mod tests;
