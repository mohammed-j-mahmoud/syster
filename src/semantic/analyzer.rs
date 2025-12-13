use crate::semantic::error::{SemanticError, SemanticResult};
use crate::semantic::resolver::NameResolver;
use crate::semantic::symbol_table::SymbolTable;

/// Context for semantic analysis passes
pub struct AnalysisContext<'a> {
    pub symbol_table: &'a SymbolTable,
    pub resolver: NameResolver<'a>,
    pub errors: Vec<SemanticError>,
}

impl<'a> AnalysisContext<'a> {
    pub fn new(symbol_table: &'a SymbolTable) -> Self {
        Self {
            symbol_table,
            resolver: NameResolver::new(symbol_table),
            errors: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: SemanticError) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Converts this context into a `Result`, returning the value if no errors were collected.
    ///
    /// # Errors
    ///
    /// Returns `Err` containing all collected semantic errors if any errors were added to this context.
    pub fn into_result<T>(self, value: T) -> SemanticResult<T> {
        if self.errors.is_empty() {
            Ok(value)
        } else {
            Err(self.errors)
        }
    }
}

/// Main semantic analyzer that orchestrates analysis passes
pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn with_symbol_table(symbol_table: SymbolTable) -> Self {
        Self { symbol_table }
    }

    pub fn symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }

    pub fn symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }

    /// Run all analysis passes on the symbol table
    /// # Errors
    ///
    /// Returns a `Vec<SemanticError>` if any semantic errors are detected during:
    /// - Symbol table structure validation
    /// - Type reference validation
    /// - Relationship validation (specialization, redefinition, etc.)
    pub fn analyze(&self) -> SemanticResult<()> {
        let mut context = AnalysisContext::new(&self.symbol_table);

        // Pass 1: Validate symbol table structure (scoping, duplicates)
        self.validate_symbol_table(&mut context);

        // Pass 2: Validate type references
        self.validate_types(&mut context);

        // Pass 3: Validate relationships (specialization, redefinition, etc.)
        self.validate_relationships(&mut context);

        context.into_result(())
    }

    fn validate_symbol_table(&self, _context: &mut AnalysisContext) {
        // Symbol table validation happens during insertion
        // This pass can check for additional structural issues
    }

    fn validate_types(&self, _context: &mut AnalysisContext) {
        // Type validation will be implemented
    }

    fn validate_relationships(&self, _context: &mut AnalysisContext) {
        // Relationship validation will be implemented
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "analyzer/tests.rs"]
mod tests;
