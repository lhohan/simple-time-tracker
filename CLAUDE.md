# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based time tracking CLI application that parses markdown files to generate time reports. The binary is named `tt` and processes time tracking data with filtering, reporting, and multiple output formats.

## Version Control

**CRITICAL: This repository uses Jujutsu (jj), not git.** Always use `jj` commands for version control operations.

- Use `jj st --no-pager` to check status
- Use `jj log --no-pager` to view history
- Use `jj commit -m "message"` to create commits
- Use `jj bookmark set main -r @- && jj git push` to push to remote repository

## Common Commands

### Building and Running
- `just build` - Build release version of the `tt` binary
- `just install` - Install `tt` locally
- `just run path period` - Run the app with specified path and period
- Example: `just run "./data.md" "this-week"`

### Testing
- `just test` - Run tests using cargo nextest
- `just test-w` - Run tests continuously on file changes
- `just test-coverage` - Run tests with coverage using llvm-cov
- `just test-coverage-report` - Run coverage and open HTML report

### Development
- `just check-w` - Run cargo check continuously
- `just run-clippy` - Run extensive Clippy linter checks
- `just clean` - Clean build artifacts

### Linting
Always run clippy after code changes: `just run-clippy`

## Code Architecture

### Domain-Driven Design Structure
- **`src/domain/`** - Core business logic and domain models
  - `dates/` - Date handling and period calculations
  - `reporting.rs` - Report generation and time aggregation
  - `tags.rs` - Tag filtering and categorization
  - `time.rs` - Time-related utilities and Clock abstraction
  - `mod.rs` - Main domain types (TimeEntry, Outcome, ParseError)

- **`src/parsing/`** - Input processing pipeline
  - `parser.rs` - Main line parsing logic
  - `filter.rs` - Data filtering implementations
  - `processor.rs` - File processing orchestration
  - `model.rs` - Parsing data structures

- **`src/reporting/`** - Output formatting
  - `format/` - Different output formatters (markdown, text)
  - `model.rs` - Report data structures

- **`src/cli/`** - Command-line interface using clap

### Key Design Patterns
- **Hexagonal Architecture**: Domain logic is isolated from parsing and reporting concerns
- **Filter Chain**: Multiple filters can be combined for data processing
- **Strategy Pattern**: Pluggable formatters for different output types
- **Clock Abstraction**: Testable time handling via dependency injection

## Development Guidelines

### TDD Approach
- Start with failing tests, implement minimal code to pass
- Focus on observable behaviors over implementation details
- Use reusable builders and utilities for test composition
- Follow Red-Green-Refactor cycle religiously

### Code Style
- Use domain types to encode business rules at compile time
- Prefer immutable data transformations
- Group code by user-facing capabilities
- Design expressive fluent interfaces
- Use types as guardrails rather than runtime checks

### Test Naming Convention
Format: `[subject]_should_[expected_behavior]_[optional_when_condition]`
- Only include `when_condition` for disambiguation or essential context
- Keep names as short as possible while maintaining clarity

### Error Handling
- Design error types to mirror domain failure modes
- Preserve error sources when propagating upward
- Validate early, handle centrally

## Testing Structure

- **Unit tests**: Embedded in source files using `#[cfg(test)]`
- **Acceptance tests**: Located in `tests/acceptance/` following matklad's single-binary pattern
- **Test utilities**: Use `rstest` for table-driven testing
- **CLI testing**: Use `assert_cmd` for command-line integration tests

## Environment Variables
- `TT_TODAY` - Override current date for testing (format: YYYY-MM-DD)

## Key Dependencies
- `clap` - CLI argument parsing with derive macros
- `chrono` - Date/time handling
- `anyhow` - Error handling
- `regex` - Text parsing
- `rstest` - Table-driven testing
- `assert_cmd` - CLI testing utilities
