use crate::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Tables(Vec<Table>);

impl Tables {
    pub fn new(v: Vec<Table>) -> Self {
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

    pub fn get_table(&self, mut index: usize) -> Table {
        if index >= self.len() {
            index = 0;
        }
        self.0[index].clone()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn make_detail(
        &self,
        songs: &Songs,
        scores: &Scores,
        updated_at: &UpdatedAt,
    ) -> Vec<DetailResult> {
        self.0
            .iter()
            .map(|table| table.make_detail(songs, scores, updated_at))
            .collect::<Vec<_>>()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Table {
    name: String,
    symbol: String,
    charts: Charts,
    levels: Levels,
}

impl Table {
    pub fn make(
        name: impl Into<String>,
        symbol: impl Into<String>,
        charts: Charts,
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

    fn level_specified_vec(&self) -> Vec<Table> {
        self.levels()
            .iter()
            .map(|level| self.level_specified(level))
            .collect()
    }

    pub fn level_specified(&self, level: &Level) -> Self {
        Table {
            name: self.name.clone(),
            symbol: self.symbol.clone(),
            charts: self.charts.level_specified(level),
            levels: vec![level.clone()],
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn symbol(&self) -> String {
        self.symbol.clone()
    }
    pub fn levels(&self) -> &Levels {
        &self.levels
    }
    pub fn get_song(&self, song_data: &Songs) -> Vec<Song> {
        self.charts.get_song(song_data)
    }
    pub fn make_detail(
        &self,
        songs: &Songs,
        scores: &Scores,
        updated_at: &UpdatedAt,
    ) -> DetailResult {
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
