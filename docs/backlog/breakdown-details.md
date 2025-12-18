# Feature: Breakdown with Task Details

Goal: When `--breakdown` and `--details` flags are combined, show task descriptions aggregated at the leaf level of the breakdown hierarchy.

## Problem & Intent

- Today `--breakdown` shows time totals per period (day/week/month/year) but no task-level details
- Today `--details` shows task descriptions aggregated across the entire period, not per sub-period
- Missing: A way to see what was actually worked on within each breakdown period
- Desired: Combine breakdown hierarchy with task details at the leaf level

## Current vs. Desired Behavior

| Command | Current Output | Desired Output |
|---------|----------------|----------------|
| `--breakdown day --details` | Only day totals | Day + tasks within each day |
| `--breakdown week --details` | Week → days hierarchy | Week → days → tasks |
| `--breakdown month --details` | Month → weeks hierarchy | Month → weeks → tasks |
| `--breakdown year --details` | Year → months hierarchy | Year → months → tasks |

## Design Decisions

### Task Details at Leaf Level

Tasks appear at the leaf level of the existing breakdown hierarchy:

| Breakdown | Hierarchy | Tasks aggregated per |
|-----------|-----------|---------------------|
| `day` | days | day |
| `week` | weeks → days | day |
| `month` | months → weeks | week |
| `year` | years → months | month |

### Percentage Calculation

