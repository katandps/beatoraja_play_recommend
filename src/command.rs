use crate::lamp::LampSum;
use crate::rank::RankSum;
use crate::score::scores::Scores;
use crate::score::updated_at::UpdatedAt;
use crate::score_log::ScoreLog;
use crate::song::{SongWithSnap, Songs};
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
        let mut lamp_sum = LampSum::new();
        for s in table.get_song(songs) {
            lamp_sum.push(score_log.get_snap(&s.song_id(), &updated_at).borrow())
        }
        format!("{}", lamp_sum)
    }

    fn rank(
        _scores: &Scores,
        songs: &Songs,
        table: &Table,
        score_log: &ScoreLog,
        updated_at: &UpdatedAt,
    ) -> String {
        let mut rank_sum = RankSum::new();
        for song in table.get_song(songs) {
            rank_sum.push(
                SongWithSnap::make(
                    &song,
                    score_log.get_snap(&song.song_id(), &updated_at).borrow(),
                )
                    .borrow(),
            )
        }
        format!("{}", rank_sum)
    }
}
