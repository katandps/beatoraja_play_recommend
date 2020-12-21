use crate::score::ex_score::ExScore;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
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
    pub fn new(
        early_pgreat: i32,
        late_pgreat: i32,
        early_great: i32,
        late_great: i32,
        early_good: i32,
        late_good: i32,
        early_bad: i32,
        late_bad: i32,
        early_poor: i32,
        late_poor: i32,
        early_miss: i32,
        late_miss: i32,
    ) -> Judge {
        Judge {
            early_pgreat,
            late_pgreat,
            early_great,
            late_great,
            early_good,
            late_good,
            early_bad,
            late_bad,
            early_poor,
            late_poor,
            early_miss,
            late_miss,
        }
    }
    pub fn default() -> Judge {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    }
    pub fn ex_score(&self) -> ExScore {
        ExScore::from_score(
            self.early_pgreat * 2 + self.late_pgreat * 2 + self.early_great + self.late_great,
        )
    }
}

impl std::ops::Sub<Judge> for Judge {
    type Output = Judge;
    fn sub(self, rhs: Judge) -> Judge {
        Judge::new(
            self.early_pgreat - rhs.early_pgreat,
            self.late_pgreat - rhs.late_pgreat,
            self.early_great - rhs.early_great,
            self.late_great - rhs.late_great,
            self.early_good - rhs.early_good,
            self.late_good - rhs.late_good,
            self.early_bad - rhs.early_bad,
            self.late_bad - rhs.late_bad,
            self.early_poor - rhs.early_poor,
            self.late_poor - rhs.late_poor,
            self.early_miss - rhs.early_miss,
            self.late_miss - rhs.late_miss,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::score::ex_score::ExScore;
    use crate::Judge;

    #[test]
    fn ex_score() {
        let judge = Judge::new(
            1, 3, 10, 30, 100, 300, 1000, 3000, 10000, 30000, 100000, 300000,
        );
        assert_eq!(judge.ex_score(), ExScore::from_score(48))
    }
}
