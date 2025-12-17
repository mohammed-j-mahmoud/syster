//! Relationship validation trait.
//!
//! Validators are language-specific and live in the adapters module.
//! This trait provides the interface for semantic relationship validation.
#![allow(clippy::result_large_err)]

use crate::semantic::symbol_table::Symbol;
use crate::semantic::types::SemanticError;

/// Trait for validating relationships between symbols.
///
/// Implementations are language-specific and typically live in the adapters module
/// where language knowledge belongs (e.g., `semantic::adapters::sysml::validator`).
pub trait RelationshipValidator: Send + Sync {
    /// Validates that a relationship between source and target is semantically correct.
    ///
    /// # Errors
    /// Returns error if relationship violates language-specific constraints.
    fn validate_relationship(
        &self,
        relationship_type: &str,
        source: &Symbol,
        target: &Symbol,
    ) -> Result<(), SemanticError>;
}

/// No-op validator that accepts all relationships.
/// Useful for testing or when validation is not needed.
pub struct NoOpValidator;

impl RelationshipValidator for NoOpValidator {
    fn validate_relationship(
        &self,
        _relationship_type: &str,
        _source: &Symbol,
        _target: &Symbol,
    ) -> Result<(), SemanticError> {
        Ok(())
    }
}
