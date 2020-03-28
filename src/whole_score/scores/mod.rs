pub mod score;

use std::fmt;

use score::Score;
use crate::whole_score::scores::score::song_id::SongId;
use std::collections::HashMap;

pub struct Scores {
    scores: HashMap<SongId, Score>
}

impl Scores {
    pub fn new(scores: HashMap<SongId, Score>) -> Scores { Scores { scores } }
    pub fn count(&self) -> usize { self.scores.len() }
    pub fn get_score(&self, song_id: &SongId) -> Option<&Score> {
        self.scores.get(song_id)
    }
}

impl fmt::Display for Scores {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for score in &self.scores {
            result.push_str(&format!("{}: {}\n", score.0, score.1));
        }
        write!(f, "{}", result)
    }
}