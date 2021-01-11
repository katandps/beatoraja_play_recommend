use serde::{Deserialize, Serialize};
use std::fmt;
use std::str;
use std::string::ParseError;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Deserialize, Serialize)]
pub struct HashMd5(String);

#[derive(PartialEq, Eq, Hash, Clone, Debug, Deserialize, Serialize)]
pub struct HashSha256(String);

impl HashMd5 {
    pub fn new(md5: String) -> HashMd5 {
        HashMd5(md5)
    }
}

impl HashSha256 {
    pub fn new(sha256: String) -> HashSha256 {
        HashSha256(sha256)
    }
    pub fn default() -> HashSha256 {
        HashSha256("".into())
    }
}

impl fmt::Display for HashSha256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write! {f, "{}", self.0}
    }
}

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

impl fmt::Display for HashMd5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write! {f, "{}", self.0}
    }
}
