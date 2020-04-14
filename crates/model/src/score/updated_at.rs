use chrono::{DateTime, Datelike, Local, TimeZone};
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Eq, Debug)]
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

impl Ord for UpdatedAt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.updated_at
            .num_days_from_ce()
            .cmp(&other.updated_at.num_days_from_ce())
    }
}

impl PartialOrd for UpdatedAt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for UpdatedAt {
    fn eq(&self, other: &Self) -> bool {
        self.updated_at == other.updated_at
    }
}

impl fmt::Display for UpdatedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.updated_at)
    }
}
