use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct RegisteredDate(NaiveDateTime);

impl RegisteredDate {
    pub fn new(date: NaiveDateTime) -> Self {
        Self(date)
    }

    pub fn to_naive_date_time(&self) -> NaiveDateTime {
        self.0
    }
}
