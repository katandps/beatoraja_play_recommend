use std::fmt;
use std::cmp::Ordering;

use crate::whole_score::scores::score::song_id::SongId;
use crate::whole_score::scores::score::Score;
use crate::table::Chart;

pub struct ScoredTable {
    charts: Vec<ScoredChart>
}

impl ScoredTable {
    pub fn new(charts: Vec<ScoredChart>) -> ScoredTable {
        ScoredTable { charts }
    }
    pub fn recent_updated(&self) -> ScoredTable {
        let mut vec: Vec<ScoredChart> = self.charts.iter().cloned().collect();
        vec.sort();
        ScoredTable::new(
            vec
                .iter()
                .take(3)
                .cloned()
                .collect()
        )
    }
}

impl fmt::Display for ScoredTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for chart in &self.charts {
            s.push_str(format!("{}\n", chart).as_ref())
        }
        write!(f, "{}", s)
    }
}

#[derive(Eq, Clone)]
pub struct ScoredChart {
    song_id: SongId,
    chart: Chart,
    score: Score,
}

impl ScoredChart {
    pub fn new(song_id: SongId, chart: Chart, score: Score) -> ScoredChart {
        ScoredChart { song_id, chart, score }
    }
}

impl Ord for ScoredChart {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for ScoredChart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ScoredChart {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl fmt::Display for ScoredChart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.chart, self.score)
    }
}