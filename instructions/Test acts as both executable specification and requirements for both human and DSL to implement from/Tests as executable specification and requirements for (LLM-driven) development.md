## Initial prompt:

```

I am a software developer that cares about verifying behavior of code written and I am working on setting up a testing approach that would allow me to:

- Write tests to verify the behavoir of LLM or developer created code
- Use those tests as requirements an LLM can use to create the code

I could use your help.  Let's start conceptually by making a plan we will use to guide a project together across multiple chat sessions.

I have setup for inspiration already:

```rs time-tracker/src/parsing/parser.rs:72-181
// Excerpt from: mod tests
use crate::parsing::test_helpers::LineSpec;

    // Module to test parsing of a plain string into a TimeEntry.
    mod line_entry_parsing {
        use crate::domain::ParseError;
        use crate::parsing::parser::tests::LineSpec;
        use rstest::rstest;

        #[test]
        fn parse_simple_complete_line() {
            let input = "- #project-alpha 10m Task A";

            LineSpec::line_is(input)
                .when_parsed()
                .expect_valid()
                .expect_minutes(10)
                .expect_main_context("project-alpha")
                .expect_description("Task A");
        }

        #[test]
        fn parse_task_description_is_optional() {
            let input = "- #project-alpha 20m";

            LineSpec::line_is(input)
                .when_parsed()
                .expect_valid()
                .expect_no_description();
        }

        #[test]
        fn parse_simple_minutes() {
            let input = "- #context 10m";

            LineSpec::line_is(input)
                .when_parsed()
                .expect_valid()
                .expect_minutes(10);
        }

        #[test]
        fn parse_simple_hours() {
            let input = "- #context 2h";

            LineSpec::line_is(input)
                .when_parsed()
                .expect_valid()
                .expect_minutes(2 * 60);
        }

        #[test]
        fn parse_pomodoros() {
            let input = "- #context 2p";

            LineSpec::line_is(input)
                .when_parsed()
                .expect_valid()
                .expect_minutes(2 * 30);
        }

        #[test]
        fn parse_multiple_time_entries() {
            let input = "- #context 1h 10m 1p";

            LineSpec::line_is(input)
                .when_parsed()
                .expect_valid()
                .expect_minutes(60 + 10 + 30);
        }

        #[rstest]
        fn parse_invalid_line_format(
            #[values("- hash (#) not in start of line", "# dash (-) not in start of line")]
            input: &str,
        ) {
            LineSpec::line_is(input)
                .when_parsed()
                .expect_invalid_with(&ParseError::InvalidLineFormat(input.to_string()));
        }

        #[test]
        fn parse_invalid_time() {
            let input = "- #context 100000000000000000000h";

            LineSpec::line_is(input)
                .when_parsed()
                .expect_invalid_with(&ParseError::InvalidTime(
                    "100000000000000000000h".to_string(),
                ));
        }

        #[rstest]
        fn parse_maybe_time(#[values('h', 'm', 'p')] supported_time_unit: char) {
            let input = format!("- #context x{}", supported_time_unit);

            LineSpec::line_is(&input)
                .when_parsed()
                .expect_invalid_with(&ParseError::MissingTime(input.to_string()));
        }

        #[test]
        fn parse_time_missing() {
            let input = "- #context only description";

            LineSpec::line_is(input)
                .when_parsed()
                .expect_invalid_with(&ParseError::MissingTime(input.to_string()));
        }
    }

```
```rs time-tracker/src/parsing/test_helpers.rs
use crate::domain::{ParseError, TimeEntry};

use super::{line_parser::parse_entry, model::EntryLine};

pub struct LineSpec {
    line: String,
}

pub struct LineParsingResult {
    entry: Result<TimeEntry, ParseError>,
}

impl LineSpec {
    pub fn line_is(line: &str) -> Self {
        LineSpec {
            line: line.to_string(),
        }
    }

