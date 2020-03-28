use std::fmt;
use chrono::{DateTime, Local};

#[derive(Clone)]
pub struct UpdatedAt {
    updated_at: DateTime<Local>
}

impl UpdatedAt {
    pub fn new(updated_at: DateTime<Local>) -> UpdatedAt { UpdatedAt { updated_at } }
}

impl fmt::Display for UpdatedAt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.updated_at)
    }
}