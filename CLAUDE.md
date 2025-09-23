# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based time tracking CLI application that parses markdown files to generate time reports. The binary is named `tt` and processes time tracking data with filtering, reporting, and multiple output formats.

## Version Control

**CRITICAL: This repository uses Jujutsu (jj), not git.** Always use `jj` commands for version control operations.

- Use `jj st --no-pager` to check status
- Use `jj log --no-pager` to view history
- Use `jj commit -m "message"` to create commits
- Use `jj split -m "message" file1 file2` to commit only specific files
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

### 1. Evolution Strategy

- Begin with the smallest end-to-end solution that works
- Start with hardcoded values; generalize once validated
- Separate feature additions from refactoring

### 2. Testing Approach

- Verify observable behaviors over implementation mechanics
- Compose tests using reusable builders and utilities
- Assert outcomes through domain language

#### Testing Best Practices
- **Use assert_fs::TempDir** for any file-based testing to ensure automatic cleanup and test isolation
- **Test through stable interfaces** (CLI/API) rather than internal implementation details
- **Commit after each working iteration** to maintain functional program state throughout development

#### Testing Architecture
- **Acceptance tests** in `tests/acceptance.rs` using CLI interface for integration testing
- **Unit tests** only when complex business logic requires isolated testing
- **Test fixtures** managed with `assert_fs::TempDir` for automatic cleanup
- **Test data** created programmatically rather than static files where possible

#### Testing Layer Analysis
- **Analyze coverage across all test layers** before adding new tests (unit, integration, acceptance)
- **For CLI tools, prefer acceptance tests** over unit tests when they provide equivalent coverage
- **Eliminate redundant tests** that duplicate coverage at different layers
- **CLI testing philosophy**: Test CLI behavior through the command-line interface rather than internal APIs

#### CLI Testing Best Practices
- **Test CLI behavior** through the command-line interface rather than internal APIs
- **Use acceptance tests** to validate end-user experience
- **Internal unit tests** should only cover complex business logic not adequately tested via CLI
- **Acceptance tests provide more valuable coverage** for CLI applications as they validate actual user experience

#### Incremental Change Testing Pattern
For multi-step refactoring tasks:
1. Make smallest possible change within a category
2. Run tests immediately
3. Commit if tests pass, debug if they fail
4. Only move to next category after current category is complete and committed
5. This prevents accumulation of breaking changes and provides safe rollback points

#### When changing or refactoring tests

- **Convert tests incrementally** - when refactoring tests, convert one test at a time and verify coverage before removing original
- **Coverage as quality gate** - use coverage analysis to validate that agent-suggested test changes maintain protection

### 3. Clippy Workflow

Use `just run-clippy` to maintain code quality and functional programming standards.

#### Clippy Configuration Policy
- **Fix functional/performance warnings** (unnecessary_wraps, needless_pass_by_value, etc.) - these often reveal actual design issues
- **Fix safety warnings** (must_use_candidate, etc.) - these prevent silent bugs
- **Configure away documentation warnings** in justfile unless specifically requested
- **Always use justfile configuration** over scattered #[allow] attributes for consistency

#### Systematic Clippy Process
1. **Use `just run-clippy`** to identify all warnings
2. **Categorize by functional impact** (analyze warnings by priority and impact)
3. **Fix high-impact functional improvements first** (unnecessary_wraps, ownership issues)
4. **Test after each category**, commit working state before moving to next
5. **Expect cascading effects** in functional pipelines - signature changes propagate through call chains
6. **Configure away low-value warnings** (like missing-errors-doc) in justfile rather than fixing

#### Clippy-Driven Refactoring Insights
- **`unnecessary_wraps` warnings are code smells** - they often reveal over-engineering where pure transformations are unnecessarily wrapped in `Result`
- **`needless_pass_by_value` warnings indicate ownership violations** - fixing these aligns with functional programming immutability principles
- **Functional pipeline improvements cascade naturally** - improving one function's signature often requires improving its callers, creating a chain of beneficial changes
- **Signature changes have cascading effects** - use `Grep` to analyze call sites before making changes to understand full impact

### 4. Error Handling

- Design error types to mirror domain failure modes
- Preserve error sources when propagating upward
- Validate early, handle centrally

### 5. Domain Modeling

- Create distinct types for domain concepts
- Express business rules through type relationships
- Derive aggregate properties at creation time

### 6. Code Structure

- Group code by user-facing capabilities
- Separate sequential processing stages
- Favor immutable data transformations

### 7. Rust Specifics

- Enforce valid states through type constraints
- Design expressive fluent interfaces
- Encode business logic in type definitions rather than runtime checks
  - Use types as guardrails rather than writing if checks scattered through business logic

### 8. Documentation

- Lead with concrete usage examples
- Anchor documentation near usage context

### 9. Agent Usage Guidelines

