# Deduplicate Chart.js Code Across Templates

**Severity**: LOW | **Effort**: Medium | **Category**: Maintainability

## Problem

The Chart.js initialization code (colors array, chart options, destroy logic) is duplicated across 5 templates:

1. `templates/dashboard.html:120-183`
2. `templates/outcomes.html:113-176`
3. `templates/dashboard_combined_partial.html:39-102`
4. `templates/outcomes_combined_partial.html:33-96`
5. `templates/chart_projects_pie.html:5-64`

Each copy contains the identical 10-color palette array and nearly identical chart configuration options.

## Impact

- Style changes (colors, font sizes, legend position) require editing 5 files
- Divergence risk between initial page load chart and OOB-swapped chart

## Solution Options

1. **Shared JavaScript file** - Extract chart creation to a shared `chart-utils.js`
2. **Askama macro/include** - Use template includes for shared chart script blocks
3. **Data attributes** - Pass data via `data-*` attributes, initialize chart from shared JS

## Acceptance Criteria

- [ ] Chart colors defined in single location
- [ ] Chart options defined in single location
- [ ] All 5 templates use shared chart code
- [ ] Charts render correctly on initial load and after filter changes
