use parse_display::Display;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Deserialize, Serialize, Default, Eq, PartialEq, Ord, PartialOrd, Display,
)]
pub struct PlayCount(pub i32);

impl PlayCount {
    pub fn new(count: i32) -> PlayCount {
        PlayCount(count)
    }
}

impl std::ops::Sub<PlayCount> for PlayCount {
    type Output = PlayCount;
    fn sub(self, rhs: PlayCount) -> PlayCount {
        PlayCount::new(self.0 - rhs.0)
    }
}
