use crate::*;
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Score {
    ScoreImpl { score: ScoreImpl },
    Dummy,
}

#[derive(Clone, Debug)]
pub struct ScoreImpl {
    clear: ClearType,
    updated_at: UpdatedAt,
    judge: Judge,
    max_combo: MaxCombo,
    play_count: PlayCount,
    min_bp: MinBP,
}

impl Score {
    pub fn from_data(
        clear: i32,
        timestamp: i32,
        epg: i32,
        lpg: i32,
        egr: i32,
        lgr: i32,
        egd: i32,
        lgd: i32,
        ebd: i32,
        lbd: i32,
        epr: i32,
        lpr: i32,
        ems: i32,
        lms: i32,
        combo: i32,
        playcount: i32,
        minbp: i32,
    ) -> Score {
        Score::ScoreImpl {
            score: ScoreImpl {
                clear: ClearType::from_integer(clear),
                updated_at: UpdatedAt::from_timestamp(timestamp),
                judge: Judge::new(epg, lpg, egr, lgr, egd, lgd, ebd, lbd, epr, lpr, ems, lms),
                max_combo: MaxCombo::from_combo(combo),
                play_count: PlayCount::new(playcount),
                min_bp: MinBP::from_bp(minbp),
            },
        }
    }

    pub fn clear_type(&self) -> &ClearType {
        match self {
            Score::ScoreImpl { score } => &score.clear,
            _ => &ClearType::Failed,
        }
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Score::ScoreImpl { score: self_score }, Score::ScoreImpl { score: other_score }) => {
                self_score.updated_at.cmp(&other_score.updated_at)
            }
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Score::ScoreImpl { score: self_score }, Score::ScoreImpl { score: other_score }) => {
                self_score.updated_at == other_score.updated_at
            }
            _ => true,
        }
    }
}

impl Eq for Score {}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Score::ScoreImpl { score } => write!(
                f,
                "{} {} score:{} bp:{} combo:{}",
                score.updated_at,
                score.clear,
                score.judge.ex_score(),
                score.min_bp,
                score.max_combo
            ),
            _ => write!(f, "dummy score"),
        }
    }
}
