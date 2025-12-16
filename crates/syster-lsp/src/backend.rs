use std::collections::HashMap;
use std::path::PathBuf;
use syster::project::ParseError;
use syster::semantic::Workspace;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range, Url};

/// Backend manages the workspace state for the LSP server
pub struct Backend {
    workspace: Workspace,
    /// Track parse errors for each file (keyed by file path)
    parse_errors: HashMap<PathBuf, Vec<ParseError>>,
}

impl Backend {
    pub fn new() -> Self {
        Self {
            workspace: Workspace::new(),
            parse_errors: HashMap::new(),
        }
    }

    pub fn workspace(&self) -> &Workspace {
        &self.workspace
    }

    pub fn workspace_mut(&mut self) -> &mut Workspace {
        &mut self.workspace
    }

    /// Open a document and add it to the workspace
    pub fn open_document(&mut self, uri: &Url, text: &str) -> Result<(), String> {
        let path = uri
            .to_file_path()
            .map_err(|_| format!("Invalid file URI: {}", uri))?;

        // Parse the file based on extension
        let parse_result = if path.extension().and_then(|s| s.to_str()) == Some("sysml") {
            syster::project::file_loader::parse_with_result(text, &path)
        } else if path.extension().and_then(|s| s.to_str()) == Some("kerml") {
            return Err("KerML files not yet fully supported".to_string());
        } else {
            return Err(format!(
                "Unsupported file extension: {:?}",
                path.extension()
            ));
        };

        // Store parse errors
        self.parse_errors.insert(path.clone(), parse_result.errors);

        // If parsing succeeded, add to workspace
        if let Some(file) = parse_result.content {
            self.workspace.add_file(path, file);
            // Populate symbols - ignore semantic errors for now
            let _ = self.workspace.populate_all();
        }

        Ok(())
    }

    /// Update an open document with new content
    pub fn change_document(&mut self, uri: &Url, text: &str) -> Result<(), String> {
        let path = uri
            .to_file_path()
            .map_err(|_| format!("Invalid file URI: {}", uri))?;

        // Parse the updated file
        let parse_result = if path.extension().and_then(|s| s.to_str()) == Some("sysml") {
            syster::project::file_loader::parse_with_result(text, &path)
        } else if path.extension().and_then(|s| s.to_str()) == Some("kerml") {
            return Err("KerML files not yet fully supported".to_string());
        } else {
            return Err(format!(
                "Unsupported file extension: {:?}",
                path.extension()
            ));
        };

        // Store parse errors
        self.parse_errors.insert(path.clone(), parse_result.errors);

        // Update in workspace (remove old first)
        self.workspace.remove_file(&path);

        // If parsing succeeded, add new version to workspace
        if let Some(file) = parse_result.content {
            self.workspace.add_file(path, file);
            // Repopulate symbols - ignore semantic errors for now
            let _ = self.workspace.populate_all();
        }

        Ok(())
    }

    /// Close a document - optionally remove from workspace
    /// For now, we keep documents in workspace even after close
    /// to maintain cross-file references
    pub fn close_document(&mut self, _uri: &Url) -> Result<(), String> {
        // We don't remove from workspace to keep cross-file references working
        // In the future, might want to track "open" vs "workspace" files separately
        Ok(())
    }

