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
