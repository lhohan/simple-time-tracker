# 004: Fix processor error messages (format strings and typos)

Status: To Do
Priority: Medium
Effort: Small

## Problem
`src/parsing/processor.rs` has broken error messages:
- `"Failed to read {{path.diplay()}}: {err}"` — double braces, `diplay` typo, won’t render path.
- `"Invalid filename: {path.display()}"` — braces used inside the string literal rather than as format args.

Tests only assert for substrings (e.g., “Failed to read”), so these issues don’t surface — but they reduce diagnostic quality.

## Fix
- Use proper formatting, e.g.:
  - `format!("Failed to read {}: {}", path.display(), err)`
  - `format!("Invalid filename: {}", path.display())`

## Acceptance criteria
- Error messages include the correct filename and error cause.
- Existing tests still pass; optionally tighten assertions to check filename appears when appropriate.