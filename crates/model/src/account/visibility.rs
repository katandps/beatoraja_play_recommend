use serde::Serialize;
use std::fmt::Formatter;

#[derive(Clone, Debug, Serialize)]
pub struct Visibility(bool);

impl Visibility {
    pub fn new(v: bool) -> Visibility {
        Visibility(v)
    }

    pub fn to_bool(&self) -> bool {
        self.0
    }
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
