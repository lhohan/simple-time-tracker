pub mod dates;
pub mod reports;
pub mod tags;
pub mod time;
use std::collections::VecDeque;

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
    pub fn parse(line: &str) -> Result<Option<TimeEntry>, ParseError> {
        let line = EntryLine::parse(line);
        match line {
            Some(line) => parse_line(line).map(Some),
            None => Ok(None),
        }
    }

    #[must_use]
    pub fn main_context(&self) -> String {
        self.tags[0].raw_value().to_string()
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

struct EntryLine<'a>(pub(crate) &'a str);

impl EntryLine<'_> {
    pub(crate) fn parse(line: &str) -> Option<EntryLine> {
        if EntryLine::is_line_entry(line) {
            Some(EntryLine(line))
        } else {
            None
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

fn parse_line(entry_line: EntryLine) -> Result<TimeEntry, ParseError> {
    let line_no_prefix = entry_line.entry();
    let parts = line_no_prefix.split_whitespace();

    let mut projects = VecDeque::new();
    let mut minutes = 0;
    let mut description = Vec::new();
    let mut time_found = false;

    for part in parts {
        match parse_part(part) {
            Ok(LinePart::Time(time)) => {
                minutes += time;
                time_found = true;
            }
            Ok(LinePart::Tag(project_found)) => {
                projects.push_back(project_found);
            }
            Ok(LinePart::DescriptionPart(desc)) => description.push(desc),
            Err(err) => return Err(err),
        }
    }

    if !time_found {
        return Err(ParseError::MissingTime(entry_line.get_line().to_string()));
    }

    let description =
        (!description.is_empty()).then(|| description.into_iter().collect::<Vec<_>>().join(" "));
    let projects: Vec<Tag> = projects.into();

    let tags = projects;
    if tags.is_empty() || tags[0].raw_value().is_empty() {
        return Err(ParseError::MissingProject(
            entry_line.get_line().to_string(),
        ));
    }

    Ok(TimeEntry {
        tags,
        minutes,
        description,
    })
}

enum LinePart<'a> {
    Time(u32),
    Tag(Tag),
    DescriptionPart(&'a str),
}

fn parse_part(part: &str) -> Result<LinePart, ParseError> {
    if part.starts_with('#') {
        let raw_tag = part
            .strip_prefix("#")
            .expect("project should have had '#' prefix");
        let tag = Tag::from_raw(raw_tag);
        Ok(LinePart::Tag(tag))
    } else {
        match parse_time(part) {
            Ok(Some(minutes)) => Ok(LinePart::Time(minutes)),
            Ok(None) => Ok(LinePart::DescriptionPart(part)),
            Err(err) => Err(err),
        }
    }
}
use std::str::FromStr;

fn parse_time(time: &str) -> Result<Option<u32>, ParseError> {
    let (value, multiplier) = match time.chars().last() {
        Some('m') => (time.trim_end_matches('m'), 1),
        Some('h') => (time.trim_end_matches('h'), 60),
        Some('p') => (time.trim_end_matches('p'), 30),
        _ => return Ok(None),
    };

    match u32::from_str(value) {
        Ok(val) => Ok(Some(val * multiplier)),
        Err(e) => match e.kind() {
            std::num::IntErrorKind::InvalidDigit => Ok(None),
            _ => Err(ParseError::InvalidTime(time.to_string())),
        },
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
    MissingProject(String),
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
            ParseError::MissingProject(line) => write!(f, "missing project: {line}"),
            ParseError::ErrorReading(file) => write!(f, "error reading file: {file}"),
            ParseError::InvalidPeriod(period) => write!(f, "invalid period: {period}"),
            ParseError::Located { error, location } => {
                write!(f, "{}: line {}: {}", location.file, location.line, error)
            }
        }
    }
}

impl std::error::Error for ParseError {}
