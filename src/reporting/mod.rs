pub mod format;
mod model;
pub use crate::reporting::model::FormatableReport;

#[cfg(test)]
mod tests {
    use crate::domain::dates::{EndDate, StartDate};
    use crate::domain::TimeEntry;
    use chrono::NaiveDate;

    mod report_tests {

        use crate::domain::{reporting::OverviewReport, TrackedTime};

        use super::helpers::*;

        #[test]
        fn test_overview_report_ordering() {
            let entries = vec![
                create_test_entry("short", 30),
                create_test_entry("longest", 120),
                create_test_entry("medium", 60),
                create_test_entry("also-long", 120),
            ];

            let (start, end) = default_period();
            let time_report = TrackedTime::new(entries, start, end, 1);
            let report = OverviewReport::overview(&time_report, None, None);

            let entries = report.entries_time_totals();
            let projects: Vec<_> = entries.iter().map(|e| e.description.as_str()).collect();
            assert_eq!(projects, vec!["also-long", "longest", "medium", "short"]);
        }
    }

    mod helpers {
        use super::*;

        pub(crate) fn create_test_entry(tag: &str, minutes: u32) -> TimeEntry {
            TimeEntry::parse(format!("- #{tag} {minutes}m").as_str())
                .unwrap()
                .unwrap()
        }

        pub(crate) fn default_period() -> (StartDate, EndDate) {
            (
                StartDate(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
                EndDate(NaiveDate::from_ymd_opt(2024, 1, 2).unwrap()),
            )
        }
    }
}
