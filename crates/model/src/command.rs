pub use prelude::*;

use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DetailResult {
    table: String,
    levels: Vec<DetailByLevel>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DetailByLevel {
    level: String,
    songs: Vec<SongDetail>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SongDetail {
    title: String,
    total_notes: i32,
    clear_rank: ClearRank,
    max_combo: MaxCombo,
    score: ScoreSnap,
    min_bp: MinBPSnap,
    clear_type: ClearTypeSnap,
    updated_at: UpdatedAt,
    play_count: PlayCount,
    hash: HashSha256,
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
    pub fn new(song: &Song, score: &Score, date: &UpdatedAt) -> SongDetail {
        let score_snap = score.score_snap(date);
        let clear_snap = score.clear_type_snap(date);
        let min_bp_snap = score.min_bp_snap(date);
        let score = score.clone().at(date).clone();
        SongDetail {
            title: song.title(),
            total_notes: song.notes(),
            clear_type: clear_snap,
            clear_rank: ClearRank::from_notes_score(song.notes(), score.score),
            max_combo: score.max_combo.clone(),
            min_bp: min_bp_snap,
            score: score_snap,
            updated_at: score.updated_at,
            play_count: score.play_count,
            hash: song.get_hash().clone(),
        }
    }

    pub fn cmp_title(&self, other: &SongDetail) -> std::cmp::Ordering {
        self.title.to_lowercase().cmp(&other.title.to_lowercase())
    }
}
