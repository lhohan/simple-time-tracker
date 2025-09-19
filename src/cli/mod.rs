use clap::Parser;
use std::path::PathBuf;

use crate::domain::reporting::OutputLimit;
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

    /// Show project details, e.g. tasks
    #[arg(short, long)]
    details: bool,

    // Project filter flag
    #[arg(long)]
    project: Option<String>,

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
    pub fn parse() -> Result<Self, String> {
        let args = Self::parse_from(std::env::args());
        args.validate()?;
        Ok(args)
    }

    fn validate(&self) -> Result<(), String> {
        // Check if details is specified without tags
        if self.details && self.tags.is_none() && self.project.is_none() {
            return Err("--details flag requires --tags to be specified".to_string());
        }

        // Add any other validations here

        Ok(())
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

    pub fn include_details(&self) -> bool {
        self.project.is_some() || self.details
    }

    /// Parses filter tags from the command line arguments.
    pub fn context_filter(&self) -> Option<TagFilter> {
        fn parse_project_tags(maybe_project: &Option<String>) -> Vec<String> {
            maybe_project.clone().map_or_else(Vec::new, |p| vec![p])
        }

        fn parse_tags(tags: &[String], maybe_tags: &Option<String>) -> Vec<String> {
            maybe_tags.as_ref().filter(|s| !s.is_empty()).map_or_else(
                || tags.to_vec(),
                |tag_list| tag_list.split(',').map(String::from).collect(),
            )
        }
        fn to_filter(tags: Vec<String>) -> Option<TagFilter> {
            (!tags.is_empty()).then(|| TagFilter::parse(tags))
        }

        let tags = parse_project_tags(&self.project);
        let tags = parse_tags(&tags, &self.tags);
        to_filter(tags)
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
            Some(OutputLimit::CummalitivePercentageThreshhold(90.00))
        } else {
            None
        }
    }

    #[must_use]
    pub fn formatter(&self) -> Box<dyn Formatter> {
        <dyn Formatter>::from_str(&self.format)
    }
}
