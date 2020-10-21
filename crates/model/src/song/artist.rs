use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Artist {
    artist: String,
}

impl Artist {
    pub fn new(artist: String) -> Artist {
        Artist { artist }
    }
}
