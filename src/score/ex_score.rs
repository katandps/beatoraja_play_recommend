use std::fmt;

pub struct ExScore {
    score: i32,
}

impl ExScore {
    pub fn new(early_pgreat: i32, late_pgreat: i32, early_great: i32, late_great: i32) -> ExScore {
        ExScore {
            score: early_pgreat * 2 + late_pgreat * 2 + early_great + late_great,
        }
    }

    pub fn make_by_score(score: i32) -> ExScore {
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
        let obj = ExScore::new(1, 10, 100, 1000);
        assert_eq!(obj.ex_score(), 1122)
    }
}
