use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct TimeEntry {
    pub project: String,
    pub minutes: u32,
    pub description: Option<String>,
}

impl TimeEntry {
    pub fn new(project: String, minutes: u32, description: Option<String>) -> Self {
        Self {
            project,
            minutes,
            description,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    ErrorReading(String),
    InvalidLineFormat(String),
    InvalidTime(String),
    InvalidDate(String),
    MissingTime(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidLineFormat(line) => write!(f, "invalid line format: {}", line),
            ParseError::InvalidTime(time) => write!(f, "invalid time format: {}", time),
            ParseError::InvalidDate(date) => write!(f, "invalid date format: {}", date),
            ParseError::MissingTime(line) => write!(f, "missing time: {}", line),
            ParseError::ErrorReading(file) => write!(f, "error reading file: {}", file),
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone)]
pub struct StartDate(pub NaiveDate);
#[derive(Debug, Clone)]
pub struct EndDate(pub NaiveDate);

#[derive(Debug, PartialEq)]
pub struct ParseResult {
    errors: Vec<ParseError>,
    days: u32,
    entries: HashMap<NaiveDate, Vec<TimeEntry>>,
}

impl ParseResult {
    pub fn new(entries: HashMap<NaiveDate, Vec<TimeEntry>>, errors: Vec<ParseError>) -> Self {
        Self {
            errors,
            days: entries.len() as u32,
            entries,
        }
    }

    pub fn entries(&self) -> Vec<TimeEntry> {
        self.entries
            .values()
            .into_iter()
            .flat_map(|vec| vec.iter().cloned())
            .collect()
    }

    pub fn errors(&self) -> &Vec<ParseError> {
        &self.errors
    }

    pub fn days(&self) -> u32 {
        self.days
    }

    pub fn start_date(&self) -> StartDate {
        let earliest = self.entries.keys().min_by_key(|date| *date).copied();
        let earliest = earliest.expect("There should always be a start date");
        StartDate(earliest)
    }

    pub fn end_date(&self) -> EndDate {
        let latest = self.entries.keys().max_by_key(|date| *date).copied();
        let latest = latest.expect("There should always be an end date");
        EndDate(latest)
    }
}
