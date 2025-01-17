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
    InvalidFormat,
    InvalidTime(String),
    InvalidDate(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidFormat => write!(f, "invalid line format"),
            ParseError::InvalidTime(line) => write!(f, "invalid time format: '{}'", line),
            ParseError::InvalidDate(line) => write!(f, "invalid date format: '{}'", line),
        }
    }
}

impl std::error::Error for ParseError {}
