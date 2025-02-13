use chrono::Datelike;
use chrono::Duration;
use chrono::NaiveDate;
use chrono::Utc;

use crate::domain::dates::{EndDate, EntryDate, StartDate};
use crate::domain::TimeEntry;

#[derive(Debug, Clone)]
pub enum Filter {
    Project(String),
    DateRange(DateRange),
    And(Box<Filter>, Box<Filter>),
}

impl Filter {
    pub fn matches(&self, entry: &TimeEntry, date: &EntryDate) -> bool {
        match self {
            Filter::Project(project) => entry.main_project().eq_ignore_ascii_case(project),
            Filter::DateRange(date_range) => date_range.matches(date),
            Filter::And(f1, f2) => f1.matches(entry, date) && f2.matches(entry, date),
        }
    }
    pub fn combine(self, other: Filter) -> Filter {
        Filter::And(Box::new(self), Box::new(other))
    }
}

#[derive(Debug, Clone)]
pub struct DateRange(pub StartDate, pub EndDate);

impl DateRange {
    pub fn week_of(date: NaiveDate) -> Self {
        let monday = date - Duration::days(i64::from(date.weekday().num_days_from_monday()));
        let sunday = monday + Duration::days(6);
        DateRange(StartDate(monday), EndDate(sunday))
    }

    pub(crate) fn month_of(date: NaiveDate) -> DateRange {
        let first_day = date.with_day(1).unwrap();
        let last_day = first_day
            .with_month0(date.month0() + 1)
            .unwrap()
            .pred_opt()
            .unwrap();
        DateRange(StartDate(first_day), EndDate(last_day))
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_week_of_date_range() {
        // Tuesday January 15th 2024
        let current = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();

        let range = DateRange::week_of(current);

        // Week should start on Monday January 15th
        assert_eq!(
            range.0 .0, // StartDate
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()
        );
        // Week should end on Sunday January 21st
        assert_eq!(
            range.1 .0, // EndDate
            NaiveDate::from_ymd_opt(2024, 1, 21).unwrap()
        );
    }
}
