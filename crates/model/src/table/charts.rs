use crate::*;
use serde::de::DeserializeOwned;
use std::collections::HashSet;
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
        let charts = self
            .charts
            .iter()
            .filter_map(|c| if &c.level == level { Some(c) } else { None })
            .cloned()
            .collect();
        Charts::make(charts)
    }
}

pub trait ChartsLevels {
    fn get_levels(&self) -> Vec<Level>;
}

impl ChartsLevels for Charts {
    fn get_levels(&self) -> Vec<Level> {
        let mut set = HashSet::new();
        for level in self.charts.iter().map(|c| c.level.clone()) {
            set.insert(level);
        }
        let mut vec: Vec<Level> = set.iter().cloned().collect();
        vec.sort_unstable();
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
        let vec: Vec<String> = self.charts.iter().map(|c| c.string()).collect();
        write!(f, "{}", vec.join("\n"))
    }
}
