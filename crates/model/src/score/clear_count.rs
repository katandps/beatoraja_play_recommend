use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ClearCount(pub i32);

impl ClearCount {
    pub fn new(count: i32) -> ClearCount {
        ClearCount(count)
    }
}

impl fmt::Display for ClearCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Sub<ClearCount> for ClearCount {
    type Output = ClearCount;
    fn sub(self, rhs: ClearCount) -> ClearCount {
        ClearCount::new(self.0 - rhs.0)
    }
}
