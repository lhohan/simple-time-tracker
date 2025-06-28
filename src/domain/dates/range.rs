use chrono::{Datelike, Duration, NaiveDate};
use regex::Regex;

use super::{EndDate, EntryDate, StartDate};
use crate::domain::{self, time::Clock, PeriodDescription};
use crate::ParseError;

#[derive(Debug, Clone, PartialEq)]
pub enum PeriodRequested {
    Day(NaiveDate),
    FromDate(NaiveDate),
    WeekOf(NaiveDate),
    MonthOf(NaiveDate),
    YearOf(NaiveDate),
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

    #[allow(clippy::missing_panics_doc)]
    pub fn from_from_date(
        from_date_requested: Option<&str>,
    ) -> Result<Option<Self>, domain::ParseError> {
        match from_date_requested {
            Some(date) => {
                let parsed_date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
                    .map_err(|_| ParseError::InvalidDate(date.to_string()))?;
                Ok(Some(PeriodRequested::FromDate(parsed_date)))
            }
            None => Ok(None),
        }
    }

    #[must_use]
    fn date_from_literal(s: &str, clock: &Clock) -> Option<PeriodRequested> {
        match s {
            "today" | "t" => Some(Self::Day(today(clock))),
            "yesterday" | "y" => Some(Self::Day(yesterday(clock))),
            "this-week" | "tw" => Some(Self::WeekOf(date_of_this_week(clock))),
            "last-week" | "lw" => Some(Self::WeekOf(date_of_last_week(clock))),
            "this-month" | "tm" => Some(Self::MonthOf(this_month(clock))),
            "last-month" | "lm" => Some(Self::MonthOf(last_month(clock))),
            _ => None,
        }
    }

    #[must_use]
    fn date_from_value(s: &str, clock: &Clock) -> Option<PeriodRequested> {
        Self::try_parse_month(s, clock)
            .or_else(|| Self::try_parse_date_value(s))
            .or_else(|| Self::try_parse_month_value(s))
            .or_else(|| Self::try_parse_week_value(s))
            .or_else(|| Self::try_parse_year_value(s))
    }

    #[must_use]
    fn try_parse_month(s: &str, clock: &Clock) -> Option<PeriodRequested> {
        let month_regex = Regex::new(r"^(month|m)-(\d+)$").unwrap();
        month_regex.captures(s).and_then(|captures| {
            captures.get(2).and_then(|month_str| {
                month_str.as_str().parse::<u32>().ok().and_then(|month| {
                    if (1..=12).contains(&month) {
                        today(clock).with_month(month).map(PeriodRequested::MonthOf)
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
        let month_value_regex = Regex::new(r"^(\d{4})-(\d{1,2})$").unwrap();
        let month = month_value_regex.captures(s).and_then(|captures| {
            captures.get(1).and_then(|year_match| {
                captures.get(2).and_then(|month_match| {
                    let year = year_match.as_str().parse::<i32>().unwrap();
                    let month = month_match.as_str().parse::<u32>().unwrap();
                    NaiveDate::from_ymd_opt(year, month, 1)
                })
            })
        });
        month.map(PeriodRequested::MonthOf)
    }

    #[must_use]
    fn try_parse_week_value(s: &str) -> Option<PeriodRequested> {
        let week_value_regex = Regex::new(r"^(\d{4})-w(\d{1,2})$").unwrap();
        let week = week_value_regex.captures(s).and_then(|captures| {
            captures.get(1).and_then(|year_match| {
                captures.get(2).and_then(|week_match| {
                    let week = week_match.as_str().parse::<u32>().unwrap();
                    if (1..=53).contains(&week) {
                        let year = year_match.as_str().parse::<i32>().unwrap();
                        Self::get_first_day_of_week(year, week)
                    } else {
                        None
                    }
                })
            })
        });
        week.map(PeriodRequested::WeekOf)
    }

    #[must_use]
    fn get_first_day_of_week(year: i32, week: u32) -> Option<NaiveDate> {
        if week == 0 || week >= 53 {
            return None;
        }

        // January 4th is always in the first week of the year according to ISO 8601
        let jan_4 = NaiveDate::from_ymd_opt(year, 1, 4)?;

        let days_to_monday = jan_4.weekday().num_days_from_monday() as i64;
        let first_monday = jan_4.checked_sub_signed(chrono::Duration::days(days_to_monday))?;

        let days_to_add = (week - 1) * 7;
        first_monday.checked_add_signed(chrono::Duration::days(days_to_add as i64))
    }

    #[must_use]
    fn try_parse_year_value(s: &str) -> Option<PeriodRequested> {
        let year_value_regex = Regex::new(r"^(\d{4})$").unwrap();
        let year = year_value_regex.captures(s).and_then(|captures| {
            captures.get(1).and_then(|year_match| {
                let year = year_match.as_str().parse::<i32>().unwrap();
                NaiveDate::from_ymd_opt(year, 1, 1)
            })
        });
        year.map(PeriodRequested::YearOf)
    }

    #[must_use]
    pub fn date_range(&self) -> DateRange {
        match self {
            Self::Day(date) => DateRange::day(*date),
            Self::FromDate(date) => DateRange::from_date(*date),
            Self::WeekOf(date) => DateRange::week_of(*date),
            Self::MonthOf(date) => DateRange::month_of(*date),
            Self::YearOf(date) => DateRange::year_of(*date),
        }
    }

    #[must_use]
    pub fn description(&self) -> PeriodDescription {
        match self {
            Self::Day(date) => PeriodDescription::day(*date),
            Self::FromDate(date) => PeriodDescription::from_date(*date),
            Self::WeekOf(date) => PeriodDescription::week_of(*date),
            Self::MonthOf(date) => PeriodDescription::month_of(*date),
            Self::YearOf(date) => PeriodDescription::year_of(*date),
        }
    }
}

fn today(clock: &Clock) -> NaiveDate {
    clock.today()
}

fn yesterday(clock: &Clock) -> NaiveDate {
    clock.today() - Duration::days(1)
}

fn date_of_this_week(clock: &Clock) -> NaiveDate {
    today(clock)
}

fn date_of_last_week(clock: &Clock) -> NaiveDate {
    date_of_this_week(clock) - Duration::days(7)
}

fn last_month(clock: &Clock) -> NaiveDate {
    let today = today(clock);
    let month_of_today = today.month();
    let previous_month = today.with_month(month_of_today - 1).unwrap();
    calculate_1st_of_month(previous_month)
}

fn this_month(clock: &Clock) -> NaiveDate {
    let today = today(clock);
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
    pub fn from_date(date: NaiveDate) -> Self {
        DateRange(StartDate(date), EndDate(NaiveDate::MAX))
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
    #[allow(clippy::missing_panics_doc)]
    pub fn year_of(date: NaiveDate) -> Self {
        let first_day = date.with_day(1).unwrap().with_month(1).unwrap();
        let last_day = first_day
            .with_year(date.year() + 1)
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
