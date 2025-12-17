//! # Semantic Validation
//!
//! Validates semantic rules and relationships after the AST has been converted
//! to the semantic model. This includes type checking, relationship validation,
//! and constraint checking.

pub mod relationship_validator;
pub mod sysml_validator;

pub use relationship_validator::{NoOpValidator, RelationshipValidator};
pub use sysml_validator::SysMLRelationshipValidator;
