use crate::*;
use serde::{Deserialize, Serialize};

pub(super) fn recommend<T: TableTrait>(
    songs: &Songs,
    table: &T,
    score_log: &ScoreLog,
    updated_at: &UpdatedAt,
) -> CommandResult {
    CommandResult::Recommend(table.make_recommend(songs, score_log, updated_at))
}

#[derive(Deserialize, Serialize)]
pub struct RecommendResult {
    table: String,
    levels: Vec<RecommendByLevel>,
}

#[derive(Deserialize, Serialize)]
pub struct RecommendByLevel {
    level: String,
    songs: Vec<RecommendSong>,
}

#[derive(Deserialize, Serialize)]
pub struct RecommendSong {
    song: String,
}

impl RecommendResult {
    pub fn new(table: String, levels: Vec<RecommendByLevel>) -> RecommendResult {
        RecommendResult { table, levels }
    }

    pub fn to_string(&self) -> String {
        let mut ret = self.table.clone() + "\n";
        for level in &self.levels {
            ret = ret + level.to_string().as_ref();
        }
        ret
    }
}

impl RecommendByLevel {
    pub fn new(level: String, songs: Vec<RecommendSong>) -> RecommendByLevel {
        RecommendByLevel { level, songs }
    }
    fn to_string(&self) -> String {
        format!("{}\n", self.level)
            + self
                .songs
                .iter()
                .map(|s| s.to_string())
                .collect::<String>()
                .as_str()
    }
}

impl RecommendSong {
    pub fn new(song: String) -> RecommendSong {
        RecommendSong { song }
    }

    fn to_string(&self) -> String {
        format!("{}\n", self.song)
    }
}
