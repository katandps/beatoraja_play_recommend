use crate::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Default, Serialize, Deserialize)]
pub struct ScoreId(HashSha256, PlayMode);

impl ScoreId {
    pub fn new(sha256: HashSha256, mode: PlayMode) -> ScoreId {
        ScoreId(sha256, mode)
    }

    pub fn sha256(&self) -> HashSha256 {
        self.0.clone()
    }

    pub fn mode(&self) -> PlayMode {
        self.1.clone()
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Serialize, Deserialize, Default)]
pub struct PlayMode(LnMode);

impl PlayMode {
    pub fn to_int(&self) -> i32 {
        self.0 as i32
    }
}

impl From<i32> for PlayMode {
    fn from(mode: i32) -> Self {
        let lm = match FromPrimitive::from_i32(mode % 10) {
            Some(lm) => lm,
            None => LnMode::Long,
        };

        PlayMode(lm)
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Serialize, Deserialize, FromPrimitive)]
pub enum LnMode {
    Long = 0,
    Charge = 1,
    HellCharge = 2,
}

impl Default for LnMode {
    fn default() -> LnMode {
        LnMode::Long
    }
}
