use std::fmt;
use std::str;
use std::string::ParseError;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct HashMd5 {
    md5: String,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct HashSha256 {
    sha256: String,
}

impl HashMd5 {
    pub fn new(md5: String) -> HashMd5 {
        HashMd5 { md5 }
    }
}

impl HashSha256 {
    pub fn new(sha256: String) -> HashSha256 {
        HashSha256 { sha256 }
    }
}

impl fmt::Display for HashSha256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write! {f, "{}", self.sha256}
    }
}

impl str::FromStr for HashSha256 {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HashSha256 {
            sha256: s.parse().unwrap(),
        })
    }
}

impl str::FromStr for HashMd5 {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HashMd5 {
            md5: s.parse().unwrap(),
        })
    }
}

impl fmt::Display for HashMd5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write! {f, "{}", self.md5}
    }
}
