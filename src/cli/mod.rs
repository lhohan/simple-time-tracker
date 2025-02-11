use chrono::NaiveDate;
use clap::Parser;
use std::path::PathBuf;

use crate::domain::time::Clock;
use crate::domain::PeriodRequested;
use crate::domain::{ParseError, StartDate};

#[derive(Parser, Debug)]
#[command(author, version, about = "Simple time tracking from markdown files")]
pub struct Args {
    /// Input file to process
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,

    /// Show verbose output
    #[arg(short, long)]
    pub verbose: bool,

    // Project filter flag
    #[arg(long)]
    pub project: Option<String>,

    /// From date filter value
    #[arg(short, long, value_name = "YYYY-MM-DD")]
    pub from: Option<String>,

    #[arg(long, value_name = "this-week, next-week, this-month, ...")]
    period: Option<String>,
}

impl Args {
    #[must_use]
    pub fn parse() -> Self {
        Self::parse_from(std::env::args())
    }

    /// Parses the from date from the command line arguments.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError::InvalidDate` if the date is not in the correct format.
    pub fn from_date(&self) -> Result<Option<StartDate>, ParseError> {
        match &self.from {
            Some(date) => {
                let parsed_date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
                    .map_err(|_| ParseError::InvalidDate(date.to_string()))?;
                Ok(Some(StartDate(parsed_date)))
            }
            None => Ok(None),
        }
    }

    /// Parses the period from the command line arguments.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError::InvalidPeriod` if the period is not valid.
    pub fn period(&self, clock: &Clock) -> Result<Option<PeriodRequested>, ParseError> {
        Ok(self
            .period
            .as_ref()
            .and_then(|s| PeriodRequested::from_str(s, clock).ok()))
    }
}
