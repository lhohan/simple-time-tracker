use crate::domain::RangeDescription;
use crate::domain::TrackingPeriod;

use crate::reporting::format::format_duration;
use crate::reporting::format::Formatter;
use crate::reporting::model::{ProjectSummary, Report, TaskSummary};

pub struct TextFormatter;

impl Formatter for TextFormatter {
    fn format(&self, report: &Report) -> String {
        match report {
            Report::Overview {
                entries,
                period,
                period_requested,
                total_minutes,
            } => {
                let description = period_requested.as_ref().map(|p| p.period_description());
                Self::format_overview(entries, period, &description, *total_minutes)
            }
            Report::ProjectDetail {
                project,
                tasks,
                period,
                total_minutes,
            } => Self::format_project_detail(project, tasks, period, *total_minutes),
        }
    }
}

fn format_interval(period: &TrackingPeriod) -> String {
    format!(
        "{} -> {}",
        period.start.0.format("%Y-%m-%d"),
        period.end.0.format("%Y-%m-%d")
    )
}

impl TextFormatter {
    fn format_overview(
        entries: &[ProjectSummary],
        period: &TrackingPeriod,
        range_description: &Option<RangeDescription>,
        total_minutes: u32,
    ) -> String {
        let mut result = String::new();

        result.push_str(&format_header(range_description.as_ref()));
        result.push('\n');
        result.push_str(format_interval(period).as_str());
        result.push('\n');
        result.push_str(&format_time_statistics(period, total_minutes));
        result.push('\n');

        for entry in entries {
            result.push_str(&format!(
                "{:.<20}..{} ({:>3}%)\n",
                entry.project,
                format_duration(entry.minutes),
                entry.percentage
            ));
        }

        result
    }

    fn format_project_detail(
        project: &str,
        tasks: &[TaskSummary],
        period: &TrackingPeriod,
        total_minutes: u32,
    ) -> String {
        let mut result = String::new();

        result.push_str(&format!("Project: {project}"));
        result.push('\n');
        result.push_str(&format_interval(period));
        result.push('\n');
        result.push_str(&format_time_statistics(period, total_minutes));
        result.push('\n');

        result.push_str("Tasks:\n");
        for task in tasks {
            result.push_str(&format!(
                "- {}{} ({}%)\n",
                format_padded_description(&task.description),
                format_duration(task.minutes),
                task.percentage
            ));
        }

        result
    }
}

fn format_time_statistics(period: &TrackingPeriod, total_minutes: u32) -> String {
    let hours_per_day = (f64::from(total_minutes) / 60.0) / f64::from(period.days);
    format!(
        "{} days, {:.1} h/day, {} total\n",
        period.days,
        hours_per_day,
        format_duration(total_minutes)
    )
}

fn format_padded_description(desc: &str) -> String {
    format!(
        "{}..{}",
        desc,
        ".".repeat(20_usize.saturating_sub(desc.len()))
    )
}

fn format_header(period_description: Option<&RangeDescription>) -> String {
    let mut result = String::new();

    result.push_str("Time tracking report ");
    let period_description_str = period_description
        .map(|description| description.to_string())
        .unwrap_or_default();
    result.push_str(period_description_str.as_str());
    result
}
