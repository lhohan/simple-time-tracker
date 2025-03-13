RULES
- The LLM should write code based on function signature and the tests ONLY.
- Compilation errors should be fixed by the LLM
- Failing tests should be fixed by changing the tests (description).

## Attempt 1 - no changes to the tests

Prompt:

```

Implement a Rust parser that passes the following tests:

```rs time-tracker/src/parsing/parser.rs:75-180
// Excerpt from: mod tests > mod line_entry_parsing
{
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

use super::line_parser::parse_entry;

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
        let entry = parse_entry(&self.line);
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



The parser should:
- Take a string input
- Return TimeEntry or ParseError
- Follow the expectations in the test

Here is the function signature:
`fn parse_entry(line: &str) -> Result<TimeEntry, ParseError>`

`TimeEntry` and `ParseError` are already defined in
```rs time-tracker/src/domain/mod.rs
pub mod dates;
pub mod reports;
pub mod time;

pub use dates::range::{DateRange, PeriodRequested};
pub use reports::{RangeDescription, TimeTrackingResult, TrackedTime, TrackingPeriod};

#[derive(Debug, PartialEq, Clone)]
pub struct TimeEntry {
    projects: Vec<String>,
    pub minutes: u32,
    pub description: Option<String>,
}

impl TimeEntry {
    #[must_use]
    pub fn new(projects: Vec<String>, minutes: u32, description: Option<String>) -> Self {
        Self {
            projects,
            minutes,
            description,
        }
    }

    #[must_use]
    pub fn main_context(&self) -> &String {
        let tags = &self.projects;
        let maybe_project = tags.iter().find(|t| t.starts_with("prj-"));
        maybe_project.unwrap_or_else(|| &self.projects[0])
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub file: String,
    pub line: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    ErrorReading(String),
    InvalidLineFormat(String),
    InvalidTime(String),
    InvalidDate(String),
    MissingTime(String),
    InvalidPeriod(String),
    Located {
        error: Box<ParseError>,
        location: Location,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidLineFormat(line) => write!(f, "invalid line format: {line}"),
            ParseError::InvalidTime(time) => write!(f, "invalid time format: {time}"),
            ParseError::InvalidDate(date) => write!(f, "invalid date format: {date}"),
            ParseError::MissingTime(line) => write!(f, "missing time: {line}"),
            ParseError::ErrorReading(file) => write!(f, "error reading file: {file}"),
            ParseError::InvalidPeriod(period) => write!(f, "invalid period: {period}"),
            ParseError::Located { error, location } => {
                write!(f, "{}: line {}: {}", location.file, location.line, error)
            }
        }
    }
}

impl std::error::Error for ParseError {}
```



Make sure to follow Rust best practices and conventions and take into account:
```md time-tracker/CONVENTIONS.md
# Development Guidelines

## 1. Evolution Strategy

- Begin with the smallest end-to-end solution that works
- Start with hardcoded values; generalize once validated
- Separate feature additions from refactoring

## 2. Testing Approach

- Verify observable behaviors over implementation mechanics
- Compose tests using reusable builders and utilities
- Assert outcomes through domain language

## 3. Error Handling

- Design error types to mirror domain failure modes
- Preserve error sources when propagating upward
- Validate early, handle centrally

## 4. Domain Modeling

- Create distinct types for domain concepts
- Express business rules through type relationships
- Derive aggregate properties at creation time

## 5. Code Structure

- Group code by user-facing capabilities
- Separate sequential processing stages
- Favor immutable data transformations

## 6. Rust Specifics

- Enforce valid states through type constraints
- Design expressive fluent interfaces
- Encode business logic in type definitions rather than runtime checks
  - Use types as guardrails rather than writing if checks scattered through business logic

## 7. Documentation

- Lead with concrete usage examples
- Anchor documentation near usage context
```


If you have questions or need clarfication, let me know!
```
