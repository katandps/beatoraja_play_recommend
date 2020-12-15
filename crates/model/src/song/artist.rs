use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Artist {
    artist: String,
}

impl Artist {
    pub fn new(artist: String) -> Artist {
        Artist { artist }
    }
}

impl fmt::Display for Artist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.artist)
    }
}
