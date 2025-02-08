use super::{EndDate, ParseError, StartDate, TimeEntry};
use chrono::IsoWeek;
use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TimeTrackingResult {
    pub time_entries: Option<TrackedTime>,
    pub errors: Vec<ParseError>,
}

#[derive(Debug, PartialEq)]
pub struct ParseResult {
    errors: Vec<ParseError>,
    days: u32,
    entries: Option<HashMap<NaiveDate, Vec<TimeEntry>>>,
}

impl ParseResult {
    pub fn new(entries: HashMap<NaiveDate, Vec<TimeEntry>>, errors: Vec<ParseError>) -> Self {
        Self {
            errors,
            days: entries.len() as u32,
            entries: Some(entries),
        }
    }

    pub fn errors_only(errors: Vec<ParseError>) -> Self {
        Self {
            errors,
            days: 0,
            entries: None,
        }
    }

    pub fn errors(&self) -> Vec<ParseError> {
        self.errors.clone()
    }

    pub fn days(&self) -> u32 {
        self.days
    }

    pub fn entries_by_date(&self) -> Option<&HashMap<NaiveDate, Vec<TimeEntry>>> {
        self.entries.as_ref()
    }

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
            (Some(entries), None) => Some(entries.clone()),
            (None, Some(entries)) => Some(entries.clone()),
            (None, None) => None,
        };

        match merged_entries {
            Some(entries) => ParseResult::new(entries, merged_errors),
            None => ParseResult::errors_only(merged_errors),
        }
    }
}

pub enum ReportType {
    Projects,
    ProjectDetails(String),
}

#[derive(Debug, Clone)]
pub struct RangeDescription(String);

impl RangeDescription {
    pub fn this_week(week: IsoWeek) -> Self {
        let week_str = format_week(week);
        let d = format!("{}", week_str);
        RangeDescription(d)
    }
    pub fn last_week(week: IsoWeek) -> Self {
        let week_str = format_week(week);
        let d = format!("{}", week_str);
        RangeDescription(d)
    }
    pub fn last_month(date: NaiveDate) -> Self {
        RangeDescription(format!("{}", date.format("%Y-%m")))
    }
    pub fn this_month(date: NaiveDate) -> Self {
        RangeDescription(format!("{}", date.format("%Y-%m")))
    }
}

fn format_week(week: IsoWeek) -> String {
    let week_number = week.week();
    let year = week.year();
    format!("Week {}, {}", week_number, year)
}

impl ToString for RangeDescription {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct TrackingPeriod {
    pub(crate) start: StartDate,
    pub(crate) end: EndDate,
    pub(crate) days: u32,
}

impl TrackingPeriod {
    pub fn new(start: StartDate, end: EndDate, days: u32) -> Self {
        Self { start, end, days }
    }
}

#[derive(Debug)]
pub struct TrackedTime {
    pub entries: Vec<TimeEntry>,
    pub period: TrackingPeriod,
    pub total_minutes: u32,
}

impl TrackedTime {
    pub fn new(entries: Vec<TimeEntry>, start: StartDate, end: EndDate, days: u32) -> Self {
        let total_minutes = entries.iter().map(|e| e.minutes).sum();
        Self {
            entries,
            period: TrackingPeriod::new(start, end, days),
            total_minutes,
        }
    }
}
