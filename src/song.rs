use std::fmt;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct HashMd5 {
    md5: String
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct HashSha256 {
    sha256: String
}

impl HashMd5 {
    pub fn new(md5: String) -> HashMd5 { HashMd5 { md5 } }
}

impl HashSha256 {
    pub fn new(sha256: String) -> HashSha256 { HashSha256 { sha256 } }
}


impl fmt::Display for HashSha256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! {f, "{}", self.sha256}
    }
}

impl fmt::Display for HashMd5 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! {f, "{}", self.md5}
    }
}