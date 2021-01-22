use crate::*;
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct MaxCombo(pub i32);

impl MaxCombo {
    pub fn from_combo(combo: i32) -> MaxCombo {
        MaxCombo(combo)
    }
}

impl fmt::Display for MaxCombo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
