# TODO: Proper Adapter/Semantic Separation

## Phase 1: Create Semantic Type System ✅ COMPLETE
- [x] Create `semantic/types/semantic_role.rs` with enum (Requirement, Action, State, UseCase, Component, Interface, Unknown)
- [x] Add `semantic_role: Option<SemanticRole>` field to `Symbol::Definition` and `Symbol::Usage`
- [x] Export SemanticRole from `semantic/types/mod.rs`

## Phase 2: Update Adapter to Map Semantic Roles ✅ COMPLETE
- [x] In `semantic/adapters/sysml/helpers.rs`: Add `definition_kind_to_semantic_role()` and `usage_kind_to_semantic_role()`
- [x] Convert directly from AST enums (DefinitionKind, UsageKind) to SemanticRole (eliminated wasteful string conversion)
- [x] In `semantic/adapters/sysml/visitors.rs`: When creating Definition/Usage symbols, call mapping function and set `semantic_role` field
- [x] Adapter now translates language → semantic during population
- [x] Fixed all test Symbol creations to include semantic_role field
- [x] All 304 unit tests + 1441 integration tests passing
- [x] Created adapter factory pattern - workspace no longer knows about specific adapter types
- [x] Workspace is now language-agnostic (only uses SyntaxFile abstraction, delegates to adapters module)

## Phase 3: Make Validator Truly Generic
- [ ] Update `semantic/analyzer/validation/relationship_validator.rs` trait to work with `SemanticRole` instead of string matching
- [ ] Delete `semantic/analyzer/validation/sysml_validator.rs` entirely
- [ ] Delete `semantic/analyzer/validation/sysml/` directory
- [ ] Create generic validator implementation `SemanticRelationshipValidator` that checks semantic rules using roles

## Phase 4: Move Language-Specific Validation to Adapter
- [ ] Create `semantic/adapters/sysml/validator.rs` for SysML-specific constraint checks during adaptation
- [ ] Validates AST before creating Symbols (syntax errors, SysML-specific naming rules)
- [ ] Adapter uses this validator during `populate()` method
- [ ] Return validation errors as part of adaptation process

## Phase 5: Update All References
- [ ] Find and update all imports of `SysMLRelationshipValidator` to use generic `SemanticRelationshipValidator`
- [ ] Find and update all imports of `SymbolTablePopulator` to use `SysmlAdapter`
- [ ] Update `semantic/workspace/populator.rs` to use `semantic::adapters::SysmlAdapter`
- [ ] Update analyzer to use new validator
- [ ] Fix all tests to use semantic roles
- [ ] Update `semantic/mod.rs` exports
- [ ] Update `semantic/analyzer.rs` exports

## Phase 6: Add Architecture Test
- [ ] Add test in `tests/architecture_tests.rs`: `test_semantic_layer_only_adapters_import_syntax()`
- [ ] Check that only files in `semantic/adapters/` import from `syntax::sysml` or `syntax::kerml`
- [ ] All other semantic files must NOT import from syntax layer

## Phase 7: Build and Test
- [ ] Run `cargo build` to check compilation
- [ ] Run `cargo test` to verify all tests pass
- [ ] Run architecture tests to verify violations reduced
- [ ] Fix any remaining import issues

## Phase 8: Documentation
- [ ] Update comments in `semantic/adapters/mod.rs` explaining this is the ONLY place that imports syntax
- [ ] Add architectural notes showing: Syntax → Adapter → Semantic (with roles) → Analyzer → Validation

---

## Key Principle
- **Adapter** = Language-aware (imports from syntax)
- **Semantic** = Language-agnostic (works with semantic roles)
- **Validator** = Constraint checker (uses semantic roles only)

## Current State
- ✅ Phase 1 COMPLETE: SemanticRole enum created with 20+ role types
- ✅ Phase 2 COMPLETE: Adapter maps SysML kinds → SemanticRole during population
- ✅ All 304 unit tests passing
- ✅ All 1441 integration tests passing
- ⚠️  Architecture violations: 57 in semantic layer (down from 102)
  - Main issue: `sysml_validator.rs` still imports `syntax::sysml::ast::constants`
  - Solution: Phase 3 - make validator generic using SemanticRole

## Next Steps
**Phase 3** is the critical next step to eliminate the validator importing from syntax!
- ❌ sysml_validator.rs still imports syntax constants (needs semantic roles)
- ❌ Symbol has `kind: String` instead of semantic type
- ❌ Old imports still need updating
