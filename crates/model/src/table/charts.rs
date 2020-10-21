use crate::*;
use itertools::Itertools;
use serde::de::DeserializeOwned;

#[derive(Serialize, Deserialize, Clone)]
pub struct Charts {
    pub(super) charts: Vec<Chart>,
}

pub trait ChartsTrait:
    LevelSpecify + ChartsLevels + Serialize + DeserializeOwned + GetSong
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
