use super::*;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

pub(super) fn rank(
    songs: &Songs,
    table: &Table,
    score_log: &ScoreLog,
    updated_at: &UpdatedAt,
    levels: &Levels,
) -> CommandResult {
    let mut str = String::new();
    for level in &levels.levels {
        let specified = table.level_specified(level);
        let mut summary = Summary::new(ClearRank::vec());
        for song in specified.get_song(songs) {
            summary.push(
                &SongWithSnap::make(
                    &song,
                    score_log.get_snap(&song.song_id(), &updated_at).borrow(),
                )
                .clear_rank(),
            )
        }
        str.push_str(format!("{}", summary).as_str());
    }
    CommandResult::recommend(RecommendResult::new(str, Vec::new()))
}

#[derive(Deserialize, Serialize)]
pub struct RankGraphResult {
    table: String,
}

impl RankGraphResult {
    pub fn to_string(&self) -> String {
        self.table.clone()
    }
}
