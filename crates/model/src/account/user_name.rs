use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct UserName(String);

impl UserName {
    pub fn new(name: String) -> Self {
        UserName(name)
    }
}

use std::fmt;
impl fmt::Display for UserName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
