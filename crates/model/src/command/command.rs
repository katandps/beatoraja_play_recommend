use super::detail;
use super::lamp;
use super::rank;
use super::recommend;
use crate::*;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Command {
    Recommend,
    LampGraph,
    RankGraph,
    Detail,
}

pub type CommandFunc<T> = fn(&Songs, &T, &Scores, &ScoreLog, &UpdatedAt) -> CommandResult;

impl Command {
    pub fn all() -> Vec<Command> {
        vec![Self::Recommend, Self::LampGraph, Self::RankGraph]
    }

    pub fn func<T: TableTrait>(&self) -> CommandFunc<T> {
        match self {
            Self::Recommend => recommend::recommend,
            Self::LampGraph => lamp::lamp,
            Self::RankGraph => rank::rank,
            Self::Detail => detail::detail,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub enum CommandResult {
    Recommend(RecommendResult),
    LampGraph(Graph<ClearType>),
    RankGraph(Graph<ClearRank>),
    Detail(DetailResult),
}

impl CommandResult {
    pub fn to_text(&self) -> String {
        match self {
            Self::Recommend(r) => r.to_string(),
            Self::LampGraph(r) => r.to_string(),
            Self::RankGraph(r) => r.to_string(),
            Self::Detail(r) => r.to_string(),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
