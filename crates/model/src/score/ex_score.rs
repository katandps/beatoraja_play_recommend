use crate::*;
use parse_display::Display;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Default, Display)]
pub struct ExScore(i32);

impl ExScore {
    pub fn from_score(score: i32) -> ExScore {
        ExScore(score)
    }

    pub fn ex_score(&self) -> i32 {
        self.0
    }
}

impl PartialOrd for ExScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ExScore {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ex_score().cmp(&other.ex_score())
    }
}
