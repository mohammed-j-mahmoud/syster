//! # Semantic Analysis
//!
//! This module provides semantic analysis for SysML v2 and KerML models, transforming
//! parsed ASTs into a queryable semantic model with cross-file symbol resolution.
pub mod analyzer;
pub mod graphs;
pub mod processors;
pub mod resolver;
pub mod symbol_table;
pub mod types;
pub mod workspace;

pub use analyzer::{AnalysisContext, SemanticAnalyzer};
pub use graphs::{DependencyGraph, RelationshipGraph};
pub use processors::{NoOpValidator, ReferenceCollector, RelationshipValidator};
pub use resolver::{NameResolver, extract_imports, is_wildcard_import, parse_import_path};
pub use symbol_table::SymbolTable;
pub use types::{
    DependencyEvent, Diagnostic, Location as DiagnosticLocation, Location, Position, Range,
    SemanticError, SemanticErrorKind, SemanticResult, Severity, SymbolTableEvent, WorkspaceEvent,
};
pub use workspace::Workspace;

pub type QualifiedName = String;
pub type SimpleName = String;
pub type ScopeId = usize;
pub type SourceFilePath = String;
