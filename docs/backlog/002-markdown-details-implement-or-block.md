# 002: Markdown details — implement or block to avoid runtime panic

Status: To Do
Priority: High
Effort: Small-Medium

## Problem
MarkdownFormatter only implements OverviewReport. For other variants (e.g., tasks/details), it calls `todo!()` and will panic at runtime if users run `--format markdown` with `--details` (or when details are implied by `--project`).

File: `src/reporting/format/markdown.rs` (match arm `_ => todo!()`).

## Options
- Implement Markdown formatting for Task details (preferred if Markdown output is a product requirement).
- Or, add CLI validation to block `--format markdown` with details and return a clear error.

## Acceptance criteria
- No runtime panic when Markdown + details are requested.
- If implemented: add a unit test or acceptance test that exercises `--format markdown` with details and validates structured output.
- If blocked: add validation with a helpful error message and a test asserting the message.

## Options analysis (keep minimal now)

1) Block at CLI validation (recommended, ~5–8 LoC)
- Detect `--format markdown|md` + details mode (`--details` or `--project`) in `Args::validate()` and return a clear error.
- Pros: smallest change, fails fast with an actionable message, no runtime panics. Cons: adds a tiny bit of CLI logic.

2) Graceful degrade inside MarkdownFormatter
- Keep accepting the combo but return a short "not supported yet" message instead of `todo!()`.
- Pros: also tiny; avoids CLI coupling. Cons: silently degrades output when user asked for details; some may expect a hard error.

3) Catching `todo!` panics at top level (not recommended)
- Using `std::panic::catch_unwind` or a panic hook to prettify `todo!` hides real bugs and adds brittle unwind boundaries (breaks with panic=abort, loses backtraces). Not idiomatic for user errors.

## Recommendation
- Do not implement full Markdown details yet. Choose either:
  - Option 1 (preferred): block unsupported combo in CLI with a clear error, or
  - Option 2: degrade in formatter with a concise "details not supported in markdown — use --format text" message.
- Avoid panic-catching approaches.

## Note
- This ticket is analysis-only for now (no implementation yet). When ready, pick Option 1 or 2 and add a small test to lock behavior in.
