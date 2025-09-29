# Agent Guidelines for Time Tracker Rust Project

## Essential Commands
- **Build**: `just build` (release), `cargo build` (debug), `just install` (local install)
- **Test**: `just test` (all), `cargo nextest run test_name` (single), `just test-w` (watch), `just test-coverage` (with llvm-cov)
- **Lint**: `just run-clippy` (always run after changes), `just fmt`
- **Run**: `just run path period`
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
- **Prefer acceptance tests** over unit tests for CLI functionality
- **Test naming**: `[subject]_should_[expected_behavior]_[optional_when_condition]`
- **Use assert_fs::TempDir** for file-based testing
- **CLI testing**: Use assert_cmd with Command::cargo_bin("tt")
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

## Environment Variables
- `TT_TODAY` - Override current date for testing (format: YYYY-MM-DD)

## Key Dependencies Context
- `clap` - CLI argument parsing with derive macros
- `chrono` - Date/time handling
- `anyhow` - Error handling
- `regex` - Text parsing
- `rstest` - Table-driven testing
- `assert_cmd` - CLI testing utilities

## Fuzzing Requirements
- **Requires**: Nightly Rust toolchain
- **Purpose**: Discover panic-inducing edge cases in time entry parsing logic
- **Location**: Separate from regular tests, in `fuzz/` directory
- **Strategy**: CLI-level fuzzing through `Command::cargo_bin("tt")` like acceptance tests
- **Scope**: Only time entry parsing logic (not file I/O or CLI arguments)
- **Analysis**: Check `fuzz/artifacts/cli_parser_fuzz/` for crash files after fuzzing
