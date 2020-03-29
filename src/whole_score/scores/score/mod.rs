pub mod clear_type;
pub mod song_id;
pub mod updated_at;

use std::cmp::Ordering;
use std::fmt;

use clear_type::ClearType;
use updated_at::UpdatedAt;

#[derive(Clone)]
pub struct Score {
    clear: ClearType,
    updated_at: UpdatedAt,
}

impl Score {
    pub fn from_data(clear: ClearType, updated_at: UpdatedAt) -> Score {
        Score { clear, updated_at }
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        self.updated_at.cmp(&other.updated_at)
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        self.updated_at == other.updated_at
    }
}

impl Eq for Score {}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.updated_at, self.clear)
    }
}
