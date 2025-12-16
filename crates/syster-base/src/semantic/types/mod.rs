pub mod diagnostic;
pub mod error;
pub mod events;

pub use diagnostic::{Diagnostic, Location as DiagnosticLocation, Position, Range, Severity};
pub use error::{Location, SemanticError, SemanticErrorKind, SemanticResult};
pub use events::{DependencyEvent, SymbolTableEvent, WorkspaceEvent};