    /// Get LSP diagnostics for a given file
    pub fn get_diagnostics(&self, uri: &Url) -> Vec<Diagnostic> {
        let path = match uri.to_file_path() {
            Ok(p) => p,
            Err(_) => return vec![],
        };

        // Convert parse errors to LSP diagnostics
        self.parse_errors
            .get(&path)
            .map(|errors| {
                errors
                    .iter()
                    .map(|e| Diagnostic {
                        range: Range {
                            start: Position {
                                line: e.position.line as u32,
                                character: e.position.column as u32,
                            },
                            end: Position {
                                line: e.position.line as u32,
                                character: (e.position.column + 1) as u32,
                            },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: e.message.clone(),
                        ..Default::default()
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_creation() {
        let backend = Backend::new();
        assert_eq!(backend.workspace().file_count(), 0);
    }

    #[test]
    fn test_backend_provides_workspace_access() {
        let mut backend = Backend::new();

        // Should be able to access workspace mutably
        let workspace = backend.workspace_mut();
        assert_eq!(workspace.file_count(), 0);

        // Should be able to access workspace immutably
        let workspace = backend.workspace();
        assert_eq!(workspace.file_count(), 0);
    }

    #[test]
    fn test_open_sysml_document() {
        let mut backend = Backend::new();
        let uri = Url::parse("file:///test.sysml").unwrap();
        let text = "part def Vehicle;";

        backend.open_document(&uri, text).unwrap();

        assert_eq!(backend.workspace().file_count(), 1);
        assert!(backend.workspace().symbol_table().all_symbols().len() > 0);
    }

    #[test]
    fn test_open_invalid_sysml() {
        let mut backend = Backend::new();
        let uri = Url::parse("file:///test.sysml").unwrap();
        let text = "invalid syntax !@#$%";

        // Should succeed (errors are captured, not returned)
        let result = backend.open_document(&uri, text);
        assert!(result.is_ok());

        // File should NOT be added to workspace (parse failed)
        assert_eq!(backend.workspace().file_count(), 0);

        // Should have diagnostics
        let diagnostics = backend.get_diagnostics(&uri);
        assert!(!diagnostics.is_empty());
        assert!(diagnostics[0].message.len() > 0);
    }

    #[test]
    fn test_open_unsupported_extension() {
        let mut backend = Backend::new();
        let uri = Url::parse("file:///test.txt").unwrap();
        let text = "some text";

        let result = backend.open_document(&uri, text);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported file extension"));
    }

    #[test]
    fn test_open_kerml_file() {
        let mut backend = Backend::new();
        let uri = Url::parse("file:///test.kerml").unwrap();
        let text = "classifier Vehicle;";

        let result = backend.open_document(&uri, text);
        // KerML not yet supported
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("KerML"));
    }

    #[test]
    fn test_change_document() {
        let mut backend = Backend::new();
        let uri = Url::parse("file:///test.sysml").unwrap();

        // Open initial document
        backend.open_document(&uri, "part def Car;").unwrap();
        assert_eq!(backend.workspace().file_count(), 1);
        let initial_symbols = backend.workspace().symbol_table().all_symbols().len();

        // Change document content
        backend
            .change_document(&uri, "part def Vehicle; part def Bike;")
            .unwrap();

        assert_eq!(backend.workspace().file_count(), 1);
        let updated_symbols = backend.workspace().symbol_table().all_symbols().len();
        assert!(updated_symbols > initial_symbols);
    }

    #[test]
    fn test_change_document_with_error() {
        let mut backend = Backend::new();
        let uri = Url::parse("file:///test.sysml").unwrap();

        // Open valid document
        backend.open_document(&uri, "part def Car;").unwrap();
        assert_eq!(backend.workspace().file_count(), 1);

        // Change to invalid content - should succeed but capture error
        let result = backend.change_document(&uri, "invalid syntax !@#");
        assert!(result.is_ok());

        // File should be removed from workspace (parse failed)
        assert_eq!(backend.workspace().file_count(), 0);

        // Should have diagnostics
        let diagnostics = backend.get_diagnostics(&uri);
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn test_change_nonexistent_document() {
        let mut backend = Backend::new();
        let uri = Url::parse("file:///test.sysml").unwrap();

        // Try to change a document that was never opened
        let result = backend.change_document(&uri, "part def Car;");
        // Should succeed - change_document handles both open and update
        assert!(result.is_ok());
    }

    #[test]
    fn test_close_document() {
        let mut backend = Backend::new();
        let uri = Url::parse("file:///test.sysml").unwrap();

        // Open and close document
        backend.open_document(&uri, "part def Car;").unwrap();
        backend.close_document(&uri).unwrap();

        // Document should still be in workspace (we keep it for cross-file refs)
        assert_eq!(backend.workspace().file_count(), 1);
    }

    #[test]
    fn test_get_diagnostics_for_valid_file() {
        let mut backend = Backend::new();
        let uri = Url::parse("file:///test.sysml").unwrap();
        let text = "part def Vehicle;";

        backend.open_document(&uri, text).unwrap();

        let diagnostics = backend.get_diagnostics(&uri);
        assert!(
            diagnostics.is_empty(),
            "Valid file should have no diagnostics"
        );
    }

    #[test]
    fn test_get_diagnostics_for_parse_error() {
        let mut backend = Backend::new();
        let uri = Url::parse("file:///test.sysml").unwrap();
        let text = "part def invalid syntax";

        backend.open_document(&uri, text).unwrap();

        let diagnostics = backend.get_diagnostics(&uri);
        assert!(
            !diagnostics.is_empty(),
            "Should have parse error diagnostic"
        );
        assert_eq!(diagnostics[0].severity, Some(DiagnosticSeverity::ERROR));
        assert!(diagnostics[0].message.len() > 0);
    }

    #[test]
    fn test_get_diagnostics_clears_on_fix() {
        let mut backend = Backend::new();
        let uri = Url::parse("file:///test.sysml").unwrap();

        // Open with error
        backend.open_document(&uri, "invalid syntax").unwrap();
        let diagnostics = backend.get_diagnostics(&uri);
        assert!(!diagnostics.is_empty());

        // Fix the error
        backend.change_document(&uri, "part def Car;").unwrap();
        let diagnostics = backend.get_diagnostics(&uri);
        assert!(
            diagnostics.is_empty(),
            "Diagnostics should be cleared after fix"
        );
    }

    #[test]
    fn test_get_diagnostics_for_nonexistent_file() {
        let backend = Backend::new();
        let uri = Url::parse("file:///nonexistent.sysml").unwrap();

        let diagnostics = backend.get_diagnostics(&uri);
        assert!(
            diagnostics.is_empty(),
            "Nonexistent file should have no diagnostics"
        );
    }
}
