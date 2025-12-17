//! SysML-specific relationship validation.
//!
//! This validator understands SysML semantic rules and validates relationships
//! using the semantic_role field populated by the SysML adapter.

#![allow(clippy::result_large_err)]

use crate::core::constants::{
    REL_EXHIBIT, REL_INCLUDE, REL_PERFORM, REL_SATISFY, ROLE_ACTION, ROLE_REQUIREMENT, ROLE_STATE,
    ROLE_USE_CASE,
};
use crate::semantic::analyzer::validation::RelationshipValidator;
use crate::semantic::symbol_table::Symbol;
use crate::semantic::types::{SemanticError, SemanticRole};

/// SysML relationship validator that uses semantic roles.
/// This is the language-specific validator for SysML.
pub struct SysmlValidator;

impl SysmlValidator {
    pub fn new() -> Self {
        Self
    }

    /// Gets the semantic role from a symbol (Definition or Usage)
    fn get_semantic_role(symbol: &Symbol) -> Option<&SemanticRole> {
        match symbol {
            Symbol::Definition { semantic_role, .. } => semantic_role.as_ref(),
            Symbol::Usage { semantic_role, .. } => semantic_role.as_ref(),
            _ => None,
        }
    }

    /// Returns the appropriate indefinite article (a/an) for a word
    fn article_for(word: &str) -> &'static str {
        // Check first character for vowel sound (excluding 'u' which sounds like 'yoo')
        if let Some(first) = word.chars().next() {
            if matches!(first, 'a' | 'e' | 'i' | 'o') {
                "an"
            } else {
                "a"
            }
        } else {
            "a"
        }
    }

    /// Generic validation function that checks if a target symbol has the required semantic role
    fn validate_target_role<F>(
        &self,
        target: &Symbol,
        role_checker: F,
        relationship_name: &str,
        expected_role: &str,
    ) -> Result<(), SemanticError>
    where
        F: Fn(&SemanticRole) -> bool,
    {
        if let Some(role) = Self::get_semantic_role(target) {
            if role_checker(role) {
                return Ok(());
            }
            return Err(SemanticError::invalid_type(format!(
                "{} relationship must target {} {}, but '{}' is a {}",
                relationship_name,
                Self::article_for(expected_role),
                expected_role,
                target.qualified_name(),
                role
            )));
        }
        // No semantic role - can't validate (shouldn't happen with proper adapter)
        Ok(())
    }
}

impl Default for SysmlValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl RelationshipValidator for SysmlValidator {
    fn validate_relationship(
        &self,
        relationship_type: &str,
        _source: &Symbol,
        target: &Symbol,
    ) -> Result<(), SemanticError> {
        match relationship_type {
            REL_SATISFY => self.validate_target_role(
                target,
                |r| r.is_requirement(),
                REL_SATISFY,
                ROLE_REQUIREMENT,
            ),
            REL_PERFORM => {
                self.validate_target_role(target, |r| r.is_action(), REL_PERFORM, ROLE_ACTION)
            }
            REL_EXHIBIT => {
                self.validate_target_role(target, |r| r.is_state(), REL_EXHIBIT, ROLE_STATE)
            }
            REL_INCLUDE => {
                self.validate_target_role(target, |r| r.is_use_case(), REL_INCLUDE, ROLE_USE_CASE)
            }
            _ => Ok(()), // No constraints for other relationships in SysML
        }
    }
}
