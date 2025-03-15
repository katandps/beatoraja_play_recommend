use parse_display::Display;

#[derive(Clone, Debug, Eq, PartialEq, Display)]
pub struct SessionKey(String);

impl SessionKey {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
