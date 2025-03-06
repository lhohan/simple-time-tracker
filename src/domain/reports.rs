use super::dates::{EndDate, StartDate};
use super::{ParseError, TimeEntry};
use chrono::NaiveDate;
use chrono::{Datelike, IsoWeek};

#[derive(Debug)]
pub struct TimeTrackingResult {
    pub time_entries: Option<TrackedTime>,
    pub errors: Vec<ParseError>,
}

#[derive(Debug, Clone)]
pub struct RangeDescription(String);

impl RangeDescription {
    #[must_use]
    pub fn today(date: NaiveDate) -> Self {
        let date_str = format_today(date);
        RangeDescription(date_str)
    }

    #[must_use]
    pub fn week_of(date: NaiveDate) -> Self {
        let week = date.iso_week();
        let week_str = format_week(week);
        RangeDescription(week_str)
    }

    #[must_use]
    pub fn month_of(date: NaiveDate) -> Self {
        let month_str = format_month(date);
        RangeDescription(month_str)
    }
}

fn format_today(date: NaiveDate) -> String {
    format!("today ({})", date.format("%Y-%m-%d"))
}

fn format_week(week: IsoWeek) -> String {
    let week_number = week.week();
    let year = week.year();
    format!("Week {week_number}, {year}")
}

fn format_month(date: NaiveDate) -> String {
    format!("{}", date.format("%Y-%m"))
}

impl std::fmt::Display for RangeDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct TrackingPeriod {
    pub(crate) start: StartDate,
    pub(crate) end: EndDate,
    pub(crate) days: u32,
}

impl TrackingPeriod {
    #[must_use]
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
    #[must_use]
    pub fn new(entries: Vec<TimeEntry>, start: StartDate, end: EndDate, days: u32) -> Self {
        let total_minutes = entries.iter().map(|e| e.minutes).sum();
        Self {
            entries,
            period: TrackingPeriod::new(start, end, days),
            total_minutes,
        }
    }
}
