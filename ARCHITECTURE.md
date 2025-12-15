# Syster Architecture

**Three-Phase Pipeline:** Parse (Pest) → Syntax (AST) → Semantic (Symbols + Graphs)

## Project Structure

Syster is a Cargo workspace with three crates:

- **syster-base** (`crates/syster-base/`) - Core library with parser, AST, semantic analysis
- **syster-cli** (`crates/syster-cli/`) - Command-line tool for file analysis
- **syster-lsp** (`crates/syster-lsp/`) - Language Server Protocol implementation (in progress)

All development work happens in `syster-base`. CLI and LSP are thin wrappers around the core library.

## Critical Rules

### NEVER Mix Phases
- **Parse:** Grammar only, no semantic logic
- **Syntax:** AST construction only, no cross-file knowledge  
- **Semantic:** All cross-file resolution and validation

### Symbol Table is Global
- AST nodes don't contain resolved references
- All symbols stored in centralized `SymbolTable`
- Use `QualifiedName` (e.g., `"Package::Element"`) for cross-references
- **Don't add back-references to AST nodes**

### Relationships Use Separate Graphs
- Don't store specialization/typing in Symbol enum
- Use `RelationshipGraph` for all relationships
- Enables efficient queries and cycle detection

### Import Resolution: Three Passes
1. **Namespace imports** (`Package::*`) - order-independent
2. **Member imports** (`Package::Member`) - may depend on #1  
3. **Recursive imports** (`Package::*::**`) - requires fully populated namespaces

**Why three passes?** Dependencies between import types require ordering.

## Module Organization

```
crates/
├── syster-base/         # Core library
│   ├── src/
│   │   ├── parser/      # Pest grammars (kerml.pest, sysml.pest)
│   │   ├── language/    # AST definitions (KerML, SysML)
│   │   │   ├── kerml/syntax/
│   │   │   └── sysml/
│   │   │       ├── syntax/      # AST nodes
│   │   │       └── populator.rs # Symbol table population
│   │   ├── semantic/    # Cross-file analysis
│   │   │   ├── symbol_table.rs  # Global symbol registry
│   │   │   ├── graph.rs         # Relationship graphs
│   │   │   ├── resolver.rs      # Name resolution (qualified names)
│   │   │   ├── analyzer.rs      # Validation passes
│   │   │   └── workspace.rs     # Multi-file coordination
│   │   └── project/     # File loading
│   │       ├── workspace_loader.rs  # Load user files on demand
│   │       └── stdlib_loader.rs     # Load standard library
│   └── sysml.library/   # Standard library files
├── syster-cli/          # Command-line tool
│   ├── src/
│   │   ├── main.rs      # CLI argument parsing
│   │   └── lib.rs       # Testable analysis logic
│   └── tests/
│       └── cli_tests.rs # Integration tests (14 tests)
└── syster-lsp/          # LSP server (in progress)
    └── src/
        └── main.rs      # Server stub
```

## Adding Features

### New SysML Element (e.g., `concern def`)

1. **Grammar:** `src/parser/sysml.pest`
2. **AST:** `src/language/sysml/syntax/types.rs`
3. **Enum:** Add to `Definition` enum in `syntax/enums.rs`
4. **Populator:** `src/language/sysml/populator.rs`
5. **Tests:** `tests/semantic/sysml_parsing_tests.rs`

### New Semantic Check

1. **Error Kind:** `src/semantic/error.rs`
2. **Check Method:** `src/semantic/analyzer.rs`
3. **Call from `analyze()`**
4. **Tests:** `tests/semantic/mod.rs`

## Key Types

```rust
pub type QualifiedName = String;     // "Package::Element"
pub type SimpleName = String;        // "Element"  
pub type ScopeId = usize;           // Scope identifier
pub type SourceFilePath = String;   // File path
```

## Symbol & Relationship Types

**Symbols:** Package, Classifier, Feature, Definition, Usage, Alias

**Relationships:** specializations (IS-A), typing (INSTANCE-OF), subsetting (REFINES), redefinitions (OVERRIDES), satisfactions (FULFILLS)

## Common Pitfalls

❌ **Don't:** Add semantic logic to AST nodes  
✅ **Do:** Keep AST immutable, use SymbolTable

❌ **Don't:** Resolve imports during symbol table population  
✅ **Do:** Build table first, then run three-pass import resolution

❌ **Don't:** Store relationships in Symbol enum  
✅ **Do:** Use RelationshipGraph

❌ **Don't:** Try single-pass import resolution  
✅ **Do:** Use three passes (namespace → member → recursive)

## Quick Reference

**Import Types:**
- `import Package::*` - All members
- `import Package::Member` - Specific member  
- `import Package::*::**` - All nested members
- `import X as Y` - Aliased

**Qualified Name:** `Package::Class::Feature` (unique across files)

---

**For detailed guides:** [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md) · [docs/SYSML_PRIMER.md](docs/SYSML_PRIMER.md)
