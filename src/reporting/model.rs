use std::collections::HashMap;

use crate::domain::{EndDate, StartDate, TimeEntry};
use itertools::Itertools;

#[derive(Debug)]
pub enum Report {
    Overview {
        entries: Vec<ProjectSummary>,
        period: ReportPeriod,
        total_minutes: u32,
    },
    ProjectDetail {
        project: String,
        tasks: Vec<TaskSummary>,
        period: ReportPeriod,
        total_minutes: u32,
    },
}

impl Report {
    pub fn new_overview(
        entries: Vec<TimeEntry>,
        start: StartDate,
        end: EndDate,
        days: u32,
    ) -> Self {
        let period = ReportPeriod::new(start, end, days);
        let summarized = summarize_entries(&entries);
        let total_minutes: u32 = summarized.iter().map(|(_, minutes)| minutes).sum();

        Report::Overview {
            entries: summarized
                .into_iter()
                .map(|(project, minutes)| ProjectSummary::new(project, minutes, total_minutes))
                .sorted_by(|a, b| b.minutes.cmp(&a.minutes).then(a.project.cmp(&b.project)))
                .collect(),
            period,
            total_minutes,
        }
    }

    pub fn new_project_detail(
        entries: Vec<TimeEntry>,
        project: String,
        start: StartDate,
        end: EndDate,
        days: u32,
    ) -> Self {
        let period = ReportPeriod::new(start, end, days);
        let summarized = summarize_tasks(&entries);
        let total_minutes: u32 = summarized.iter().map(|(_, minutes)| minutes).sum();

        Report::ProjectDetail {
            project: project.clone(),
            tasks: summarized
                .into_iter()
                .map(|(desc, minutes)| TaskSummary::new(desc, minutes, total_minutes))
                .sorted_by(|a, b| b.minutes.cmp(&a.minutes))
                .collect(),
            period,
            total_minutes,
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

#[derive(Debug, Clone)]
pub struct ReportPeriod {
    pub(crate) start: StartDate,
    pub(crate) end: EndDate,
    pub(crate) days: u32,
}

impl ReportPeriod {
    pub fn new(start: StartDate, end: EndDate, days: u32) -> Self {
        Self { start, end, days }
    }
}

fn calculate_percentage(minutes: u32, total_minutes: u32) -> u32 {
    ((minutes as f64 / total_minutes as f64) * 100.0).round() as u32
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

    for entry in entries.iter() {
        let key = entry
            .description
            .clone()
            .unwrap_or_else(|| "<no description>".to_string());
        *summary.entry(key).or_insert(0) += entry.minutes;
    }

    summary.into_iter().collect()
}
