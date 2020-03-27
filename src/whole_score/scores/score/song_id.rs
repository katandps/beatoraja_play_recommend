pub struct SongId {
    sha256: String,
    mode: i32,
}

impl SongId {
    pub fn new(sha256: String, mode: i32) -> SongId {
        SongId { sha256, mode }
    }
}
