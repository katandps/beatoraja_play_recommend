use parse_display::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Default, Display)]
pub struct ClearCount(pub i32);

impl ClearCount {
    pub fn new(count: i32) -> ClearCount {
        ClearCount(count)
    }
}

impl std::ops::Sub<ClearCount> for ClearCount {
    type Output = ClearCount;
    fn sub(self, rhs: ClearCount) -> ClearCount {
        ClearCount::new(self.0 - rhs.0)
    }
}
