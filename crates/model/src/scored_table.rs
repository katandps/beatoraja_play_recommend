use crate::*;
use std::fmt;

pub struct ScoredTable {
    charts: Vec<ScoredChart>,
}

impl ScoredTable {
    pub fn new(charts: Vec<ScoredChart>) -> ScoredTable {
        ScoredTable { charts }
    }
    pub fn recent_updated(&self) -> ScoredTable {
        let mut vec: Vec<ScoredChart> = self.charts.iter().cloned().collect();
        vec.sort_by(|a, b| a.score.cmp(&b.score));
        ScoredTable::new(
            vec.iter()
                .take(config::config().recommend_song_number())
                .cloned()
                .collect(),
        )
    }
}

impl fmt::Display for ScoredTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for chart in &self.charts {
            s.push_str(format!("{}\n", chart).as_ref())
        }
        write!(f, "{}", s)
    }
}

#[derive(Clone)]
pub struct ScoredChart {
    song_id: SongId,
    chart: Chart,
    score: Score,
}

impl ScoredChart {
    pub fn new(song_id: SongId, chart: Chart, score: Score) -> ScoredChart {
        ScoredChart {
            song_id,
            chart,
            score,
        }
    }
}

impl fmt::Display for ScoredChart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.chart, self.score)
    }
}
