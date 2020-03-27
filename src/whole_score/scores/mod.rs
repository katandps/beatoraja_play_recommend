pub mod score;

use std::fmt;

use score::Score;

pub struct Scores {
    scores: Vec<Score>
}

impl Scores {
    pub fn new(scores: Vec<Score>) -> Scores { Scores { scores } }
    pub fn count(&self) -> usize { self.scores.len() }
}

impl fmt::Display for Scores {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for score in &self.scores {
            result.push_str(&format!("{}", score));
            result.push_str("\n");
        }
        write!(f, "{}", result)
    }
}