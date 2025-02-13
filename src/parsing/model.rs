use crate::domain::{ParseError, TimeEntry};
use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub(crate) struct ParseState {
    pub(crate) entries: HashMap<NaiveDate, Vec<TimeEntry>>,
    pub(crate) current_date: Option<NaiveDate>,
    pub(crate) errors: Vec<ParseError>,
}

impl ParseState {
    pub(crate) fn in_time_tracking_section(&self) -> bool {
        self.current_date.is_some()
    }
}

pub(crate) struct ParsedLine<'a> {
    pub(crate) content: &'a str,
    pub(crate) line_number: usize,
}

pub(crate) enum LineType {
    Header(Option<NaiveDate>),
    Entry(TimeEntry),
    Other,
}

#[derive(Debug, PartialEq)]
pub struct ParseResult {
    errors: Vec<ParseError>,
    days: u32,
    entries: Option<HashMap<NaiveDate, Vec<TimeEntry>>>,
}

impl ParseResult {
    #[must_use]
    pub fn new(entries: HashMap<NaiveDate, Vec<TimeEntry>>, errors: Vec<ParseError>) -> Self {
        Self {
            errors,
            days: u32::try_from(entries.len()).unwrap_or(0),
            entries: Some(entries),
        }
    }

    #[must_use]
    pub fn errors_only(errors: Vec<ParseError>) -> Self {
        Self {
            errors,
            days: 0,
            entries: None,
        }
    }

    #[must_use]
    pub fn errors(&self) -> Vec<ParseError> {
        self.errors.clone()
    }

    #[must_use]
    pub fn days(&self) -> u32 {
        self.days
    }

    #[must_use]
    pub fn entries_by_date(&self) -> Option<&HashMap<NaiveDate, Vec<TimeEntry>>> {
        self.entries.as_ref()
    }

    #[must_use]
    pub fn merge(&self, other: &ParseResult) -> ParseResult {
        // Merge errors
        let mut merged_errors = self.errors.clone();
        merged_errors.extend(other.errors.clone());

        // Merge entries
        let merged_entries = match (self.entries.as_ref(), other.entries.as_ref()) {
            (Some(first_entries), Some(second_entries)) => {
                let mut merged = first_entries.clone();
                // Merge entries from second
                for (date, entries) in second_entries {
                    merged
                        .entry(*date)
                        .or_insert_with(Vec::new)
                        .extend(entries.iter().cloned());
                }
                Some(merged)
            }
            (Some(entries), None) | (None, Some(entries)) => Some(entries.clone()),
            (None, None) => None,
        };

        match merged_entries {
            Some(entries) => ParseResult::new(entries, merged_errors),
            None => ParseResult::errors_only(merged_errors),
        }
    }
}
