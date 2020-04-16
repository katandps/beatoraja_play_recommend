use chrono::{DateTime, Local, TimeZone};
use std::fmt;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct UpdatedAt {
    updated_at: DateTime<Local>,
}

impl UpdatedAt {
    pub fn new() -> UpdatedAt {
        UpdatedAt {
            updated_at: DateTime::from(Local.timestamp(0, 0)),
        }
    }
    pub fn from_timestamp(timestamp: i32) -> UpdatedAt {
        UpdatedAt {
            updated_at: DateTime::from(Local.timestamp(timestamp as i64, 0)),
        }
    }
}

impl fmt::Display for UpdatedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.updated_at)
    }
}
