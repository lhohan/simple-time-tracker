use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub enum Clock {
    System,
    Test(NaiveDate),
}

impl Clock {
    pub fn system() -> Self {
        Clock::System
    }
    pub fn with_today(today: NaiveDate) -> Self {
        Clock::Test(today)
    }
}

impl Clock {
    pub fn today(&self) -> NaiveDate {
        match self {
            Clock::System => chrono::Utc::now().date_naive(),
            Clock::Test(c) => *c,
        }
    }
}
