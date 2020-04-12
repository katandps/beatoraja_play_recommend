use super::*;

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
    pub fn level_specified(&self, level: &Level) -> Table {
        Table::make(
            &self.name,
            &self.symbol,
            self.charts.level_specified(level),
            None,
        )
    }

    pub fn ls(&self) -> &Levels {
        &self.levels
    }

    pub fn merge_score(&self, scores: &Scores, song_data: &Songs) -> ScoredTable {
        self.charts.merge_score(scores, song_data)
    }

    pub fn get_song<'a>(&self, song_data: &'a Songs) -> Vec<&'a Song> {
        self.charts.get_song(song_data)
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn symbol(&self) -> String {
        self.symbol.clone()
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}] {}", self.name, self.symbol, self.charts)
    }
}
