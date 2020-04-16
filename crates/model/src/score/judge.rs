use crate::score::ex_score::ExScore;

#[derive(Clone, Debug)]
pub enum Judge {
    JudgeImpl { judge: JudgeImpl },
    Dummy { score: i32 },
}
#[derive(Clone, Debug)]
pub struct JudgeImpl {
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
        Judge::JudgeImpl {
            judge: JudgeImpl {
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
            },
        }
    }
    pub fn ex_score(&self) -> ExScore {
        let score = match self {
            Judge::JudgeImpl { judge } => {
                judge.early_pgreat * 2
                    + judge.late_pgreat * 2
                    + judge.early_great
                    + judge.late_great
            }
            Judge::Dummy { score } => *score,
        };
        ExScore::from_score(score)
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
