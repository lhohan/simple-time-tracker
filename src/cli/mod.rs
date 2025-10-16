use clap::Parser;
use std::path::PathBuf;

use crate::domain::reporting::{BreakdownUnit, OutputLimit};
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

    #[arg(long, value_name = "text, markdown", default_value = "text")]
    pub format: Option<String>,

    #[arg(long, value_name = "day, week, month, year, auto")]
    pub breakdown: Option<String>,
}

impl Args {
    /// Parses command line arguments and validates them.
    ///
    /// # Errors
    ///
    /// Returns a `String` error if the arguments are invalid or violate validation rules.
    #[must_use = "parsed arguments must be used to configure the application"]
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

        // Check if breakdown is specified without tags or project
        if self.breakdown.is_some() && self.tags.is_none() && self.project.is_none() {
            return Err(
                "--breakdown flag requires --tags or --project to be specified".to_string(),
            );
        }

        Ok(())
    }

    /// Parses exclude tags from the command line arguments.
    #[must_use]
    pub fn exclude_tags(&self) -> Vec<String> {
        match &self.exclude_tags {
            Some(tags) => {
                let parsed_tags = tags
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>();
                parsed_tags
            }
            None => vec![],
        }
    }

    #[must_use]
    pub fn include_details(&self) -> bool {
        self.project.is_some() || self.details
    }

    /// Parses filter tags from the command line arguments.
    #[must_use]
    pub fn context_filter(&self) -> Option<TagFilter> {
        fn parse_project_tags(maybe_project: Option<&String>) -> Vec<String> {
            maybe_project.map_or_else(Vec::new, |p| vec![p.clone()])
        }

        fn parse_tags(tags: &[String], maybe_tags: Option<&String>) -> Vec<String> {
            maybe_tags.filter(|s| !s.is_empty()).map_or_else(
                || tags.to_vec(),
                |tag_list| tag_list.split(',').map(|s| s.trim().to_string()).collect(),
            )
        }
        fn to_filter(tags: Vec<String>) -> Option<TagFilter> {
            (!tags.is_empty()).then(|| TagFilter::parse(tags))
        }

        let tags = parse_project_tags(self.project.as_ref());
        let tags = parse_tags(&tags, self.tags.as_ref());
        to_filter(tags)
    }

    /// Parses the period from the command line arguments.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError::InvalidPeriod` if the period is not valid.
    pub fn period(&self, clock: &Clock) -> Result<Option<PeriodRequested>, ParseError> {
        match self.parse_period(clock) {
            Ok(Some(period)) => Ok(Some(period)),
            Err(err) => Err(err),
            Ok(None) => self.parse_date(),
        }
    }

    fn parse_period(&self, clock: &Clock) -> Result<Option<PeriodRequested>, ParseError> {
        match self.period.as_ref() {
            Some(period) => PeriodRequested::from_str(period, clock).map(Some),
            None => Ok(None),
        }
    }

    fn parse_date(&self) -> Result<Option<PeriodRequested>, ParseError> {
        PeriodRequested::parse_from_date(self.from.as_deref())
    }

    #[must_use]
    pub fn limit(&self) -> Option<OutputLimit> {
        if self.limit {
            Some(OutputLimit::CumulativePercentageThreshold(90.00))
        } else {
            None
        }
    }

    #[must_use]
    pub fn formatter(&self) -> Box<dyn Formatter> {
        <dyn Formatter>::from_str(self.format.as_ref())
    }

    #[must_use]
    pub fn breakdown_unit(&self, period: Option<&PeriodRequested>) -> Option<BreakdownUnit> {
        self.breakdown.as_ref().and_then(|b| {
            let breakdown_str = b.to_lowercase();
            match breakdown_str.as_str() {
                "auto" => Self::auto_breakdown_unit(period),
                "day" => Some(BreakdownUnit::Day),
                "week" => Some(BreakdownUnit::Week),
                "month" => Some(BreakdownUnit::Month),
                "year" => Some(BreakdownUnit::Year),
                _ => None,
            }
        })
    }

    #[must_use]
    fn auto_breakdown_unit(period: Option<&PeriodRequested>) -> Option<BreakdownUnit> {
        // Resolve to one level above the period:
        // day -> week, week -> month, month -> year, year -> year
        period.map(|p| match p {
            PeriodRequested::Day(_) | PeriodRequested::FromDate(_) => BreakdownUnit::Week,
            PeriodRequested::WeekOf(_) => BreakdownUnit::Month,
            PeriodRequested::MonthOf(_) | PeriodRequested::YearOf(_) => BreakdownUnit::Year,
        })
    }
}
