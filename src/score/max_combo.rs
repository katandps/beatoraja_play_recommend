use std::fmt;

#[derive(Clone, Debug)]
pub struct MaxCombo {
    combo: i32,
}

impl MaxCombo {
    pub fn new() -> MaxCombo {
        MaxCombo { combo: 0 }
    }
    pub fn from_combo(combo: i32) -> MaxCombo {
        MaxCombo { combo }
    }
}

impl fmt::Display for MaxCombo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.combo)
    }
}
