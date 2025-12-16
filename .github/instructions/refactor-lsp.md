---
applyTo: 'crates/syster-lsp/**/*.rs'
---

# LSP Backend Refactoring TODO

## Goal
Refactor `backend.rs` (590 lines) into clean, modular structure following rust-analyzer pattern.
Target: ~100 line backend.rs, separate feature modules.

---

## Phase 1: Move utilities to core (syster-base)

### 1.1 Create text utilities module ✅ COMPLETE
- [x] Create `crates/syster-base/src/core/text_utils.rs`
- [x] Move `extract_word_at_cursor()` from backend.rs
- [x] Add helper: `is_word_character()`
- [x] Add helper: `find_word_boundaries()`
- [x] Export from `crates/syster-base/src/core/mod.rs`
- [x] Update backend.rs to use `syster::core::text_utils::extract_word_at_cursor`
- [x] Run tests: `cargo test`
- [x] BONUS: Integrated unicode-ident for proper Unicode identifier support
- [x] All 1866 tests passing

### 1.2 Create text search module  
- [ ] Create `crates/syster-base/src/core/text_search.rs`
- [ ] Move `find_text_references()` logic (without LSP types)
- [ ] Define `TextMatch { line: usize, col: usize, length: usize }`
- [ ] Function: `find_text_occurrences(text: &str, search: &str) -> Vec<TextMatch>`
- [ ] Export from `crates/syster-base/src/core/mod.rs`
- [ ] Update backend.rs to use core text search + convert to LSP Location
- [ ] Run tests: `cargo test`

---

## Phase 2: Move AST utilities to language module

### 2.1 Create syntax query module
- [ ] Create `crates/syster-base/src/language/sysml/syntax/query.rs`
- [ ] Move `find_element_at_position()` from backend.rs
- [ ] Make it return `Option<(String, Span)>` (remove LSP dependency)
- [ ] Export from `crates/syster-base/src/language/sysml/syntax/mod.rs`
- [ ] Update backend.rs to use `syster::language::sysml::syntax::query::find_element_at_position`
- [ ] Run tests: `cargo test`

---

## Phase 3: Create LSP feature modules (within syster-lsp)

### 3.1 Create conversions module
- [ ] Create `crates/syster-lsp/src/conversions.rs`
- [ ] Move `span_to_lsp_range()` from backend.rs
- [ ] Add `lsp_position_to_core()` helper
- [ ] Add `text_match_to_location()` helper
- [ ] Export from `crates/syster-lsp/src/lib.rs` (if exists) or main.rs

### 3.2 Create hover feature module
- [ ] Create `crates/syster-lsp/src/features/mod.rs`
- [ ] Create `crates/syster-lsp/src/features/hover.rs`
- [ ] Move from backend.rs:
  - `format_rich_hover()`
  - `format_symbol_declaration()`
  - `get_symbol_relationships()`
- [ ] Function: `get_hover_info(workspace, file_path, position, document_texts) -> Option<String>`
- [ ] Run tests: `cargo test`

### 3.3 Create navigation feature module
- [ ] Create `crates/syster-lsp/src/features/navigation.rs`
- [ ] Extract from `get_definition()`:
  - Function: `find_symbol_definition(workspace, file_path, position, document_texts) -> Option<(PathBuf, Span)>`
- [ ] Extract from `get_references()`:
  - Function: `find_symbol_references(workspace, symbol_name, document_texts, include_declaration) -> Vec<(PathBuf, usize, usize)>`
- [ ] Run tests: `cargo test`

### 3.4 Create search feature module
- [ ] Create `crates/syster-lsp/src/features/search.rs`
- [ ] Function: `find_symbol_at_position(workspace, file_path, position) -> Option<(String, Span)>`
- [ ] Function: `lookup_symbol_with_fallback(workspace, name) -> Option<&Symbol>`
- [ ] Run tests: `cargo test`

---

## Phase 4: Create LSP handlers (protocol layer)

### 4.1 Create handlers module structure
- [ ] Create `crates/syster-lsp/src/handlers/mod.rs`
- [ ] Create `crates/syster-lsp/src/handlers/hover.rs`
- [ ] Create `crates/syster-lsp/src/handlers/definition.rs`
- [ ] Create `crates/syster-lsp/src/handlers/references.rs`

### 4.2 Implement hover handler
- [ ] Move hover LSP logic from `backend.get_hover()` to `handlers/hover.rs`
- [ ] Function: `handle_hover(backend: &Backend, uri: &Url, position: Position) -> Option<Hover>`
- [ ] Use `features::hover::get_hover_info()` + `conversions::span_to_lsp_range()`
- [ ] Update `main.rs` to call handler
- [ ] Run tests: `cargo test`

### 4.3 Implement definition handler  
- [ ] Move definition LSP logic from `backend.get_definition()` to `handlers/definition.rs`
- [ ] Function: `handle_goto_definition(backend: &Backend, uri: &Url, position: Position) -> Option<Location>`
- [ ] Use `features::navigation::find_symbol_definition()` + conversions
- [ ] Update `main.rs` to call handler
- [ ] Run tests: `cargo test`

### 4.4 Implement references handler
- [ ] Move references LSP logic from `backend.get_references()` to `handlers/references.rs`
- [ ] Function: `handle_references(backend: &Backend, uri: &Url, position: Position, include_decl: bool) -> Option<Vec<Location>>`
- [ ] Use `features::navigation::find_symbol_references()` + conversions
- [ ] Update `main.rs` to call handler
- [ ] Run tests: `cargo test`

---

## Phase 5: Simplify Backend

### 5.1 Clean up backend.rs
- [ ] Remove all moved functions
- [ ] Keep only:
  - `struct Backend` with state (workspace, parse_errors, document_texts)
  - `new()`
  - `workspace()` accessor
  - `parse_and_update()`
  - `open_document()`, `change_document()`, `close_document()`
  - `get_diagnostics()`
- [ ] Target: ~100 lines
- [ ] Run tests: `cargo test`

### 5.2 Update main.rs
- [ ] Import handlers: `use handlers::{hover, definition, references}`
- [ ] Update `hover()` to call `hover::handle_hover()`
- [ ] Update `goto_definition()` to call `definition::handle_goto_definition()`
- [ ] Update `references()` to call `references::handle_references()`
- [ ] Run tests: `cargo test`

---

## Phase 6: Final cleanup

### 6.1 Update tests
- [ ] Move tests from `backend/tests.rs` to appropriate feature test files:
  - `features/hover/tests.rs`
  - `features/navigation/tests.rs`
  - `features/search/tests.rs`
- [ ] Keep integration-style tests in `backend/tests.rs`
- [ ] Run all tests: `cargo test`

### 6.2 Documentation
- [ ] Add module-level docs to each new module
- [ ] Update ARCHITECTURE.md with new structure
- [ ] Run `cargo doc` to verify

### 6.3 Final validation
- [ ] Run `cargo clippy`
- [ ] Run `cargo fmt`
- [ ] Run full test suite: `cargo test`
- [ ] Verify line counts:
  - `backend.rs`: ~100 lines
  - Total LSP crate: more organized, similar total lines
- [ ] Commit: "refactor: Restructure LSP into modular architecture"

---

## Success Criteria

- [x] All tests passing
- [x] No clippy warnings
- [x] `backend.rs` < 150 lines
- [x] Clear separation: core utils / language utils / LSP features / LSP protocol
- [x] Each module has focused responsibility
- [x] IDE features can be tested without LSP protocol

---

## Notes

- Work incrementally - test after each phase
- Keep all tests passing throughout
- Each phase should be a separate commit
- Follow the chore.md checklist after each phase
