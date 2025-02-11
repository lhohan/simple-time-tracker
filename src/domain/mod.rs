mod aggregates;
pub mod time;

pub use aggregates::{
    ParseResult, RangeDescription, ReportType, TimeTrackingResult, TrackedTime, TrackingPeriod,
};
use chrono::Datelike;
use time::Clock;

use chrono::NaiveDate;

use crate::parsing::DateRange;

// Organisation:
// - Core domain primitives in `mod.rs`
// - Aggregates and composite structures in `aggregates.rs`

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

#[derive(Debug, Clone)]
pub struct StartDate(pub NaiveDate);
#[derive(Debug, Clone)]
pub struct EndDate(pub NaiveDate);
#[derive(Debug, Clone)]
pub struct EntryDate(pub NaiveDate);

#[derive(Clone, Debug)]
pub enum PeriodRequested {
    ThisWeek(NaiveDate),
    LastWeek(NaiveDate),
    ThisMonth(NaiveDate),
    LastMonth(NaiveDate),
}

impl PeriodRequested {
    /// Parses a string into a `PeriodRequested` enum variant.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the string slice is not a valid period.
    ///
    /// # Panics
    ///
    /// Panics if the `clock.today()` returns a date that is not valid.
    #[allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
    pub fn from_str(s: &str, clock: &Clock) -> Result<Self, ParseError> {
        match s {
            "this-week" | "tw" => {
                let date = clock.today();
                Ok(PeriodRequested::ThisWeek(date))
            }
            "last-week" | "lw" => {
                let date = clock.today();
                let previous_week_date = date - chrono::Duration::days(7);
                Ok(PeriodRequested::LastWeek(previous_week_date))
            }
            "this-month" | "tm" => {
                let first_day_of_month = clock.today().with_day(1).unwrap();
                Ok(PeriodRequested::ThisMonth(first_day_of_month))
            }
            "last-month" | "lm" => {
                let first_day_of_last_month = match clock.today().with_day(1) {
                    Some(first_day) => first_day
                        .pred_opt()
                        .expect("previous day should exist")
                        .with_day(1)
                        .unwrap(),
                    None => {
                        return Err(ParseError::InvalidPeriod(s.to_string()));
                    }
                };
                Ok(PeriodRequested::LastMonth(first_day_of_last_month))
            }

            _ => Err(ParseError::InvalidPeriod(s.to_string())), // Add InvalidPeriod variant to ParseError
        }
    }

    #[must_use]
    pub fn date_range(&self) -> DateRange {
        match self {
            PeriodRequested::ThisWeek(date) | PeriodRequested::LastWeek(date) => {
                DateRange::week_of(*date)
            }
            PeriodRequested::ThisMonth(date) | PeriodRequested::LastMonth(date) => {
                DateRange::month_of(*date)
            }
        }
    }

    #[must_use]
    pub fn period_description(&self) -> RangeDescription {
        match self {
            PeriodRequested::ThisWeek(naive_date) => {
                RangeDescription::this_week(naive_date.iso_week())
            }
            PeriodRequested::LastWeek(date) => RangeDescription::last_week(date.iso_week()),
            PeriodRequested::ThisMonth(date) => RangeDescription::this_month(*date),
            PeriodRequested::LastMonth(date) => RangeDescription::last_month(*date),
        }
    }
}
