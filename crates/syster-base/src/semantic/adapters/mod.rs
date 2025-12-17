//! # Semantic Adapters
//!
//! Adapters convert language-specific ASTs into the language-agnostic semantic model.
//! Each source language (SysML, KerML) has its own adapter that knows how to extract
//! symbols, relationships, and semantic information from that language's AST.
//!
//! This is the **only** place in the semantic layer that should import from syntax/sysml
//! or syntax/kerml.

mod factory;
mod sysml;
pub mod sysml_adapter;

pub use factory::populate_syntax_file;
pub use sysml_adapter::SysmlAdapter;
