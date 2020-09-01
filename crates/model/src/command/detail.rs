use crate::*;
use serde::{Deserialize, Serialize};
use std::fmt;

pub(super) fn detail<T: TableTrait>(
    songs: &Songs,
    table: &T,
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
    title: String,
    snap: SnapShot,
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
        SongDetail { title, snap }
    }
}

impl fmt::Display for DetailResult {
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

impl fmt::Display for DetailByLevel {
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

impl fmt::Display for SongDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.snap.format(self.title.clone()))
    }
}
