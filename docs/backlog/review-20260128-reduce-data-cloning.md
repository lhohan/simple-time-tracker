# Reduce Excessive Data Cloning

**Severity**: MEDIUM | **Effort**: Medium | **Category**: Performance

## Problem

17+ instances of `.clone()` on data structures per request:

- `state.data_path.clone()` (every handler)
- `overview.entries_time_totals().clone()` (lines 140, 314, 457)
- `overview.outcome_time_totals().clone()` (lines 496, 562, 622)
- `tag_name.clone()` (lines 361, 394, 400, 407)

## Files Affected

- `src/web/handlers.rs`
- `src/domain/reporting/` (OverviewReport methods)

## Impact

Unnecessary memory allocations and copies for large datasets.

## Solution Options

1. **Return references** - Change `entries_time_totals()` to return `&[TimeTotal]`
2. **Use Arc** - Wrap shared data in Arc for cheap clones
3. **Restructure templates** - Accept references instead of owned values

## Acceptance Criteria

- [ ] Reduce clones where lifetime permits
- [ ] No functional behavior changes
- [ ] `just run-clippy` passes
- [ ] `just test-web` passes
