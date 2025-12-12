---
applyTo: '**/*.rs'
---

# Rust Development Instructions

## Test-Driven Development (TDD) - MANDATORY

1. **Always write tests first** before implementing functionality
   - Write a failing test that describes the desired behavior
   - Run the test to confirm it fails for the right reason
   - Implement the minimal code to make the test pass
   - Refactor if needed while keeping tests green

2. **Test execution is required**
   - After writing a test, you MUST run it using `runTests` tool or `cargo test`
   - Never proceed to implementation without seeing the test fail first
   - Never claim completion without running tests to verify success

## Incremental Development - STRICT LIMITS

1. **One function at a time**
   - Edit only ONE function per change cycle
   - Complete the full TDD cycle (test → implement → verify) for that function
   - Do not move to the next function until current one is complete and tested

2. **Small, focused changes**
   - Each change should be minimal and focused
   - If a change requires modifying multiple functions, STOP and break it down
   - Prefer multiple small commits over one large change

3. **Restart trigger**
   - If you find yourself making changes across multiple files simultaneously, STOP
   - If you're modifying more than ~10-15 lines in a single function, STOP and reconsider
   - If the scope is growing beyond the original small task, STOP and restart with a smaller goal
   - Ask the user to break down the task into smaller pieces if needed

## Rust Best Practices

1. **Error handling**
   - Use `Result<T, E>` for fallible operations
   - Use `Option<T>` for values that may or may not exist
   - Avoid `.unwrap()` and `.expect()` in production code (tests are okay)
   - Propagate errors with `?` operator when appropriate

2. **Ownership and borrowing**
   - Prefer borrowing (`&T`, `&mut T`) over transferring ownership
   - Use `.clone()` judiciously - only when necessary
   - Leverage the borrow checker to ensure memory safety

3. **Idiomatic Rust**
   - Use iterator methods (`.map()`, `.filter()`, `.collect()`) over explicit loops when clearer
   - Prefer pattern matching over if-let chains
   - Use `derive` macros for common traits (Debug, Clone, PartialEq, etc.)
   - Follow Rust naming conventions (snake_case for functions/variables, CamelCase for types)

4. **Code organization**
   - Keep modules focused and cohesive
   - Use `mod.rs` or module files appropriately
   - Make items private by default, expose only what's needed
   - Document public APIs with doc comments (`///`)

## Workflow Checklist

Before each change:
- [ ] Have I identified ONE specific function to modify?
- [ ] Have I written a test for this change?
- [ ] Is this change small enough (< 15 lines in the function)?

After each change:
- [ ] Did I run the tests?
- [ ] Did the tests pass?
- [ ] Is the code formatted with `cargo fmt`?
- [ ] Does `cargo clippy` pass without warnings?

If you answer "no" to any of these, STOP and address it before proceeding.

## Development Commands

- `cargo test` - Run all tests
- `cargo test <name>` - Run specific test
- `cargo fmt` - Format code
- `cargo clippy` - Run linter
- `cargo build` - Build project
- `cargo run` - Run the application

## Red Flags - When to STOP

⛔ You're editing multiple functions simultaneously
⛔ You're making changes across multiple modules at once  
⛔ The diff is growing beyond 20-30 lines
⛔ You haven't run tests in the last change cycle
⛔ You're implementing before writing tests
⛔ You're unsure how to test the change

**When you see these flags, pause and restart with a smaller scope.**
