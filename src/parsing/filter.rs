use crate::domain::{EndDate, EntryDate, StartDate, TimeEntry};

#[derive(Debug, Clone)]
pub enum Filter {
    Project(String),
    // MinimumMinutes(u32),
    DateRange(StartDate, EndDate),
    And(Box<Filter>, Box<Filter>),
    Or(Box<Filter>, Box<Filter>),
}

impl Filter {
    pub fn matches(&self, entry: &TimeEntry, date: &EntryDate) -> bool {
        match self {
            Filter::Project(project) => entry.project.eq_ignore_ascii_case(project),
            // Filter::MinimumMinutes(min) => entry.minutes >= *min,
            Filter::DateRange(start, end) => date.0 >= start.0 && date.0 <= end.0,
            Filter::And(f1, f2) => f1.matches(entry, date) && f2.matches(entry, date),
            Filter::Or(f1, f2) => f1.matches(entry, date) || f2.matches(entry, date),
        }
    }
}
