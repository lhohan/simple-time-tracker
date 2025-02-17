use super::header_parser::maybe_date_from_header;
use super::line_parser::parse_entry;
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

impl LineType {
    pub(crate) fn parse(line: &str, in_tt_section: bool) -> Result<Self, ParseError> {
        if line.starts_with('#') {
            let maybe_date = maybe_date_from_header(line);
            let maybe_date = maybe_date.map(|date_str| {
                NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                    .map_err(|_| ParseError::InvalidDate(date_str.to_string()))
            });
            let maybe_date = maybe_date.transpose()?;
            Ok(LineType::Header(maybe_date))
        } else if in_tt_section {
            EntryLine::new(line).map_or(Ok(LineType::Other), |line| {
                parse_entry(line).map(LineType::Entry)
            })
        } else {
            Ok(LineType::Other)
        }
    }
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

pub(crate) struct EntryLine<'a>(pub(crate) &'a str);

impl EntryLine<'_> {
    pub(crate) fn new(line: &str) -> Result<EntryLine, ParseError> {
        if EntryLine::is_line_entry(line) {
            Ok(EntryLine(line))
        } else {
            Err(ParseError::InvalidLineFormat(line.to_string()))
        }
    }

    fn is_line_entry(line: &str) -> bool {
        line.starts_with("- #")
    }

    pub(crate) fn get_line(&self) -> &str {
        &self.0
    }

    // Return the actual content of the line, without the prefix that ids the line is an entry line.
    pub(crate) fn entry(&self) -> &str {
        &self.0.strip_prefix("- ").expect("invalid struct state")
    }
}
