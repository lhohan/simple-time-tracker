use crate::reporting::format::{format_duration, Formatter};
use crate::reporting::model::{FormatableReport, Report};

pub struct MarkdownFormatter;

impl Formatter for MarkdownFormatter {
    fn format(&self, report: &FormatableReport) -> String {
        match report {
            FormatableReport::LegacyReport(Report::Overview {
                entries,
                period,
                period_requested: _,
                total_minutes,
            }) => Self::format_overview(entries, period, *total_minutes),
            _ => todo!(),
        }
    }
}

impl MarkdownFormatter {
    fn format_overview(
        entries: &[crate::reporting::model::ContextSummary],
        period: &crate::domain::TrackingPeriod,
        total_minutes: u32,
    ) -> String {
        let mut result = String::new();

        // Format summary
        let hours_per_day = (f64::from(total_minutes) / 60.0) / f64::from(period.days);
        result.push_str(&format!(
            "# Time Tracking Report\n\n## Overview\n\n- **Period**: {} -> {}\n- **Days Tracked**: {}\n- **Hours per Day**: {:.1}\n- **Total Time**: {}\n\n",
            period.start.0.format("%Y-%m-%d"),
            period.end.0.format("%Y-%m-%d"),
            period.days,
            hours_per_day,
            format_duration(total_minutes)
        ));

        result.push_str("### Projects\n");
        for entry in entries {
            result.push_str(&format!(
                "- **{}**: {} ({}%)\n",
                entry.description,
                format_duration(entry.minutes),
                entry.percentage
            ));
        }

        result
    }
}
