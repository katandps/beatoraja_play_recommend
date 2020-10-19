use crate::*;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct ScoredTable {
    charts: Vec<ScoredChart>,
}

impl ScoredTable {
    pub fn new(charts: Vec<ScoredChart>) -> ScoredTable {
        ScoredTable { charts }
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

trait ScoreCmp {
    fn cmp(&self, other: &Self) -> Ordering;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScoredChart {
    ScoredChart {
        song_id: SongId,
        chart: Chart,
        score: Score,
    },
    Dummy(i32),
}

impl ScoreCmp for ScoredChart {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (
                ScoredChart::ScoredChart {
                    song_id: _,
                    chart: _,
                    score: self_score,
                },
                ScoredChart::ScoredChart {
                    song_id: _,
                    chart: _,
                    score: other_score,
                },
            ) => self_score.cmp(&other_score),
            (ScoredChart::Dummy(s), ScoredChart::Dummy(o)) => s.cmp(o),
            _ => panic!("実体とDummyを比較しようとしました。"),
        }
    }
}

impl ScoredChart {
    pub fn new(song_id: SongId, chart: Chart, score: Score) -> ScoredChart {
        ScoredChart::ScoredChart {
            song_id,
            chart,
            score,
        }
    }
}

impl fmt::Display for ScoredChart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScoredChart::ScoredChart {
                song_id: _,
                chart,
                score,
            } => write!(f, "{}\n{}", chart, score),
            _ => write!(f, "Dummy Object"),
        }
    }
}
