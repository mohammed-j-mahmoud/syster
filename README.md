# Syster

**Status: Alpha** - Active development, APIs may change

A Rust-based parser and tooling for SysML v2 (Systems Modeling Language) and KerML (Kernel Modeling Language).

## Overview

Syster provides a comprehensive implementation of parsers and Abstract Syntax Trees (AST) for both SysML v2 and its foundation language KerML. The project follows a three-layer architecture:

1. **Parse Layer** - Grammar-based parsing using Pest
2. **Syntax Layer** - Language-specific Abstract Syntax Trees (AST)
3. **Model Layer** - Semantic model representation (planned)

## Features

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
