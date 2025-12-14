use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub path: String,
    pub is_recursive: bool,
    pub is_namespace: bool, // true for ::*, false for specific member
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Package {
        name: String,
        qualified_name: String,
        scope_id: usize,
        source_file: Option<String>,
    },
    Classifier {
        name: String,
        qualified_name: String,
        kind: String,
        is_abstract: bool,
        scope_id: usize,
        source_file: Option<String>,
    },
    Feature {
        name: String,
        qualified_name: String,
        feature_type: Option<String>,
        scope_id: usize,
        source_file: Option<String>,
    },
    Definition {
        name: String,
        qualified_name: String,
        kind: String,
        scope_id: usize,
        source_file: Option<String>,
    },
    Usage {
        name: String,
        qualified_name: String,
        kind: String,
        scope_id: usize,
        source_file: Option<String>,
    },
    Alias {
        name: String,
        qualified_name: String,
        target: String,
        scope_id: usize,
        source_file: Option<String>,
    },
}

impl Symbol {
    /// Returns the qualified name of this symbol
    pub fn qualified_name(&self) -> &str {
        match self {
            Symbol::Package { qualified_name, .. }
            | Symbol::Classifier { qualified_name, .. }
            | Symbol::Feature { qualified_name, .. }
            | Symbol::Definition { qualified_name, .. }
            | Symbol::Usage { qualified_name, .. }
            | Symbol::Alias { qualified_name, .. } => qualified_name,
        }
    }

    /// Returns the simple name of this symbol
    pub fn name(&self) -> &str {
        match self {
            Symbol::Package { name, .. }
            | Symbol::Classifier { name, .. }
            | Symbol::Feature { name, .. }
            | Symbol::Definition { name, .. }
            | Symbol::Usage { name, .. }
            | Symbol::Alias { name, .. } => name,
        }
    }

    /// Returns the scope ID where this symbol was defined
    pub fn scope_id(&self) -> usize {
        match self {
            Symbol::Package { scope_id, .. }
            | Symbol::Classifier { scope_id, .. }
            | Symbol::Feature { scope_id, .. }
            | Symbol::Definition { scope_id, .. }
            | Symbol::Usage { scope_id, .. }
            | Symbol::Alias { scope_id, .. } => *scope_id,
        }
    }

    /// Returns the source file path where this symbol was defined
    pub fn source_file(&self) -> Option<&str> {
        match self {
            Symbol::Package { source_file, .. }
            | Symbol::Classifier { source_file, .. }
            | Symbol::Feature { source_file, .. }
            | Symbol::Definition { source_file, .. }
            | Symbol::Usage { source_file, .. }
            | Symbol::Alias { source_file, .. } => source_file.as_deref(),
        }
    }

    /// Returns true if this symbol can be used as a type
    pub fn is_type(&self) -> bool {
        matches!(self, Symbol::Classifier { .. } | Symbol::Definition { .. })
    }

