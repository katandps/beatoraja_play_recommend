use crate::*;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Serialize, Debug, Clone, Default)]
pub struct Charts {
    pub(super) charts: Vec<Chart>,
}

impl Charts {
    pub fn make(charts: Vec<Chart>) -> Self {
        Charts { charts }
    }
    pub fn level_specified(&self, level: &Level) -> Self {
        Charts::make(
            self.charts
                .iter()
                .filter(|&c| &c.level() == level)
                .cloned()
                .collect(),
        )
    }
    pub fn get_levels(&self) -> Vec<Level> {
        self.charts
            .iter()
            .map(Chart::level)
            .unique()
            .sorted()
            .collect()
    }

    pub fn get_charts(&self) -> Vec<&Chart> {
        self.charts.iter().collect()
    }

    pub fn make_levels(&self, order: &[LevelVariant]) -> TableLevels {
        let mut m = HashMap::new();
        for chart in &self.charts {
            m.entry(chart.level().to_string())
                .or_insert_with(Vec::new)
                .push(chart.clone());
        }
        let a: Vec<Chart> = Vec::new();
        let v = order
            .iter()
            .map(|label| {
                let l = label.to_string();
                TableLevel::make(
                    l.clone(),
                    Charts::make(m.get(&l).cloned().unwrap_or_else(|| a.clone())),
                )
            })
            .collect();
        TableLevels::make(v)
    }

    pub fn pick_old_score_chart<'a>(
        &'a self,
        scores: &Scores,
        songs: &Songs,
    ) -> impl Iterator<Item = &'a Chart> {
        self.charts
            .iter()
            .map(|chart| {
                let score_id = songs
                    .song(chart)
                    .map(|song| song.song_id())
                    .unwrap_or_default();
                (scores.get(&score_id).cloned().unwrap_or_default(), chart)
            })
            .sorted_by(|a, b| a.0.updated_at.cmp(&b.0.updated_at))
            .map(|(_s, c)| c)
            .take(3)
    }
}
