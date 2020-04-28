use crate::*;
use serde::{Deserialize, Serialize};
use std::fmt;

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
}

impl fmt::Display for RecommendResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}",
            self.table,
            self.levels
                .iter()
                .map(ToString::to_string)
                .collect::<String>()
        )
    }
}

impl RecommendByLevel {
    pub fn new(level: String, songs: Vec<RecommendSong>) -> RecommendByLevel {
        RecommendByLevel { level, songs }
    }
}

impl fmt::Display for RecommendByLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}",
            self.level,
            self.songs
                .iter()
                .map(ToString::to_string)
                .collect::<String>()
        )
    }
}

impl RecommendSong {
    pub fn new(song: String) -> RecommendSong {
        RecommendSong { song }
    }
}

impl fmt::Display for RecommendSong {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.song)
    }
}
