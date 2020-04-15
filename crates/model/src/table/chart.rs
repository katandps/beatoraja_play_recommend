use crate::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Chart {
    pub(super) title: Title,
    pub(super) artist: Artist,
    pub(super) md5: HashMd5,
    pub(super) level: Level,
}

impl Chart {
    pub fn new(title: String, artist: String, md5: HashMd5, level: String) -> Chart {
        Chart {
            title: Title::make(title),
            artist: Artist::make(artist),
            md5,
            level: Level::make(level),
        }
    }

    pub fn string(&self) -> String {
        format!("{}: {}, {}", self.title, self.artist, self.md5)
    }
}

impl fmt::Display for Chart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.level, self.title)
    }
}

impl Eq for Chart {}
