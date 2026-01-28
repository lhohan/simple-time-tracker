# Add Pagination for Large Datasets

**Severity**: HIGH | **Effort**: Medium | **Category**: Performance

## Problem

No pagination implemented. Full dataset returned in single response regardless of size.

## Files Affected

- `src/web/handlers.rs` - all table-rendering handlers
- `src/web/models.rs` - query parameters
- `templates/dashboard.html`
- `templates/outcomes.html`
- `templates/projects_partial.html`
- `templates/outcomes_partial.html`

## Impact

- Memory pressure on server rendering large tables
- Slow page loads for users with large datasets
- Browser performance issues

## Solution

Add pagination query parameters and UI:

```rust
// models.rs
pub struct DashboardParams {
    // existing fields...
    pub page: Option<u32>,
    pub per_page: Option<u32>,  // default 50
}
```

## Acceptance Criteria

- [ ] `?page=N&per_page=M` query parameters supported
- [ ] Default limit of 50-100 items per page
- [ ] Total count returned for pagination UI
- [ ] Pagination controls in templates
- [ ] `just test-web` passes
