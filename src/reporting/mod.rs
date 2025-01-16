use crate::domain::TimeEntry;
use itertools::Itertools;

pub struct Report {
    entries: Vec<TimeEntry>,
    total_minutes: u32,
    days: u32,
}

impl Report {
    pub fn new(entries: Vec<TimeEntry>, days: u32) -> Self {
        let entries: Vec<_> = entries
            .into_iter()
            .sorted_by(|a, b| b.minutes.cmp(&a.minutes).then(a.project.cmp(&b.project)))
            .collect();
        let total_minutes = entries.iter().map(|e| e.minutes).sum();
        Self {
            entries,
            total_minutes,
            days: days,
        }
    }

    pub fn display(&self) {
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
        let entries = vec![
            TimeEntry::new("short".to_string(), 30),
            TimeEntry::new("longest".to_string(), 120),
            TimeEntry::new("medium".to_string(), 60),
            TimeEntry::new("also-long".to_string(), 120),
        ];

        let report = Report::new(entries, 1);

        // First two entries should be the 120-minute ones, alphabetically ordered
        assert_eq!(report.entries[0].project, "also-long");
        assert_eq!(report.entries[1].project, "longest");

        // Then the shorter ones
        assert_eq!(report.entries[2].project, "medium");
        assert_eq!(report.entries[3].project, "short");
    }
}
