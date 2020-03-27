pub mod song_id;
pub mod clear_type;

use song_id::SongId;
use clear_type::ClearType;

pub struct Score {
    id: SongId,
    clear: ClearType,
}

impl Score {
    pub fn new(id: SongId, clear: ClearType) -> Score { Score { id, clear } }
}
