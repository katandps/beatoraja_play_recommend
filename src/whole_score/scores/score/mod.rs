pub mod song_id;
pub mod clear_type;
pub mod updated_at;

use std::fmt;

use clear_type::ClearType;
use updated_at::UpdatedAt;
use chrono::{DateTime, Local};

pub struct Score {
    clear: ClearType,
    updated_at: UpdatedAt,
}

impl Score {
    pub fn from_data(clear: ClearType, updated_at: UpdatedAt) -> Score { Score { clear, updated_at } }
    pub fn new() -> Score { Score { clear: ClearType::NoPlay, updated_at: UpdatedAt::new(Local::now()) } }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.clear, self.updated_at)
    }
}
