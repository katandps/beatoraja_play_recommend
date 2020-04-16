use crate::*;
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Table {
    name: String,
    symbol: String,
    charts: Charts,
    levels: Levels,
}

impl Table {
    pub fn new() -> Table {
        Table {
            name: "Not Loaded".to_string(),
            symbol: "No".to_string(),
            charts: Charts { charts: Vec::new() },
            levels: Levels::new(),
        }
    }
    pub fn make(
        name: impl Into<String>,
        symbol: impl Into<String>,
        charts: Charts,
        levels: Option<Vec<String>>,
    ) -> Table {
        let levels: Vec<Level> = match levels {
            Some(l) => l.iter().map(|s| Level::make(s.clone())).collect(),
            _ => charts.get_levels(),
        };
        Table {
            name: name.into(),
            symbol: symbol.into(),
            charts,
            levels: Levels::make(levels),
        }
    }
}
pub trait TableTrait: TableName + TableSymbol + TableLevels + TableCharts + TableFilter {}

pub trait TableFilter: Sized {
    fn level_specified(&self, level: &Level) -> Self;
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
pub trait TableCharts {
    fn merge_score(&self, scores: &Scores, song_data: &Songs) -> ScoredTable;
    fn get_song<'a>(&self, song_data: &'a Songs) -> Vec<&'a Song>;
}

impl TableTrait for Table {}
impl TableFilter for Table {
    fn level_specified(&self, level: &Level) -> Self {
        Table::make(
            &self.name,
            &self.symbol,
            self.charts.level_specified(level),
            None,
        )
    }
}
impl TableName for Table {
    fn name(&self) -> String {
        self.name.clone()
    }
}
impl TableSymbol for Table {
    fn symbol(&self) -> String {
        self.symbol.clone()
    }
}
impl TableLevels for Table {
    fn levels(&self) -> &Levels {
        &self.levels
    }
}
impl TableCharts for Table {
    fn merge_score(&self, scores: &Scores, song_data: &Songs) -> ScoredTable {
        self.charts.merge_score(scores, song_data)
    }

    fn get_song<'a>(&self, song_data: &'a Songs) -> Vec<&'a Song> {
        self.charts.get_song(song_data)
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}] {}", self.name, self.symbol, self.charts)
    }
}
