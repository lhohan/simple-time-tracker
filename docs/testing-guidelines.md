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
