use std::collections::HashMap;

use crate::domain::reporting::DetailReport;
use crate::domain::tags::Tag;
use crate::domain::{
    reporting::OutputLimit, PeriodRequested, TimeEntry, TrackedTime, TrackingPeriod,
};
use itertools::Itertools;

pub enum FormatableReport<'a> {
    LegacyReport(&'a Report),
    TasksReport(&'a DetailReport),
}

#[derive(Debug)]
pub enum Report {
    Overview {
        entries: Vec<ContextSummary>,
        period: TrackingPeriod,
        period_requested: Option<PeriodRequested>,
        total_minutes: u32,
    },
}

#[derive(Debug)]
pub enum ReportTypeRequested {
    Overview,
    ProjectDetails(Vec<Tag>),
}

impl Report {
    pub fn overview(
        time_report: &TrackedTime,
        limit: Option<OutputLimit>,
        period_requested: &Option<PeriodRequested>,
    ) -> Self {
        let summarized = summarize_entries(&time_report.entries);

        let summaries_sorted = summarized
            .into_iter()
            .map(|(project, minutes)| {
                ContextSummary::new(project, minutes, time_report.total_minutes)
            })
            .sorted_by(|a, b| {
                b.minutes
                    .cmp(&a.minutes)
                    .then(a.description.cmp(&b.description))
            });

        let entries = match limit {
            Some(OutputLimit::CummalitivePercentageThreshhold(threshold)) => {
                let total_minutes = time_report.total_minutes as f64;
                limit_number_of_entries(total_minutes, summaries_sorted, threshold)
            }
            None => summaries_sorted.collect(),
        };

        Report::Overview {
            entries,
            period: time_report.period,
            period_requested: period_requested.clone(),
            total_minutes: time_report.total_minutes,
        }
    }
}

fn limit_number_of_entries(
    total_minutes: f64,
    summaries_sorted: std::vec::IntoIter<ContextSummary>,
    cumulative_percentage_threshold: f64,
) -> Vec<ContextSummary> {
    summaries_sorted
        .scan(0.0, |cumulative_percentage, entry| {
            let percentage = (entry.minutes as f64 / total_minutes) * 100.0;
            *cumulative_percentage += percentage;
            Some((entry, *cumulative_percentage))
        })
        .take_while(|(_, cumulative_percentage)| {
            *cumulative_percentage <= cumulative_percentage_threshold
        })
        .map(|(entry, _)| entry)
        .collect()
}

#[derive(Debug)]
pub struct ContextSummary {
    pub(crate) description: String,
    pub(crate) minutes: u32,
    pub(crate) percentage: u32,
}

impl ContextSummary {
    pub fn new(description: String, minutes: u32, total_minutes: u32) -> Self {
        Self {
            description,
            minutes,
            percentage: calculate_percentage(minutes, total_minutes),
        }
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn calculate_percentage(minutes: u32, total_minutes: u32) -> u32 {
    ((f64::from(minutes) / f64::from(total_minutes)) * 100.0).round() as u32
}

fn summarize_entries(entries: &[TimeEntry]) -> Vec<(String, u32)> {
    let mut summary = HashMap::new();

    for entry in entries {
        *summary.entry(entry.main_context().clone()).or_insert(0) += entry.minutes;
    }

    summary.into_iter().collect()
}
