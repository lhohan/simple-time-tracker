# Feature: Per-tag/project time breakdown by day/week/month/year

Goal: Add a reporting mode that provides a detailed time breakdown for a selected tag/project across a period, grouped by calendar units (day, week, month, year) with human-friendly labels.

## Problem & intent
- Today the CLI can filter by tag(s)/project and print an overview or task details for a period.
- Missing: A chronological breakdown per day/week/month/year for a given tag/project to reveal distribution over time.
- Desired: For a chosen period, show cumulative time per higher unit and, within it, per lower unit (e.g., month → weeks → days, or week → days), with weekday names and ISO week numbers.

## Scope (MVP)
- Input focus: one or more tags via `--project`/`--tags` and an optional `--period`/`--from` (existing flags).
- Output: hierarchical breakdown for matching entries inside the resolved `DateRange`.
- Granularities supported: day, week (ISO), month, year.
- Labels:
  - Day: `YYYY-MM-DD (Mon|Tue|...|Sun)`
  - Week: `YYYY-Www` (ISO week, e.g., `2025-W41`)
  - Month: `YYYY-MM`
  - Year: `YYYY`

Out of scope (MVP): aggregation by multiple independent group-by dimensions, CSV/JSON export, custom calendars.

## CLI design
- New flag: `--breakdown <unit>` where `<unit> ∈ {day, week, month, year, auto}`; default: `auto`.
  - `auto` chooses a reasonable hierarchy based on the requested period:
    - Day period → days only
    - Week period → days inside the week
    - Month period → weeks then days
    - Year period → months then weeks (days optional; see below)
- Requires at least one context filter: `--project` or `--tags` (aligns with `--details` validation).
- Works with existing flags: `--period`, `--from`, `--format text|markdown`, `--limit` (limit applies only to top-level entry totals, not breakdown items).

Examples:
```bash path=null start=null
# This week, per-day breakdown for project alpha
cargo run -- -i "./samples" --project prj-alpha --period this-week --breakdown day

# October 2025, per-week then per-day breakdown for #consulting context
cargo run -- -i "./samples" --tags consulting --period 2025-10 --breakdown auto

# Year 2025, per-month totals (no deeper levels)
cargo run -- -i "./samples" --tags prj-alpha --period 2025 --breakdown month
```

## User-visible behavior
- When `--breakdown` is set, the report prints a chronological tree for the selected context(s):
  - Period header (existing)
  - Totals for the selected tag(s) (existing overview totals remain available)
  - Breakdown sections per chosen unit, in ascending date order
  - Each item shows total duration (e.g., `3h 30m`)
- Text format (example sketch):
```text path=null start=null
Project: prj-alpha
2025-10-01 -> 2025-10-31
31 days, 1.8 h/day, 56h 00m total

2025-10 (month)
  2025-W41 (week)
    2025-10-06 (Mon) .. 1h 30m
    2025-10-07 (Tue) .. 2h 00m
  2025-W42 (week)
    2025-10-14 (Tue) .. 45m
```

Markdown format mirrors the same structure with headings/lists.

```text
# Breakdown for prj-alpha
- Period: 2025-10-01 → 2025-10-31
- Total: 56h 00m

## 2025-10
### 2025-W41
- 2025-10-06 (Mon): 1h 30m
- 2025-10-07 (Tue): 2h 00m

### 2025-W42
- 2025-10-14 (Tue): 45m
```

## Domain & architecture changes
Follow hexagonal architecture and types-as-guardrails:
- Domain additions in `src/domain/reporting.rs`:
  - New pure aggregator(s) to group `TimeEntry` by calendar unit(s):
    - `group_by_day(entries) -> BTreeMap<NaiveDate, u32>`
    - `group_by_week(entries) -> BTreeMap<(i32, u32 /*iso week*/), u32>`
    - `group_by_month(entries) -> BTreeMap<(i32, u32 /*month*/), u32>`
    - `group_by_year(entries) -> BTreeMap<i32, u32>`
  - New report type `BreakdownReport` with immutable value objects:
    - `enum BreakdownUnit { Day, Week, Month, Year }`
    - `struct BreakdownGroup { label: String, minutes: u32, children: Vec<BreakdownGroup> }`
    - `impl BreakdownReport::from(tracked: &TrackedTime, tags: &[Tag], unit: BreakdownUnit, period: &DateRange)`
- Label helpers:
  - Day label with weekday via chrono: `YYYY-MM-DD (Mon)`
  - ISO week label via `IsoWeek`: `YYYY-Www`
  - Month label `YYYY-MM`
  - Year label `YYYY`
- Keep aggregation pure; no formatting concerns in domain.

## CLI & validation
- Add `--breakdown <unit>` to `cli::Args` with enum parsing.
- Validation: if `--breakdown` is present and neither `--tags` nor `--project` are set → error message mirroring `--details` rule.
- Parsing period remains as-is (`--period`/`--from`).

## Formatting
- `reporting/format/text.rs` and `reporting/format/markdown.rs` gain `format_breakdown(report: &BreakdownReport)`.
- Respect existing headers and period statistics blocks.
- Indentation shows hierarchy; sort chronologically.

## Acceptance criteria
- Given a week period and `--breakdown day`, output lists each day (Mon..Sun) with zero-entries omitted and totals summing to the tag’s weekly total.
- Given a month period and `--breakdown auto`, output month header with nested weeks, each week with nested days in chronological order.
- Day labels include weekday short name; week labels include ISO week number; month/year labels as specified.
- Works for multiple tags passed via `--tags=a,b` (combined by logical OR as today).
- `--format markdown` outputs semantically equivalent structure with headings/lists.
- No panics for empty data; prints "No data found." consistently with existing behavior.

## Test plan (TDD)
Prefer acceptance tests:
- Use `assert_cmd` over CLI with fixture files in `tests/acceptance/`.
- Create a new module for these tests `tests/acceptance/breakdown.rs`.
- Stabilize relative periods using a fixed clock (e.g., `Clock::with_today` via existing `TT_TODAY` env in tests or explicit period values like `2025-10`).
- Cases:
  - week → days (single-tag)
  - month → weeks → days (single-tag)
  - year → months (shallow breakdown)
  - multiple tags OR-filtered
  - no matches (graceful output)
- Avoid unit tests.
- Guidelines for writing a testing DSL are in `docs/UNIVERSAL_DSL_TESTING_GUIDE.md`. Examples are in`tests/acceptance/general.rs`,`tests/acceptance/tags.rs`, `tests/acceptance/periods.rs`
- REMEMBER: follow project guidelines and follow a TTD style: first write a test, watch it fail, implement, refactor.

## Open questions
- Should `year` breakdown include weeks or only months for readability? (MVP: months only.)
- When multiple tags are provided, do we show a combined breakdown only, or a section per tag? (MVP: combined; future: per-tag sections.)
- Do we want a `--group-by project|context` now, or defer until there’s a stronger need? (MVP: defer; use existing tag filter semantics.)

## Iteration plan
1) Design slice
- Finalize CLI flag shape and validation rules
- Define `BreakdownReport` and grouping APIs

2) Red slice
- Add failing acceptance tests for week→day and month→week→day

3) Green slice
- Implement domain grouping + formatter for text

4) Refine
- Add markdown formatter, edge-case tests (ISO week/year edges)

5) Hygiene
- Run clippy and fmt, backfill docs in `README.md`
