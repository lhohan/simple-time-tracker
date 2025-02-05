use crate::domain::TrackingPeriod;

use super::model::{ProjectSummary, Report, TaskSummary};
use std::fmt;

pub struct TextFormatter;

impl TextFormatter {
    pub fn format(report: &Report) -> String {
        match report {
            Report::Overview {
                entries,
                period,
                total_minutes,
            } => Self::format_overview(entries, period, *total_minutes),
            Report::ProjectDetail {
                project,
                tasks,
                period,
                total_minutes,
            } => Self::format_project_detail(project, tasks, period, *total_minutes),
        }
    }

    fn format_overview(
        entries: &[ProjectSummary],
        period: &TrackingPeriod,
        total_minutes: u32,
    ) -> String {
        let mut result = String::new();

        // Format entries
        for entry in entries {
            result.push_str(&format!(
                "{}..{} ({:>3}%)\n",
                format!("{:.<20}", entry.project),
                format_duration(entry.minutes),
                entry.percentage
            ));
        }

        // Format summary
        result.push_str(&"-".repeat(40));
        result.push('\n');

        let hours_per_day = (total_minutes as f64 / 60.0) / period.days as f64;
        result.push_str(&format!(
            "{} days, {:.1} h/day\n",
            period.days, hours_per_day
        ));

        // Add date range
        result.push_str(&format!(
            "{} -> {}",
            period.start.0.format("%Y-%m-%d"),
            period.end.0.format("%Y-%m-%d")
        ));

        result
    }

    fn format_project_detail(
        project: &str,
        tasks: &[TaskSummary],
        period: &TrackingPeriod,
        total_minutes: u32,
    ) -> String {
        let mut result = String::new();

        result.push_str(&format!("Project: {}\n", project));
        result.push_str(&format!(
            "Total time: {}\n\n",
            format_duration(total_minutes)
        ));

        result.push_str("Tasks:\n");
        for task in tasks {
            result.push_str(&format!(
                "- {}{} ({}%)\n",
                format_padded_description(&task.description),
                format_duration(task.minutes),
                task.percentage
            ));
        }

        result.push_str(&format!(
            "{} -> {}",
            period.start.0.format("%Y-%m-%d"),
            period.end.0.format("%Y-%m-%d")
        ));

        result
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", TextFormatter::format(self))
    }
}

fn format_padded_description(desc: &str) -> String {
    format!(
        "{}..{}",
        desc,
        ".".repeat(20_usize.saturating_sub(desc.len()))
    )
}

pub(crate) fn format_duration(minutes: u32) -> String {
    let hours = minutes / 60;
    let remaining_minutes = minutes % 60;
    format!("{:2}h {:2}m", hours, remaining_minutes)
}
