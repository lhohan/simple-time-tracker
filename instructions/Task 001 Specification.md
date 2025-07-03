# Specification: Implement Outcomes/Goals Support for Time Tracker

## Context

This is a Rust-based time tracking application that parses markdown files and generates reports. The application follows TDD principles and has an existing acceptance test suite.

### Current Format
Time entries use this format:
```markdown
## TT 2025-01-15
- #prj-1 1h Task A
- #prj-2 1h Task B
```

### Desired Format
Add optional outcome tags using `##` notation:
```markdown
## TT 2025-01-15
- #prj-1 ##outcome-x 1h Task A
- #prj-2 ##outcome-y 1h Task B
```

## Requirements

### Hierarchy
```
Outcomes (##domain)
├── Projects (#prj-*)
└── Activities (#activity)
```

### Design Decisions
- **Optional outcomes** - entries without `##` tags continue to work
- **Single outcome per entry** - maximum one `##outcome` tag per entry
- **Explicit tagging only** - no mapping system for now
- **Backward compatibility** - all existing entries must continue to work

Expected Output Format

### With Outcomes:
```
Time tracking report 2020-01-01 -> 2020-01-01

Projects:
- prj-1.......... 1h 00m ( 67%)
- prj-2.......... 0h 30m ( 33%)

Outcomes:
- outcome-x....... 1h 00m ( 67%)
- outcome-y....... 0h 30m ( 33%)

1 days, 1.5 h/day,  1h 30m total
```

### Without Outcomes (unchanged):
```
Time tracking report 2020-01-01 -> 2020-01-01

Projects:
- prj-1.......... 1h 00m ( 67%)
- prj-2.......... 0h 30m ( 33%)

1 days, 1.5 h/day,  1h 30m total
```

## Implementation Plan

### Phase 1: Basic Outcome Recognition and Reporting

#### Step 1: Write Baseline Acceptance Test (Should Pass)
Create test to verify existing functionality doesn't break:

```rust
#[test]
fn app_should_ignore_outcome_tags_for_now() {
    let content = r#"## TT 2020-01-01
- #prj-a ##outcome-x 1h Task A"#;

    Cmd::given()
        .a_file_with_content(content)
        .when_run()
        .should_succeed()
        .expect_project("prj-a")
        .taking("1h 00m")
        .validate();
}
```

**Expected**: Should pass (app ignores unknown tags)

#### Step 2: Write Failing Acceptance Test for Outcome Reporting
Add test for new outcome reporting behavior:

```rust
#[test]
fn app_should_show_outcomes_in_summary() {
    let content = r#"## TT 2020-01-01
- #prj-a ##outcome-x 1h Task 1
- #activity ##outcome-y 30m Task 2"#;

    Cmd::given()
        .a_file_with_content(content)
        .when_run()
        .should_succeed()
        .expect_output("Outcomes:")
        .expect_output("outcome-x: 1h 00m")
        .expect_output("outcome-y: 0h 30m");
}
```

**Expected**: Should fail (we don't parse/report outcomes yet)

#### Step 3: Make the Test Pass
Implement minimal changes to make Step 2 pass:
- Extend parser to recognize `##outcome` tags
- Add outcome tracking to data model
- Add outcome section to report output

### Phase 2: Basic Outcome Reporting
- Add outcome summaries to existing reports
- Show time allocation per outcome alongside projects
- Maintain backward compatibility with existing report formats

## Development Guidelines

- **Start with smallest end-to-end solution** - Begin with hardcoded values, generalize once validated
- **TDD approach** - Write failing test → minimal implementation → refactor if needed
- **Maintain functionality** - Application remains functional after each step
- **Test behavior, not implementation** - Focus on observable outcomes
- **Backward compatibility** - All existing functionality must continue to work

## File Structure Context

The project uses:
- Acceptance tests in `tests/acceptance/` following matklad's recommendations
- Common test utilities in `tests/acceptance/common.rs`
- Test helper methods like `Cmd::given()`, `.expect_output()`, `.expect_project()`

## Success Criteria

After Phase 1:
1. Existing time tracking entries continue to work unchanged
2. New entries with `##outcome` tags are parsed correctly
3. Reports show outcome summaries alongside project summaries
4. No regressions in existing functionality
5. All tests pass

## Implementation Notes

- Follow existing code patterns in the test suite
- Use the established `CommandSpec` and `CommandResult` test builders
- Maintain the same error handling and warning patterns
- Keep the same report formatting style for consistency
