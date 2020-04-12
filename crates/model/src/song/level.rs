use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
pub struct Levels {
    pub levels: Vec<Level>,
}

impl Levels {
    pub fn new() -> Levels {
        Levels { levels: Vec::new() }
    }

    pub fn make(levels: Vec<Level>) -> Levels {
        Levels { levels }
    }
}
