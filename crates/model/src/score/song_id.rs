use crate::*;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct SongId(HashSha256, PlayMode);

impl SongId {
    pub fn new(sha256: HashSha256, mode: PlayMode) -> SongId {
        SongId(sha256, mode)
    }

    pub fn sha256(&self) -> HashSha256 {
        self.0.clone()
    }

    pub fn mode(&self) -> PlayMode {
        self.1.clone()
    }
}

impl fmt::Display for SongId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {}]", self.0, self.1)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct PlayMode(pub i32);

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
