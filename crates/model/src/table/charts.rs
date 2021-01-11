use crate::*;
use itertools::Itertools;

#[derive(Serialize, Deserialize, Clone)]
pub struct Charts {
    pub(super) charts: Vec<Chart>,
}

impl Charts {
    pub fn make(charts: Vec<Chart>) -> Self {
        Charts { charts }
    }
    pub fn new() -> Self {
        Charts { charts: Vec::new() }
    }
    pub fn get_song(&self, song_data: &Songs) -> Vec<Song> {
        self.charts
            .iter()
            .map(|c| c.matched_song(song_data))
            .collect()
    }
    pub fn level_specified(&self, level: &Level) -> Self {
        Charts::make(
            self.charts
                .iter()
                .filter(|&c| &c.level == level)
                .cloned()
                .collect(),
        )
    }
    pub fn get_levels(&self) -> Vec<Level> {
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
