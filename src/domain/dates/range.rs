use chrono::{Datelike, Duration, NaiveDate};
use regex::Regex;
use std::sync::LazyLock;

use super::{EndDate, EntryDate, StartDate};
use crate::domain::{self, time::Clock, PeriodDescription};
use crate::ParseError;

static MONTH_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(month|m)-(\d+)$").unwrap());

static MONTH_VALUE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d{4})-(\d{1,2})$").unwrap());

static WEEK_VALUE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d{4})-w(\d{1,2})$").unwrap());

static YEAR_VALUE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\d{4})$").unwrap());

#[derive(Debug, Clone, PartialEq)]
pub enum PeriodRequested {
    Day(NaiveDate),
    FromDate(NaiveDate),
    WeekOf(NaiveDate),
    MonthOf(NaiveDate),
    YearOf(NaiveDate),
}

impl PeriodRequested {
    /// Parses a period request from a string representation.
    ///
    /// # Errors
    ///
    /// Returns `ParseError::InvalidPeriod` if the period string is not recognized.
    pub fn from_str(period_requested: &str, clock: &Clock) -> Result<Self, domain::ParseError> {
        Self::date_from_literal(period_requested, clock)
            .or_else(|| Self::date_from_value(period_requested, clock))
            .ok_or(domain::ParseError::InvalidPeriod(
                period_requested.to_string(),
            ))
    }

    /// Parses a period request from a from-date string.
    ///
    /// # Errors
    ///
    /// Returns `ParseError::InvalidDate` if the date string cannot be parsed.
    pub fn parse_from_date(
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

    fn date_from_value(s: &str, clock: &Clock) -> Option<PeriodRequested> {
        Self::try_parse_month(s, clock)
            .or_else(|| Self::try_parse_date_value(s))
            .or_else(|| Self::try_parse_month_value(s))
            .or_else(|| Self::try_parse_week_value(s))
            .or_else(|| Self::try_parse_year_value(s))
    }

    fn parse_month(s: &str) -> Option<u32> {
        let month = s.parse::<u32>().ok()?;
        (1..=12).contains(&month).then_some(month)
    }

    fn parse_year(s: &str) -> Option<i32> {
        let year = s.parse::<i32>().ok()?;
        (1000..=9999).contains(&year).then_some(year)
    }

    fn validate_week_bounds(week: u32) -> Option<u32> {
        (week > 0 && week < 53).then_some(week)
    }

    fn parse_week(s: &str) -> Option<u32> {
        let week = s.parse::<u32>().ok()?;
        Self::validate_week_bounds(week)
    }

    fn try_parse_month(s: &str, clock: &Clock) -> Option<PeriodRequested> {
        let captures = MONTH_REGEX.captures(s)?;
        let month_str = captures.get(2)?.as_str();
        let month = Self::parse_month(month_str)?;

        let date = today(clock).with_month(month)?;
        Some(PeriodRequested::MonthOf(date))
    }

    fn try_parse_date_value(s: &str) -> Option<PeriodRequested> {
        NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .ok()
            .map(PeriodRequested::Day)
    }

    fn try_parse_month_value(s: &str) -> Option<PeriodRequested> {
        let captures = MONTH_VALUE_REGEX.captures(s)?;

        let year_str = captures.get(1)?.as_str();
        let month_str = captures.get(2)?.as_str();

        let year = Self::parse_year(year_str)?;
        let month = Self::parse_month(month_str)?;

        let date = NaiveDate::from_ymd_opt(year, month, 1)?;
        Some(PeriodRequested::MonthOf(date))
    }

    fn try_parse_week_value(s: &str) -> Option<PeriodRequested> {
        let captures = WEEK_VALUE_REGEX.captures(s)?;

        let year_str = captures.get(1)?.as_str();
        let week_str = captures.get(2)?.as_str();

        let year = Self::parse_year(year_str)?;
        let week = Self::parse_week(week_str)?;

        let date = Self::get_first_day_of_week(year, week)?;
        Some(PeriodRequested::WeekOf(date))
    }

    fn get_first_day_of_week(year: i32, week: u32) -> Option<NaiveDate> {
        let week = Self::validate_week_bounds(week)?;

        // January 4th is always in the first week of the year according to ISO 8601
        let jan_4 = NaiveDate::from_ymd_opt(year, 1, 4)?;

        let days_to_monday = i64::from(jan_4.weekday().num_days_from_monday());
        let first_monday = jan_4.checked_sub_signed(chrono::Duration::days(days_to_monday))?;

        let days_to_add = (week - 1) * 7;
        first_monday.checked_add_signed(chrono::Duration::days(i64::from(days_to_add)))
    }

    fn try_parse_year_value(s: &str) -> Option<PeriodRequested> {
        let captures = YEAR_VALUE_REGEX.captures(s)?;
        let year_str = captures.get(1)?.as_str();
        let year = Self::parse_year(year_str)?;

        let date = NaiveDate::from_ymd_opt(year, 1, 1)?;
        Some(PeriodRequested::YearOf(date))
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
    // Get first day of current month, then step back one day to land in previous month
    let first_of_current_month = today.with_day(1).unwrap();
    let last_day_of_previous_month = first_of_current_month.pred_opt().unwrap();
    // Get first day of previous month
    set_to_1st_of_month(last_day_of_previous_month)
}

fn this_month(clock: &Clock) -> NaiveDate {
    let today = today(clock);
    set_to_1st_of_month(today)
}

fn set_to_1st_of_month(date: NaiveDate) -> NaiveDate {
    date.with_day(1).unwrap()
}

fn set_to_last_of_month(date: NaiveDate) -> NaiveDate {
    date.with_day(1)
        .unwrap()
        .checked_add_months(chrono::Months::new(1))
        .unwrap()
        .checked_sub_days(chrono::Days::new(1))
        .unwrap()
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
        let first_day = set_to_1st_of_month(date);
        let last_day = set_to_last_of_month(date);
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
