use crate::*;
use parse_display::Display;

#[derive(Clone, Debug, Serialize, Deserialize, Default, Display)]
pub struct MaxCombo(pub i32);

impl MaxCombo {
    pub fn from_combo(combo: i32) -> MaxCombo {
        MaxCombo(combo)
    }
}
