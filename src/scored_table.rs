use std::fmt;

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

impl fmt::Display for ScoredChart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.song_id, self.chart, self.score)
    }
}