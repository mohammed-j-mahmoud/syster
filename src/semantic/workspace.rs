use crate::language::sysml::SymbolTablePopulator;
use crate::language::sysml::syntax::SysMLFile;
use crate::semantic::RelationshipGraph;
use crate::semantic::symbol_table::SymbolTable;
use std::collections::HashMap;
use std::path::PathBuf;

/// Represents a file in the workspace with its path and parsed content
#[derive(Debug)]
pub struct WorkspaceFile {
    path: PathBuf,
    content: SysMLFile,
}

impl WorkspaceFile {
    pub fn new(path: PathBuf, content: SysMLFile) -> Self {
        Self { path, content }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn content(&self) -> &SysMLFile {
        &self.content
    }
}

/// A workspace manages multiple SysML files with a shared symbol table and relationship graph
#[derive(Debug)]
pub struct Workspace {
    files: HashMap<PathBuf, WorkspaceFile>,
    symbol_table: SymbolTable,
    relationship_graph: RelationshipGraph,
}

impl Workspace {
    /// Creates a new empty workspace
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            symbol_table: SymbolTable::new(),
            relationship_graph: RelationshipGraph::new(),
        }
    }

    /// Adds a file to the workspace
    pub fn add_file(&mut self, path: PathBuf, content: SysMLFile) {
        let file = WorkspaceFile::new(path.clone(), content);
        self.files.insert(path, file);
    }

    /// Populates the symbol table and relationship graph for all files in the workspace
    ///
    /// # Errors
    ///
    /// Returns an error if any file in the workspace fails to populate due to invalid syntax
    /// or semantic errors in the SysML content.
    pub fn populate_all(&mut self) -> Result<(), String> {
        // Sort files by path for deterministic ordering
        let mut paths: Vec<_> = self.files.keys().cloned().collect();
        paths.sort();

        for path in paths {
            self.populate_file(&path)?;
        }

        Ok(())
    }

    /// Populates the symbol table and relationship graph for a specific file
    ///
    /// # Errors
    ///
    /// Returns an error if the file is not found in the workspace or if the file fails
    /// to populate due to invalid syntax or semantic errors in the SysML content.
    pub fn populate_file(&mut self, path: &PathBuf) -> Result<(), String> {
        let file = self
            .files
            .get(path)
            .ok_or_else(|| format!("File not found in workspace: {}", path.display()))?;

        // Set the current file context in the symbol table
        self.symbol_table
            .set_current_file(Some(path.to_string_lossy().to_string()));

        // Populate the symbol table and relationship graph
        let mut populator = SymbolTablePopulator::with_relationships(
            &mut self.symbol_table,
            &mut self.relationship_graph,
        );

        populator
            .populate(file.content())
            .map_err(|e| format!("Failed to populate {}: {:?}", path.display(), e))?;

        Ok(())
    }

    /// Returns a reference to the symbol table
    pub fn symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }

    /// Returns a mutable reference to the symbol table
    pub fn symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }

    /// Returns a reference to the relationship graph
    pub fn relationship_graph(&self) -> &RelationshipGraph {
        &self.relationship_graph
    }

    /// Returns a mutable reference to the relationship graph
    pub fn relationship_graph_mut(&mut self) -> &mut RelationshipGraph {
        &mut self.relationship_graph
    }

    /// Returns the number of files in the workspace
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Returns an iterator over all file paths in the workspace
    pub fn file_paths(&self) -> impl Iterator<Item = &PathBuf> {
        self.files.keys()
    }

    /// Checks if a file exists in the workspace
    pub fn contains_file(&self, path: &PathBuf) -> bool {
        self.files.contains_key(path)
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "workspace/tests.rs"]
mod tests;
