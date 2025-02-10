use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub enum Clock {
    System,
    Test(NaiveDate),
}

impl Clock {
    #[must_use]
    pub fn system() -> Self {
        Clock::System
    }
    #[must_use]
    pub fn with_today(today: NaiveDate) -> Self {
        Clock::Test(today)
    }
}

impl Clock {
    #[must_use]
    pub fn today(&self) -> NaiveDate {
        match self {
            Clock::System => chrono::Utc::now().date_naive(),
            Clock::Test(c) => *c,
        }
    }
}
