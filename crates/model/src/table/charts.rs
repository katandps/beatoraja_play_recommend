use crate::*;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Charts {
    pub(super) charts: Vec<Chart>,
}

impl Charts {
    pub fn make(charts: Vec<Chart>) -> Self {
        Charts { charts }
    }
    pub fn new() -> Self {
        Charts { charts: Vec::new() }
    }
    pub fn level_specified(&self, level: &Level) -> Self {
        Charts::make(
            self.charts
                .iter()
                .filter(|&c| &c.level == level)
                .cloned()
                .collect(),
        )
    }
    pub fn get_levels(&self) -> Vec<Level> {
        let mut vec = self
            .charts
            .iter()
            .map(Chart::level)
            .unique()
            .collect::<Vec<Level>>();
        vec.sort();
        vec
    }

    pub fn get_charts(&self) -> Vec<&Chart> {
        self.charts.iter().map(|c| c).collect()
    }

    pub fn make_levels(&self, order: &Vec<String>) -> TableLevels {
        let mut m = HashMap::new();
        for chart in &self.charts {
            m.entry(chart.level.to_string())
                .or_insert(Vec::new())
                .push(chart.clone());
        }
        let a: Vec<Chart> = Vec::new();
        let v = order
            .iter()
            .map(|l| {
                TableLevel::make(
                    l.clone(),
                    Charts::make(m.get(l).cloned().unwrap_or(a.clone())),
                )
            })
            .collect();
        TableLevels::make(v)
    }
}
