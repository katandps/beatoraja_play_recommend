use crate::score::ex_score::ExScore;

#[derive(Clone, Debug)]
pub struct Judge {
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
