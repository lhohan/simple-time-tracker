# Issue 006: Optimize Regex Compilation Using LazyLock Patterns

**Priority:** Medium
**Type:** Performance Optimization
**Location:** `/src/domain/dates/range.rs:91,104,119,138`

## Problem Description

Regex patterns are compiled on every function call instead of being cached, causing unnecessary performance overhead for date parsing operations.

### Code Locations
```rust
// Line 91 - try_parse_date_value()
let date_regex = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})$").unwrap();

// Line 104 - try_parse_month_value()
let month_value_regex = Regex::new(r"^(\d{4})-(\d{1,2})$").unwrap();

// Line 119 - try_parse_week_value()
let week_value_regex = Regex::new(r"^(\d{4})-w(\d{1,2})$").unwrap();

// Line 138 - try_parse_year_value()
let year_value_regex = Regex::new(r"^(\d{4})$").unwrap();
```

### Performance Impact Analysis
- These functions are called in sequence (lines 68-71) for every period parsing attempt
- For a CLI processing many time entries, this could mean hundreds of regex compilations
- Regex compilation typically takes **10-100x longer** than pattern matching itself
- The pattern strings are static and never change - perfect candidates for lazy static compilation

### Inconsistency in Codebase
The codebase already demonstrates the correct pattern:
```rust
// Line 9 - Correct usage
static MONTH_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(month|m)-(\d+)$").unwrap()
});
```

## Root Cause
The functions compile regex patterns every time they're called instead of using static compilation like `MONTH_REGEX`.

## Proposed Solution

Move all regex patterns to static `LazyLock` declarations:

```rust
static DATE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(\d{4})-(\d{2})-(\d{2})$").unwrap()
});

static MONTH_VALUE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(\d{4})-(\d{1,2})$").unwrap()
});

static WEEK_VALUE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(\d{4})-w(\d{1,2})$").unwrap()
});

static YEAR_VALUE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(\d{4})$").unwrap()
});
```

Then update the functions to use these static patterns instead of compiling on each call.

## Expected Benefits
- **Performance**: Significant reduction in regex compilation overhead
- **Consistency**: Matches existing pattern used by `MONTH_REGEX`
- **Memory**: Reduced memory allocations from repeated compilations
- **Maintainability**: Centralized regex pattern definitions

## Verification Steps
1. Replace inline regex compilation with static LazyLock patterns
2. Update function implementations to use static patterns
3. Run performance benchmarks on date parsing operations
4. Verify all existing tests still pass
5. Run `just run-clippy` to ensure no new warnings