pub mod scores;

use scores::Scores;

pub struct WholeScore {
    scores: Scores
}

impl WholeScore {
    pub fn new(scores: Scores) -> WholeScore { WholeScore { scores } }
    pub fn count(&self) -> usize { self.scores.count() }
}