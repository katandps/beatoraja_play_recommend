use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Deserialize, Serialize, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct PlayCount(pub i32);

impl PlayCount {
    pub fn new(count: i32) -> PlayCount {
        PlayCount(count)
    }
}

impl fmt::Display for PlayCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Sub<PlayCount> for PlayCount {
    type Output = PlayCount;
    fn sub(self, rhs: PlayCount) -> PlayCount {
        PlayCount::new(self.0 - rhs.0)
    }
}
