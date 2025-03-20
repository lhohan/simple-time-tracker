use std::collections::HashMap;

use crate::domain::{
    reports::OutputLimit, PeriodRequested, TimeEntry, TrackedTime, TrackingPeriod,
};
use itertools::Itertools;

#[derive(Debug)]
pub enum Report {
    Overview {
        entries: Vec<ProjectSummary>,
        period: TrackingPeriod,
        period_requested: Option<PeriodRequested>,
        total_minutes: u32,
    },
    ProjectDetail {
        project: String,
        tasks: Vec<TaskSummary>,
        period: TrackingPeriod,
        total_minutes: u32,
    },
}

#[derive(Debug)]
pub enum ReportTypeRequested {
    Overview,
    ProjectDetails(Vec<String>),
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
                ProjectSummary::new(project, minutes, time_report.total_minutes)
            })
            .sorted_by(|a, b| b.minutes.cmp(&a.minutes).then(a.project.cmp(&b.project)));

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

    pub fn project_details(time_report: &TrackedTime, project: &str) -> Self {
        let summarized = summarize_tasks(&time_report.entries);

        Report::ProjectDetail {
            project: project.to_string(),
            tasks: summarized
                .into_iter()
                .map(|(desc, minutes)| TaskSummary::new(desc, minutes, time_report.total_minutes))
                .sorted_by(|a, b| b.minutes.cmp(&a.minutes))
                .collect(),
            period: time_report.period,
            total_minutes: time_report.total_minutes,
        }
    }

    pub fn period(&self) -> &TrackingPeriod {
        match self {
            Report::Overview { period, .. } => &period,
            Report::ProjectDetail { period, .. } => &period,
        }
    }
}

fn limit_number_of_entries(
    total_minutes: f64,
    summaries_sorted: std::vec::IntoIter<ProjectSummary>,
    cumulative_percentage_threshold: f64,
) -> Vec<ProjectSummary> {
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
pub struct ProjectSummary {
    pub(crate) project: String,
    pub(crate) minutes: u32,
    pub(crate) percentage: u32,
}

impl ProjectSummary {
    pub fn new(project: String, minutes: u32, total_minutes: u32) -> Self {
        Self {
            project,
            minutes,
            percentage: calculate_percentage(minutes, total_minutes),
        }
    }
}

#[derive(Debug)]
pub struct TaskSummary {
    pub(crate) description: String,
    pub(crate) minutes: u32,
    pub(crate) percentage: u32,
}

impl TaskSummary {
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

fn summarize_tasks(entries: &[TimeEntry]) -> Vec<(String, u32)> {
    let mut summary = HashMap::new();

    for entry in entries {
        let key = entry
            .description
            .clone()
            .unwrap_or_else(|| "<no description>".to_string());
        *summary.entry(key).or_insert(0) += entry.minutes;
    }

    summary.into_iter().collect()
}
