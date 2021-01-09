use crate::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

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

#[derive(Eq, PartialEq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct PlayMode(pub LnMode);

impl PlayMode {
    pub fn new(mode: i32) -> Self {
        let lm = match FromPrimitive::from_i32(mode % 10) {
            Some(lm) => lm,
            None => LnMode::LongNote,
        };

        PlayMode(lm)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Serialize, Deserialize, FromPrimitive)]
pub enum LnMode {
    LongNote = 0,
    ChargeNote = 1,
    HellChargeNote = 2,
}
