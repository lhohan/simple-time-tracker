#[cfg(test)]
mod tests {
    use crate::domain::{EndDate, StartDate, TimeEntry};
    use chrono::NaiveDate;

    use crate::reporting::Report;
    mod report_tests {

        use crate::domain::TrackedTime;

        use super::helpers::*;
        use super::*;

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
            let report = Report::new_overview(time_report);

            if let Report::Overview { entries, .. } = report {
                let projects: Vec<_> = entries.iter().map(|e| e.project.as_str()).collect();
                assert_eq!(projects, vec!["also-long", "longest", "medium", "short"]);
            } else {
                panic!("Expected Overview report");
            }
        }
    }

    mod formatting_tests {
        use crate::reporting::format::format_duration;

        #[test]
        fn test_format_duration() {
            assert_eq!(format_duration(90), " 1h 30m");
            assert_eq!(format_duration(60), " 1h 00m");
            assert_eq!(format_duration(45), " 0h 45m");
        }
    }

    mod helpers {
        use super::*;

        pub(crate) fn create_test_entry(
            project: &str,
            minutes: u32,
            description: Option<&str>,
        ) -> TimeEntry {
            TimeEntry::new(
                vec![project.to_string()],
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
