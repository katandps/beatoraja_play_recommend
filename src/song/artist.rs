use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Artist {
    artist: String,
}

impl Artist {
    pub fn make(artist: String) -> Artist {
        Artist { artist }
    }
}

impl fmt::Display for Artist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.artist)
    }
}