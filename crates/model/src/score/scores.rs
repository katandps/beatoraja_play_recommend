use crate::*;
use std::collections::HashMap;
use std::fmt;

/// 最新スコアのみが入っている
#[derive(Clone)]
pub struct Scores {
    scores: HashMap<SongId, Score>,
}

impl Scores {
    pub fn new(scores: HashMap<SongId, Score>) -> Scores {
        Scores { scores }
    }
    pub fn count(&self) -> usize {
        self.scores.len()
    }
    pub fn merge(&self, song_id: SongId, chart: &Chart) -> Option<ScoredChart> {
        match self.scores.get(&song_id) {
            Some(score) => Some(ScoredChart::new(song_id, chart.clone(), score.clone())),
            _ => None,
        }
    }
    pub fn get(&self, song_id: &SongId) -> Option<Score> {
        self.scores.get(song_id).map(|s| s.clone())
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

pub trait ScoreRepository {
    fn score(&self) -> Scores;
}
