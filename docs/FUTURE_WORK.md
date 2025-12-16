# Future Work

## LSP Feature Implementation (Priority Order)

### In Progress / Next Tasks
  - [ ] Add `get_document_symbols()` method to Backend
  - [ ] Extract all symbols from a file with hierarchy
  - [ ] Map Symbol types to LSP SymbolKind (Package → Namespace, Definition → Class, etc.)
  - [ ] Include symbol ranges for navigation
  - [ ] Wire up `textDocument/documentSymbol` LSP handler
  - [ ] Tests: flat file, nested packages, definitions with members

- [ ] **Task 13: Semantic Tokens** (Syntax highlighting)
  - [ ] Add `get_semantic_tokens()` method to Backend
  - [ ] Walk AST and classify each token (keyword, type, variable, etc.)
  - [ ] Use `extract_word_at_cursor()` pattern for precise token ranges
  - [ ] Map to LSP semantic token types and modifiers
  - [ ] Wire up `textDocument/semanticTokens/full` LSP handler
  - [ ] Tests: keywords, types, variables, relationships

- [ ] **Task 14: Code Completion**
  - [ ] Add `get_completions()` method to Backend
  - [ ] Detect completion context (after `def`, after `:`, etc.)
  - [ ] Query symbol table for available symbols in scope
  - [ ] Include relationship keywords (specializes, subsets, etc.)
  - [ ] Wire up `textDocument/completion` LSP handler
  - [ ] Tests: keyword completion, type completion, member completion

- [ ] **Task 15: Rename Symbol**
  - [ ] Add `get_rename_edits()` method to Backend
  - [ ] Find all references to symbol (reuse Task 11 logic)
  - [ ] Generate WorkspaceEdit with all locations
  - [ ] Update qualified names in symbol table
  - [ ] Wire up `textDocument/rename` LSP handler
  - [ ] Tests: rename definition, rename usage, cross-file rename

### Architecture Notes
- **Reusable patterns:**
  - `extract_word_at_cursor()` - Used in: go-to-def, find-refs, semantic-tokens
  - `find_symbol_at_position()` - Used in: hover, go-to-def, find-refs
  - Symbol lookup fallback (qualified → simple → all_symbols) - Used in: go-to-def, find-refs

## Event System
- [ ] Event batching for bulk operations
- [ ] Event replay/history for debugging
- [ ] Async event handlers (tokio/async-std)
- [ ] Priority-based listener ordering

## Architecture & Code Organization

### Module Refactoring (Priority Order)
1. [ ] **symbol_table.rs (566 lines)** → Split into `symbol_table/` folder
   - Extract: symbol operations, scope management, lookup logic
   - Keep main file as module exports only
   
2. [ ] **analyzer.rs (345 lines)** → Split into `analyzer/` folder
   - Extract: validation rules, analysis passes, helper functions
   - Keep main file as module exports only

3. [ ] **error.rs (194 lines)** → Split into `error/` folder
   - Extract: error types by category (parse, semantic, resolution, etc.)
   - Keep main file as module exports only

4. [ ] **resolver.rs (180 lines)** → Split into `resolver/` folder
   - Extract: resolution strategies, helper logic
   - Keep main file as module exports only

5. [ ] **events.rs (171 lines)** → Consider splitting if grows
   - Could split into: workspace_events.rs, dependency_events.rs, symbol_table_events.rs
   - Currently borderline - revisit if exceeds 200 lines
- [ ] Metrics/observability layer for EventEmitter

## LSP Features
- [ ] Incremental symbol resolution (fine-grained updates)
- [ ] Workspace-wide event aggregation
- [ ] Snapshot/restore state for crash recovery

## Performance
- [ ] Parallel file population with Rayon
- [ ] Specialized symbol index (trie/inverted index)

## Testing & Quality
- [ ] Property-based testing with proptest
- [ ] Benchmark suite with criterion
- [ ] 100% public API documentation coverage

## Code Cleanup
- [ ] Replace hardcoded strings in `language/sysml/populator.rs` with SYSML_KIND_* constants
- [ ] Create relationship type constants (RELATIONSHIP_SATISFY, RELATIONSHIP_PERFORM, etc.)
- [ ] Extract `is_abstract` and `is_variation` from definition_prefix in AST
- [ ] Add KerML parser support in file_loader
- [ ] Add annotation properties to KerML types
