# Issue 007: Add Integer Overflow Protection and Bounds Checking

**Priority:** Medium
**Type:** Input Validation / Security Hardening
**Location:** `/src/domain/dates/range.rs:108,109,123,125,141`

## Problem Description

Integer parsing operations use `.unwrap()` without bounds checking, creating potential panic conditions when processing malicious or malformed input.

### Code Locations
```rust
// Lines 108-109 - try_parse_month_value()
let year = year_match.as_str().parse::<i32>().unwrap();
let month = month_match.as_str().parse::<u32>().unwrap();

// Lines 123,125 - try_parse_week_value()
let week = week_match.as_str().parse::<u32>().unwrap();
let year = year_match.as_str().parse::<i32>().unwrap();

// Line 141 - try_parse_year_value()
let year = year_match.as_str().parse::<i32>().unwrap();
```

### Attack Scenarios
While regex patterns limit input format, they don't prevent extreme values:

1. **Year overflow**: Input like "99999999999999-01" matches month regex but overflows `i32`
2. **Month overflow**: Input like "2024-999999999" matches month regex but overflows `u32`
3. **Week overflow**: Input like "2024-w999999999" matches week regex but overflows `u32`

### Why Regex Protection Is Insufficient
- Month regex `^(\d{4})-(\d{1,2})$` allows 1-2 digits for month but doesn't enforce 1-12 range
- Week regex `^(\d{4})-w(\d{1,2})$` allows 1-2 digits but doesn't enforce 1-53 range
- Year regex patterns could be bypassed by malformed input

### Current Partial Protection
The code has some bounds checking, but it happens **after** the panic-prone `.unwrap()`:
```rust
// Line 124 - Good bounds check, but too late
if (1..=53).contains(&week) {
    let year = year_match.as_str().parse::<i32>().unwrap(); // Panic risk
```

## Impact Assessment
- **Reliability**: User input could crash the CLI unexpectedly
- **Security**: Could be used for denial of service attacks
- **User Experience**: Cryptic panic messages instead of helpful error messages

## Root Cause
Functions assume regex validation guarantees valid integer parsing, but this isn't true for edge cases involving overflow or extreme values.

## Proposed Solution

Replace `.unwrap()` calls with proper error handling and bounds validation:

```rust
fn try_parse_month_value(s: &str) -> Option<PeriodRequested> {
    let month = MONTH_VALUE_REGEX.captures(s).and_then(|captures| {
        captures.get(1).and_then(|year_match| {
            captures.get(2).and_then(|month_match| {
                // Safe parsing with bounds checking
                year_match.as_str().parse::<i32>().ok().and_then(|year| {
                    month_match.as_str().parse::<u32>().ok().and_then(|month| {
                        // Validate ranges before using
                        if (1..=12).contains(&month) && (1000..=9999).contains(&year) {
                            NaiveDate::from_ymd_opt(year, month, 1)
                        } else {
                            None
                        }
                    })
                })
            })
        })
    });
    month.map(PeriodRequested::MonthOf)
}
```

### Specific Bounds to Enforce
- **Years**: 1000-9999 (reasonable range for time tracking)
- **Months**: 1-12 (standard calendar months)
- **Weeks**: 1-53 (ISO week numbering)

## Expected Benefits
- **Reliability**: Graceful handling of malformed input instead of crashes
- **Security**: Protection against denial of service via crafted input
- **User Experience**: Clear error messages for invalid date formats
- **Robustness**: Proper input validation following defensive programming practices

## Verification Steps
1. Create tests with extreme values that would cause integer overflow
2. Create tests with out-of-bounds but parseable values (month 99, week 99)
3. Verify graceful error handling instead of panics
4. Ensure existing valid inputs still work correctly
5. Run full test suite to verify no regressions
6. Test with malformed input files containing edge cases