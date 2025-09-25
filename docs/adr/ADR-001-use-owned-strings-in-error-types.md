# ADR-001: Use Owned Strings in Error Types

## Status

**Accepted** - 2024-09-25

## Context

During development of the time tracking CLI application, we implemented a lifetime-based optimization in our error handling system to avoid string allocations. The `ParseError<'a>` enum used borrowed string references where possible, with a `StaticParseError` type alias for the common case of owned strings and a `to_static()` method for conversion between the two.

This approach introduced significant API complexity:

1. **Dual error types**: Callers needed to choose between `ParseError<'a>` and `StaticParseError`, creating confusion about which to use
2. **Lifetime propagation**: Functions returning parse errors had to propagate lifetimes through their signatures, complicating the API
3. **Conversion overhead**: The `to_static()` method required careful usage and added cognitive overhead for developers
4. **Test complexity**: Testing code needed to handle both lifetime variants, as seen in the extensive use of `StaticParseError` in test cases

The lifetime optimization was motivated by a desire to avoid string allocations in error paths. However, analysis revealed that:

- Error paths are not performance-critical in this application
- Parse errors are rare events that don't impact overall application performance
- The complexity cost significantly outweighed the marginal performance benefit

## Decision

**Revert to always using owned strings in ParseError for API simplicity.**

We will:

1. Remove the lifetime parameter from `ParseError<'a>`, making it `ParseError` with owned `String` fields
2. Remove the `StaticParseError` type alias
3. Remove the `to_static()` conversion method
4. Accept the small allocation cost in error paths as an acceptable trade-off for API simplicity

## Rationale

### Alternatives Considered

1. **Keep lifetime-based optimization**: Rejected due to API complexity concerns and questionable performance benefits in error paths
2. **Use `Cow<str>` (Clone on Write)**: Would still require lifetime management in some cases and add different complexity
3. **Custom error types with intern strings**: Over-engineered for our use case and would introduce additional dependencies

### Trade-offs

**Benefits of reverting to owned strings:**
- **Simplified API**: Single error type eliminates choice paralysis and lifetime complexity
- **Easier testing**: No need to handle multiple error type variants in tests
- **Better ergonomics**: Functions don't need to propagate lifetimes through error handling
- **Reduced cognitive load**: Developers can focus on business logic rather than lifetime management

**Costs of reverting:**
- **Minor performance impact**: String allocations in error paths (acceptable since errors are rare)
- **Slightly increased memory usage**: Owned strings vs borrowed references (negligible impact)

The performance cost is acceptable because:
- Parse errors are exceptional cases, not hot paths
- String allocation cost is minimal compared to file I/O and parsing work
- Application performance is dominated by parsing logic, not error handling

## Consequences

### Positive Consequences

1. **Cleaner codebase**: Removal of lifetime complexity will make the code more maintainable
2. **Faster development**: Developers won't need to reason about error lifetime management
3. **Simplified testing**: Test code becomes more straightforward with single error type
4. **Better API usability**: Library consumers have a simpler interface to work with

### Negative Consequences

1. **Minor performance regression**: Additional allocations when errors occur (minimal impact)
2. **Memory overhead**: Owned strings use more memory than borrowed references (negligible)

### Migration Impact

- Update all `ParseError<'a>` references to `ParseError`
- Remove `StaticParseError` usage throughout codebase
- Simplify function signatures by removing unnecessary lifetime parameters
- Update tests to use the unified error type

## Verification

This decision's success will be measured by:

### Quantitative Metrics
- **Code complexity reduction**: Measure reduction in lifetime annotations and type aliases
- **Compilation time**: No significant increase in build times despite additional allocations
- **Test coverage maintenance**: All error handling tests continue to pass after migration

### Qualitative Indicators
- **Developer experience**: Easier onboarding and contribution to error handling code
- **API clarity**: Function signatures become more readable without lifetime parameters
- **Maintainability**: Fewer concepts to understand when working with error types

### Success Criteria (3-month evaluation)
- Zero developers report confusion about which error type to use
- No performance regressions in actual CLI usage (parsing times remain consistent)
- Error handling code changes require fewer iterations in code review

### Warning Signs
- If parse performance degrades significantly (>10% regression in parsing benchmarks)
- If memory usage grows beyond acceptable limits in production scenarios
- If developers request return to lifetime-optimized approach

This decision prioritizes developer productivity and code maintainability over micro-optimizations in non-critical error paths, aligning with our project's focus on sustainable, readable code.