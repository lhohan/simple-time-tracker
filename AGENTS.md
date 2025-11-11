# Agent Guidelines for Time Tracker Rust Project

## Essential Commands
- **Build**: `just build` (CLI release), `just build-web` (web with features), `cargo build` (debug), `just install` (local install)
- **Test**: `just test` (all), `just test-web` (web only), `cargo nextest run test_name` (single), `just test-w` (watch), `just test-coverage` (with llvm-cov)
- **Lint**: `just run-clippy` (always run after changes), `cargo fmt --all -- --check`
- **Run CLI**: `just run path period` or `cargo run -- -i "path" --period "period"`
- **Run Web**: `just web -i path` (run web server), `just web-w -i path` (with auto-reload), or `cargo run --features web -- --web -i path`
- **Performance**: `just bench` (benchmarks), `just bench-w` (continuous benchmarks)
- **Fuzzing**: `just fuzz` (5-min), `just fuzz-long` (30-min), `just fuzz-custom <seconds>` (custom time)
- **Development**: `just check-w` (continuous check), `just clean` (clean artifacts)

## Code Style Requirements
- **Architecture**: Domain-driven design with hexagonal architecture (src/domain/, src/parsing/, src/reporting/)
- **Functional approach**: Immutable data transformations, pure functions over side effects
- **Types as guardrails**: Encode business rules in type definitions, avoid runtime if-checks
- **Error handling**: Design error types to mirror domain failure modes, preserve sources
- **File organization**: Public API first, logical sectioning, minimal public surface

## Testing Standards
- **Prefer acceptance tests** over unit tests for functionality
- **Unit tests** only when complex business logic helps with isolated testing
- **Test naming**: `[subject]_should_[expected_behavior]_[optional_when_condition]`
- **Use assert_fs::TempDir** for file-based testing
- **CLI testing**: Use assert_cmd with Command::cargo_bin("tt")
- **Web testing**: Use WebApp test DSL (tests/web/common.rs) following same Given-When-Then pattern as CLI
- **Coverage**: Analyze across layers before adding tests

## Version Control (Jujutsu)
**CRITICAL: This repository uses Jujutsu (jj), not git.** Always use `jj` commands for version control operations.
- `jj st --no-pager` - Check status
- `jj log --no-pager` - View history
- `jj commit -m "message"` - Create commits
- `jj split -m "message" file1 file2` - Commit only specific files
- `jj bookmark set main -r @- && jj git push` - Push to remote repository

## Critical Constraints
- **No comments**: Add comments only if explicitly requested
- **Clippy-driven**: Fix functional/performance warnings, use justfile configuration
- **Domain modeling**: Create distinct types for domain concepts, express rules through relationships

## Development Methodology

### Evolution Strategy
- **Begin with smallest end-to-end solution that works**
- **Start with hardcoded values; generalize once validated**
- **Separate feature additions from refactoring**

### TDD Approach
- **Start with failing tests, implement minimal code to pass**
- **Focus on observable behaviors over implementation details**
- **Follow Red-Green-Refactor cycle religiously**
- **TDD with primitive whole**: Start with simplest end-to-end working test

### Incremental Development Pattern
For multi-step tasks:
1. Make smallest possible change within a category
2. Run tests immediately
3. Commit if tests pass, debug if they fail
4. Only move to next category after current category is complete and committed
5. This prevents accumulation of breaking changes and provides safe rollback points

### Functional Refactoring Patterns
- **Function composition over conditionals**: Replace `if` statements with `Result` chaining and `and_then`
- **Separate pure validation from side effects**: Keep domain validation side-effect free and testable
- **Clippy-guided improvements**: Use clippy warnings as functional programming guidance

## Environment Variables
- `TT_TODAY` - Override current date for testing (format: YYYY-MM-DD)

## Key Dependencies Context
### CLI & Core
- `clap` - CLI argument parsing with derive macros
- `chrono` - Date/time handling
- `anyhow` - Error handling
- `regex` - Text parsing
- `rstest` - Table-driven testing
- `assert_cmd` - CLI testing utilities

### Web
- `axum` - Async web framework (v0.7)
- `tokio` - Async runtime with spawn_blocking for I/O
- `askama` - Compile-time type-safe templating (v0.12)
- `tower` - Service middleware (with util feature for testing)
- `serde` - Serialization for template data binding

## Fuzzing Requirements
- **Requires**: Nightly Rust toolchain
- **Purpose**: Discover panic-inducing edge cases in time entry parsing logic
- **Location**: Separate from regular tests, in `fuzz/` directory
- **Strategy**: CLI-level fuzzing through `Command::cargo_bin("tt")` like acceptance tests
- **Scope**: Only time entry parsing logic (not file I/O or CLI arguments)
- **Analysis**: Check `fuzz/artifacts/cli_parser_fuzz/` for crash files after fuzzing
