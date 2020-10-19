use crate::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fmt;

pub(super) fn detail<T: TableTrait>(
    songs: &Songs,
    table: &T,
    scores: &Scores,
    _score_log: &ScoreLog,
    updated_at: &UpdatedAt,
) -> CommandResult {
    CommandResult::Detail(table.make_detail(songs, scores, updated_at))
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
    total_notes: i32,
    clear_type: ClearType,
    clear_rank: ClearRank,
    max_combo: MaxCombo,
    min_bp: MinBP,
    score: ExScore,
    updated_at: UpdatedAt,
    play_count: PlayCount,
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
                            .map(|sd| sd.clear_type)
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
    pub fn new(song: &Song, score: Score) -> SongDetail {
        SongDetail {
            title: song.title(),
            total_notes: song.notes(),
            clear_type: score.clear,
            clear_rank: ClearRank::from_notes_score(song.notes(), score.judge.ex_score()),
            max_combo: score.max_combo,
            min_bp: score.min_bp,
            score: score.judge.ex_score(),
            updated_at: score.updated_at,
            play_count: score.play_count,
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
