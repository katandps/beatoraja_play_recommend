use crate::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chart {
    pub(super) title: Title,
    pub(super) artist: Artist,
    pub(super) md5: HashMd5,
    pub(super) level: Level,
}

impl Chart {
    pub fn new(title: String, artist: String, md5: HashMd5, level: String) -> Chart {
        Chart {
            title: Title::new(title),
            artist: Artist::new(artist),
            md5,
            level: Level::make(level),
        }
    }

    pub fn level(&self) -> Level {
        self.level.clone()
    }
}

impl fmt::Display for Chart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.level, self.title, self.artist)
    }
}
