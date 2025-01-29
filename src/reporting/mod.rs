use crate::domain::{EndDate, StartDate, TimeEntry};
use itertools::Itertools;
use std::fmt;

pub struct Report {
    entries: Vec<TimeEntry>,
    total_minutes: u32,
    days: u32,
    project_filter: Option<String>,
    start_date: StartDate,
    end_date: EndDate,
}

impl Report {
    pub fn new(
        entries: Vec<TimeEntry>,
        days: u32,
        project_filter: Option<String>,
        start_date: StartDate,
        end_date: EndDate,
    ) -> Self {
        let entries = if let Some(project) = &project_filter {
            // Summarize tasks for the filtered project
            Self::summarize_tasks_by_description(entries, project)
        } else {
            // Keep existing project summarization for overview
            Self::summarize_entries(entries)
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
            start_date,
            end_date,
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

    fn summarize_tasks_by_description(entries: Vec<TimeEntry>, project: &str) -> Vec<TimeEntry> {
        let mut summary = std::collections::HashMap::new();

        for entry in entries
            .into_iter()
            .filter(|e| e.project.eq_ignore_ascii_case(project))
        {
            let key = entry
                .description
                .unwrap_or_else(|| "<no description>".to_string());
            *summary.entry(key).or_insert(0) += entry.minutes;
        }

        summary
            .into_iter()
            .map(|(description, minutes)| {
                TimeEntry::new(project.to_string(), minutes, Some(description))
            })
            .collect()
    }

    fn format_project_details(&self, f: &mut fmt::Formatter<'_>, project: &str) -> fmt::Result {
        let filtered = self.filtered_entries();
        let project_total: u32 = filtered.iter().map(|e| e.minutes).sum();
        let total_percentage = self.calculate_percentage(project_total);

        writeln!(f, "Project: {}", project)?;
        writeln!(
            f,
            "Total time: {} ({}% of total time)",
            format_duration(project_total),
            total_percentage
        )?;
        writeln!(f)?;
        writeln!(f, "Tasks:")?;

        for entry in filtered {
            self.format_task_entry(f, entry, project_total)?;
        }
        Ok(())
    }

    fn format_task_entry(
        &self,
        f: &mut fmt::Formatter<'_>,
        entry: &TimeEntry,
        project_total: u32,
    ) -> fmt::Result {
        let percentage = ((entry.minutes as f64 / project_total as f64) * 100.0).round() as u32;
        let description = entry
            .description
            .as_deref()
            .unwrap_or("<task without description>");

        writeln!(
            f,
            "- {}{} ({}%)",
            format_padded_description(description),
            format_duration(entry.minutes),
            percentage
        )
    }

    fn format_overview(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format entries
        for entry in &self.entries {
            let percentage = self.calculate_percentage(entry.minutes);
            writeln!(
                f,
                "{}..{} ({:>3}%)",
                format!("{:.<20}", entry.project),
                format_duration(entry.minutes),
                percentage
            )?;
        }

        // Format summary
        writeln!(f, "{}", "-".repeat(40))?;
        let hours_per_day = (self.total_minutes as f64 / 60.0) / self.days as f64;
        writeln!(f, "{} days, {:.1} h/day", self.days, hours_per_day)
    }

    fn format_date_range(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} -> {}",
            self.start_date.0.format("%Y-%m-%d"),
            self.end_date.0.format("%Y-%m-%d")
        )
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.project_filter {
            Some(project) => self.format_project_details(f, project),
            None => self.format_overview(f),
        }?;

        self.format_date_range(f)
    }
}

fn format_padded_description(desc: &str) -> String {
    format!(
        "{}..{}",
        desc,
        ".".repeat(20_usize.saturating_sub(desc.len()))
    )
}

fn format_duration(minutes: u32) -> String {
    let hours = minutes / 60;
    let remaining_minutes = minutes % 60;
    format!("{:2}h {:2}m", hours, remaining_minutes)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use chrono::NaiveDate;

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
            .should_find(1)
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
        let report = Report::new(
            entries,
            1,
            project_filter,
            StartDate(NaiveDate::default()),
            EndDate(NaiveDate::default()),
        );
        ReportAssertion {
            filtered_entries: report.filtered_entries().into_iter().cloned().collect(), // Convert references to owned values
        }
    }

    fn assert_report(entries: Vec<TimeEntry>) -> ReportAssertion {
        let report = Report::new(
            entries,
            1,
            None,
            StartDate(NaiveDate::default()),
            EndDate(NaiveDate::default()),
        );
        ReportAssertion {
            filtered_entries: report.filtered_entries().into_iter().cloned().collect(), // Convert references to owned values
        }
    }
}