Task percentages are relative to the immediate parent period (e.g., for day breakdown, percentage is of that day's total).

### No Description Handling

Entries without descriptions show `<no description>` placeholder (consistent with existing `--details` behavior).

## CLI Design

No new flags required. Combines existing flags:

```bash
# Day breakdown with task details
tt -i data.md --tags work --breakdown day --details

# Week breakdown with task details per day
tt -i data.md --tags work --breakdown week --details --period this-week

# Month breakdown with task details per week
tt -i data.md --tags work --breakdown month --details --period this-month
```

Validation: `--details` already requires `--tags`, so no additional validation needed.

## User-Visible Behavior

### Text Format Examples

**Day breakdown with details:**
```text
2020-01-01 -> 2020-01-02
Total: 5h 00m

2020-01-01 (Wed)..3h 00m
  - Task A..1h 00m (33%)
  - Task B..2h 00m (67%)
2020-01-02 (Thu)..2h 00m
  - Task C..2h 00m (100%)
```

**Week breakdown with details:**
```text
2020-01-01 -> 2020-01-03
Total: 5h 00m

2020-W01..5h 00m
  2020-01-01 (Wed)..3h 00m
    - Task A..1h 00m (33%)
    - Task B..2h 00m (67%)
  2020-01-02 (Thu)..2h 00m
    - Task C..2h 00m (100%)
```

**Month breakdown with details:**
```text
2020-01-01 -> 2020-01-15
Total: 3h 00m

2020-01..3h 00m
  2020-W01..1h 00m
    - Task A..1h 00m (100%)
  2020-W02..2h 00m
    - Task B..2h 00m (100%)
```

### Markdown Format

```markdown
# Time Breakdown Report

## 2020-W01
### 2020-01-01 (Wed)
- Task A: 1h 00m (33%)
- Task B: 2h 00m (67%)

### 2020-01-02 (Thu)
- Task C: 2h 00m (100%)
```

## Domain & Architecture Changes

### Model Extension

**File: `src/domain/reporting.rs`**

Extend `BreakdownGroup` to hold optional task details:

```rust
#[derive(Debug, Clone)]
pub struct BreakdownGroup {
    pub label: String,
    pub minutes: u32,
    pub children: Vec<BreakdownGroup>,
    pub tasks: Vec<TaskSummary>,  // NEW: task details at leaf level
}
```

### Report Generation

**File: `src/domain/reporting.rs`**

- Update `BreakdownReport::from_tracked_time()` to accept `include_details: bool`
- Modify breakdown functions to aggregate tasks when flag is true:
  - `break_down_by_day_with_dates()` → populate tasks per day
  - `break_down_by_week_with_entries()` → populate tasks per day (leaf)
  - `break_down_by_month_with_entries()` → populate tasks per week (leaf)
  - `break_down_by_year_with_entries()` → populate tasks per month (leaf)

### Formatting

**File: `src/reporting/format/text.rs`**

Update `format_breakdown_group()` to render tasks after children:

```rust
fn format_breakdown_group(result: &mut String, group: &BreakdownGroup, depth: usize) {
    // ... existing label/minutes formatting ...
    
    for child in &group.children {
        Self::format_breakdown_group(result, child, depth + 1);
    }
    
    // NEW: render tasks at leaf level
    for task in &group.tasks {
        let indent = "  ".repeat(depth + 1);
        writeln!(result, "{}- {}..{} ({}%)", 
            indent, task.description, format_duration(task.minutes), task.percentage_of_total);
    }
}
```

**File: `src/reporting/format/markdown.rs`**

Similar updates for markdown output.

### CLI Integration

**File: `src/lib.rs`**

Pass `include_details` to breakdown report generation:

```rust
if let Some(unit) = breakdown_unit {
    let report = BreakdownReport::from_tracked_time(time_report, unit, include_details);
    // ...
}
```

## Test Plan (TDD)

Add tests to `tests/acceptance/breakdown.rs` following existing patterns.

### Test Helper Addition

Add to `tests/acceptance/common.rs`:
- Existing `details_flag()` method already available
- Tests will combine `breakdown_flag()` and `details_flag()`

## Iteration Plan (Incremental TDD)

Each test follows the complete Red-Green-Refactor cycle before moving to the next.
Do NOT write all tests first. Complete each cycle fully before starting the next.

### Test 1: Validation
**Test:** `breakdown_with_details_should_require_tags`
- **Red:** Write test, verify behavior
- **Green:** Likely passes (inherits `--details` validation)
- **Refactor:** None needed

### Test 2: Day Breakdown - Basic
**Test:** `breakdown_day_with_details_should_show_tasks_per_day`
- **Red:** Write test expecting tasks under day header
- **Green:** 
  - Extend `BreakdownGroup` with `tasks: Vec<TaskSummary>` field
  - Update `break_down_by_day_with_dates()` to populate tasks
  - Update `TextFormatter::format_breakdown_group()` to render tasks
  - Pass `include_details` through call chain in `lib.rs`
- **Refactor:** Run clippy, clean up

### Test 3: Day Breakdown - Multiple Days
**Test:** `breakdown_day_with_details_should_show_tasks_grouped_by_day`
- **Red:** Write test with entries on different days
- **Green:** Should pass if Test 2 correct; fix if not
- **Refactor:** Clean up duplication

### Test 4: Task Aggregation
**Test:** `breakdown_day_with_details_should_aggregate_same_task_descriptions`
- **Red:** Write test with duplicate task descriptions
- **Green:** Implement task aggregation (sum minutes for same description)
- **Refactor:** Extract aggregation helper if needed

### Test 5: Week Breakdown
**Test:** `breakdown_week_with_details_should_show_tasks_per_day`
- **Red:** Write test expecting week → days → tasks
- **Green:** Update `break_down_by_week_with_entries()` to populate tasks at day level
- **Refactor:** Look for common patterns

### Test 6: Month Breakdown
**Test:** `breakdown_month_with_details_should_show_tasks_per_week`
- **Red:** Write test expecting month → weeks → tasks
- **Green:** Update `break_down_by_month_with_entries()` to populate tasks at week level
- **Refactor:** Extract common task aggregation pattern

### Test 7: Year Breakdown
**Test:** `breakdown_year_with_details_should_show_tasks_per_month`
- **Red:** Write test expecting year → months → tasks
- **Green:** Update `break_down_by_year_with_entries()` to populate tasks at month level
- **Refactor:** Consolidate remaining duplication

### Test 8: No Description Placeholder
**Test:** `breakdown_with_details_should_show_no_description_placeholder`
- **Red:** Write test with entry lacking description
- **Green:** Should pass using existing `<no description>` logic
- **Refactor:** None expected

### Test 9: Markdown Format
**Test:** `breakdown_with_details_and_markdown_format_should_show_markdown_output`
- **Red:** Write test expecting markdown formatting
- **Green:** Update `MarkdownFormatter` to render tasks in breakdown
- **Refactor:** Final cleanup, ensure text/markdown consistency

## Acceptance Criteria

- [ ] `--breakdown day --details` shows tasks per day with percentages
- [ ] `--breakdown week --details` shows week → days → tasks hierarchy
- [ ] `--breakdown month --details` shows month → weeks → tasks hierarchy  
- [ ] `--breakdown year --details` shows year → months → tasks hierarchy
- [ ] Same task descriptions within a period are aggregated
- [ ] Entries without descriptions show `<no description>`
- [ ] Markdown format produces valid markdown with proper hierarchy
- [ ] All existing breakdown tests continue to pass
- [ ] All existing details tests continue to pass
