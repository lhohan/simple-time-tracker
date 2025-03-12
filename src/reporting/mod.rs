pub mod format;
mod model;
pub use crate::reporting::model::Report;
pub use crate::reporting::model::ReportTypeRequested;

#[cfg(test)]
mod tests {
    use crate::domain::dates::{EndDate, StartDate};
    use crate::domain::TimeEntry;
    use chrono::NaiveDate;

    mod report_tests {

        use crate::domain::TrackedTime;

        use super::helpers::*;
        use crate::Report;

        #[test]
        fn test_overview_report_ordering() {
            let entries = vec![
                create_test_entry("short", 30, None),
                create_test_entry("longest", 120, None),
                create_test_entry("medium", 60, None),
                create_test_entry("also-long", 120, None),
            ];

            let (start, end) = default_period();
            let time_report = TrackedTime::new(entries, start, end, 1);
            let report = Report::overview(&time_report, None, &None);

            if let Report::Overview { entries, .. } = report {
                let projects: Vec<_> = entries.iter().map(|e| e.project.as_str()).collect();
                assert_eq!(projects, vec!["also-long", "longest", "medium", "short"]);
            } else {
                panic!("Expected Overview report");
            }
        }
    }

    mod helpers {
        use crate::domain::tags::Tag;

        use super::*;

        pub(crate) fn create_test_entry(
            tag: &str,
            minutes: u32,
            description: Option<&str>,
        ) -> TimeEntry {
            TimeEntry::new(
                vec![Tag::from_raw(tag)],
                minutes,
                description.map(String::from),
            )
        }

        pub(crate) fn default_period() -> (StartDate, EndDate) {
            (
                StartDate(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
                EndDate(NaiveDate::from_ymd_opt(2024, 1, 2).unwrap()),
            )
        }
    }
}
