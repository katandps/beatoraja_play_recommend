use crate::*;
use std::fmt;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct MinBP(pub i32);

impl MinBP {
    pub fn new() -> MinBP {
        MinBP(-1)
    }
    pub fn from_bp(bp: i32) -> MinBP {
        MinBP(bp)
    }
}

impl fmt::Display for MinBP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
