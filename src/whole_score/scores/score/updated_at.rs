use std::fmt;
use std::cmp::Ordering;

use chrono::{DateTime, Local, Datelike};

#[derive(Clone, Eq)]
pub struct UpdatedAt {
    updated_at: DateTime<Local>
}

impl UpdatedAt {
    pub fn new(updated_at: DateTime<Local>) -> UpdatedAt { UpdatedAt { updated_at } }
}

impl Ord for UpdatedAt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.updated_at.num_days_from_ce().cmp(&other.updated_at.num_days_from_ce())
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.updated_at)
    }
}