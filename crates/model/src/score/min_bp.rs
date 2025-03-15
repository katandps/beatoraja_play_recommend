use crate::*;
use parse_display::Display;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Display)]
pub struct MinBP(pub i32);

impl Default for MinBP {
    fn default() -> Self {
        MinBP(-1)
    }
}

impl MinBP {
    pub fn from_bp(bp: i32) -> MinBP {
        MinBP(bp)
    }
}

impl PartialOrd for MinBP {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MinBP {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
