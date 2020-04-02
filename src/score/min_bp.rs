use std::fmt;

#[derive(Clone)]
pub struct MinBP {
    bp: i32,
}

impl MinBP {
    pub fn new(bp: i32) -> MinBP {
        MinBP { bp }
    }
}

impl fmt::Display for MinBP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.bp)
    }
}
