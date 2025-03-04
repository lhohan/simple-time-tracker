use chrono::NaiveDate;
use clap::Parser;
use std::path::PathBuf;

use crate::domain::dates::StartDate;
use crate::domain::time::Clock;
use crate::domain::ParseError;
use crate::domain::PeriodRequested;

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

    // Tags exclude filter
    #[arg(long)]
    pub exclude_tags: Option<String>,

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

    /// Parses exclude tags from the command line arguments.
    pub fn exclude_tags(&self) -> Vec<String> {
        match &self.exclude_tags {
            Some(tags) => {
                let parsed_tags = tags.split(',').map(String::from).collect::<Vec<String>>();
                parsed_tags
            }
            None => vec![],
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
