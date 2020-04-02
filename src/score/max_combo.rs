use std::fmt;

#[derive(Clone)]
pub struct MaxCombo {
    combo: i32,
}

impl MaxCombo {
    pub fn new(combo: i32) -> MaxCombo {
        MaxCombo { combo }
    }
}

impl fmt::Display for MaxCombo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.combo)
    }
}
