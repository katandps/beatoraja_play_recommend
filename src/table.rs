use std::fmt;
use crate::song::HashMd5;

pub struct Table {
    name: String,
    symbol: String,
    charts: Charts,
}

pub struct Charts {
    pub charts: Vec<Chart>
}

#[derive(Clone, PartialEq)]
pub struct Chart
{
    title: String,
    artist: String,
    pub md5: HashMd5,
    level: String,
}

impl Table {
    pub fn new(name: impl Into<String>, symbol: impl Into<String>, charts: Charts) -> Table {
        Table { name: name.into(), symbol: symbol.into(), charts }
    }
    pub fn level_specified(&self, level: String) -> Table {
        Table::new(&self.name, &self.symbol, self.charts.level_specified(level))
    }

    pub fn get_charts(&self) -> &Vec<Chart> { &self.charts.charts }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} [{}] {}", self.name, self.symbol, self.charts)
    }
}

impl Charts {
    pub fn new(charts: Vec<Chart>) -> Charts {
        Charts { charts }
    }
    pub fn level_specified(&self, level: String) -> Charts {
        let charts = self.charts.iter()
            .filter_map(|c| if c.level == level { Some(c) } else { None })
            .cloned()
            .collect();
        Charts::new(charts)
    }
}

impl Eq for Chart {}

impl fmt::Display for Charts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec: Vec<String> = self.charts.iter().map(|c| c.string()).collect();
        write!(f, "{}", vec.join("\n"))
    }
}

impl Chart {
    pub fn new(title: String, artist: String, md5: HashMd5, level: String) -> Chart {
        Chart { title, artist, md5, level }
    }

    pub fn string(&self) -> String {
        format!("{}: {}, {}", self.title, self.artist, self.md5)
    }
}

impl fmt::Display for Chart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.level, self.title)
    }
}