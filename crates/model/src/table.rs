mod chart;
mod charts;
mod prelude;

pub use prelude::*;

use crate::*;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TableSource {
    v: HashMap<usize, String>,
}

#[derive(Debug, Clone)]
pub struct TableId(i64);

#[derive(Debug, Clone, Default)]
pub struct Tables {
    v: HashMap<usize, Table>,
}

impl Tables {
    pub fn update(&mut self, i: usize, t: Table) {
        self.v.insert(i, t);
    }

    pub fn get_charts(&self) -> Vec<&Chart> {
        self.v.iter().flat_map(|(_i, t)| t.get_charts()).collect()
    }

    pub fn get(&self, index: usize) -> Option<&Table> {
        self.v.get(&index)
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    title: TableName,
    symbol: TableSymbol,
    levels: TableLevels,
}

impl Table {
    pub fn make(title: impl Into<String>, symbol: impl Into<String>, levels: TableLevels) -> Self {
        Table {
            title: TableName(title.into()),
            symbol: TableSymbol(symbol.into()),
            levels,
        }
    }

    pub fn title(&self) -> String {
        self.title.clone().into()
    }

    pub fn get_charts(&self) -> Vec<&Chart> {
        self.levels.get_charts()
    }

    pub fn symbol(&self) -> String {
        self.symbol.0.clone()
    }

    pub fn get_level_list(&self) -> Vec<String> {
        self.levels.get_list()
    }

    pub fn filter_score(&self, scores: &Scores, songs: &Songs) -> Vec<&Chart> {
        self.levels.filter_score(scores, songs)
    }
}

#[derive(Debug, Clone)]
pub struct TableName(String);

impl From<TableName> for String {
    fn from(name: TableName) -> Self {
        name.0
    }
}

#[derive(Debug, Clone)]
pub struct TableSymbol(String);

impl From<TableSymbol> for String {
    fn from(symbol: TableSymbol) -> Self {
        symbol.0
    }
}

#[derive(Debug, Clone)]
pub struct TableLevels {
    v: Vec<TableLevel>,
}

impl TableLevels {
    pub fn make(v: Vec<TableLevel>) -> Self {
        Self { v }
    }

    pub fn get_charts(&self) -> Vec<&Chart> {
        self.v.iter().flat_map(|l| l.get_charts()).collect()
    }

    pub fn get_list(&self) -> Vec<String> {
        self.v.iter().map(|l| l.label.clone()).collect()
    }

    pub fn filter_score(&self, scores: &Scores, songs: &Songs) -> Vec<&Chart> {
        self.v
            .iter()
            .flat_map(|l| l.pick_old_score_chart(scores, songs))
            .collect()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum LevelVariant {
    Str(String),
    Number(i64),
}

impl ToString for LevelVariant {
    fn to_string(&self) -> String {
        match self {
            Self::Str(s) => s.clone(),
            Self::Number(i) => i.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableLevel {
    label: String,
    charts: Charts,
}

impl TableLevel {
    pub fn make(label: String, charts: Charts) -> Self {
        Self { label, charts }
    }

    pub fn get_charts(&self) -> Vec<&Chart> {
        self.charts.get_charts()
    }

    pub fn get_label(&self, t: &Table) -> String {
        format!("{}{}", t.symbol(), self.label)
    }

    pub fn pick_old_score_chart(&self, scores: &Scores, songs: &Songs) -> Vec<&Chart> {
        self.charts.pick_old_score_chart(scores, songs)
    }
}

use itertools::Itertools;
/// フロント出力用フォーマット
/// name: 難易度表名
/// levels: HashMap<レベル名, 曲のHashMd5>
use std::collections::HashMap;

#[derive(Serialize)]
pub struct TableFormat {
    name: String,
    level_list: Vec<String>,
    levels: HashMap<String, Vec<String>>,
}

impl From<&Table> for TableFormat {
    fn from(t: &Table) -> TableFormat {
        let mut map = HashMap::new();
        for level in &t.levels.v {
            for chart in &level.charts.charts {
                map.entry(level.get_label(t))
                    .or_insert_with(Vec::new)
                    .push(chart.md5().to_string())
            }
        }
        TableFormat {
            name: t.title.0.clone(),
            level_list: t.levels.v.iter().map(|l| l.get_label(t)).collect(),
            levels: map,
        }
    }
}

#[derive(Serialize)]
pub struct TablesFormat(Vec<TableFormat>);

impl TablesFormat {
    pub fn format(t: &Tables) -> TablesFormat {
        let indexes = t.v.iter().map(|(&table_index, _t)| table_index).sorted();

        TablesFormat(
            indexes
                .map(|i| TableFormat::from(t.get(i).unwrap()))
                .collect(),
        )
    }
}

#[derive(Serialize)]
pub struct CustomTableHeader {
    name: String,
    data_url: String,
    symbol: String,
    level_order: Vec<String>,
}

impl CustomTableHeader {
    pub fn set_name(&self, name: String) -> CustomTableHeader {
        CustomTableHeader {
            name,
            data_url: self.data_url.clone(),
            symbol: self.symbol.clone(),
            level_order: self.level_order.clone(),
        }
    }
}

impl From<&Table> for CustomTableHeader {
    fn from(t: &Table) -> CustomTableHeader {
        CustomTableHeader {
            name: t.title.clone().into(),
            data_url: "score.json".to_string(),
            symbol: t.symbol.clone().into(),
            level_order: t.get_level_list(),
        }
    }
}
