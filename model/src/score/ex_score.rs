use std::fmt;

#[derive(Clone, Debug)]
pub struct ExScore {
    score: i32,
}

impl ExScore {
    pub fn new() -> ExScore {
        ExScore { score: 0 }
    }

    pub fn from_judge(
        early_pgreat: i32,
        late_pgreat: i32,
        early_great: i32,
        late_great: i32,
    ) -> ExScore {
        ExScore {
            score: early_pgreat * 2 + late_pgreat * 2 + early_great + late_great,
        }
    }

    pub fn from_score(score: i32) -> ExScore {
        ExScore { score }
    }

    pub fn ex_score(&self) -> i32 {
        self.score
    }
}

impl fmt::Display for ExScore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ex_score())
    }
}

#[cfg(test)]
mod tests {
    use crate::score::ex_score::ExScore;

    #[test]
    fn ex_score() {
        let obj = ExScore::from_judge(1, 10, 100, 1000);
        assert_eq!(obj.ex_score(), 1122)
    }
}
