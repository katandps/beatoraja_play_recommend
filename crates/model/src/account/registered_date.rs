use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct RegisteredDate(NaiveDateTime);

impl RegisteredDate {
    pub fn new(date: NaiveDateTime) -> Self {
        Self(date)
    }

    pub fn to_naive_date_time(&self) -> NaiveDateTime {
        self.0
    }
}
