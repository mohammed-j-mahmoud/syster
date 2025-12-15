use crate::core::constants::SUPPORTED_EXTENSIONS;
use crate::language::sysml::syntax::SysMLFile;
use crate::semantic::Workspace;
use from_pest::FromPest;
use pest::Parser;
use std::fs;
use std::path::PathBuf;

/// Loads workspace files on demand
pub struct WorkspaceLoader;

impl WorkspaceLoader {
    pub fn new() -> Self {
        Self
    }

    pub fn load_file<P: Into<PathBuf>>(
        &self,
        path: P,
        workspace: &mut Workspace,
    ) -> Result<(), String> {
        let path = path.into();
        self.load_file_internal(&path, workspace)
    }

    pub fn load_directory<P: Into<PathBuf>>(
        &self,
        path: P,
        workspace: &mut Workspace,
    ) -> Result<(), String> {
        let path = path.into();
        if !path.exists() || !path.is_dir() {
            return Err(format!("Directory not found: {}", path.display()));
        }
        self.load_directory_internal(&path, workspace)
    }

    fn load_directory_internal(
        &self,
        dir: &PathBuf,
        workspace: &mut Workspace,
    ) -> Result<(), String> {
        let entries = fs::read_dir(dir)
            .map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                // Recursively process subdirectories
                self.load_directory_internal(&path, workspace)?;
            } else if path.is_file()
                && path
                    .extension()
                    .and_then(|e| e.to_str())
                    .is_some_and(|ext| SUPPORTED_EXTENSIONS.contains(&ext))
            {
                self.load_file_internal(&path, workspace)?;
            }
        }

        Ok(())
    }

    fn load_file_internal(&self, path: &PathBuf, workspace: &mut Workspace) -> Result<(), String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| format!("Invalid file extension for {}", path.display()))?;

        match ext {
            "sysml" => {
                let mut pairs =
                    crate::parser::SysMLParser::parse(crate::parser::sysml::Rule::model, &content)
                        .map_err(|e| format!("Parse error in {}: {}", path.display(), e))?;

                let file = SysMLFile::from_pest(&mut pairs)
                    .map_err(|e| format!("AST error in {}: {:?}", path.display(), e))?;

                workspace.add_file(path.clone(), file);
            }
            "kerml" => {
                // TODO: Add KerML parser support
            }
            _ => return Err(format!("Unsupported file extension: {}", ext)),
        }

        Ok(())
    }
}

impl Default for WorkspaceLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_loader_creation() {
        let _loader = WorkspaceLoader::new();
    }
}
