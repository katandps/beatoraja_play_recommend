mod chart;
mod charts;
mod prelude;
mod table;

pub use prelude::*;

use serde::Serialize;

#[derive(Debug, Clone)]
pub struct NewTables {
    v: Vec<NewTable>,
}

#[derive(Debug, Clone)]
pub struct NewTable {
    title: TableName,
    symbol: TableSymbol,
    levels: NewLevels,
}

#[derive(Debug, Clone)]
pub struct TableName(String);

#[derive(Debug, Clone)]
pub struct TableSymbol(String);

#[derive(Debug, Clone)]
pub struct NewLevels {
    v: Vec<NewLevel>,
}

#[derive(Debug, Clone)]
pub struct NewLevel {
    label: String,
    charts: Charts,
}

/// フロント出力用フォーマット
/// name: 難易度表名
/// levels: HashMap<レベル名, 曲のHashMd5>
use std::collections::HashMap;
#[derive(Serialize)]
pub struct TableFormat {
    name: String,
    levels: HashMap<String, Vec<String>>,
}

#[derive(Serialize)]
pub struct TablesFormat(Vec<TableFormat>);

impl From<NewTables> for TablesFormat {
    fn from(_t: NewTables) -> TablesFormat {
        TablesFormat(Vec::new())
    }
}
