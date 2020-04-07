use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Title {
    title: String,
}

impl Title {
    pub fn make(title: String) -> Title {
        Title { title }
    }
}

impl fmt::Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.title)
    }
}