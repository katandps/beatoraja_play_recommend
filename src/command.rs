use crate::rank::ClearRank;
use crate::score::clear_type::ClearType;
use crate::score::scores::Scores;
use crate::score::updated_at::UpdatedAt;
use crate::score_log::ScoreLog;
use crate::song::{SongWithSnap, Songs};
use crate::summary::Summary;
use crate::table::Table;
pub use diesel::prelude::*;
use std::borrow::Borrow;

pub enum Command {
    Recommend,
    LampGraph,
    RankGraph,
}

pub type CommandFunc = fn(&Scores, &Songs, &Table, &ScoreLog, &UpdatedAt) -> String;

impl Command {
    pub fn all() -> Vec<Command> {
        vec![Self::Recommend, Self::LampGraph, Self::RankGraph]
    }

    pub fn func(&self) -> CommandFunc {
        match self {
            Self::Recommend => Self::rec,
            Self::LampGraph => Self::lamp,
            Self::RankGraph => Self::rank,
        }
    }

    fn rec(
        scores: &Scores,
        songs: &Songs,
        table: &Table,
        _score_log: &ScoreLog,
        _updated_at: &UpdatedAt,
    ) -> String {
        format!("{}\n", table.merge_score(scores, songs).recent_updated())
    }

    fn lamp(
        _scores: &Scores,
        songs: &Songs,
        table: &Table,
        score_log: &ScoreLog,
        updated_at: &UpdatedAt,
    ) -> String {
        let mut summary = Summary::new(ClearType::vec());
        for song in table.get_song(songs) {
            summary.push(
                score_log
                    .get_snap(&song.song_id(), &updated_at)
                    .clear_type(),
            )
        }
        format!("{}", summary)
    }

    fn rank(
        _scores: &Scores,
        songs: &Songs,
        table: &Table,
        score_log: &ScoreLog,
        updated_at: &UpdatedAt,
    ) -> String {
        let mut summary = Summary::new(ClearRank::vec());
        for song in table.get_song(songs) {
            summary.push(
                &SongWithSnap::make(
                    &song,
                    score_log.get_snap(&song.song_id(), &updated_at).borrow(),
                )
                .clear_rank(),
            )
        }
        format!("{}", summary)
    }
}
