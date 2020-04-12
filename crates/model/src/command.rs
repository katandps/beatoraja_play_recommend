use crate::rank::ClearRank;
use crate::score::clear_type::ClearType;
use crate::score::updated_at::UpdatedAt;
use crate::score_log::ScoreLog;
use crate::song::level::Level;
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

pub type CommandFunc = fn(&Songs, &Table, &ScoreLog, &UpdatedAt, &Level) -> String;

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
        songs: &Songs,
        table: &Table,
        score_log: &ScoreLog,
        updated_at: &UpdatedAt,
        level: &Level,
    ) -> String {
        let t = format!("{} {}\n", table.symbol(), level);
        let f: String = score_log
            .filter_by_table(table, songs, updated_at)
            .for_recommend(updated_at)
            .iter()
            .flat_map(|snap| snap.str(songs))
            .map(|s| format!("{}\n", s))
            .collect();
        t + &f
    }

    fn lamp(
        songs: &Songs,
        table: &Table,
        score_log: &ScoreLog,
        updated_at: &UpdatedAt,
        _level: &Level,
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
        songs: &Songs,
        table: &Table,
        score_log: &ScoreLog,
        updated_at: &UpdatedAt,
        _level: &Level,
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
