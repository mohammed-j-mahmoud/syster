---
applyTo: '**/*.rs'
---

# Pre-Commit Checklist

This checklist MUST be completed after every task/todo before committing changes.

## Code Quality Checks

- [ ] Remove unnecessary documentation from each changed file
  - Delete redundant comments that don't add value
  - Keep only essential API docs and complex logic explanations
  
- [ ] Remove unused methods (unless marked with TODO prefix)
  - Search for `#[allow(dead_code)]` warnings
  - Delete or mark for future use with `// TODO: ...`
  
- [ ] Move tests into their own test files
  - Inline tests → `<module>/tests.rs`
  - Keep test modules focused and organized

## Test Quality Checks

- [ ] Add missing tests for each changed file
  - Every public function should have at least one test
  - Cover edge cases and error paths
  
- [ ] Remove if-else match logic from tests
  - Use `let-else` pattern: `let Some(x) = y else { panic!("...") };`
  - Use `assert_matches!` macro where appropriate
  
- [ ] Make tests more concrete
  - Replace `assert!(x >= 5)` with `assert_eq!(x, 5)`
  - Use exact values instead of ranges when possible
  - Make assertions specific and meaningful

## Final Checks

- [ ] Address any TODOs added during the task
  - Either implement them or file as future work
  - Document why TODOs remain if not addressed
  
- [ ] Clean up any temporary notes or debug code
  - Remove `println!` debug statements
  - Remove commented-out code blocks
  - Clean up experimental code

## Repeat Process

Iterate through this checklist until no further changes are needed.

## Validation

After completing checklist:
```bash
cargo clippy --fix --allow-dirty
cargo test
git add -A && git commit -m "..."
```

