pub mod song_id;
pub mod clear_type;
pub mod updated_at;

use std::fmt;

use song_id::SongId;
use clear_type::ClearType;
use updated_at::UpdatedAt;

pub struct Score {
    id: SongId,
    clear: ClearType,
    updated_at: UpdatedAt,
}

impl Score {
    pub fn new(id: SongId, clear: ClearType, updated_at: UpdatedAt) -> Score { Score { id, clear, updated_at } }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " {} {} {}", self.id, self.clear, self.updated_at)
    }
}
