use super::{EndDate, ParseError, StartDate, TimeEntry};
use chrono::NaiveDate;
use std::collections::HashMap;

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

    pub fn errors(&self) -> Vec<ParseError> {
        self.errors.clone() // todo: look into this, can clone be avoided?
    }

    pub fn days(&self) -> u32 {
        self.days
    }

    pub fn entries_by_date(&self) -> &HashMap<NaiveDate, Vec<TimeEntry>> {
        &self.entries
    }

    pub fn start_date(&self) -> StartDate {
        let earliest = self.entries.keys().min_by_key(|date| *date).copied();
        let earliest = earliest.expect("There should always be a start date for a parse result");
        StartDate(earliest)
    }

    pub fn end_date(&self) -> EndDate {
        let latest = self.entries.keys().max_by_key(|date| *date).copied();
        let latest = latest.expect("There should always be an end date for a parse result");
        EndDate(latest)
    }

    pub fn merge(&self, other: &ParseResult) -> ParseResult {
        let mut merged_entries = self.entries.clone();

        // Merge entries from other
        for (date, entries) in &other.entries {
            merged_entries
                .entry(*date)
                .or_insert_with(Vec::new)
                .extend(entries.iter().cloned());
        }

        // Combine errors
        let mut merged_errors = self.errors.clone();
        merged_errors.extend(other.errors.clone());

        ParseResult::new(merged_entries, merged_errors)
    }
}

pub enum ReportType {
    Projects,
    ProjectDetails(String),
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
