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
            ParseError::InvalidLineFormat(line) => write!(f, "invalid line format: '{}'", line),
            ParseError::InvalidTime(time) => write!(f, "invalid time format: '{}'", time),
            ParseError::InvalidDate(date) => write!(f, "invalid date format: '{}'", date),
            ParseError::MissingTime(line) => write!(f, "missing time: '{}'", line),
            ParseError::ErrorReading(file) => write!(f, "error reading file: '{}'", file),
        }
    }
}

impl std::error::Error for ParseError {}
