//! # Semantic Adapters
//!
//! Adapters convert language-specific ASTs into the language-agnostic semantic model.
//! Each source language (SysML, KerML) has its own adapter that knows how to extract
//! symbols, relationships, and semantic information from that language's AST.
//!
//! This is the **only** place in the semantic layer that should import from syntax/sysml
//! or syntax/kerml.

//! # Semantic Adapters
//!
//! Adapters convert language-specific ASTs into the language-agnostic semantic model.
//! Each source language (SysML, KerML) has its own adapter that knows how to extract
//! symbols, relationships, and semantic information from that language's AST.
//!
//! This is the **only** place in the semantic layer that should import from syntax/sysml
//! or syntax/kerml.

mod syntax_factory;
mod sysml;
pub mod sysml_adapter;
mod validator_factory;

pub use syntax_factory::populate_syntax_file;
pub use sysml::validator::SysmlValidator;
pub use sysml_adapter::SysmlAdapter;
pub use validator_factory::create_validator;

#[cfg(test)]
mod tests;
