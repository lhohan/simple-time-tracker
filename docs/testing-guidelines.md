# Testing guidelines

## TDD with Primitive Whole

Follow this progression for robust development:

1. **Red**: Write simplest possible end-to-end test that fails
2. **Green**: Implement minimal solution that passes
3. **Refactor**: Extract reusable patterns while keeping tests green
4. **Iterate**: Add complexity only when needed for next feature

## Integration with Project Workflow

1. **Start with acceptance test** using CLI interface
2. **Use TempDir for file operations**
3. **Commit after each working iteration**
4. **Add unit tests only for complex business logic on stable interfaces or APIs**

## Writing tests using a domain oriented testing DSL to describe behaviour

- Write tests using a domain oriented testing DSL to describe behaviour
- Write executable specifications: the tests read like specifications and can be executed directly against the codebase to verify the behaviour

## File-Based CLI Testing with assert_fs

This pattern provides robust, isolated testing for CLI applications that process files:

```rust
use assert_cmd::Command;
use assert_fs::{
    assert::PathAssert,
    fixture::{FileWriteStr, PathChild},
    TempDir,
};
use predicates::str;

#[test]
fn cli_should_process_files() {
    let temp = TempDir::new().unwrap();
    let input_file = temp.child("input.ext");
    let output_file = temp.child("output.ext");

    input_file.write_str("test content").unwrap();

    Command::cargo_bin("binary_name")
        .unwrap()
        .arg(input_file.path())
        .arg(output_file.path())
        .assert()
        .success();

    output_file.assert(str::contains("expected content"));
    // TempDir automatically cleans up when dropped
}
```

### Benefits
- ✅ Automatic cleanup even on test failure
- ✅ Thread-safe test isolation
- ✅ No manual file management required
- ✅ Tests actual CLI interface (stable interface testing)

### Required Dependencies
```toml
[dev-dependencies]
assert_fs = "1.0"
assert_cmd = "2.0"
predicates = "3.1"
```

## Web Testing with DSL

Web endpoints are tested using a fluent Given-When-Then DSL that exercises the full HTTP stack:

```rust
#[tokio::test]
async fn dashboard_should_filter_by_today() {
    WebApp::given()
        .a_file_with_content(
            "## TT 2025-01-15\n\
             - #project-alpha 2h Work\n",
        )
        .at_date("2025-01-15")
        .when_get("/api/dashboard")
        .with_query("period=today")
        .should_succeed()
        .await
        .expect_status(200)
        .expect_contains("project-alpha")
        .expect_not_contains("old-project");
}
```

### Web Testing Principles

- **DSL-based integration tests only**: All web tests use the WebApp DSL and go through the full HTTP request/response cycle
- **No direct handler calls**: Never test handlers or templates directly - always call the server endpoints
- **Test observable behavior**: Verify HTML output contains expected content, not template implementation details
- **Edge case coverage**: Test empty data, single items, many items, zero values, large values for each endpoint

### Template Testing Strategy

Templates are tested indirectly through integration tests, not directly:

**What We Test:**
- ✅ Templates render without errors
- ✅ Expected data appears in HTML output
- ✅ Filtering affects what's displayed
- ✅ Edge cases (empty data, single items, large values)
- ✅ Both branches of template conditionals (e.g., empty vs non-empty lists)

**What We Don't Test:**
- ❌ Internal template logic (Askama generates Rust code at compile-time)
- ❌ HTML structure validation (no CSS selector assertions)
- ❌ JavaScript functionality (Chart.js initialization, HTMX attributes)

**Why Templates Show as Uncovered:**
- Askama generates Rust code from `.html` templates during macro expansion
- Coverage tools (llvm-cov) don't track macro-generated code
- Templates are excluded from coverage reports via `--ignore-filename-regex templates/`
- This is expected and acceptable - integration tests verify template behavior functionally

### Template Edge Cases to Test

For each template-backed endpoint, ensure tests cover:

1. **Data cardinality**: Empty list, single item, many items
2. **Value ranges**: Zero values, normal values, very large values (999+ hours)
3. **Filter combinations**: period filters, limit filters, date ranges, combined filters
4. **Conditional branches**: Both true and false paths for template `{% if %}` statements
5. **Time formats**: Hours only, minutes only, mixed formats

### Coverage Configuration

Coverage is run with template directory excluded:
```bash
just test-coverage  # Runs: cargo llvm-cov nextest --features web --ignore-filename-regex templates/
```

This configuration:
- Excludes `.html` template files from coverage metrics (they're not Rust source)
- Focuses coverage on actual `.rs` source files
- Generated template code is still exercised by integration tests
