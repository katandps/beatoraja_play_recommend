use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayCount(i32);

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
