use crate::domain::dates::EntryDate;
use crate::domain::tags::Tag;
use crate::domain::{DateRange, TimeEntry};

#[derive(Debug, Clone)]
pub enum Filter {
    MainContext(String),
    ExcludeTags(Vec<String>),
    DateRange(DateRange),
    And(Box<Filter>, Box<Filter>),
}

impl Filter {
    pub fn matches(&self, entry: &TimeEntry, date: &EntryDate) -> bool {
        match self {
            Filter::MainContext(project) => entry.main_context().eq_ignore_ascii_case(project),
            Filter::ExcludeTags(tags) => !tags
                .iter()
                .any(|tag| entry.get_tags().contains(&Tag::from_raw(tag))),
            Filter::DateRange(date_range) => date_range.matches(date),
            Filter::And(f1, f2) => f1.matches(entry, date) && f2.matches(entry, date),
        }
    }
    pub fn combine(self, other: Filter) -> Filter {
        Filter::And(Box::new(self), Box::new(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_week_of_date_range() {
        // Tuesday January 15th 2024
        let current = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();

        let range = DateRange::week_of(current);

        // Week should start on Monday January 15th
        assert_eq!(
            range.0 .0, // StartDate
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()
        );
        // Week should end on Sunday January 21st
        assert_eq!(
            range.1 .0, // EndDate
            NaiveDate::from_ymd_opt(2024, 1, 21).unwrap()
        );
    }
}
