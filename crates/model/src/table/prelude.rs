pub use crate::table::{
    chart::Chart,
    charts::{Charts, ChartsLevels, ChartsTrait, GetSong, LevelSpecify},
    table::{Table, TableLevels, TableName, TableSymbol, TableTrait},
};

pub type Tables = Vec<Table<Charts>>;
