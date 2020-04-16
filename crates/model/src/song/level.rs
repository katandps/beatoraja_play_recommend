use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Level(String);

impl Level {
    pub fn make(str: String) -> Level {
        Level(format!("{:>3}", str))
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.trim())
    }
}

pub type Levels = Vec<Level>;
