use crate::song::{HashMd5, HashSha256};

pub struct SongData {}

impl SongData {
    pub fn get_md5(sha256: HashSha256) -> HashMd5 {
        HashMd5::new("md5".parse().unwrap())
    }

    pub fn get_sha256(md5: HashMd5) -> HashSha256 {
        HashSha256::new("sha256".parse().unwrap())
    }
}