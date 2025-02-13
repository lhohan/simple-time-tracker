use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct StartDate(pub NaiveDate);
#[derive(Debug, Clone)]
pub struct EndDate(pub NaiveDate);
#[derive(Debug, Clone)]
pub struct EntryDate(pub NaiveDate);
