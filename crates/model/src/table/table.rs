use crate::*;

#[derive(Debug, Clone)]
pub struct Tables {
    v: Vec<Table>,
}

impl Tables {
    pub fn make(v: Vec<Table>) -> Self {
        Self { v }
    }

    pub fn get_charts(&self) -> Vec<&Chart> {
        self.v.iter().map(|t| t.get_charts()).flatten().collect()
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

    pub fn get_charts(&self) -> Vec<&Chart> {
        self.levels.get_charts()
    }

    pub fn symbol(&self) -> String {
        self.symbol.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct TableName(String);

#[derive(Debug, Clone)]
pub struct TableSymbol(String);

#[derive(Debug, Clone)]
pub struct TableLevels {
    v: Vec<TableLevel>,
}

impl TableLevels {
    pub fn make(v: Vec<TableLevel>) -> Self {
        Self { v }
    }

    pub fn get_charts(&self) -> Vec<&Chart> {
        self.v.iter().map(|l| l.get_charts()).flatten().collect()
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
}

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
                    .or_insert(Vec::new())
                    .push(chart.md5.to_string())
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

impl From<Tables> for TablesFormat {
    fn from(t: Tables) -> TablesFormat {
        TablesFormat(t.v.iter().map(TableFormat::from).collect())
    }
}
