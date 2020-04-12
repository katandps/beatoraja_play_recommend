use crate::rank::ClearRank;
use crate::score::prelude::*;
use crate::score_log::prelude::*;
use crate::song::prelude::*;
use crate::summary::Summary;
use crate::table::prelude::*;
use recommend::*;
use std::borrow::Borrow;
pub mod recommend;

#[derive(Eq, PartialEq)]
pub enum Command {
    Recommend,
    LampGraph,
    RankGraph,
}

pub type CommandFunc = fn(&Songs, &Table, &ScoreLog, &UpdatedAt, &Levels) -> CommandResult;

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
        levels: &Levels,
    ) -> CommandResult {
        let ret_levels = levels
            .levels
            .iter()
            .map(|level| {
                let specified_table = table.level_specified(level);
                RecommendByLevel::new(
                    format!("{}{}", table.symbol(), level),
                    score_log
                        .filter_by_table(&specified_table, songs, updated_at)
                        .for_recommend(updated_at)
                        .iter()
                        .flat_map(|snap| snap.recommend_song(songs))
                        .collect(),
                )
            })
            .collect();

        let ret = RecommendResult::new((&table.name()).parse().unwrap(), ret_levels);
        CommandResult::recommend(ret)
    }

    fn lamp(
        songs: &Songs,
        table: &Table,
        score_log: &ScoreLog,
        updated_at: &UpdatedAt,
        levels: &Levels,
    ) -> CommandResult {
        let mut str = String::new();
        for level in &levels.levels {
            let specified = table.level_specified(level);
            let mut summary = Summary::new(ClearType::vec());
            for song in specified.get_song(songs) {
                summary.push(
                    score_log
                        .get_snap(&song.song_id(), &updated_at)
                        .clear_type(),
                )
            }
            str.push_str(format!("{}", summary).as_str());
        }

        CommandResult::recommend(RecommendResult::new(str, Vec::new()))
    }

    fn rank(
        songs: &Songs,
        table: &Table,
        score_log: &ScoreLog,
        updated_at: &UpdatedAt,
        levels: &Levels,
    ) -> CommandResult {
        let mut str = String::new();
        for level in &levels.levels {
            let specified = table.level_specified(level);
            let mut summary = Summary::new(ClearRank::vec());
            for song in specified.get_song(songs) {
                summary.push(
                    &SongWithSnap::make(
                        &song,
                        score_log.get_snap(&song.song_id(), &updated_at).borrow(),
                    )
                    .clear_rank(),
                )
            }
            str.push_str(format!("{}", summary).as_str());
        }
        CommandResult::recommend(RecommendResult::new(str, Vec::new()))
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
