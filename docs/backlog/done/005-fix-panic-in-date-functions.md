# Issue 005: Fix Potential Panic in start_date() and end_date() Functions

**Priority:** Medium
**Type:** Bug - Reliability Issue
**Location:** `/src/parsing/mod.rs:54,60`

## Problem Description

The `start_date()` and `end_date()` functions use `.unwrap()` on operations that could fail, creating potential panic conditions that would crash the application.

### Code Location
```rust
fn start_date(mapped_entries: &HashMap<NaiveDate, Vec<TimeEntry>>) -> StartDate {
    StartDate(*mapped_entries.keys().min().unwrap())  // Line 54 - Could panic
}

fn end_date(mapped_entries: &HashMap<NaiveDate, Vec<TimeEntry>>) -> EndDate {
    EndDate(*mapped_entries.keys().max().unwrap())    // Line 60 - Could panic
}
```

### Root Cause
The logical flaw occurs in the calling code at line 79:
```rust
.filter(|entries| !entries.is_empty())  // Checks HashMap container exists
.map(|mapped_entries| {
    // But doesn't guarantee the HashMap has valid date keys
    start_date(mapped_entries),  // Could panic if no keys exist
    end_date(mapped_entries),    // Could panic if no keys exist
})
```

### Scenarios That Trigger This Issue
- Empty markdown files
- Files with only invalid time entries
- Aggressive filtering that removes all valid entries
- Malformed date parsing that results in no valid dates

### Impact Assessment
- **Reliability**: CLI could crash unexpectedly on user data
- **User Experience**: Cryptic panic messages instead of helpful error messages
- **Production Risk**: Application appears buggy and unreliable

## Proposed Solution

Modify the functions to return `Option<T>` and handle the empty case gracefully:

```rust
fn start_date(mapped_entries: &HashMap<NaiveDate, Vec<TimeEntry>>) -> Option<StartDate> {
    mapped_entries.keys().min().map(|&date| StartDate(date))
}

fn end_date(mapped_entries: &HashMap<NaiveDate, Vec<TimeEntry>>) -> Option<EndDate> {
    mapped_entries.keys().max().map(|&date| EndDate(date))
}
```

Update calling code to handle the `None` case appropriately.

## Verification Steps
1. Create test with empty HashMap
2. Create test with filtered entries that result in empty dates
3. Verify graceful error handling instead of panic
4. Run existing test suite to ensure no regressions