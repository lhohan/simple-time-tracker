pub mod dates;
pub mod reports;
pub mod tags;
pub mod time;

pub use dates::range::{DateRange, PeriodRequested};
pub use reports::{RangeDescription, TimeTrackingResult, TrackedTime, TrackingPeriod};
use tags::Tag;

#[derive(Debug, PartialEq, Clone)]
pub struct TimeEntry {
    tags: Vec<Tag>,
    pub minutes: u32,
    pub description: Option<String>,
}

impl TimeEntry {
    #[must_use]
    pub fn new(tags: Vec<Tag>, minutes: u32, description: Option<String>) -> Self {
        Self {
            tags,
            minutes,
            description,
        }
    }

    #[must_use]
    pub fn main_context(&self) -> String {
        self.tags
            .iter()
            .find(|t| t.is_project())
            .unwrap_or_else(|| &self.tags[0])
            .raw_value()
            .to_string()
    }

    #[must_use]
    pub fn get_tags(&self) -> &[Tag] {
        &self.tags
    }

    #[must_use]
    pub fn project_tags(&self) -> Vec<&Tag> {
        self.tags.iter().filter(|t| t.is_project()).collect()
    }

    #[must_use]
    pub fn context_tags(&self) -> Vec<&Tag> {
        self.tags.iter().filter(|t| !t.is_project()).collect()
    }

    #[must_use]
    pub fn has_project_tag(&self) -> bool {
        self.tags.iter().any(|t| t.is_project())
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
