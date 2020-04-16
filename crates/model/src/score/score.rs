use crate::*;
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Score {
    ScoreImpl(ScoreImpl),
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
    pub fn new(
        clear: ClearType,
        updated_at: UpdatedAt,
        judge: Judge,
        max_combo: MaxCombo,
        play_count: PlayCount,
        min_bp: MinBP,
    ) -> Score {
        Score::ScoreImpl(ScoreImpl {
            clear,
            updated_at,
            judge,
            max_combo,
            play_count,
            min_bp,
        })
    }

    pub fn clear_type(&self) -> &ClearType {
        match self {
            Score::ScoreImpl(score) => &score.clear,
            _ => &ClearType::Failed,
        }
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Score::ScoreImpl(self_score), Score::ScoreImpl(other_score)) => {
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
            (Score::ScoreImpl(self_score), Score::ScoreImpl(other_score)) => {
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
            Score::ScoreImpl(score) => write!(
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
