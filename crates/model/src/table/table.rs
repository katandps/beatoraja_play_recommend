use crate::*;
use serde::de::DeserializeOwned;

#[derive(Serialize, Deserialize, Clone)]
pub struct Tables(Vec<Table<Charts>>);

impl Tables {
    pub fn new(v: Vec<Table<Charts>>) -> Self {
        Tables(v)
    }

    pub fn format(&self) -> Vec<TableFormat> {
        self.0
            .iter()
            .map(|t| TableFormat {
                name: t.name(),
                levels: t
                    .levels()
                    .iter()
                    .cloned()
                    .map(|l| format!("{}{}", t.symbol(), l.to_string()))
                    .collect::<Vec<_>>(),
            })
            .collect()
    }

    pub fn get_table(&self, mut index: usize) -> Table<Charts> {
        if index >= self.len() {
            index = 0;
        }
        self.0[index].clone()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn make_detail(&self, songs: &Songs, scores: &Scores, updated_at: &UpdatedAt) -> String {
        serde_json::to_string(
            &self
                .0
                .iter()
                .map(|table| table.make_detail(songs, scores, updated_at))
                .collect::<Vec<_>>(),
        )
        .unwrap()
    }
}

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
                    let level = table
                        .levels
                        .first()
                        .unwrap()
                        .clone()
                        .add_symbol(self.symbol());
                    DetailByLevel::new(
                        level.to_string(),
                        scores.detail(table, songs, updated_at, level),
                    )
                })
                .collect(),
        )
    }
}

#[derive(Serialize)]
pub struct TableFormat {
    name: String,
    levels: Vec<String>,
}
