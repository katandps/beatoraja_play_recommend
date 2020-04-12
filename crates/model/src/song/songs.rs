use super::*;
use crate::score::prelude::*;
use std::collections::HashMap;

pub struct Songs {
    songs: HashMap<hash::HashSha256, Song>,
    converter: Converter,
}

impl Songs {
    pub fn song(&self, hash: &HashMd5) -> Option<&Song> {
        match self.get_sha256(hash) {
            Some(sha256) => self.songs.get(&sha256),
            _ => None,
        }
    }

    pub fn song_by_sha256(&self, hash: &HashSha256) -> Option<&Song> {
        self.songs.get(hash)
    }

    pub fn get_md5(&self, sha256: &HashSha256) -> Option<HashMd5> {
        self.converter.get_md5(sha256)
    }

    pub fn get_sha256(&self, md5: &HashMd5) -> Option<HashSha256> {
        self.converter.get_sha256(md5)
    }

    pub fn song_id(&self, md5: &HashMd5) -> Option<SongId> {
        match self.get_sha256(md5) {
            Some(s) => Some(SongId::new(s, PlayMode::new(0))),
            _ => None,
        }
    }
}

pub struct SongsBuilder {
    songs: HashMap<hash::HashSha256, Song>,
    md5_to_sha256: HashMap<HashMd5, HashSha256>,
    sha256_to_md5: HashMap<HashSha256, HashMd5>,
}

impl SongsBuilder {
    pub fn new() -> SongsBuilder {
        SongsBuilder {
            songs: HashMap::new(),
            md5_to_sha256: HashMap::new(),
            sha256_to_md5: HashMap::new(),
        }
    }

    pub fn push(
        &mut self,
        md5: HashMd5,
        sha256: HashSha256,
        title: Title,
        artist: Artist,
        notes: i32,
    ) {
        let song = Song {
            hash: sha256.clone(),
            title,
            artist,
            notes,
        };
        self.songs.insert(sha256.clone(), song);
        self.sha256_to_md5.insert(sha256.clone(), md5.clone());
        self.md5_to_sha256.insert(md5, sha256);
    }

    pub fn build(builder: Self) -> Songs {
        let hash_converter = Converter::new(builder.md5_to_sha256, builder.sha256_to_md5);
        Songs {
            songs: builder.songs,
            converter: hash_converter,
        }
    }
}
