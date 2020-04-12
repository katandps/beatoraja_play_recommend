use crate::command::Command::*;
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

pub type CommandFunc = fn(&Songs, &Table, &ScoreLog, &UpdatedAt, &Level) -> CommandResult;

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
    ) -> CommandResult {
        let mut ret = RecommendResult::new((&table.name()).parse().unwrap());
        let t = format!("{} {}\n", table.symbol(), level);

        let mut f = score_log
            .filter_by_table(table, songs, updated_at)
            .for_recommend(updated_at)
            .iter()
            .flat_map(|snap| snap.str(songs))
            .collect();
        ret.push(t);
        ret.insert(&mut f);
        CommandResult::recommend(ret)
    }

    fn lamp(
        songs: &Songs,
        table: &Table,
        score_log: &ScoreLog,
        updated_at: &UpdatedAt,
        _level: &Level,
    ) -> CommandResult {
        let mut summary = Summary::new(ClearType::vec());
        for song in table.get_song(songs) {
            summary.push(
                score_log
                    .get_snap(&song.song_id(), &updated_at)
                    .clear_type(),
            )
        }
        CommandResult::recommend(RecommendResult::new(format!("{}", summary)))
    }

    fn rank(
        songs: &Songs,
        table: &Table,
        score_log: &ScoreLog,
        updated_at: &UpdatedAt,
        _level: &Level,
    ) -> CommandResult {
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
        CommandResult::recommend(RecommendResult::new(format!("{}", summary)))
    }
}

pub enum CommandResult {
    Recommend(RecommendResult),
}

impl CommandResult {
    pub fn recommend(rec: RecommendResult) -> CommandResult {
        Self::Recommend(rec)
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Recommend(r) => r.to_string(),
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RecommendResult {
    table: String,
    levels: Vec<RecommendSong>,
}

#[derive(Deserialize, Serialize)]
pub struct RecommendSong {
    song: String,
}

impl RecommendResult {
    pub fn new(table: String) -> RecommendResult {
        RecommendResult {
            table,
            levels: Vec::new(),
        }
    }

    pub fn insert(&mut self, v: &mut Vec<RecommendSong>) {
        self.levels.append(v)
    }

    pub fn push(&mut self, song: String) {
        self.levels.push(RecommendSong { song })
    }

    fn to_string(&self) -> String {
        let mut ret = self.table.clone();
        for level in &self.levels {
            ret = ret + level.to_string().as_ref();
        }
        ret
    }
}

impl RecommendSong {
    pub fn new(song: String) -> RecommendSong {
        RecommendSong { song }
    }

    fn to_string(&self) -> String {
        format!("{}\n", self.song)
    }
}
