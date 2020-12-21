#[derive(Debug, Clone)]
pub struct GmailAddress(String);

impl GmailAddress {
    pub fn new(email: String) -> Self {
        GmailAddress(email)
    }
}

use std::fmt;
impl fmt::Display for GmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
