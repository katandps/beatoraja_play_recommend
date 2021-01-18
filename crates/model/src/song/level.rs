use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Level(String);

impl Level {
    pub fn make(str: String) -> Level {
        Level(str)
    }

    pub fn cmp(&self, b: &Level) -> std::cmp::Ordering {
        format!("{:>3}", self.0).cmp(&format!("{:>3}", b.0))
    }

    pub fn add_symbol(self, symbol: String) -> Self {
        Level(format!("{}{}", symbol, self.0.trim()))
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.trim())
    }
}

pub type Levels = Vec<Level>;
