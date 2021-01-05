use crate::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fmt;

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
    pub title: String,
    total_notes: i32,
    level: Level,
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

    pub fn make_rank_graph(self) -> Graph<ClearRank> {
        Graph::make(
            self.table,
            self.levels
                .iter()
                .map(|dbl| {
                    CountByLevel::make(
                        dbl.songs
                            .iter()
                            .map(|sd| sd.clear_rank)
                            .fold(Summary::new(), Summary::tally),
                    )
                })
                .collect(),
        )
    }

    pub fn make_lamp_graph(self) -> Graph<ClearType> {
        Graph::make(
            self.table,
            self.levels
                .iter()
                .map(|dbl| {
                    CountByLevel::make(
                        dbl.songs
                            .iter()
                            .map(|sd| sd.clear_type.current)
                            .fold(Summary::new(), Summary::tally),
                    )
                })
                .collect(),
        )
    }
}

impl DetailByLevel {
    pub fn new(level: String, songs: Vec<SongDetail>) -> DetailByLevel {
        DetailByLevel { level, songs }
    }
}

impl SongDetail {
    pub fn new(song: &Song, score: Score, date: &UpdatedAt, level: Level) -> SongDetail {
        let score_snap = score.score_snap(date);
        let clear_snap = score.clear_type_snap(date);
        let min_bp_snap = score.min_bp_snap(date);
        let score = score.at(date).clone();
        SongDetail {
            title: song.title(),
            total_notes: song.notes(),
            level,
            clear_type: clear_snap,
            clear_rank: ClearRank::from_notes_score(song.notes(), score.score),
            max_combo: score.max_combo.clone(),
            min_bp: min_bp_snap,
            score: score_snap,
            updated_at: score.updated_at,
            play_count: score.play_count,
            hash: song.hash.clone(),
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
            self.title,
            self.updated_at,
            self.clear_type.current,
            self.score.current,
            self.min_bp.current,
            self.max_combo
        )
    }
}
