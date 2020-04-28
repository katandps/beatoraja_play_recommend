use crate::*;
use itertools::Itertools;
use serde::de::DeserializeOwned;
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Charts {
    pub(super) charts: Vec<Chart>,
}

pub trait ChartsTrait:
    LevelSpecify + ChartsLevels + MergeScore + Serialize + DeserializeOwned + GetSong + fmt::Display
{
    fn make(charts: Vec<Chart>) -> Self;
    fn new() -> Self;
}
impl ChartsTrait for Charts {
    fn make(charts: Vec<Chart>) -> Self {
        Charts { charts }
    }

    fn new() -> Self {
        Charts { charts: Vec::new() }
    }
}

pub trait GetSong {
    fn get_song<'a>(&self, song_data: &'a Songs) -> Vec<&'a Song>;
}

impl GetSong for Charts {
    fn get_song<'a>(&self, song_data: &'a Songs) -> Vec<&'a Song> {
        self.charts
            .iter()
            .flat_map(|c| match song_data.song(&c.md5) {
                Some(s) => Some(s),
                _ => None,
            })
            .collect()
    }
}

pub trait LevelSpecify: Sized {
    fn level_specified(&self, level: &Level) -> Self;
}

impl LevelSpecify for Charts {
    fn level_specified(&self, level: &Level) -> Self {
        Charts::make(
            self.charts
                .iter()
                .filter(|&c| &c.level == level)
                .cloned()
                .collect(),
        )
    }
}

pub trait ChartsLevels {
    fn get_levels(&self) -> Vec<Level>;
}

impl ChartsLevels for Charts {
    fn get_levels(&self) -> Vec<Level> {
        let mut vec = self
            .charts
            .iter()
            .map(Chart::level)
            .unique()
            .collect::<Vec<Level>>();
        vec.sort();
        vec
    }
}

pub trait MergeScore {
    fn merge_score(&self, scores: &Scores, song_data: &Songs) -> ScoredTable;
}

impl MergeScore for Charts {
    fn merge_score(&self, scores: &Scores, song_data: &Songs) -> ScoredTable {
        ScoredTable::new(
            self.charts
                .iter()
                .flat_map(|chart| match song_data.song_id(&chart.md5) {
                    Some(song_id) => scores.merge(song_id, chart),
                    _ => None,
                })
                .collect(),
        )
    }
}

impl fmt::Display for Charts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.charts
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
