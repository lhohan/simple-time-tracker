pub mod dates;
pub mod reporting;
pub mod tags;
pub mod time;
use std::collections::VecDeque;

pub use dates::range::{DateRange, PeriodRequested};
pub use reporting::{PeriodDescription, TimeTrackingResult, TrackedTime, TrackingPeriod};
use tags::Tag;

#[derive(Debug, PartialEq, Clone)]
pub struct TimeEntry {
    tags: Vec<Tag>,
    pub minutes: u32,
    pub description: Option<String>,
    pub outcome: Option<Outcome>,
}

impl TimeEntry {
    /// Parses a line into a time entry, if it represents a valid entry.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the line appears to be an entry but contains invalid data.
    pub fn parse(line: &str) -> Result<Option<TimeEntry>, ParseError> {
        let line = EntryLine::parse(line);
        match line {
            Some(line) => parse_line(&line).map(Some),
            None => Ok(None),
        }
    }

    /// Returns the main context (first tag) of this time entry.
    ///
    /// This method is guaranteed to succeed for valid entries since the parser
    /// ensures that all entries have at least one non-empty tag.
    #[must_use]
    pub fn main_context(&self) -> String {
        self.tags
            .first()
            .expect("TimeEntry must have at least one tag (validated during parsing)")
            .raw_value()
            .to_string()
    }

    #[must_use]
    pub fn get_tags(&self) -> &[Tag] {
        &self.tags
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Outcome(String);
impl Outcome {
    #[must_use]
    pub fn new(description: String) -> Self {
        Outcome(description)
    }

    #[must_use]
    pub fn description(&self) -> &str {
        &self.0
    }
}

struct EntryLine<'a>(pub(crate) &'a str);

impl EntryLine<'_> {
    pub(crate) fn parse(line: &str) -> Option<EntryLine> {
        if EntryLine::is_line_entry(line) {
            Some(EntryLine(line))
        } else {
            None
        }
    }

    fn is_line_entry(line: &str) -> bool {
        line.starts_with("- #")
    }

    pub(crate) fn get_line(&self) -> &str {
        self.0
    }

    /// Returns the actual content of the line, without the prefix that identifies the line as an entry line.
    ///
    /// # Panics
    ///
    /// Panics if the entry line does not start with "- " prefix, which indicates invalid struct state.
    pub(crate) fn entry(&self) -> &str {
        self.0.strip_prefix("- ").expect("invalid struct state")
    }
}

fn parse_line(entry_line: &EntryLine) -> Result<TimeEntry, ParseError> {
    let line_no_prefix = entry_line.entry();
    let parts = line_no_prefix.split_whitespace();

    let mut projects = VecDeque::new();
    let mut minutes = 0;
    let mut description = Vec::new();
    let mut time_found = false;
    let mut multiple_outcomes_found = false;
    let mut outcome = None;

    for part in parts {
        match parse_part(part) {
            Ok(LinePart::Time(time)) => {
                minutes += time;
                time_found = true;
            }
            Ok(LinePart::Tag(project_found)) => {
                projects.push_back(project_found);
            }
            Ok(LinePart::Outcome(outcome_found)) => {
                if outcome.is_some() {
                    multiple_outcomes_found = true;
                }
                outcome = Some(Outcome::new(outcome_found));
            }
            Ok(LinePart::DescriptionPart(desc)) => description.push(desc),
            Err(err) => return Err(err),
        }
    }

    if !time_found {
        return Err(ParseError::MissingTime(entry_line.get_line().to_string()));
    }

    if multiple_outcomes_found {
        return Err(ParseError::MultipleOutcomes(
            entry_line.get_line().to_string(),
        ));
    }

    let description =
        (!description.is_empty()).then(|| description.into_iter().collect::<Vec<_>>().join(" "));
    let projects: Vec<Tag> = projects.into();

    let tags = projects;
    if tags.is_empty() || tags[0].raw_value().is_empty() {
        return Err(ParseError::MissingProject(
            entry_line.get_line().to_string(),
        ));
    }

    Ok(TimeEntry {
        tags,
        minutes,
        description,
        outcome,
    })
}

enum LinePart<'a> {
    Time(u32),
    Tag(Tag),
    Outcome(String),
    DescriptionPart(&'a str),
}

fn parse_part(part: &str) -> Result<LinePart, ParseError> {
    if part.starts_with("##") {
        let outcome = part
            .strip_prefix("##")
            .expect("outcome should have had '##' prefix");
        Ok(LinePart::Outcome(outcome.to_string()))
    } else if part.starts_with('#') {
        let raw_tag = part
            .strip_prefix("#")
            .expect("project should have had '#' prefix");
        let tag = Tag::from_raw(raw_tag);
        Ok(LinePart::Tag(tag))
    } else {
        match parse_time(part) {
            Ok(Some(minutes)) => Ok(LinePart::Time(minutes)),
            Ok(None) => Ok(LinePart::DescriptionPart(part)),
            Err(err) => Err(err),
        }
    }
}
use std::str::FromStr;