#### Agent Validation Principles
- **User domain knowledge trumps agent suggestions** - question recommendations against actual requirements
- **Present → Analyze → Approve workflow**: Present agent recommendations, analyze applicability to use case, get explicit approval before implementing
- **Document decisions** - capture reasoning for accepting or rejecting agent advice

#### Conservative Integration Approach
- **Start minimal**: Test the waters with small, optional enhancements before expanding scope
- **Preserve flexibility**: Avoid coupling commands or processes too tightly to agent workflows
- **Minimal viable enhancement**: Small, targeted changes often provide better value than comprehensive overhauls
- **Optional over prescriptive**: Integrate agent suggestions as context-sensitive guidance rather than requirements

#### Working with Specific Agents
- **dev-mentor agent**: Invoke early for Rust functional programming guidance, critical design review, and architectural decisions
- **Validate agent expertise scope** - ensure agent recommendations align with project context and requirements

### 10. Development Methodology
- **TDD with primitive whole**: Start with simplest end-to-end working test
- **Functional pipeline approach**: Separate pure functions from side effects
- **Iterative refinement**: Build on working state with focused commits
- **Architecture-first design**: Plan extensible solutions before implementation

### 11. Functional Refactoring Patterns
- **Critical questioning**: Challenge design assumptions, especially mixed concerns and misleading names
- **Function composition over conditionals**: Replace `if` statements with `Result` chaining and `and_then`
- **Consolidate operations**: Multi-step processes become single, composable functions on target types
- **Separate pure validation from side effects**: Keep domain validation side-effect free and testable
- **Honest domain modeling**: Ensure type names accurately reflect their contents and responsibilities
- **Smart constructors**: Use static methods for natural transformation pipelines (e.g., `Type::parse()`)
- **Clippy-guided improvements**: Use clippy warnings as functional programming guidance - `unnecessary_wraps` reveals over-engineering, `needless_pass_by_value` indicates ownership violations
- **Pipeline cascading**: Functional pipeline improvements naturally cascade - fixing one function's signature often improves the entire call chain

### 12. Rust File Organization
- **Public API First**: Prioritize reader experience by placing public entry points at the very top of files
- **Error types are domain types**: Treat errors as first-class domain concepts representing business failure modes, not technical plumbing
- **Visibility-driven encapsulation**: Make internal utility functions private to create clean API boundaries
- **Logical sectioning structure**: Organize files in clear sections: Public API → Domain Types → Type Implementations → Private Implementation
- **Abstraction-level ordering**: High-level workflow functions before low-level helpers, with related functions grouped together
- **Type grouping by relationship**: Group types by domain relationships (config types, content types, error hierarchy) rather than alphabetically
- **Minimal public API surface**: Only expose types and functions necessary for external use

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

### Formatter Testing Strategy

- **Console/Text format tests**: Test all business logic and behavior (tag filtering, time aggregation, period calculations, error handling, edge cases)
- **Other format tests**: Focus on format-specific concerns only (syntax, structure, escaping) plus one sensible integration test

This treats console/text as the canonical behavioral test suite, while other formats verify correct presentation of the same data in their specific syntax. Business logic is format-agnostic and should be tested once in the primary format.

## Environment Variables
- `TT_TODAY` - Override current date for testing (format: YYYY-MM-DD)

## Key Dependencies
- `clap` - CLI argument parsing with derive macros
- `chrono` - Date/time handling
- `anyhow` - Error handling
- `regex` - Text parsing
- `rstest` - Table-driven testing
- `assert_cmd` - CLI testing utilities

# Claude Code configuration

## Serena MCP Integration

This project includes project-scoped Serena MCP integration for enhanced Claude Code capabilities.

### Prerequisites
- Nix (for isolated dependency management)
- Claude Code CLI (installed globally)
- direnv (optional but recommended for automatic environment loading)

### Usage

This project features a seamless, automatic integration with Serena MCP.

```bash
# Just enter the directory. direnv handles the rest.
# Then, simply run the claude command.
claude
```

Serena's tools will be automatically available within your Claude session.

### Architecture

- **Zero-Configuration**: `direnv` automatically configures your shell. Just run `claude`.

- **No Global Installs**: All Python dependencies managed via Nix flake
- **Project Scoped**: Serena MCP only active when using this repo's configuration
- **Zero Pollution**: No changes to global Claude Code configuration
- **Reproducible**: Same environment across machines via Nix

### Files
- `scripts/serena-mcp` - Serena MCP server launcher (uses `nix develop -c uvx`)
- `.envrc` - Automatically configures the `claude` command with project-specific settings.
- `flake.nix` - Updated with Python 3.12 and uv dependencies

### Maintenance
- Serena updates automatically via `uvx --from git+https://github.com/oraios/serena`
- To pin a specific version, modify the git reference in `scripts/serena-mcp`
- If Serena changes its invocation pattern, update `scripts/serena-mcp`
