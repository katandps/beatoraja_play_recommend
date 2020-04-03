use std::fmt;

#[derive(Clone, Debug)]
pub struct MinBP {
    bp: i32,
}

impl MinBP {
    pub fn new() -> MinBP {
        MinBP { bp: 0 }
    }
    pub fn from_bp(bp: i32) -> MinBP {
        MinBP { bp }
    }
}

impl fmt::Display for MinBP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.bp)
    }
}
