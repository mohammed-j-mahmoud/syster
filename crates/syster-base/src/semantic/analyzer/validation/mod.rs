//! # Semantic Validation
//!
//! Defines the validation trait for relationship validation.
//! Language-specific validators live in the adapters module.

pub mod relationship_validator;

pub use relationship_validator::{NoOpValidator, RelationshipValidator};
