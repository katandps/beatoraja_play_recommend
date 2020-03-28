use crate::song::{HashMd5, HashSha256};
use std::collections::HashMap;

pub struct SongData {
    md5_to_sha256: HashMap<HashMd5, HashSha256>,
    sha256_to_md5: HashMap<HashSha256, HashMd5>,
}

impl SongData {
    pub fn new(
        md5_to_sha256: HashMap<HashMd5, HashSha256>,
        sha256_to_md5: HashMap<HashSha256, HashMd5>,
    ) -> SongData {
        SongData { md5_to_sha256, sha256_to_md5 }
    }

    pub fn get_md5(&self, sha256: &HashSha256) -> Option<HashMd5> {
        self.sha256_to_md5.get(&sha256).cloned()
    }

    pub fn get_sha256(&self, md5: &HashMd5) -> Option<HashSha256> {
        self.md5_to_sha256.get(&md5).cloned()
    }
}