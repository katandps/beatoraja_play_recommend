use crate::*;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Serialize)]
pub struct SongId {
    sha256: HashSha256,
    mode: PlayMode,
}

impl SongId {
    pub fn new(sha256: HashSha256, mode: PlayMode) -> SongId {
        SongId { sha256, mode }
    }

    pub fn sha256(&self) -> HashSha256 {
        self.sha256.clone()
    }
}

impl fmt::Display for SongId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {}]", self.sha256, self.mode)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Serialize)]
pub struct PlayMode(i32);

impl PlayMode {
    pub fn new(mode: i32) -> Self {
        PlayMode(mode)
    }
}

impl fmt::Display for PlayMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
