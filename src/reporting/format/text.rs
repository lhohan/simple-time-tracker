use crate::domain::reports;
use crate::domain::reports::TasksReport;
use crate::domain::PeriodDescription;
use crate::domain::TrackingPeriod;

use crate::reporting::format::format_duration;
use crate::reporting::format::Formatter;
use crate::reporting::model::FormatableReport;
use crate::reporting::model::{ContextSummary, Report};

pub struct TextFormatter;

impl Formatter for TextFormatter {
    fn format(&self, report: &FormatableReport) -> String {
        match report {
            FormatableReport::LegacyReport(Report::Overview {
                entries,
                period,
                period_requested,
                total_minutes,
            }) => {
                let description = period_requested.as_ref().map(|p| p.description());
                Self::format_overview(entries, period, &description, *total_minutes)
            }
            FormatableReport::TasksReport(report) => Self::format_tasks_report(report),
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
        entries: &[ContextSummary],
        period: &TrackingPeriod,
        range_description: &Option<PeriodDescription>,
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
                entry.description,
                format_duration(entry.minutes),
                entry.percentage
            ));
        }

        result
    }

    fn format_tasks_report(report: &TasksReport) -> String {
        let mut result = String::new();
        for context_summary in report.summaries() {
            result.push_str(&Self::format_tasks_context(
                context_summary.context().raw_value().as_str(),
                &context_summary.task_summaries(),
                &report.period(),
                context_summary.total_minutes(),
            ));
            result.push('\n');
        }
        result
    }

    fn format_tasks_context(
        context: &str,
        tasks: &[reports::TaskSummary],
        period: &TrackingPeriod,
        total_minutes: u32,
    ) -> String {
        let mut result = String::new();

        result.push_str(&format!("Project: {context}"));
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
                task.percentage_of_total
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

fn format_header(period_description: Option<&PeriodDescription>) -> String {
    let mut result = String::new();

    result.push_str("Time tracking report ");
    let period_description_str = period_description
        .map(|description| description.to_string())
        .unwrap_or_default();
    result.push_str(period_description_str.as_str());
    result
}
