use crate::*;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExScore(i32);

impl ExScore {
    pub fn new() -> ExScore {
        ExScore(0)
    }

    pub fn from_score(score: i32) -> ExScore {
        ExScore(score)
    }

    pub fn ex_score(&self) -> i32 {
        self.0
    }
}

impl fmt::Display for ExScore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ex_score())
    }
}
