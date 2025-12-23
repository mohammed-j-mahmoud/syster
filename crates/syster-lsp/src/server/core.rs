use std::collections::HashMap;
use std::path::PathBuf;
use syster::core::ParseError;
use syster::project::StdLibLoader;
use syster::semantic::{Workspace, resolver::Resolver};
use syster::syntax::SyntaxFile;

/// LspServer manages the workspace state for the LSP server
pub struct LspServer {
    pub(super) workspace: Workspace<SyntaxFile>,
    /// Track parse errors for each file (keyed by file path)
    pub(super) parse_errors: HashMap<PathBuf, Vec<ParseError>>,
    /// Track document text for hover and other features (keyed by file path)
    pub(super) document_texts: HashMap<PathBuf, String>,
    /// Stdlib loader for lazy loading
    pub(super) stdlib_loader: StdLibLoader,
    /// Whether stdlib loading is enabled
    stdlib_enabled: bool,
}

impl Default for LspServer {
    fn default() -> Self {
        Self::new()
    }
}

impl LspServer {
    pub fn new() -> Self {
        Self::with_config(true, None)
    }

    /// Create a new LspServer with custom configuration
    pub fn with_config(stdlib_enabled: bool, custom_stdlib_path: Option<PathBuf>) -> Self {
        // Initialize workspace without loading stdlib
        // Stdlib loading is expensive and not needed for most LSP operations
        // Files can load stdlib symbols through explicit imports
        let workspace = Workspace::<SyntaxFile>::new();

        // Determine stdlib path - try multiple locations
        let stdlib_path = if let Some(path) = custom_stdlib_path {
            path
        } else {
            // Get the binary directory (where syster-lsp executable is located)
            let binary_dir = std::env::current_exe()
                .ok()
                .and_then(|exe| exe.parent().map(|p| p.to_path_buf()))
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

            // Look for sysml.library next to the binary (copied by build script)
            let stdlib_next_to_binary = binary_dir.join("sysml.library");

            if stdlib_next_to_binary.exists() && stdlib_next_to_binary.is_dir() {
                stdlib_next_to_binary
            } else {
                PathBuf::from("sysml.library")
            }
        };

        Self {
            workspace,
            parse_errors: HashMap::new(),
            document_texts: HashMap::new(),
            stdlib_loader: StdLibLoader::with_path(stdlib_path),
            stdlib_enabled,
        }
    }

    pub fn workspace(&self) -> &Workspace<SyntaxFile> {
        &self.workspace
    }

    #[allow(dead_code)] // Used in integration tests
    pub fn workspace_mut(&mut self) -> &mut Workspace<SyntaxFile> {
        &mut self.workspace
    }

    pub fn resolver(&self) -> Resolver<'_> {
        Resolver::new(self.workspace.symbol_table())
    }

    #[allow(dead_code)]
    pub fn document_texts_mut(&mut self) -> &mut HashMap<PathBuf, String> {
        &mut self.document_texts
    }

    /// Ensure stdlib is loaded (lazy loading)
    /// Only loads stdlib once, on first call
    /// Returns Ok(()) even if stdlib loading is disabled
    pub fn ensure_stdlib_loaded(&mut self) -> Result<(), String> {
        if !self.stdlib_enabled {
            return Ok(());
        }

        // Load stdlib files into workspace
        self.stdlib_loader.ensure_loaded(&mut self.workspace)?;

        // Populate symbols from loaded files
        let _ = self.workspace.populate_all();

        // Sync document texts so hover can access source code
        self.sync_document_texts_from_workspace();

        Ok(())
    }

    /// Sync document_texts with all files currently in the workspace
    /// This ensures hover and other features work on all workspace files without disk reads
    pub fn sync_document_texts_from_workspace(&mut self) {
        for path in self.workspace.files().keys() {
            // Only load if not already tracked (avoid overwriting editor versions)
            if !self.document_texts.contains_key(path)
                && let Ok(text) = std::fs::read_to_string(path)
            {
                self.document_texts.insert(path.clone(), text);
            }
        }
    }
}
