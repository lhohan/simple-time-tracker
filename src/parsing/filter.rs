use chrono::Datelike;
use chrono::Utc;

use crate::domain::{EndDate, EntryDate, StartDate, TimeEntry};

#[derive(Debug, Clone)]
pub enum Filter {
    Project(String),
    DateRange(DateRange),
    And(Box<Filter>, Box<Filter>),
}

impl Filter {
    pub fn matches(&self, entry: &TimeEntry, date: &EntryDate) -> bool {
        match self {
            Filter::Project(project) => entry.project.eq_ignore_ascii_case(project),
            Filter::DateRange(date_range) => date_range.matches(date),
            Filter::And(f1, f2) => f1.matches(entry, date) && f2.matches(entry, date),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DateRange(pub StartDate, pub EndDate);

impl DateRange {
    pub fn new_from_date(from_date: StartDate) -> Self {
        let default = DateRange::default();
        DateRange(from_date, default.1)
    }

    fn matches(&self, date: &EntryDate) -> bool {
        date.0 >= self.0 .0 && date.0 <= self.1 .0
    }
}

impl Default for DateRange {
    fn default() -> Self {
        let now = Utc::now().date_naive();
        let first_day_of_month = now
            .with_day(1)
            .expect("Going back to first of the month should never fail."); // to avoid going x years back on Feb 29 in next operation:
        let x_years_ago = first_day_of_month
            .with_year(first_day_of_month.year() - 100)
            .expect("Failed going back years");
        let default_start = x_years_ago;
        let x_years_in_future = first_day_of_month
            .with_year(first_day_of_month.year() + 100)
            .expect("Failed going years in the future");
        let default_end = x_years_in_future;
        DateRange(StartDate(default_start), EndDate(default_end))
    }
}
