use crate::domain::PeriodRequested;
use crate::domain::RangeDescription;
use crate::domain::TrackingPeriod;

use crate::reporting::format::format_duration;
use crate::reporting::format::Formatter;
use crate::reporting::model::{ProjectSummary, Report, TaskSummary};

pub struct TextFormatter;

impl Formatter for TextFormatter {
    fn format(&self, report: &Report) -> String {
        let mut result = String::new();

        result.push_str(format_interval(&report.period()).as_str());

        let report_str = match report {
            Report::Overview {
                entries,
                period,
                period_requested,
                total_minutes,
            } => Self::format_overview(entries, period, period_requested, *total_minutes),
            Report::ProjectDetail {
                project,
                tasks,
                period: _,
                total_minutes,
            } => Self::format_project_detail(project, tasks, *total_minutes),
        };
        result.push_str(&report_str);

        result
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
        period_requested: &Option<PeriodRequested>,
        total_minutes: u32,
    ) -> String {
        let mut result = String::new();

        let period_description = period_requested.as_ref().map(|p| p.period_description());
        result.push_str(&format_header(period_description.as_ref()));

        result.push_str(format_interval(period).as_str());

        // Format summary
        let hours_per_day = (f64::from(total_minutes) / 60.0) / f64::from(period.days);
        result.push_str(&format!(
            "{} days, {:.1} h/day, {} total\n",
            period.days,
            hours_per_day,
            format_duration(total_minutes)
        ));

        result.push('\n');

        // Format entries
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

    fn format_project_detail(project: &str, tasks: &[TaskSummary], total_minutes: u32) -> String {
        let mut result = String::new();

        result.push_str(&format!("Project: {project}\n"));
        result.push_str(&format!("{} total\n", format_duration(total_minutes)));
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

fn format_padded_description(desc: &str) -> String {
    format!(
        "{}..{}",
        desc,
        ".".repeat(20_usize.saturating_sub(desc.len()))
    )
}

fn format_header(period_description: Option<&RangeDescription>) -> String {
    let mut result = String::new();

    result.push_str("Time tracking report for ");
    let period_description_str = period_description
        .map(|description| description.to_string())
        .unwrap_or_default();
    result.push_str(period_description_str.as_str());
    result
}