    pub fn when_parsed(self) -> LineParsingResult {
        let entry = EntryLine::new(&self.line).and_then(parse_entry);
        LineParsingResult { entry }
    }
}

impl LineParsingResult {
    pub fn expect_valid(self) -> TimeEntry {
        self.entry.expect("Expected time entry but was error")
    }

    pub fn expect_invalid_with(self, expected_error: &ParseError) {
        let error = self.entry.expect_err("Expected error but was valid");
        assert_eq!(error, *expected_error);
    }
}

impl TimeEntry {
    pub fn expect_minutes(self, expected_minutes: u32) -> TimeEntry {
        assert_eq!(self.minutes, expected_minutes);
        self
    }

    pub fn expect_main_context(self, expected_project: &str) -> TimeEntry {
        assert_eq!(*self.main_context(), expected_project.to_string());
        self
    }

    pub fn expect_description(self, expected_description: &str) -> TimeEntry {
        assert_eq!(self.description, Some(expected_description.to_string()));
        self
    }

    pub fn expect_no_description(self) -> TimeEntry {
        assert!(self.description.is_none());
        self
    }
}
```


What I have been thinking about is, but please consider this background informartion I would like to be open to the best way to implement such testing and implementation approach:
- an descriptive testing DSL that could serve as input to an LLM as is to implement the behavior specified (maybe with the test_helpers infrastructure?)
- or do we need to generate a requirements text file from the tests? So instead of running the tests we have an alternate way of running the DSL?

Help me clarify the goal if not clear.
```

Asked: Claude Sonnet, Gemini-2.0-flash-001 and Deepseek R1

## Goal

**Project Goal:**  Develop a system where code-based tests (using a DSL) act as both executable specification and unambiguous requirements for both human and AI to generate code that an LLM can understand and implement from.

**Workflow:**
```
Tests/Specs ─► LLM Prompt ─► Generated Code ─► Test Verification
      │                                              │
      └──────── Feedback ─────-────────┘
```

## Core Objectives & Key Properties

We need a system that ensures:

- **Test Clarity**: Tests describe requirements transparently (serving as a spec).
- **LLM-Actionable**: Tests/specs can be used or transformed into prompts/context for LLMs to generate compliant code.
- **Maintainability**: Dual use of tests as specs doesn't create maintenance overhead.
- **Validation**: Code (human/LLM) must pass all relevant tests to be accepted.

The DSL should be fluent testing DSL with clear intent. The DSL should be:
- **Single Source of Truth**: Tests and specs for implementation (by human/LLM) should automatically stay in sync.
- **Readable for humans:** Easy to understand the intent of each test.
- **Parsable by LLMs:**  Minimal ambiguity, consistent structure is key.
  - **Explicit Requirement Extraction**: Tests or requirements should be in standardized format for LLM consumption.
  - **Metadata Enrichment**: Adding descriptions/error messages tailored for LLM context.
- **Expressive:** Can cover a wide range of potential behaviors for your target functionality.
- **Separation of Concerns**: Decouple test logic from spec definition.
- **Parameterizable**: to allow for combinatorial testing (`rstest` in Rust).

## Main options

### Direct Test-to-Code Generation

Could using the DSL directly to generate code work?

*   LLM takes the DSL as input to generate the corresponding implementation
*   Pros: Simplest conceptually, avoids translation step.
*   Cons: Might require a very capable/finely tuned LLM that understands Rust and the nuances of your testing framework.

#### Advantages:

- Single source of truth
- More elegant/unified approach
- Potentially more maintainable (one system)
- Direct mapping between test and requirements

#### Challenges:

- Need to design a DSL that's both:
  1. Expressive enough for testing
  2. Clear enough for LLMs to understand
  3. Flexible enough for edge cases
- Might need complex macro magic in Rust
- Risk of creating a DSL that's too domain-specific
- Harder to evolve as requirements change

### Test-to-Requirements-to-Code Generation

Generate requirements text file from test:

*   Take the DSL and generate an alternate representation, like a plain text requirements file.
  *   LLM takes that Requirements file as input and generates the code.
