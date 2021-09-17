use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Level(String);

impl Level {
    pub fn make(str: String) -> Level {
        Level(str)
    }

    pub fn add_symbol(self, symbol: String) -> Self {
        Level(format!("{}{}", symbol, self.0.trim()))
    }
}

impl PartialOrd<Self> for Level {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Level {
    fn cmp(&self, other: &Self) -> Ordering {
        format!("{:>12}", self.0).cmp(&format!("{:>12}", other.0))
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.trim())
    }
}

pub type Levels = Vec<Level>;
