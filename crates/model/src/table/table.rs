use crate::*;
use serde::de::DeserializeOwned;

#[derive(Serialize, Deserialize, Clone)]
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

    fn level_specified_vec(&self) -> Vec<Table<T>> {
        self.levels()
            .iter()
            .map(|level| self.level_specified(level))
            .collect()
    }
}
pub trait TableTrait:
    TableName
    + TableSymbol
    + TableLevels
    + GetSong
    + LevelSpecify
    + Serialize
    + DeserializeOwned
    + MakeDetail
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

pub trait MakeDetail {
    fn make_detail(&self, songs: &Songs, scores: &Scores, updated_at: &UpdatedAt) -> DetailResult;
}

impl<T: ChartsTrait> TableTrait for Table<T> {}
impl<T: ChartsTrait> LevelSpecify for Table<T> {
    fn level_specified(&self, level: &Level) -> Self {
        Table {
            name: self.name.clone(),
            symbol: self.symbol.clone(),
            charts: self.charts.level_specified(level),
            levels: vec![level.clone()],
        }
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
impl<T: ChartsTrait> GetSong for Table<T> {
    fn get_song<'a>(&self, song_data: &'a Songs) -> Vec<&'a Song> {
        self.charts.get_song(song_data)
    }
}

impl<T: ChartsTrait> MakeDetail for Table<T> {
    fn make_detail(&self, songs: &Songs, scores: &Scores, updated_at: &UpdatedAt) -> DetailResult {
        DetailResult::new(
            self.name(),
            self.level_specified_vec()
                .iter()
                .map(|table| {
                    DetailByLevel::new(
                        format!("{}{}", self.symbol(), table.levels.first().unwrap()),
                        scores.detail(table, songs, updated_at),
                    )
                })
                .collect(),
        )
    }
}
