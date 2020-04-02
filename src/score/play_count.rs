use std::fmt;

#[derive(Clone)]
pub struct PlayCount {
    count: i32,
}

impl PlayCount {
    pub fn new(count: i32) -> PlayCount {
        PlayCount { count }
    }
}

impl fmt::Display for PlayCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.count)
    }
}
