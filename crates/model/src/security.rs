use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionKey(String);

impl Display for SessionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl SessionKey {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
