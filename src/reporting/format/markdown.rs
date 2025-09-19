use std::fmt::Write;

use crate::domain::reporting::{DetailReport, TimeTotal};
use crate::reporting::format::{format_duration, Formatter};
use crate::reporting::model::FormatableReport;

pub struct MarkdownFormatter;

impl Formatter for MarkdownFormatter {
    fn format(&self, report: &FormatableReport) -> String {
        match report {
            FormatableReport::OverviewReport(report) => Self::format_overview(
                report.entries_time_totals().clone(),
                report.period(),
                report.total_minutes(),
            ),
            FormatableReport::TasksReport(report) => Self::format_tasks_report(report),
        }
    }
}

impl MarkdownFormatter {
    fn format_overview(
        entries: Vec<TimeTotal>,
        period: &crate::domain::TrackingPeriod,
        total_minutes: u32,
    ) -> String {
        let mut result = String::new();

        // Format summary
        let hours_per_day = (f64::from(total_minutes) / 60.0) / f64::from(period.days);
        write!(
            &mut result,
            "# Time Tracking Report\n\n## Overview\n\n- **Period**: {} -> {}\n- **Days Tracked**: {}\n- **Hours per Day**: {:.1}\n- **Total Time**: {}\n\n",
            period.start.0.format("%Y-%m-%d"),
            period.end.0.format("%Y-%m-%d"),
            period.days,
            hours_per_day,
            format_duration(total_minutes)
        ).expect("Writing to String should never fail");

        result.push_str("### Projects\n");
        for entry in entries {
            writeln!(
                &mut result,
                "- **{}**: {} ({}%)",
                entry.description,
                format_duration(entry.minutes),
                entry.percentage
            ).expect("Writing to String should never fail");
        }

        result
    }

    fn format_tasks_report(report: &DetailReport) -> String {
        let mut result = String::new();

        result.push_str("# Time Tracking Details Report\n\n");

        for context_summary in report.summaries() {
            result.push_str(&Self::format_tasks_context(
                context_summary.context().raw_value().as_str(),
                context_summary.task_summaries(),
                report.period(),
                context_summary.total_minutes(),
            ));
        }
        result
    }

    fn format_tasks_context(
        context: &str,
        tasks: &[crate::domain::reporting::TaskSummary],
        period: &crate::domain::TrackingPeriod,
        total_minutes: u32,
    ) -> String {
        let mut result = String::new();

        writeln!(&mut result, "## Project: {context}\n").expect("Writing to String should never fail");

        // Format period and statistics
        let hours_per_day = (f64::from(total_minutes) / 60.0) / f64::from(period.days);
        write!(
            &mut result,
            "- **Period**: {} -> {}\n- **Days Tracked**: {}\n- **Hours per Day**: {:.1}\n- **Total Time**: {}\n\n",
            period.start.0.format("%Y-%m-%d"),
            period.end.0.format("%Y-%m-%d"),
            period.days,
            hours_per_day,
            format_duration(total_minutes)
        ).expect("Writing to String should never fail");

        result.push_str("### Tasks\n\n");
        for task in tasks {
            writeln!(
                &mut result,
                "- **{}**: {} ({}%)",
                task.description,
                format_duration(task.minutes),
                task.percentage_of_total
            ).expect("Writing to String should never fail");
        }
        result.push('\n');

        result
    }
}
