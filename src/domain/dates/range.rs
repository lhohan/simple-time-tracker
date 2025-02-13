use chrono::{Datelike, Duration, NaiveDate};

use super::{EndDate, EntryDate, StartDate};
use crate::domain::{time::Clock, RangeDescription};

#[derive(Debug, Clone, PartialEq)]
pub enum PeriodRequested {
    ThisWeek(NaiveDate),
    LastWeek(NaiveDate),
    ThisMonth(NaiveDate),
    LastMonth(NaiveDate),
}

impl PeriodRequested {
    pub fn from_str(s: &str, clock: &Clock) -> Result<Self, crate::domain::ParseError> {
        match s {
            "this-week" | "tw" => Ok(Self::ThisWeek(clock.today())),
            "last-week" | "lw" => Ok(Self::LastWeek(clock.today() - Duration::days(7))),
            "this-month" | "tm" => Ok(Self::ThisMonth(clock.today().with_day(1).unwrap())),
            "last-month" | "lm" => Ok(Self::LastMonth(
                clock
                    .today()
                    .with_day(1)
                    .unwrap()
                    .pred_opt()
                    .unwrap()
                    .with_day(1)
                    .unwrap(),
            )),
            _ => Err(crate::domain::ParseError::InvalidPeriod(s.to_string())),
        }
    }

    pub fn date_range(&self) -> DateRange {
        match self {
            Self::ThisWeek(date) | Self::LastWeek(date) => DateRange::week_of(*date),
            Self::ThisMonth(date) | Self::LastMonth(date) => DateRange::month_of(*date),
        }
    }

    pub fn period_description(&self) -> RangeDescription {
        match self {
            Self::ThisWeek(date) => RangeDescription::this_week(date.iso_week()),
            Self::LastWeek(date) => RangeDescription::last_week(date.iso_week()),
            Self::ThisMonth(date) => RangeDescription::this_month(*date),
            Self::LastMonth(date) => RangeDescription::last_month(*date),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DateRange(pub StartDate, pub EndDate);

impl DateRange {
    pub fn week_of(date: NaiveDate) -> Self {
        let monday = date - Duration::days(date.weekday().num_days_from_monday() as i64);
        let sunday = monday + Duration::days(6);
        DateRange(StartDate(monday), EndDate(sunday))
    }

    pub fn month_of(date: NaiveDate) -> Self {
        let first_day = date.with_day(1).unwrap();
        let last_day = first_day
            .with_month0(date.month0() + 1)
            .unwrap()
            .pred_opt()
            .unwrap();
        DateRange(StartDate(first_day), EndDate(last_day))
    }

    pub fn matches(&self, date: &EntryDate) -> bool {
        date.0 >= self.0 .0 && date.0 <= self.1 .0
    }

    pub fn new_from_date(from_date: StartDate) -> Self {
        let default = DateRange::default();
        DateRange(from_date, default.1)
    }
}

impl Default for DateRange {
    fn default() -> Self {
        DateRange(StartDate(NaiveDate::MIN), EndDate(NaiveDate::MAX))
    }
}
