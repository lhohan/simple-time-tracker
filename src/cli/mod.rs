use chrono::NaiveDate;
use clap::Parser;
use std::path::PathBuf;

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
}

impl Args {
    pub fn parse() -> Self {
        Self::parse_from(std::env::args())
    }

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
}
