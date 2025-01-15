use crate::*;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Songs {
    pub songs: HashMap<HashSha256, Song>,
    pub converter: Converter,
}

impl Songs {
    pub fn song(&self, chart: &Chart) -> Option<&Song> {
        let sha256 = self.get_sha256(chart.md5());
        sha256.map(|hash| self.songs.get(hash)).flatten()
    }

    pub fn song_by_sha256(&self, sha256: &HashSha256) -> Option<&Song> {
        self.songs.get(sha256)
    }

    pub fn get_md5(&self, sha256: &HashSha256) -> Option<&HashMd5> {
        self.converter.get_md5(sha256)
    }

    pub fn get_sha256(&self, md5: &HashMd5) -> Option<&HashSha256> {
        self.converter.get_sha256(md5)
    }

    pub fn get_list<'a>(&self, chart: impl Iterator<Item = &'a Chart>) -> Vec<SongFormat> {
        chart
            .filter_map(|c| self.song(c))
            .map(SongFormat::from)
            .collect()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct SongFormat {
    title: String,
    notes: i32,
    sha256: HashSha256,
    md5: HashMd5,
}

impl Default for SongFormat {
    fn default() -> Self {
        Self {
            title: "曲データなし".to_string(),
            notes: 0,
            sha256: Default::default(),
            md5: Default::default(),
        }
    }
}

impl From<&Song> for SongFormat {
    fn from(s: &Song) -> Self {
        Self {
            title: s.title(),
            notes: s.notes(),
            sha256: s.get_sha256().clone(),
            md5: s.get_md5().clone(),
        }
    }
}

#[derive(Default)]
pub struct SongsBuilder {
    songs: HashMap<HashSha256, Song>,
    md5_to_sha256: HashMap<HashMd5, HashSha256>,
    sha256_to_md5: HashMap<HashSha256, HashMd5>,
}

impl SongsBuilder {
    pub fn push(
        &mut self,
        md5: HashMd5,
        sha256: HashSha256,
        title: Title,
        artist: Artist,
        notes: i32,
        include_features: IncludeFeatures,
    ) {
        let song = Song::new(
            md5.clone(),
            sha256.clone(),
            title,
            artist,
            notes,
            include_features,
        );
        self.songs.insert(sha256.clone(), song);
        self.sha256_to_md5.insert(sha256.clone(), md5.clone());
        self.md5_to_sha256.insert(md5, sha256);
    }

    pub fn build(self) -> Songs {
        Songs {
            songs: self.songs,
            converter: Converter::new(self.md5_to_sha256, self.sha256_to_md5),
        }
    }
}
