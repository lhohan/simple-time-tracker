TO REVIEW - Generated Deepseek R1

# Iterative Implementation Plan

## Phase 0: Quick Feedback Loop (1 session)

**Goal**: Validate if current tests can serve as LLM requirements with minimal changes

**Session 0.1: Initial Prompt Test**
1. Take existing test code as-is:
```rust
#[test]
fn parse_simple_minutes() {
    let input = "- #context 10m";
    LineSpec::line_is(input)
        .when_parsed()
        .expect_valid()
        .expect_minutes(10);
}
```
2. Create raw prompt:
```
Implement a Rust parser that passes these tests:

${TEST_CODE}

The parser should:
- Take a string input
- Return TimeEntry or ParseError
- Follow the expectations in tests
```
3. Generate code with LLM
4. Run tests against generated code
5. Analyze failures

**Expected Outcomes**:
- Discover what test aspects LLMs misunderstand
- Identify critical DSL enhancements needed
- Validate core hypothesis

## Phase 1: Minimal DSL Enhancements (1-2 sessions)

**Session 1.1: Essential Clarity Additions**
Based on Phase 0 findings, add critical improvements:
```rust
TimeEntrySpec::new()
    .describe("Parsing basic minutes format")
    .given("- #context 10m")
    .expect_valid()
    .expect_minutes(10);
```

**Possible Enhancements**:
- Add `.describe()` method
- Standardize expectation method names
- Simplify chain where possible

**Session 1.2: Second Feedback Cycle**
1. Update prompt with enhanced DSL
2. Generate code
3. Compare failure patterns with Phase 0
4. Document LLM misunderstanding patterns

## Phase 2: Targeted DSL Refinement (2 sessions)

**Session 2.1: Error Handling Improvements**
Address common LLM misunderstandings:
```rust
.expect_error(ParseError::InvalidTime)
    .with_message_containing("invalid time format")
```

**Session 2.2: Contextual Documentation**
Add requirements explanations:
```rust
.with_rule("1h = 60 minutes")
.with_rule("Time must come after context")
```

**Session 2.3: Third Feedback Cycle**
1. Test refined DSL
2. Measure improvement in LLM success rate
3. Identify remaining gaps

## Phase 3: Full Integration (1-2 sessions)

**Session 3.1: Automation Pipeline**
Build:
- Test case extraction
- Prompt templating
- Code validation

**Session 3.2: Final Validation**
Run full test suite against LLM-generated code
Document coverage metrics

## Key Advantages
1. **Early Validation**: Get LLM feedback before heavy DSL investment
2. **Data-Driven Improvements**: Base DSL changes on actual LLM behavior
3. **Risk Reduction**: Fail fast if core approach is unworkable
4. **Iterative Refinement**: Make targeted improvements where needed

## Next Steps
1. Run Phase 0 with current tests
2. Analyze results
3. Decide on first DSL changes

Shall we start with Phase 0? I can:
1. Help format the initial prompt
2. Analyze the LLM output
3. Identify failure patterns
```

This approach lets us validate our core assumption ("Can LLMs understand our test format?") within 1-2 chat exchanges while still leaving room for systematic improvements.


# Detailed Implementation Plan: Test-Driven Development with LLM Integration

## Overview

Building a system where a testing DSL serves as both executable tests and clear requirements for LLMs to implement code against. The DSL will be the single source of truth - expressed through a clear, fluent interface.

## Core Architecture

```
Test DSL <──► Test Runner
   ▲              │
   │              ▼
   └──── LLM ←── Test Results
```

## Implementation Phases

### Phase 1: Enhanced DSL Foundation (2-3 sessions)

#### Session 1: DSL Core Enhancement
Starting point: Current `LineSpec` implementation
```rust
LineSpec::line_is(input)
    .when_parsed()
    .expect_valid()
    .expect_minutes(10)
```

Goals:
1. Create more expressive fluent interface:
```rust
TimeEntrySpec::new()
    .describing("Time entry parsing with explicit minutes")
    .given("- #context 10m")
    .should("parse into valid entry with 10 minutes")
    .when_parsed()
    .expect_valid()
    .expect_minutes(10)
