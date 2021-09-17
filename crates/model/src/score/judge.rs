use crate::score::ex_score::ExScore;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Judge {
    pub early_pgreat: i32,
    pub late_pgreat: i32,
    pub early_great: i32,
    pub late_great: i32,
    pub early_good: i32,
    pub late_good: i32,
    pub early_bad: i32,
    pub late_bad: i32,
    pub early_poor: i32,
    pub late_poor: i32,
    pub early_miss: i32,
    pub late_miss: i32,
}

impl Judge {
    pub fn ex_score(&self) -> ExScore {
        ExScore::from_score(
            self.early_pgreat * 2 + self.late_pgreat * 2 + self.early_great + self.late_great,
        )
    }
}

impl std::ops::Sub<Judge> for Judge {
    type Output = Judge;
    fn sub(self, rhs: Judge) -> Judge {
        Judge {
            early_pgreat: self.early_pgreat - rhs.early_pgreat,
            late_pgreat: self.late_pgreat - rhs.late_pgreat,
            early_great: self.early_great - rhs.early_great,
            late_great: self.late_great - rhs.late_great,
            early_good: self.early_good - rhs.early_good,
            late_good: self.late_good - rhs.late_good,
            early_bad: self.early_bad - rhs.early_bad,
            late_bad: self.late_bad - rhs.late_bad,
            early_poor: self.early_poor - rhs.early_poor,
            late_poor: self.late_poor - rhs.late_poor,
            early_miss: self.early_miss - rhs.early_miss,
            late_miss: self.late_miss - rhs.late_miss,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::score::ex_score::ExScore;
    use crate::Judge;

    #[test]
    fn ex_score() {
        let judge = Judge {
            early_pgreat: 1,
            late_pgreat: 3,
            early_great: 10,
            late_great: 30,
            early_good: 100,
            late_good: 300,
            early_bad: 1000,
            late_bad: 3000,
            early_poor: 10000,
            late_poor: 30000,
            early_miss: 100000,
            late_miss: 300000,
        };
        assert_eq!(judge.ex_score(), ExScore::from_score(48))
    }
}
