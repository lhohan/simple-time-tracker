use crate::domain::TimeEntry;
use itertools::Itertools;

pub struct Report {
    entries: Vec<TimeEntry>,
    total_minutes: u32,
    days: u32,
    project_filter: Option<String>,
}

impl Report {
    pub fn new(entries: Vec<TimeEntry>, days: u32, project_filter: Option<String>) -> Self {
        let entries = if project_filter.is_none() {
            // Only summarize for the overview display
            Self::summarize_entries(entries)
        } else {
            // Keep original entries for filtered view
            entries
        };

        let entries: Vec<_> = entries
            .into_iter()
            .sorted_by(|a, b| b.minutes.cmp(&a.minutes).then(a.project.cmp(&b.project)))
            .collect();

        let total_minutes = entries.iter().map(|e| e.minutes).sum();

        Self {
            entries,
            total_minutes,
            days,
            project_filter,
        }
    }

    fn summarize_entries(entries: Vec<TimeEntry>) -> Vec<TimeEntry> {
        let mut summary = std::collections::HashMap::new();

        for entry in entries {
            *summary.entry(entry.project).or_insert(0) += entry.minutes;
        }

        summary
            .into_iter()
            .map(|(project, minutes)| TimeEntry::new(project, minutes, None))
            .collect()
    }

    pub fn display(&self) {
        if let Some(project_filter) = &self.project_filter {
            let filtered = self.filtered_entries();
            let project_total: u32 = filtered.iter().map(|e| e.minutes).sum();
            let total_percentage =
                (project_total as f64 / self.total_minutes as f64 * 100.0).round() as u32;

            println!("Project: {}", project_filter);
            println!(
                "Total time: {} ({}% of total time)",
                format_duration(project_total),
                total_percentage
            );
            println!();
            println!("Tasks:");

            for entry in filtered {
                let percentage =
                    (entry.minutes as f64 / project_total as f64 * 100.0).round() as u32;
                if let Some(desc) = &entry.description {
                    println!(
                        "- {}{} ({}%)",
                        format!(
                            "{}..{}",
                            desc,
                            ".".repeat(20_usize.saturating_sub(desc.len()))
                        ),
                        format_duration(entry.minutes),
                        percentage
                    );
                }
            }
        } else {
            for entry in &self.entries {
                let percentage = self.calculate_percentage(entry.minutes);
                println!(
                    "{}..{} ({:>3}%)",
                    format!("{:.<20}", entry.project),
                    format_duration(entry.minutes),
                    percentage
                );
            }

            println!("{}", "-".repeat(40));
            print!("{} days", self.days);
            print!(", ");
            println!(
                "{:.1} h/day",
                (self.total_minutes as f64 / 60.0) / self.days as f64,
            );
        }
    }

    fn filtered_entries(&self) -> Vec<&TimeEntry> {
        match &self.project_filter {
            Some(project) => self
                .entries
                .iter()
                .filter(|entry| entry.project.eq_ignore_ascii_case(project))
                .collect(),
            None => self.entries.iter().collect(),
        }
    }

    fn calculate_percentage(&self, minutes: u32) -> u32 {
        ((minutes as f64 / self.total_minutes as f64) * 100.0).round() as u32
    }
}

fn format_duration(minutes: u32) -> String {
    let hours = minutes / 60;
    let remaining_minutes = minutes % 60;
    format!("{:2}h {:2}m", hours, remaining_minutes)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn test_report_ordering() {
        let entries = create_entries_with_duration(&[
            ("short", 30),
            ("longest", 120),
            ("medium", 60),
            ("also-long", 120),
        ]);

        assert_report(entries).should_be_ordered_as(&["also-long", "longest", "medium", "short"]);
    }

    #[test]
    fn test_filtered_entries_with_filter() {
        let entries = create_entries(&["dev", "dev", "sport"]);
        assert_report_with_filter(entries, Some("dev".to_string()))
            .should_find(2)
            .entries_of("dev");
    }

    #[test]
    fn test_filtered_entries_without_filter() {
        let entries = create_entries(&["dev", "sport", "read"]);
        assert_report_with_filter(entries, None).should_find(3);
    }

    fn entry_with_duration(project: &str, minutes: u32) -> TimeEntry {
        TimeEntry::new(project.to_string(), minutes, None)
    }

    fn create_entries_with_duration(projects: &[(&str, u32)]) -> Vec<TimeEntry> {
        projects
            .iter()
            .map(|&(project, minutes)| entry_with_duration(project, minutes))
            .collect()
    }

    fn create_entries(projects: &[&str]) -> Vec<TimeEntry> {
        // Reuse create_entries_with_duration by mapping each project to a tuple with default duration
        create_entries_with_duration(&projects.iter().map(|&p| (p, 60)).collect::<Vec<_>>())
    }

    struct ReportAssertion {
        filtered_entries: Vec<TimeEntry>, // Changed to owned TimeEntry instead of references
    }

    impl ReportAssertion {
        fn should_find(&self, count: usize) -> &Self {
            assert_eq!(self.filtered_entries.len(), count);
            self
        }

        fn entries_of(&self, project: &str) {
            assert!(self.filtered_entries.iter().all(|e| e.project == project));
        }
        fn should_be_ordered_as(&self, expected_projects: &[&str]) {
            let actual_projects: Vec<&str> = self
                .filtered_entries
                .iter()
                .map(|e| e.project.as_str())
                .collect();

            assert_eq!(
                actual_projects.len(),
                expected_projects.len(),
                "Number of entries doesn't match.\nExpected: {:?}\nActual: {:?}",
                expected_projects,
                actual_projects
            );

            for (entry, &expected) in self.filtered_entries.iter().zip(expected_projects) {
                assert_eq!(
                    entry.project, expected,
                    "Project ordering mismatch.\nExpected: {:?}\nActual: {:?}",
                    expected_projects, actual_projects
                );
            }
        }
    }

    fn assert_report_with_filter(
        entries: Vec<TimeEntry>,
        project_filter: Option<String>,
    ) -> ReportAssertion {
        let report = Report::new(entries, 1, project_filter);
        ReportAssertion {
            filtered_entries: report.filtered_entries().into_iter().cloned().collect(), // Convert references to owned values
        }
    }

    fn assert_report(entries: Vec<TimeEntry>) -> ReportAssertion {
        let report = Report::new(entries, 1, None);
        ReportAssertion {
            filtered_entries: report.filtered_entries().into_iter().cloned().collect(), // Convert references to owned values
        }
    }
}
