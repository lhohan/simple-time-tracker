use std::collections::HashMap;

use crate::domain::{TimeEntry, TrackedTime, TrackingPeriod};
use itertools::Itertools;

#[derive(Debug)]
pub enum Report {
    Overview {
        entries: Vec<ProjectSummary>,
        period: TrackingPeriod,
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
pub enum ReportType {
    Projects,
    ProjectDetails(String),
}

impl Report {
    pub fn new_overview(time_report: TrackedTime) -> Self {
        let summarized = summarize_entries(&time_report.entries);

        Report::Overview {
            entries: summarized
                .into_iter()
                .map(|(project, minutes)| {
                    ProjectSummary::new(project, minutes, time_report.total_minutes)
                })
                .sorted_by(|a, b| b.minutes.cmp(&a.minutes).then(a.project.cmp(&b.project)))
                .collect(),
            period: time_report.period,
            total_minutes: time_report.total_minutes,
        }
    }

    pub fn new_project_detail(time_report: TrackedTime, project: &str) -> Self {
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
        *summary.entry(entry.main_project().clone()).or_insert(0) += entry.minutes;
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
