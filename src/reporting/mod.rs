use crate::domain::TimeEntry;

pub struct Report {
    entries: Vec<TimeEntry>,
    total_minutes: u32,
}

impl Report {
    pub fn new(entries: Vec<TimeEntry>) -> Self {
        let total_minutes = entries.iter().map(|e| e.minutes).sum();
        Self {
            entries,
            total_minutes,
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
        println!(
            "{}..{}",
            format!("{:.<20}", "Total"),
            format_duration(self.total_minutes),
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
