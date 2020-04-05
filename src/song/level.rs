use std::fmt;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Level {
    level: String,
}

impl Level {
    pub fn make(str: String) -> Level {
        Level {
            level: format!("{:>3}", str),
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.level.trim())
    }
}