*   Pros: Potentially easier for LLMs to understand high-level requirements.
*   Cons: Introduces a translation step (Tests -> Requirements), potentially losing information.  Also could be hard to keep in sync.

- **Human-Centric DSL**: Keep your fluent Rust-based test builders for developers.
- **LLM-Centric Specs**: Generate a structured doc (JSON/YAML) from tests, containing:
  ```yaml
  - description: "Parsing a task with hours and minutes"
    input: "- #project 1h 10m"
    expectations:
      valid: true
      minutes: 70
      project: "project"
  - description: "Error on missing time"
    input: "- #project just a description"
    expected_error: "MissingTime"
  ```
This becomes LLM prompt context.

#### Advantages

1. **Lower Initial Risk**
   - Can build on your existing test infrastructure
   - Easier to experiment with LLM interaction formats
   - Can evolve the DSL gradually

2. **Separation of Concerns**
   - Testing logic can be optimized for testing
   - LLM specs can be optimized for LLM understanding
   - Easier to adapt each independently

3. **Practical Experience**
   - Many LLMs work better with structured formats (JSON/YAML)
   - Translation layer gives flexibility in LLM prompting

## Implementation options

### Enhanced Testing DSL

Some examples:

Example 1:

```rust
#[specification]  // New attribute to mark tests as LLM requirements
#[test]
fn parse_simple_complete_line() {
    Spec::define("Parse a time entry with project, duration, and description")
        .given("- #project-alpha 10m Task A")
        .should("parse into a valid time entry")
        .with(|entry| {
            entry.has_minutes(10)
                .has_context("project-alpha")
                .has_description("Task A")
        });
}
```

Example 2:

```rust
#[behavior_spec(
    description = "Parsing time entries with minutes",
    category = "basic_parsing"
)]
#[test]
fn parse_simple_minutes() {
    TimeEntrySpec::new()
        .given("- #context 10m")
        .should("parse into valid entry with 10 minutes")
        .when_parsed()
        .expect_valid()
        .expect_minutes(10);
}
```

### Separate Requirements DSL

Defining the valid structure of the DSL will REALLY help when you start interacting with LLMs. Can be Informal like a text description or formal like a context free grammar.

Option 1:

```yaml
     feature: "Time Entry Parsing"
     scenario: "Parse simple complete line"
     given: "- #project-alpha 10m Task A"
     expect:
       - valid_entry:
           minutes: 10
           context: "project-alpha"
           description: "Task A"
```

Option 2:

```yaml
  - description: "Parsing a task with hours and minutes"
    input: "- #project 1h 10m"
    expectations:
      valid: true
      minutes: 70
      project: "project"
  - description: "Error on missing time"
    input: "- #project just a description"
    expected_error: "MissingTime"
```

Option 3:

In case the line is: `- #context 1d` the input for the LLM becomes:

```
"The system MUST be able to parse durations in days (e.g., '1d') and convert them to minutes.  A duration of '1d' MUST be equivalent to 1440 minutes."
```

## Current approach

We start with a pure DSL approach, keeping in mind to prevent from becoming overly ambitious:

1. Start small with core functionality
2. Ensure the DSL is extensible
3. Keep LLM comprehension in mind while designing
4. Add complexity gradually

## Next steps

**Phase 1: Enhance Test DSL for Interpreation by LLM or Spec Extraction**
- Identify DSL limitations: What can't your current DSL express? We'll add features or modify the syntax as need it. (e.g., Handling edge cases, error conditions, complex interactions).
- Add metadata (descriptions, tags) to tests.

This seems like a great first step as we need to evaluate the DSL in either option we take.

**Phase 2: LLM Integration Pipeline**
- Set up feedback loop: Run LLM-generated code against tests, report failures.

**Phase 3: Expand Test Coverage & Error Cases**
- Use combinatorial testing (like `rstest`) to generate edge cases.
- Ensure specs capture domain logic exhaustively.
