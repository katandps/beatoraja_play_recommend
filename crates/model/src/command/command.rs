use super::detail;
use super::lamp;
use super::rank;
use crate::*;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Command {
    LampGraph,
    RankGraph,
    Detail,
}

pub type CommandFunc<T> = fn(&Songs, &T, &Scores, &UpdatedAt) -> CommandResult;

impl Command {
    pub fn all() -> Vec<Command> {
        vec![Self::LampGraph, Self::RankGraph]
    }

    pub fn func<T: TableTrait>(&self) -> CommandFunc<T> {
        match self {
            Self::LampGraph => lamp::lamp,
            Self::RankGraph => rank::rank,
            Self::Detail => detail::detail,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub enum CommandResult {
    LampGraph(Graph<ClearType>),
    RankGraph(Graph<ClearRank>),
    Detail(DetailResult),
}

impl CommandResult {
    pub fn to_text(&self) -> String {
        match self {
            Self::LampGraph(r) => r.to_string(),
            Self::RankGraph(r) => r.to_string(),
            Self::Detail(r) => r.to_string(),
        }
    }

    pub fn to_json(&self) -> String {
        match self {
            Self::LampGraph(r) => serde_json::to_string(r).unwrap(),
            Self::RankGraph(r) => serde_json::to_string(r).unwrap(),
            Self::Detail(r) => serde_json::to_string(r).unwrap(),
        }
    }
}
