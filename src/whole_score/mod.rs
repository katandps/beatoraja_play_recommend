pub mod scores;

use std::fmt;

use crate::scored_table::ScoredTable;
use crate::song_data::SongData;
use crate::table::Table;
use crate::whole_score::scores::score::song_id::SongId;
use crate::whole_score::scores::score::Score;
use scores::Scores;

pub struct WholeScore {
    scores: Scores,
}

impl WholeScore {
    pub fn new(scores: Scores) -> WholeScore {
        WholeScore { scores }
    }
    pub fn count(&self) -> usize {
        self.scores.count()
    }
    pub fn get_score(&self, song_id: &SongId) -> Option<&Score> {
        self.scores.get_score(&song_id)
    }
    pub fn merge_score(&self, table: &Table, song_data: &SongData) -> ScoredTable {
        self.scores.merge_score(table, song_data)
    }
}

impl fmt::Display for WholeScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Whole score: \n {}", self.scores)
    }
}
