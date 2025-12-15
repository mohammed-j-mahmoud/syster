# Syster

**Status: Alpha** - Active development, APIs may change

A Rust-based parser and tooling for SysML v2 (Systems Modeling Language) and KerML (Kernel Modeling Language).

## Documentation

- **[ARCHITECTURE.md](ARCHITECTURE.md)** ⭐ Core patterns and rules (start here)
- **[docs/CONTRIBUTING.md](docs/CONTRIBUTING.md)** - Development workflow
- **[docs/SYSML_PRIMER.md](docs/SYSML_PRIMER.md)** - SysML v2 concepts for humans

## Overview

Syster provides a comprehensive implementation of parsers and Abstract Syntax Trees (AST) for both SysML v2 and its foundation language KerML. The project follows a three-layer architecture:

1. **Parse Layer** - Grammar-based parsing using Pest
2. **Syntax Layer** - Language-specific Abstract Syntax Trees (AST)
3. **Model Layer** - Semantic model representation (planned)

## Features

### Semantic Analysis
- **Symbol Table**: Cross-file symbol resolution with qualified names
- **Import Resolution**: 
  - Namespace imports (`import Package::*`)
  - Member imports (`import Package::Member`)
  - Recursive imports (`import Package::*::**`)
  - Alias support (`alias NewName for Target`)
- **Relationship Graph**: Tracks specialization, typing, subsetting, redefinition, and satisfaction
- **Workspace Management**: Multi-file projects with shared symbol table
- **Standard Library Support**: Optional stdlib loading via `Workspace::with_stdlib()`

### KerML Support
- Complete grammar implementation for Kernel Modeling Language
- AST representations for all classifier types:
  - Type, Classifier, DataType, Class, Structure, Behavior, Function
  - Association, AssociationStructure, Metaclass
- Feature support with modifiers (readonly, derived, abstract)
- Direction qualifiers (in, out, inout)
- Package and namespace management
- Import and annotation support
- Comprehensive test coverage (28 tests)

### SysML v2 Support
- Grammar implementation for Systems Modeling Language v2
- AST representations for all definition types:
  - Part, Port, Action, Item, Attribute, Requirement
  - Concern, Case, AnalysisCase, VerificationCase, UseCase
  - View, Viewpoint, Rendering
- Usage types for all major elements
- Package, comment, and import support
- Visitor pattern for AST traversal
- Comprehensive test coverage (29 tests)

## Architecture

```
src/
├── core/                    # Shared infrastructure
│   ├── traits.rs           # AstNode, Named, ToSource traits
│   └── visitor.rs          # Visitor pattern implementation
├── language/
│   ├── kerml/
│   │   ├── syntax/         # KerML-specific AST
│   │   │   ├── ast.rs      # FromPest implementations
│   │   │   ├── types.rs    # Type definitions
│   │   │   ├── enums.rs    # Enum types
│   │   │   └── tests.rs    # Unit tests
│   │   └── model/          # Semantic model (planned)
│   └── sysml/
│       ├── syntax/         # SysML-specific AST
│       │   ├── ast.rs      # FromPest implementations
│       │   ├── types.rs    # Type definitions
│       │   ├── enums.rs    # Enum types
│       │   └── tests.rs    # Unit tests
│       └── model/          # Semantic model (planned)
└── parser/
    ├── kerml.pest          # KerML grammar
    └── sysml.pest          # SysML grammar
```

## Building

```bash
cargo build
```

## Usage

### Basic Usage

```rust
use syster::parser::{SysMLParser, sysml::Rule};
use syster::language::sysml::syntax::SysMLFile;
use syster::semantic::Workspace;
use from_pest::FromPest;
use pest::Parser;

// Parse a SysML file
let source = r#"
    package MyPackage {
        part def Vehicle;
    }
"#;

let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
let file = SysMLFile::from_pest(&mut pairs).unwrap();

// Create a workspace (without stdlib for tests, with stdlib for real projects)
let mut workspace = Workspace::new();
workspace.add_file("example.sysml".into(), file);
workspace.populate_all().unwrap();

// Look up symbols
let vehicle = workspace.symbol_table().lookup_qualified("MyPackage::Vehicle");
```

### Using the Standard Library

For real projects, load the standard library once at startup, then load project files on demand:

```rust
use syster::semantic::Workspace;
use syster::project::{StdLibLoader, WorkspaceLoader};

// Create workspace
let mut workspace = Workspace::new();

// Load standard library once at startup (from sysml.library/)
let stdlib_loader = StdLibLoader::new();
stdlib_loader.load(&mut workspace).unwrap();

// Load project files on demand
let workspace_loader = WorkspaceLoader::new();
workspace_loader.load_file("my_model.sysml", &mut workspace).unwrap();
// Or load an entire directory
workspace_loader.load_directory("src/models", &mut workspace).unwrap();

// Populate symbols and resolve references
workspace.populate_all().unwrap();

// Now query the symbol table
let symbol = workspace.symbol_table().lookup_qualified("MyPackage::MyElement");
```

**StdLibLoader** - Load once at startup from `sysml.library/`  
**WorkspaceLoader** - Load project files on demand as they're opened or referenced

### Working with Imports

```rust
let source = r#"
    package Base {
        part def Vehicle;
    }
    
    package Derived {
        // Namespace import - imports all members
        public import Base::*;
        
        // Member import - imports specific member
        import Base::Vehicle;
        
        // Use imported types
        part myCar : Vehicle;
    }
"#;
```

## Testing

Run all tests:
```bash
cargo test
```

Run tests for specific language:
```bash
cargo test language::kerml::syntax::tests
cargo test language::sysml::syntax::tests
```

## Development

### Key Design Patterns

1. **Language-Specific AST**: Each language (KerML, SysML) has its own syntax module with specific node types
2. **Shared Core**: Common traits and patterns are defined in `src/core/`
3. **Macro-Based FromPest**: Uses `impl_from_pest!` macro to reduce boilerplate in CST→AST conversion
4. **Recursive Name Finding**: Handles various grammar patterns for extracting identifications

### Grammar Rules

- Grammar rules use named markers (e.g., `abstract_marker`) instead of string literals to expose them in the parse tree
- This allows AST layer to properly extract flags like `is_abstract`, `is_readonly`, etc.

## Contributing

When adding new AST node types:

1. Define the type in `types.rs`
2. Add enum variants in `enums.rs` if needed
3. Implement `FromPest` in `ast.rs` using the `impl_from_pest!` macro
4. Add comprehensive unit tests in `tests.rs`
5. Update the `Element` enum to handle the new type

## License

[Add license information]

## References

- [SysML v2 Specification](https://www.omg.org/spec/SysML)
- [Pest Parser](https://pest.rs/)
