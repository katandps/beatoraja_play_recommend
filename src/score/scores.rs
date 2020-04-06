use std::fmt;

use crate::score::song_id::{PlayMode, SongId};
use crate::score::Score;
use crate::scored_table::{ScoredChart, ScoredTable};
use crate::song::Songs;
use crate::table::Table;
use std::collections::HashMap;

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
    pub fn get_score(&self, song_id: &SongId) -> Option<&Score> {
        self.scores.get(song_id)
    }
    pub fn merge_score(&self, table: &Table, song_data: &Songs) -> ScoredTable {
        let mut charts = Vec::new();
        for chart in table.get_charts() {
            let sha256 = song_data.get_sha256(&chart.md5);
            if !sha256.is_some() {
                continue;
            }
            let song_id = SongId::new(sha256.unwrap(), PlayMode::new(0));
            let score = self.get_score(&song_id);
            if score.is_none() {
                continue;
            }
            let scored_chart = ScoredChart::new(song_id, chart.clone(), score.unwrap().clone());
            charts.push(scored_chart);
        }
        ScoredTable::new(charts)
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
