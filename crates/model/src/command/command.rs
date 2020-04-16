use super::lamp_graph;
use super::rank_graph;
use super::recommend;
use crate::*;

#[derive(Eq, PartialEq)]
pub enum Command {
    Recommend,
    LampGraph,
    RankGraph,
}

pub type CommandFunc<T> = fn(&Songs, &T, &ScoreLog, &UpdatedAt, &Levels) -> CommandResult;

impl Command {
    pub fn all() -> Vec<Command> {
        vec![Self::Recommend, Self::LampGraph, Self::RankGraph]
    }

    pub fn func<T: TableTrait>(&self) -> CommandFunc<T> {
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
