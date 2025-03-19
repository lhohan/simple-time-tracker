use clap::Parser;
use std::path::PathBuf;

use crate::domain::reports::OutputLimit;
use crate::domain::tags::TagFilter;
use crate::domain::time::Clock;
use crate::domain::ParseError;
use crate::domain::PeriodRequested;
use crate::reporting::format::Formatter;

#[derive(Parser, Debug)]
#[command(author, version, about = "Simple time tracking from markdown files")]
pub struct Args {
    /// Input file to process
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,

    /// Show verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Limit output
    #[arg(short, long)]
    limit: bool,

    // Project filter flag
    #[arg(long)]
    pub project: Option<String>,

    // Tags filter
    #[arg(long)]
    pub tags: Option<String>,

    // Tags exclude filter
    #[arg(long)]
    pub exclude_tags: Option<String>,

    /// From date filter value
    #[arg(short, long, value_name = "YYYY-MM-DD")]
    pub from: Option<String>,

    #[arg(
        long,
        value_name = "this-week, tw, last-week, lw, this-month, tm, last-month, lm, month-n,m-n"
    )]
    period: Option<String>,

    /// Format of the output
    #[arg(long, value_name = "text, markdown", default_value = "text")]
    pub format: Option<String>,
}

impl Args {
    #[must_use]
    pub fn parse() -> Self {
        Self::parse_from(std::env::args())
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

    /// Parses filter tags from the command line arguments.
    pub fn tags(&self) -> Option<TagFilter> {
        self.tags
            .as_ref()
            .map(|tags| TagFilter::parse(tags.clone()))
    }

    /// Parses the period from the command line arguments.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError::InvalidPeriod` if the period is not valid.
    pub fn period(&self, clock: &Clock) -> Result<Option<PeriodRequested>, ParseError> {
        match self.from_period(clock) {
            Ok(Some(period)) => Ok(Some(period)),
            Err(err) => Err(err),
            Ok(None) => self.from_date(),
        }
    }

    fn from_period(&self, clock: &Clock) -> Result<Option<PeriodRequested>, ParseError> {
        match self.period.as_ref() {
            Some(period) => PeriodRequested::from_str(period, clock).map(Some),
            None => Ok(None),
        }
    }

    fn from_date(&self) -> Result<Option<PeriodRequested>, ParseError> {
        PeriodRequested::from_from_date(self.from.as_deref())
    }

    #[must_use]
    pub fn limit(&self) -> Option<OutputLimit> {
        if self.limit {
            Some(OutputLimit::CummalitivePercentageThreshhold(90.01))
        } else {
            None
        }
    }

    #[must_use]
    pub fn formatter(&self) -> Box<dyn Formatter> {
        <dyn Formatter>::from_str(&self.format)
    }
}
