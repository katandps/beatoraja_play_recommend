use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct GoogleId(String);

impl GoogleId {
    pub fn new(id: String) -> Self {
        GoogleId(id)
    }
}

use std::fmt;
impl fmt::Display for GoogleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
