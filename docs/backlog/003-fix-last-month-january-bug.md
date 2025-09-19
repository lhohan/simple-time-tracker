# 003: Fix last_month January wrap bug in DateRange helpers

Status: To Do
Priority: High
Effort: Small

## Problem
`last_month(clock)` in `src/domain/dates/range.rs` does:

- `let previous_month = today.with_month(month_of_today - 1).unwrap();`

This panics in January since `with_month(0)` is invalid. Similar unwrap-heavy code exists in `calculate_1st_of_month` and month/year helpers.

## Approach
- Compute previous month/year safely (e.g., if month == 1 then (year-1, 12) else (year, month-1)).
- Use `with_year`/`with_month` only with valid values; avoid unwrap() chains where possible.
- Add a dedicated test for “last month” when `today` is in January (e.g., 2020-01-02 → 2019-12-01).

## Acceptance criteria
- No panic when `TT_TODAY` is any January date.
- Tests cover year boundary for last_month.
- Overall period-related tests remain green.