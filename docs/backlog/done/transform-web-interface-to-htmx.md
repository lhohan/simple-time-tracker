# Transform Web Interface to HTMX-First

## Summary
Refactor dashboard and outcomes pages to a cohesive HTMX-first interaction model. Replace mixed custom JS + ad‑hoc `hx-get` usage with a single form-driven request model and out-of-band swaps to update multiple regions consistently.

## Current Pain
- Period buttons had `hx-get` while custom JS also triggered HTMX requests, causing competing updates and param drift (e.g., `limit=true` not applied to projects table).
- Pie chart respected limit because it was only updated via JS with the full query; projects list sometimes came from element `hx-get` without `limit`.
- Outcomes page works because it consistently uses the JS path; still not HTMX-first.

## Proposed Direction
- Adopt a single `<form id="filters">` with inputs: `period`, `from`, `to`, `limit`.
- Add `hx-get="/api/dashboard" hx-target="#projects" hx-swap="innerHTML" hx-include="#filters"` on the form or submit button.
- Server returns:
  - `projects_partial.html` for `#projects`.
  - `summary_partial.html` and `chart_projects_pie.html` marked with `hx-swap-oob="true"` to update `#summary` and `#chart-pie` from the same response.
- Period/date/limit changes trigger a single HTMX request; all targets stay in sync.

## Scope
- Dashboard: convert filters, remove custom `updateFilters()` JS, wire OOB partials.
- Outcomes: mirror the same pattern for parity.
- Tests: ensure existing web tests pass; add integration tests for OOB updates and combined filters.

## Risks
- Template changes affect IDs/targets; ensure Tag drill-down links preserve query params (append via `hx-include` or hidden inputs).
- Large responses if multiple partials are bundled; keep partials small.

## Acceptance Criteria
- One HTMX request updates summary, projects list, and chart with identical query params.
- Limit checkbox consistently filters both chart and projects.
- Period/date range filters work without custom JS.
- All `just test-web` tests pass.

## Follow-ups
- Consider server-side defaults for filter state and deep-linkable URLs.
