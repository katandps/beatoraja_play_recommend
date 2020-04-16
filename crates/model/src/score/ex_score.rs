use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExScore {
    score: i32,
}

impl ExScore {
    pub fn new() -> ExScore {
        ExScore { score: 0 }
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
