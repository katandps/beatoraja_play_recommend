pub mod scores;

use std::fmt;

use scores::Scores;

pub struct WholeScore {
    scores: Scores
}

impl WholeScore {
    pub fn new(scores: Scores) -> WholeScore { WholeScore { scores } }
    pub fn count(&self) -> usize { self.scores.count() }
}

impl fmt::Display for WholeScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Whole score: \n {}", self.scores)
    }
}