fn parse_time(time: &str) -> Result<Option<u32>, ParseError> {
    let (value, multiplier) = match time.chars().last() {
        Some('m') => (time.trim_end_matches('m'), 1),
        Some('h') => (time.trim_end_matches('h'), 60),
        Some('p') => (time.trim_end_matches('p'), 30),
        _ => return Ok(None),
    };

    match u32::from_str(value) {
        Ok(val) => Ok(Some(val * multiplier)),
        Err(e) => match e.kind() {
            std::num::IntErrorKind::InvalidDigit => Ok(None),
            _ => Err(ParseError::InvalidTime(time.to_string())),
        },
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
    MissingProject(String),
    MultipleOutcomes(String),
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
            ParseError::MissingProject(line) => write!(f, "missing project: {line}"),
            ParseError::ErrorReading(file) => write!(f, "error reading file: {file}"),
            ParseError::InvalidPeriod(period) => write!(f, "invalid period: {period}"),
            ParseError::MultipleOutcomes(line) => write!(f, "multiple outcomes: {line}"),
            ParseError::Located { error, location } => {
                write!(f, "{}: line {}: {}", location.file, location.line, error)
            }
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    mod line_entry_parsing {
        use crate::domain::ParseError;
        use rstest::rstest;
        use spec::LineSpec;

        #[test]
        fn parse_simple_complete_line() {
            let _ = LineSpec::given_line("- #project-alpha 10m Task A")
                .when_parsed()
                .expect_valid_entry()
                .expect_minutes(10)
                .expect_context("project-alpha")
                .expect_description("Task A");
        }

        #[test]
        fn parse_task_description_is_optional() {
            let _ = LineSpec::given_line("- #project-alpha 20m")
                .when_parsed()
                .expect_valid_entry()
                .expect_no_description();
        }

        #[test]
        fn parse_simple_minutes() {
            let _ = LineSpec::given_line("- #context 15m")
                .when_parsed()
                .expect_valid_entry()
                .expect_minutes(15);
        }

        #[test]
        fn parse_simple_hours() {
            let _ = LineSpec::given_line("- #context 2h")
                .when_parsed()
                .expect_valid_entry()
                .expect_minutes(2 * 60);
        }

        #[test]
        fn parse_pomodoros() {
            let _ = LineSpec::given_line("- #context 2p")
                .when_parsed()
                .expect_valid_entry()
                .expect_minutes(2 * 30);
        }

        #[test]
        fn parse_multiple_time_entries() {
            let _ = LineSpec::given_line("- #context 1h 15m 1p")
                .when_parsed()
                .expect_valid_entry()
                .expect_minutes(60 + 15 + 30);
        }

        #[rstest]
        fn parse_non_entries(
            #[values(
                "- hash (#) not in start of line",
                "# dash (-) not in start of line",
                "", // empty line
                " ", // whitespace line
                "some text", // text line
                "* #not_a_tag", // alternate bullet not considered entry
                "+ #not_a_tag", // alternate bullet not considered entry
            )]
            line: &str,
        ) {
            LineSpec::given_line(line)
                .when_parsed()
                .expect_not_an_entry_and_not_an_error();
        }

        #[test]
        fn parse_invalid_time() {
            LineSpec::given_line("- #context 100000000000000000000h")
                .when_parsed()
                .expect_invalid_with(&ParseError::InvalidTime(
                    "100000000000000000000h".to_string(),
                ));
        }

        #[rstest]
        fn parse_invalid_times_ending_in_time_unit(
            #[values('h', 'm', 'p')] supported_time_unit: char,
        ) {
            let input = format!("- #context x{supported_time_unit}");

            LineSpec::given_line(&input)
                .when_parsed()
                .expect_invalid_with(&ParseError::MissingTime(input.to_string()));
        }

        #[test]
        fn parse_time_missing() {
            let input = "- #context only description";

            LineSpec::given_line(input)
                .when_parsed()
                .expect_invalid_with(&ParseError::MissingTime(input.to_string()));
        }

        #[test]
        fn parse_project_missing() {
            let input = "- # 30m only description";

            LineSpec::given_line(input)
                .when_parsed()
                .expect_invalid_with(&ParseError::MissingProject(input.to_string()));
        }

        #[test]
        fn parse_outcome() {
            let input = "- #project ##my-outcome 1h";

            let _ = LineSpec::given_line(input)
                .when_parsed()
                .expect_valid_entry()
                .expect_outcome("my-outcome");
        }
        #[test]
        fn parse_outcome_should_fail_when_multiple_outcomes() {
            let input = "- #project ##my-outcome ##another-outcome 1h";

            LineSpec::given_line(input)
                .when_parsed()
                .expect_invalid_with(&ParseError::MultipleOutcomes(input.to_string()));
        }

        #[test]
        fn parse_outcome_when_outcome_is_missing() {
            let input = "- #project 1h";

            let _ = LineSpec::given_line(input)
                .when_parsed()
                .expect_valid_entry()
                .expect_no_outcome();
        }

        #[test]
        fn parse_outcome_first_hash_in_line() {
            let input = "- ##my-outcome #project 1h";

            let _ = LineSpec::given_line(input)
                .when_parsed()
                .expect_valid_entry()
                .expect_outcome("my-outcome");
        }

        mod tag_parsing {
            use super::*;

            #[test]
            fn parse_line_with_project_prefix_tag() {
                let _ = LineSpec::given_line("- #prj-alpha 1h Task A")
                    .when_parsed()
                    .expect_valid_entry()
                    .expect_context("prj-alpha");
            }

            #[test]
            fn parse_line_with_project_and_tags() {
                let _ = LineSpec::given_line("- #tag1 #prj-alpha 1h Task A")
                    .when_parsed()
                    .expect_valid_entry()
                    .expect_context("tag1");
            }

            #[test]
            fn parse_line_with_only_context_tags() {
                let _ = LineSpec::given_line("- #tag1 #tag2 #tag3 1h Task A")
                    .when_parsed()
                    .expect_valid_entry()
                    .expect_context("tag1");
            }
        }

        mod spec {
            use crate::domain::{Outcome, ParseError, TimeEntry};

            pub struct LineSpec {
                line: String,
            }

            impl LineSpec {
                pub fn given_line(line: &str) -> Self {
                    LineSpec {
                        line: line.to_string(),
                    }
                }

                pub fn when_parsed(self) -> LineParsingResult {
                    let entry = TimeEntry::parse(&self.line);
                    LineParsingResult { entry }
                }
            }

            pub struct LineParsingResult {
                entry: Result<Option<TimeEntry>, ParseError>,
            }

            impl LineParsingResult {
                /// Asserts that this line parsing result is a valid time entry and returns it.
                ///
                /// # Panics
                ///
                /// Panics if the parsing result is an error or if no time entry was found.
                #[must_use]
                pub fn expect_valid_entry(self) -> TimeEntry {
                    self.entry
                        .expect("Expected time entry but was error")
                        .expect("Expected time entry but was not")
                }

                /// Asserts that this line parsing result is neither an entry nor an error.
                ///
                /// # Panics
                ///
                /// Panics if the parsing result is an error or if a time entry was found.
                pub fn expect_not_an_entry_and_not_an_error(self) {
                    let maybe_entry = self.entry.expect("Expected no entry but is error");
                    assert_eq!(maybe_entry, None);
                }

                /// Asserts that this line parsing result is an error matching the expected error.
                ///
                /// # Panics
                ///
                /// Panics if the parsing result is valid or if the error doesn't match the expected error.
                pub fn expect_invalid_with(self, expected_error: &ParseError) {
                    let error = self.entry.expect_err("Expected error but was valid");
                    assert_eq!(error, *expected_error);
                }
            }

            impl TimeEntry {
                /// Asserts that this time entry has the expected number of minutes.
                ///
                /// # Panics
                ///
                /// Panics if the minutes don't match the expected value.
                #[must_use]
                pub fn expect_minutes(self, expected_minutes: u32) -> TimeEntry {
                    assert_eq!(self.minutes, expected_minutes);
                    self
                }

                /// Asserts that this time entry has the expected main context.
                ///
                /// # Panics
                ///
                /// Panics if the main context doesn't match the expected value.
                #[must_use]
                pub fn expect_context(self, expected_project: &str) -> TimeEntry {
                    assert_eq!(*self.main_context(), expected_project.to_string());
                    self
                }

                /// Asserts that this time entry has the expected description.
                ///
                /// # Panics
                ///
                /// Panics if the description doesn't match the expected value.
                #[must_use]
                pub fn expect_description(self, expected_description: &str) -> TimeEntry {
                    assert_eq!(self.description, Some(expected_description.to_string()));
                    self
                }

                /// Asserts that this time entry has no description.
                ///
                /// # Panics
                ///
                /// Panics if the time entry has a description.
                #[must_use]
                pub fn expect_no_description(self) -> TimeEntry {
                    assert!(self.description.is_none());
                    self
                }

                /// Asserts that this time entry has the expected outcome.
                ///
                /// # Panics
                ///
                /// Panics if the outcome doesn't match the expected value.
                #[must_use]
                pub fn expect_outcome(self, expected_outcome: &str) -> TimeEntry {
                    assert_eq!(
                        self.outcome,
                        Some(Outcome::new(expected_outcome.to_string()))
                    );
                    self
                }

                /// Asserts that this time entry has no outcome.
                ///
                /// # Panics
                ///
                /// Panics if the time entry has an outcome.
                #[must_use]
                pub fn expect_no_outcome(self) -> TimeEntry {
                    assert!(self.outcome.is_none());
                    self
                }
            }
        }
    }

    mod parser_error_handling {
        use crate::domain::ParseError;
        #[test]
        fn error_messages() {
            assert_eq!(
                ParseError::InvalidTime("Xh".to_string()).to_string(),
                "invalid time format: Xh"
            );
        }

        #[test]
        fn error_conversion() {
            let err = ParseError::InvalidTime("a".to_string());
            let _: Box<dyn std::error::Error> = Box::new(err); // Should compile
        }
    }
}
