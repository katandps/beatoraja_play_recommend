use crate::rank::ClearRank;
use crate::score::prelude::*;
use crate::score_log::prelude::*;
use crate::song::prelude::*;
use crate::summary::Summary;
use crate::table::prelude::*;
use lamp_graph::*;
use rank_graph::*;
use recommend::*;
use serde::{Deserialize, Serialize};

pub mod lamp_graph;
pub mod rank_graph;
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
            Self::Recommend => recommend::recommend,
            Self::LampGraph => lamp_graph::lamp,
            Self::RankGraph => rank_graph::rank,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub enum CommandResult {
    Recommend(RecommendResult),
    LampGraph(LampGraphResult),
    RankGraph(RankGraphResult),
}

impl CommandResult {
    pub fn to_string(&self) -> String {
        match self {
            Self::Recommend(r) => r.to_string(),
            Self::LampGraph(r) => r.to_string(),
            Self::RankGraph(r) => r.to_string(),
        }
    }
}
