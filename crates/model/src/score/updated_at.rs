use chrono::{DateTime, Local, TimeZone};
use serde::Serialize;
use std::fmt;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize)]
pub struct UpdatedAt(DateTime<Local>);

impl UpdatedAt {
    pub fn new() -> UpdatedAt {
        UpdatedAt(DateTime::from(Local.timestamp(0, 0)))
    }
    pub fn from_timestamp(timestamp: i32) -> UpdatedAt {
        UpdatedAt(DateTime::from(Local.timestamp(timestamp as i64, 0)))
    }
}

impl fmt::Display for UpdatedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
