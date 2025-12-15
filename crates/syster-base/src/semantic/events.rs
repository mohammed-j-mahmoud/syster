//! # Semantic Event System
//!
//! Defines all event types emitted by semantic analysis components.
//!
//! These events enable observability and decoupled logic for:
//! - LSP implementations (subscribe to changes)
//! - Testing (verify events fire without side effects)
//! - Extensibility (add listeners without modifying core code)
//!
//! ## Event Types
//!
//! - **WorkspaceEvent**: File additions, updates, removals
//! - **DependencyEvent**: Dependency graph changes
//!
//! Future additions:
//! - SymbolTableEvent: Symbol insertions, import additions
//! - RelationshipEvent: Relationship graph changes
//! - AnalyzerEvent: Semantic error detection

use crate::core::events::Event;
use std::path::PathBuf;

/// Events emitted by the workspace during file operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceEvent {
    /// A file was added to the workspace
    FileAdded { path: PathBuf },

    /// A file's content was updated
    FileUpdated { path: PathBuf },

    /// A file was removed from the workspace
    FileRemoved { path: PathBuf },
}

impl Event for WorkspaceEvent {}

/// Events emitted by the dependency graph during updates
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyEvent {
    /// A dependency was added between two files
    DependencyAdded {
        /// The file that imports another file
        from: PathBuf,
        /// The file being imported
        to: PathBuf,
    },

    /// A file and all its dependencies were removed
    FileRemoved { path: PathBuf },
}

impl Event for DependencyEvent {}

/// Events emitted by the symbol table during symbol operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolTableEvent {
    /// A symbol was inserted into the symbol table
    SymbolInserted {
        /// The qualified name of the symbol
        qualified_name: String,
        /// The unique identifier of the symbol
        symbol_id: usize,
    },

    /// An import was added to a file's import list
    ImportAdded {
        /// The import path that was added
        import_path: String,
    },

    /// The current file context was changed
    FileChanged {
        /// The path of the file that is now current
        file_path: String,
    },
}

impl Event for SymbolTableEvent {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_event_creation() {
        let path = PathBuf::from("test.sysml");

        let added = WorkspaceEvent::FileAdded { path: path.clone() };
        let updated = WorkspaceEvent::FileUpdated { path: path.clone() };
        let removed = WorkspaceEvent::FileRemoved { path: path.clone() };

        assert!(matches!(added, WorkspaceEvent::FileAdded { .. }));
        assert!(matches!(updated, WorkspaceEvent::FileUpdated { .. }));
        assert!(matches!(removed, WorkspaceEvent::FileRemoved { .. }));
    }

    #[test]
    fn test_workspace_event_equality() {
        let path = PathBuf::from("test.sysml");

        let event1 = WorkspaceEvent::FileUpdated { path: path.clone() };
        let event2 = WorkspaceEvent::FileUpdated { path: path.clone() };

        assert_eq!(event1, event2);
    }

    #[test]
    fn test_dependency_event_creation() {
        let from = PathBuf::from("app.sysml");
        let to = PathBuf::from("base.sysml");

        let added = DependencyEvent::DependencyAdded {
            from: from.clone(),
            to: to.clone(),
        };
        let removed = DependencyEvent::FileRemoved { path: from.clone() };

        assert!(matches!(added, DependencyEvent::DependencyAdded { .. }));
        assert!(matches!(removed, DependencyEvent::FileRemoved { .. }));
    }

    #[test]
    fn test_dependency_event_equality() {
        let from = PathBuf::from("app.sysml");
        let to = PathBuf::from("base.sysml");

        let event1 = DependencyEvent::DependencyAdded {
            from: from.clone(),
            to: to.clone(),
        };
        let event2 = DependencyEvent::DependencyAdded {
            from: from.clone(),
            to: to.clone(),
        };

        assert_eq!(event1, event2);
    }

    #[test]
    fn test_symbol_table_event_creation() {
        let inserted = SymbolTableEvent::SymbolInserted {
            qualified_name: "Package::Vehicle".to_string(),
            symbol_id: 42,
        };
        let import_added = SymbolTableEvent::ImportAdded {
            import_path: "Base::*".to_string(),
        };
        let file_changed = SymbolTableEvent::FileChanged {
            file_path: "test.sysml".to_string(),
        };

        assert!(matches!(inserted, SymbolTableEvent::SymbolInserted { .. }));
        assert!(matches!(import_added, SymbolTableEvent::ImportAdded { .. }));
        assert!(matches!(file_changed, SymbolTableEvent::FileChanged { .. }));
    }

    #[test]
    fn test_symbol_table_event_equality() {
        let event1 = SymbolTableEvent::SymbolInserted {
            qualified_name: "Package::Vehicle".to_string(),
            symbol_id: 42,
        };
        let event2 = SymbolTableEvent::SymbolInserted {
            qualified_name: "Package::Vehicle".to_string(),
            symbol_id: 42,
        };

        assert_eq!(event1, event2);
    }
}
