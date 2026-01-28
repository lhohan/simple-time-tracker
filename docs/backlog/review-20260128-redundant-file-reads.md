# Eliminate Redundant File Reads (N+1 Pattern)

**Severity**: CRITICAL | **Effort**: Medium | **Category**: Performance

## Problem

Every HTMX partial request triggers a complete file re-read and parse. A single filter change fires 3-4 requests:
- `/api/dashboard` → full parse
- `/api/dashboard/summary` → full parse
- `/api/chart/projects-pie` → full parse

## Files Affected

- `src/web/handlers.rs` lines: 125-130, 198-203, 249-254, 300-305, 370-374, 443-448, 486, 548-553, 608-613

## Impact

- 3-4x file I/O per user interaction
- Blocking pool exhaustion under load
- Unnecessary CPU for repeated parsing
- Poor scalability

## Solution Options

1. **Request-level caching** - Cache parsed data with short TTL (5-30 seconds)
2. **Consolidate endpoints** - Single endpoint returns combined response
3. **Use hx-swap-oob** - HTMX out-of-band swaps to update multiple regions from single response

## Acceptance Criteria

- [ ] Filter change triggers single file read (or cached response)
- [ ] Response latency reduced measurably
- [ ] `just test-web` passes
- [ ] Manual testing: dashboard filters work correctly
