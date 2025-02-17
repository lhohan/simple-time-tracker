use crate::domain::{ParseError, TimeEntry};

use super::{line_parser::parse_entry, model::LineEntry};

pub struct LineSpec {
    line: String,
}

pub struct LineParsingResult {
    entry: Result<TimeEntry, ParseError>,
}

impl LineSpec {
    pub fn new(line: &str) -> Self {
        LineSpec {
            line: line.to_string(),
        }
    }

    pub fn when_parsed(self) -> LineParsingResult {
        let obtained = parse_entry(LineEntry(&self.line));
        LineParsingResult { entry: obtained }
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

    pub fn expect_project(self, expected_project: &str) -> TimeEntry {
        assert_eq!(*self.main_project(), expected_project.to_string());
        self
    }

    pub fn expect_description(self, expected_description: &str) -> TimeEntry {
        assert_eq!(self.description, Some(expected_description.to_string()));
        self
    }
}