```

Tasks:
- [ ] Define new `TimeEntrySpec` builder focusing on clarity
- [ ] Add documentation methods (`describing`, `should`)
- [ ] Implement expectation methods
- [ ] Migrate existing tests
- [ ] Add helper methods for common patterns

#### Session 2: Edge Cases & Error Handling
Enhance DSL to better express error conditions and edge cases:

```rust
TimeEntrySpec::new()
    .describing("Handling invalid time formats")
    .given("- #context invalidm")
    .should("reject invalid time format with clear error")
    .when_parsed()
    .expect_error(ParseError::InvalidTime)
    .with_message("'invalidm' is not a valid time format")
```

Tasks:
- [ ] Add error expectation builders
- [ ] Implement detailed error messaging
- [ ] Add combination testing support (`rstest`)
- [ ] Document error scenarios

#### Session 3: Rich Test Context
Add rich context through the fluent interface:

```rust
TimeEntrySpec::new()
    .describing("Compound duration parsing")
    .with_rule("Hours (h) are converted to minutes (*60)")
    .with_rule("Pomodoros (p) are converted to minutes (*30)")
    .given("- #context 1h 30m")
    .should("combine multiple duration units")
    .when_parsed()
    .expect_valid()
    .expect_minutes(90)
```

Tasks:
- [ ] Add context building methods
- [ ] Implement rule specification
- [ ] Add example validation
- [ ] Create test documentation extractor

### Phase 2: LLM Integration (2-3 sessions)

#### Session 4: LLM Prompt Engineering
Design prompt structure using DSL tests directly:

```rust
// Example prompt template
"""
Implement a parser that satisfies these test cases:

${formatted_test_cases}

The implementation should:
1. Match the exact behavior shown in tests
2. Handle all error cases explicitly
3. Follow Rust best practices

Required types:
${type_definitions}
"""
```

Tasks:
- [ ] Design prompt template
- [ ] Create test case formatter
- [ ] Implement type extraction
- [ ] Build prompt generator

#### Session 5: Implementation & Validation
Create pipeline for:
1. Extracting test cases
2. Generating prompts
3. Getting LLM response
4. Validating generated code

Tasks:
- [ ] Build test extraction tool
- [ ] Implement code validation
- [ ] Create feedback mechanism
- [ ] Add error reporting

### Phase 3: Refinement & Extension (1-2 sessions)

#### Session 6: Test Generation & Coverage
Enhance test coverage through systematic generation:

```rust
TimeEntrySpec::new()
    .describing("Generated time format combinations")
    .testing_combinations(|c| {
        c.with_units(&["h", "m", "p"])
         .with_quantities(&[1, 2, 60])
         .with_descriptions(&[None, Some("Task")])
    })
```

Tasks:
- [ ] Implement combination testing helpers
- [ ] Add property-based testing integration
- [ ] Create coverage analysis tools
- [ ] Document generation patterns

## Success Criteria

1. **DSL Clarity**
   - Tests are self-documenting through fluent interface
   - Intent is clear without external documentation
   - Error cases are explicit and descriptive

2. **LLM Effectiveness**
   - LLM can understand test intent from DSL structure
   - Generated code passes tests
   - Error feedback is actionable

3. **Maintainability**
   - Adding new tests follows consistent patterns
   - DSL changes preserve test clarity
   - Test intent stays visible in code

## Risk Mitigation

1. **DSL Complexity**
   - Keep method names descriptive and consistent
   - Focus on essential behaviors first
   - Build complexity gradually

2. **LLM Understanding**
   - Test with multiple prompt formats
   - Gather feedback on failures
   - Iterate on test clarity

3. **Testing Coverage**
   - Use systematic test generation
   - Focus on edge cases
   - Validate error handling

## Next Immediate Steps

1. Start with Session 1: DSL Core Enhancement
   - Design initial `TimeEntrySpec` interface
   - Implement core fluent methods
   - Convert simple test cases
   - Review and refine

2. Review results and adjust plan as needed
   - Validate test clarity
   - Check LLM understanding
   - Assess implementation complexity
```

The revised plan removes all macro-based complexity and focuses on building a clear, expressive fluent interface. Each phase builds on the previous one, starting with core functionality and adding features as needed.

Shall we begin with implementing the new `TimeEntrySpec` builder?
