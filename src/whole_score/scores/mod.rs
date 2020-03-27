pub mod score;

use score::Score;

pub struct Scores {
    scores: Vec<Score>
}

impl Scores {
    pub fn new(scores: Vec<Score>) -> Scores { Scores { scores } }
    pub fn count(&self) -> usize { self.scores.len() }
}
