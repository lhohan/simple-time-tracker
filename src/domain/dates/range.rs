use chrono::{Datelike, Duration, NaiveDate};
use regex::Regex;

use super::{EndDate, EntryDate, StartDate};
use crate::domain::{self, time::Clock, RangeDescription};

#[derive(Debug, Clone, PartialEq)]
pub enum PeriodRequested {
    Day(NaiveDate),
    Month(NaiveDate),
    Week(NaiveDate),
}

impl PeriodRequested {
    #[allow(clippy::missing_panics_doc)]
    pub fn from_str(period_requested: &str, clock: &Clock) -> Result<Self, domain::ParseError> {
        Self::date_from_literal(period_requested, clock)
            .or_else(|| Self::date_from_value(period_requested, clock))
            .ok_or(domain::ParseError::InvalidPeriod(
                period_requested.to_string(),
            ))
    }

    #[must_use]
    fn date_from_literal(s: &str, clock: &Clock) -> Option<PeriodRequested> {
        match s {
            "today" | "t" => Some(Self::Day(clock.today())),
            "this-week" | "tw" => Some(Self::Week(clock.today())),
            "last-week" | "lw" => Some(Self::Week(clock.today() - Duration::days(7))),
            "this-month" | "tm" => Some(Self::Month(this_month(clock))),
            "last-month" | "lm" => Some(Self::Month(last_month(clock))),
            _ => None,
        }
    }

    #[must_use]
    fn date_from_value(s: &str, clock: &Clock) -> Option<PeriodRequested> {
        Self::try_parse_month(s, clock)
            .or_else(|| Self::try_parse_date_value(s))
            .or_else(|| Self::try_parse_month_value(s))
    }

    #[must_use]
    fn try_parse_month(s: &str, clock: &Clock) -> Option<PeriodRequested> {
        let month_regex = Regex::new(r"^(month|m)-(\d+)$").unwrap();
        month_regex.captures(s).and_then(|captures| {
            captures.get(2).and_then(|month_str| {
                month_str.as_str().parse::<u32>().ok().and_then(|month| {
                    if (1..=12).contains(&month) {
                        clock.today().with_month(month).map(PeriodRequested::Month)
                    } else {
                        None
                    }
                })
            })
        })
    }

    #[must_use]
    fn try_parse_date_value(s: &str) -> Option<PeriodRequested> {
        let date_regex = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})$").unwrap();
        if date_regex.is_match(s) {
            match NaiveDate::parse_from_str(s, "%Y-%m-%d") {
                Ok(date) => Some(PeriodRequested::Day(date)),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    #[must_use]
    fn try_parse_month_value(s: &str) -> Option<PeriodRequested> {
        let month_value_regex = Regex::new(r"^(\d{4})-(\d{2})$").unwrap();
        let month = month_value_regex.captures(s).and_then(|captures| {
            captures.get(1).and_then(|year_match| {
                captures.get(2).and_then(|month_match| {
                    let year = year_match.as_str().parse::<i32>().unwrap();
                    let month = month_match.as_str().parse::<u32>().unwrap();
                    NaiveDate::from_ymd_opt(year, month, 1)
                })
            })
        });
        month.map(PeriodRequested::Month)
    }

    #[must_use]
    pub fn date_range(&self) -> DateRange {
        match self {
            Self::Day(date) => DateRange::day(*date),
            Self::Week(date) => DateRange::week_of(*date),
            Self::Month(date) => DateRange::month_of(*date),
        }
    }

    #[must_use]
    pub fn period_description(&self) -> RangeDescription {
        match self {
            Self::Day(date) => RangeDescription::day(*date),
            Self::Week(date) => RangeDescription::week_of(*date),
            Self::Month(date) => RangeDescription::month_of(*date),
        }
    }
}

fn last_month(clock: &Clock) -> NaiveDate {
    let today = clock.today();
    let previous_month = today.pred_opt().unwrap();
    calculate_1st_of_month(previous_month)
}

fn this_month(clock: &Clock) -> NaiveDate {
    let today = clock.today();
    calculate_1st_of_month(today)
}

fn calculate_1st_of_month(date: NaiveDate) -> NaiveDate {
    date.with_day(1).unwrap()
}

#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct DateRange(pub StartDate, pub EndDate);

impl DateRange {
    #[must_use]
    pub fn day(date: NaiveDate) -> Self {
        DateRange(StartDate(date), EndDate(date))
    }

    #[must_use]
    pub fn week_of(date: NaiveDate) -> Self {
        let monday = date - Duration::days(i64::from(date.weekday().num_days_from_monday()));
        let sunday = monday + Duration::days(6);
        DateRange(StartDate(monday), EndDate(sunday))
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn month_of(date: NaiveDate) -> Self {
        let first_day = date.with_day(1).unwrap();
        let last_day = first_day
            .with_month0(date.month0() + 1)
            .unwrap()
            .pred_opt()
            .unwrap();
        DateRange(StartDate(first_day), EndDate(last_day))
    }

    #[must_use]
    pub fn matches(&self, date: &EntryDate) -> bool {
        date.0 >= self.0 .0 && date.0 <= self.1 .0
    }

    #[must_use]
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
