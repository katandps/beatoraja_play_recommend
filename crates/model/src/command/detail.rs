use crate::*;
use serde::{Deserialize, Serialize};
use std::fmt;

pub(super) fn detail<T: TableTrait>(
    songs: &Songs,
    table: &T,
    score_log: &ScoreLog,
    updated_at: &UpdatedAt,
) -> CommandResult {
    CommandResult::Detail(DetailResult {
        table: "test".into(),
        levels: Vec::new(),
    })
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
    level: String,
    ex_score: i32,
    max_score: i32,
    combo: i32,
    total_notes: i32,
    bp: i32,
    updated_at: UpdatedAt,
}

impl fmt::Display for DetailResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "hogehoge")
    }
}
