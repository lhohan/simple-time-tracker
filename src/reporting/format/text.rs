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
                total_minutes,
            } => Self::format_overview(entries, period, *total_minutes),
            Report::ProjectDetail {
                project,
                tasks,
                total_minutes,
            } => Self::format_project_detail(project, tasks, *total_minutes),
        }
    }
}

impl TextFormatter {
    fn format_overview(
        entries: &[ProjectSummary],
        period: &TrackingPeriod,
        total_minutes: u32,
    ) -> String {
        let mut result = String::new();

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
