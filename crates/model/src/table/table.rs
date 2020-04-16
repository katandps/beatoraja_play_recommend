use crate::*;
use serde::de::DeserializeOwned;
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Table<T> {
    name: String,
    symbol: String,
    charts: T,
    levels: Levels,
}

impl<T: ChartsTrait> Table<T> {
    pub fn make(
        name: impl Into<String>,
        symbol: impl Into<String>,
        charts: T,
        levels: Option<Vec<String>>,
    ) -> Self {
        let levels: Levels = match levels {
            Some(l) => l.iter().map(|s| Level::make(s.clone())).collect(),
            _ => charts.get_levels(),
        };
        Table {
            name: name.into(),
            symbol: symbol.into(),
            charts,
            levels,
        }
    }
}
pub trait TableTrait:
    TableName
    + TableSymbol
    + TableLevels
    + MergeScore
    + GetSong
    + LevelSpecify
    + Serialize
    + DeserializeOwned
    + fmt::Display
{
}

pub trait TableName {
    fn name(&self) -> String;
}
pub trait TableSymbol {
    fn symbol(&self) -> String;
}
pub trait TableLevels {
    fn levels(&self) -> &Levels;
}

impl<T: ChartsTrait> TableTrait for Table<T> {}
impl<T: ChartsTrait> LevelSpecify for Table<T> {
    fn level_specified(&self, level: &Level) -> Self {
        Table::make(
            &self.name,
            &self.symbol,
            self.charts.level_specified(level),
            None,
        )
    }
}
impl<T: ChartsTrait> TableName for Table<T> {
    fn name(&self) -> String {
        self.name.clone()
    }
}
impl<T: ChartsTrait> TableSymbol for Table<T> {
    fn symbol(&self) -> String {
        self.symbol.clone()
    }
}
impl<T: ChartsTrait> TableLevels for Table<T> {
    fn levels(&self) -> &Levels {
        &self.levels
    }
}
impl<T: ChartsTrait> MergeScore for Table<T> {
    fn merge_score(&self, scores: &Scores, song_data: &Songs) -> ScoredTable {
        self.charts.merge_score(scores, song_data)
    }
}
impl<T: ChartsTrait> GetSong for Table<T> {
    fn get_song<'a>(&self, song_data: &'a Songs) -> Vec<&'a Song> {
        self.charts.get_song(song_data)
    }
}

impl<T: ChartsTrait> fmt::Display for Table<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}] {}", self.name, self.symbol, self.charts)
    }
}
