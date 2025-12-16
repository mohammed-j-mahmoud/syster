//! Semantic analyzer that orchestrates validation passes over the symbol table
//! and relationship graphs, detecting errors and enforcing SysML/KerML rules.

mod context;
mod semantic_analyzer;
mod validator;

pub use context::AnalysisContext;
pub use semantic_analyzer::SemanticAnalyzer;
