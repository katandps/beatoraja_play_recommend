use crate::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fmt;

pub(super) fn detail<T: TableTrait>(
    songs: &Songs,
    table: &T,
    _scores: &Scores,
    score_log: &ScoreLog,
    updated_at: &UpdatedAt,
) -> CommandResult {
    CommandResult::Detail(table.make_detail(songs, score_log, updated_at))
}

#[derive(Deserialize, Serialize)]
pub struct DetailResult {
    table: String,
    levels: Vec<DetailByLevel>,
}

#[derive(Deserialize, Serialize)]
pub struct DetailByLevel {
    level: String,
    songs: Vec<SongDetail>,
}

#[derive(Deserialize, Serialize)]
pub struct SongDetail {
    pub title: String,
    clear_type: ClearType,
    max_combo: MaxCombo,
    min_bp: MinBP,
    score: ExScore,
    updated_at: UpdatedAt,
}

impl DetailResult {
    pub fn new(table: String, levels: Vec<DetailByLevel>) -> DetailResult {
        DetailResult { table, levels }
    }
}

impl DetailByLevel {
    pub fn new(level: String, songs: Vec<SongDetail>) -> DetailByLevel {
        DetailByLevel { level, songs }
    }
}

impl SongDetail {
    pub fn new(title: String, snap: SnapShot) -> SongDetail {
        SongDetail {
            title,
            clear_type: snap.clear_type,
            max_combo: snap.max_combo,
            min_bp: snap.min_bp,
            score: snap.score,
            updated_at: snap.updated_at,
        }
    }
}

impl fmt::Display for DetailResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.table, self.levels.iter().join("\n"))
    }
}

impl fmt::Display for DetailByLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.level, self.songs.iter().join("\n"))
    }
}

impl fmt::Display for SongDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{} {} score:{} bp:{} combo:{}",
            self.title, self.updated_at, self.clear_type, self.score, self.min_bp, self.max_combo
        )
    }
}
