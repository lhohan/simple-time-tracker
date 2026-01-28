# Replace Imperative onclick Handlers with Declarative HTMX

**Severity**: MEDIUM | **Effort**: Easy | **Category**: Maintainability

## Problem

Period filter buttons use imperative `onclick` JavaScript to set a hidden field value before the HTMX request fires:

```html
<!-- dashboard.html:54 -->
<button type="button"
    hx-get="/api/dashboard"
    hx-target="#projects"
    hx-include="#filter-form"
    onclick="document.getElementById('selected-period').value = 'today'">Today</button>
```

This mixes declarative HTMX with imperative DOM manipulation — an HTMX anti-pattern. The `onclick` fires synchronously before `hx-get`, which works by accident of browser event ordering but is fragile.

## Files Affected

- `templates/dashboard.html:54-57` (4 buttons)
- `templates/outcomes.html:53-56` (4 buttons)

## Solution

Use `hx-vals` to declare the period value declaratively:

```html
<button type="button"
    hx-get="/api/dashboard"
    hx-target="#projects"
    hx-include="#filter-form"
    hx-vals='{"period": "today"}'>Today</button>
```

This eliminates the hidden `selected-period` input field and the onclick handlers entirely.

## Acceptance Criteria

- [ ] All period buttons use `hx-vals` instead of `onclick`
- [ ] Hidden `selected-period` input removed from both pages
- [ ] Filter state correctly composed (period + limit + date range)
- [ ] `just test-web` passes
