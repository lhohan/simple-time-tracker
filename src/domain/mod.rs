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
    pub fn main_project(&self) -> &String {
        &self.projects[0]
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
