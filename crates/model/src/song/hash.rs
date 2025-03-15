use parse_display::Display;
use serde::{Deserialize, Serialize};
use std::str;
use std::string::ParseError;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Deserialize, Serialize, Default, Display)]
pub struct HashMd5(String);

impl HashMd5 {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, Deserialize, Serialize, Default, Display)]
pub struct HashSha256(String);

impl str::FromStr for HashSha256 {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HashSha256(s.parse().unwrap()))
    }
}

impl str::FromStr for HashMd5 {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HashMd5(s.parse().unwrap()))
    }
}