    /// Returns the type reference for Features that have one
    pub fn type_reference(&self) -> Option<&str> {
        match self {
            Symbol::Feature { feature_type, .. } => feature_type.as_deref(),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Scope {
    parent: Option<usize>,
    symbols: HashMap<String, Symbol>,
    children: Vec<usize>,
    imports: Vec<Import>,
}

impl Scope {
    fn new(parent: Option<usize>) -> Self {
        Self {
            parent,
            symbols: HashMap::new(),
            children: Vec::new(),
            imports: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
    current_scope: usize,
    current_file: Option<String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new(None)],
            current_scope: 0,
            current_file: None,
        }
    }

    /// Sets the current source file context for subsequently created symbols
    pub fn set_current_file(&mut self, file_path: Option<String>) {
        self.current_file = file_path;
    }

    /// Gets the current source file context
    pub fn current_file(&self) -> Option<&str> {
        self.current_file.as_deref()
    }

    pub fn enter_scope(&mut self) -> usize {
        let parent = self.current_scope;
        let scope_id = self.scopes.len();
        self.scopes.push(Scope::new(Some(parent)));
        self.scopes[parent].children.push(scope_id);
        self.current_scope = scope_id;
        scope_id
    }

    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.scopes[self.current_scope].parent {
            self.current_scope = parent;
        }
    }

    /// Inserts a symbol into the current scope.
    ///
    /// # Errors
    ///
    /// Returns an error if a symbol with the same name already exists in the current scope.
    pub fn insert(&mut self, name: String, symbol: Symbol) -> Result<(), String> {
        let scope = &mut self.scopes[self.current_scope];
        if scope.symbols.contains_key(&name) {
            return Err(format!("Symbol '{}' already defined in this scope", name));
        }
        scope.symbols.insert(name, symbol);
        Ok(())
    }

    /// Adds an import to the current scope
    pub fn add_import(&mut self, path: String, is_recursive: bool) {
        let is_namespace = path.ends_with("::*") || path.ends_with("::**");
        let import = Import {
            path,
            is_recursive,
            is_namespace,
        };
        self.scopes[self.current_scope].imports.push(import);
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        let mut current = self.current_scope;
        loop {
            // First check local symbols
            if let Some(symbol) = self.scopes[current].symbols.get(name) {
                // If it's an alias, resolve it
                if let Symbol::Alias { target, .. } = symbol {
                    return self.lookup(target);
                }
                return Some(symbol);
            }

            // Then check imported namespaces
            if let Some(symbol) = self.lookup_in_imports(name, current) {
                // If it's an alias, resolve it
                if let Symbol::Alias { target, .. } = symbol {
                    return self.lookup(target);
                }
                return Some(symbol);
            }

            match self.scopes[current].parent {
                Some(parent) => current = parent,
                None => return None,
            }
        }
    }

    /// Looks up a symbol in imported namespaces from a given scope
    fn lookup_in_imports(&self, name: &str, scope_id: usize) -> Option<&Symbol> {
        for import in &self.scopes[scope_id].imports {
            if import.is_namespace {
                // For namespace imports (::*), strip the ::* and look for name in that namespace
                let namespace = import.path.trim_end_matches("::*").trim_end_matches("::**");
                let qualified = format!("{}::{}", namespace, name);

                // Look through all scopes for the qualified name
                for scope in &self.scopes {
                    if let Some(symbol) = scope.symbols.get(&qualified) {
                        return Some(symbol);
                    }
                }

                // If recursive, also check nested namespaces
                if import.is_recursive
                    && let Some(symbol) = self.lookup_recursive_import(name, namespace)
                {
                    return Some(symbol);
                }
            } else {
                // For member imports, look for the exact path
                for scope in &self.scopes {
                    if let Some(symbol) = scope.symbols.get(&import.path) {
                        // Check if the name matches the last component of the path
                        if import.path.ends_with(&format!("::{}", name)) || import.path == name {
                            return Some(symbol);
                        }
                    }
                }
            }
        }
        None
    }

    /// Recursively searches for a symbol in nested namespaces
    fn lookup_recursive_import(&self, name: &str, namespace: &str) -> Option<&Symbol> {
        // Look for name in any sub-namespace of the given namespace
        let prefix = format!("{}::", namespace);
        for scope in &self.scopes {
            for (qualified_name, symbol) in &scope.symbols {
                if qualified_name.starts_with(&prefix)
                    && qualified_name.ends_with(&format!("::{}", name))
                {
                    return Some(symbol);
                }
            }
        }
        None
    }

    pub fn lookup_from_scope(&self, name: &str, scope_id: usize) -> Option<&Symbol> {
        let mut current = scope_id;
        loop {
            if let Some(symbol) = self.scopes[current].symbols.get(name) {
                return Some(symbol);
            }
            match self.scopes[current].parent {
                Some(parent) => current = parent,
                None => return None,
            }
        }
    }

    pub fn lookup_local(&self, name: &str) -> Option<&Symbol> {
        self.scopes[self.current_scope].symbols.get(name)
    }

    pub fn current_scope_id(&self) -> usize {
        self.current_scope
    }

    pub fn symbols_in_scope(&self, scope_id: usize) -> Option<&HashMap<String, Symbol>> {
        self.scopes.get(scope_id).map(|s| &s.symbols)
    }

    pub fn all_symbols(&self) -> Vec<(&String, &Symbol)> {
        self.scopes
            .iter()
            .flat_map(|scope| scope.symbols.iter())
            .collect()
    }

    /// Looks up a symbol by its qualified name across all scopes
    pub fn lookup_qualified(&self, qualified_name: &str) -> Option<&Symbol> {
        for scope in &self.scopes {
            for symbol in scope.symbols.values() {
                if symbol.qualified_name() == qualified_name {
                    return Some(symbol);
                }
            }
        }
        None
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "symbol_table/tests.rs"]
mod tests;
