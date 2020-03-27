use std::fmt;

pub struct SongId {
    sha256: String,
    mode: i32,
}

impl SongId {
    pub fn new(sha256: String, mode: i32) -> SongId { SongId { sha256, mode } }
}

impl fmt::Display for SongId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} {}]", self.sha256, self.mode)
    }
}