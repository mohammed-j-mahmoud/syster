# Future Work


---

## LSP Feature Implementation (Priority Order)

### In Progress / Next Tasks
- [ ] **CRITICAL BUG: Duplicate symbol definitions from qualified redefinitions** - The semantic visitor incorrectly treats qualified names in redefinitions (e.g., `ref item :>> Shell::edges::vertices`) as new symbol definitions, causing duplicate definition errors. In ShapeItems.sysml, both "Shell" and "Disc" are reported as duplicates multiple times despite each being defined only once. The visitor needs to distinguish between symbol references in qualified names vs actual symbol definitions.
- [ ] **Fix architecture tests** - Add proper assertions to architecture violation summary test instead of just printing. The test should fail when violations are found.
- [x] **CRITICAL BUG FIX: Stdlib cross-file reference resolution** - Fixed parser not capturing `abstract` flag from `abstract attribute def` declarations. Added `extract_definition_flags()` to properly extract abstract and variation markers. Fixed `populate_all()` to continue processing files even when one fails, preventing early exit that blocked MeasurementReferences.sysml and other files from loading.
- [ ] Add tests for KerML visitor
- [ ] Add tests for semantic tokens support
- [ ] Do a code coverage analysis of missing tests
- [ ] Remove logic from tests (match/if/else statements)
- [ ] Scan folder to see cross cutting concerns
- [ ] Refactor semantic tokens
- [ ] Refactor LSP components
- [ ] Clean up floating tests

### Recently Completed
- [x] **CRITICAL BUG FIX: LSP message format** - Fixed tracing_subscriber writing to stdout which corrupted LSP protocol messages. LSP uses stdin/stdout for JSON-RPC, so all logging must go through client.log_message() instead.

### Architecture Notes
- **Reusable patterns:**
  - `extract_word_at_cursor()` - Used in: go-to-def, find-refs, semantic-tokens
  - `find_symbol_at_position()` - Used in: hover, go-to-def, find-refs
  - Symbol lookup fallback (qualified → simple → all_symbols) - Used in: go-to-def, find-refs
  - Main files ~10-20 lines with focused submodules handling specific concerns.

## Event System
- [ ] Event batching for bulk operations
- [ ] Event replay/history for debugging
- [ ] Async event handlers (tokio/async-std)
- [ ] Priority-based listener ordering

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
- [ ] **Test Organization & Separation of Concerns**
  - Review test files for proper organization (unit vs integration vs end-to-end)
  - Separate test helpers from test code (extract common test utilities)
  - Move integration tests to tests/ directory where appropriate
  - Ensure tests follow same modularization pattern as main code
  - Create test fixtures/builders for complex test data setup
  - Review workspace/tests.rs (934 lines) - consider splitting by feature area
  - Extract common test patterns (e.g., unwrap_sysml helper, parse_sysml helper)
  
## Architecture & Code Cleanup
### Next Module Refactoring Tasks
- [ ] **lsp/ folder** (lsp-server crate) - Review files for refactoring opportunities
- Check file sizes and identify files >100 lines
- Apply same modularization pattern as semantic/
- [ ] Metrics/observability layer for EventEmitter

### Code Cleanup
- [ ] Replace hardcoded strings in `language/sysml/populator.rs` with SYSML_KIND_* constants
- [ ] Create relationship type constants (RELATIONSHIP_SATISFY, RELATIONSHIP_PERFORM, etc.)
- [ ] Extract `is_abstract` and `is_variation` from definition_prefix in AST
- [ ] Add annotation properties to KerML types