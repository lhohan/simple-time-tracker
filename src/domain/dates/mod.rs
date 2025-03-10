pub mod range;

use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct StartDate(pub NaiveDate);
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct EndDate(pub NaiveDate);
#[derive(Debug, Clone, PartialEq)]
pub struct EntryDate(pub NaiveDate);
