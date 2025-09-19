# 001: Refactor Function Signatures to Align with Rust Best Practices

**Status**: To Do  
**Priority**: High  
**Effort**: Medium

## Description

The codebase currently has numerous function signatures that do not follow idiomatic Rust conventions. This leads to unnecessary memory allocations, reduced API flexibility, and less readable code. This task focuses on refactoring these signatures to improve performance and align with best practices.

The work can be broken down into three main categories of issues identified by `clippy`.

### 1. Pass by Reference Instead of Value (`needless_pass_by_value`)

-   **Problem**: Many functions take ownership of arguments (e.g., `Vec<String>`) when they only need to read from them. This forces the caller to clone the data, which is inefficient.
-   **Solution**: Change function signatures to accept references (e.g., `&[String]`) instead of taking ownership of the value.
-   **Example**: In `src/lib.rs`, the `create_filter` function takes `exclude_tags: Vec<String>`. This should be changed to `exclude_tags: &[String]`.

### 2. Use Slice References (`&[T]`) Instead of `&Vec<T>` (`ptr_arg`)

-   **Problem**: Some functions accept `&Vec<T>` as an argument, which is overly specific. This restricts the function to only accept `Vec<T>` types.
-   **Solution**: Generalize the function signatures to accept slice references (`&[T]`), which makes the API more flexible. It can then accept any contiguous sequence of `T`, including vectors, arrays, and slices.
-   **Example**: In `src/lib.rs`, the `print_warnings` function takes `parse_errors: &Vec<ParseError>`. This should be changed to `&[ParseError]`.

### 3. Use `Option<&T>` Instead of `&Option<T>` (`ref_option`)

-   **Problem**: The code frequently uses `&Option<T>` in function arguments. This is less idiomatic and requires callers to create a reference to an `Option`.
-   **Solution**: Change the signatures to use `Option<&T>`, which allows passing an optional reference to the value *inside* the `Option`. This makes the API cleaner and more ergonomic.
-   **Example**: In `src/cli/mod.rs`, the `parse_project_tags` function takes `maybe_project: &Option<String>`. This should be changed to `Option<&String>`.
