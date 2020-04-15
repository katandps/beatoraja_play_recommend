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
    pub fn old_updated(&self) -> ScoredTable {
        let mut vec: Vec<ScoredChart> = self.charts.iter().cloned().collect();
        vec.sort_by(ScoredChart::cmp);
        ScoredTable::new(
            vec.iter()
                .take(config().recommend_song_number())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let charts = vec![
            ScoredChart::Dummy(3),
            ScoredChart::Dummy(2),
            ScoredChart::Dummy(4),
            ScoredChart::Dummy(1),
        ];
        let table = ScoredTable { charts };

        let expect_vec = vec![
            ScoredChart::Dummy(1),
            ScoredChart::Dummy(2),
            ScoredChart::Dummy(3),
        ];
        let expect = ScoredTable { charts: expect_vec };
        assert_eq!(table.old_updated(), expect)
    }
}
