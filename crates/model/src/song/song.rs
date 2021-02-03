use crate::song::include_features::IncludeFeatures;
use crate::*;

///
/// 楽曲データ
/// 所持しているのでsha256がわかっている
///
#[derive(Clone, Debug)]
pub struct Song {
    md5: HashMd5,
    sha256: HashSha256,
    title: Title,
    artist: Artist,
    notes: i32,
    include_features: IncludeFeatures,
}

impl Song {
    pub fn new(
        md5: HashMd5,
        sha256: HashSha256,
        title: Title,
        artist: Artist,
        notes: i32,
        include_features: IncludeFeatures,
    ) -> Song {
        Song {
            md5,
            sha256,
            title,
            artist,
            notes,
            include_features,
        }
    }

    pub fn make_from_chart(chart: &Chart) -> Song {
        Song {
            md5: chart.md5().clone(),
            sha256: HashSha256::default(),
            title: chart.title(),
            artist: chart.artist(),
            notes: 0,
            include_features: IncludeFeatures::from(0),
        }
    }

    pub fn song_id(&self) -> ScoreId {
        ScoreId::new(self.sha256.clone(), PlayMode::new(0))
    }

    pub fn title(&self) -> String {
        self.title.to_string()
    }
    pub fn artist(&self) -> String {
        self.artist.to_string()
    }
    pub fn notes(&self) -> i32 {
        self.notes
    }
    pub fn get_md5(&self) -> &HashMd5 {
        &self.md5
    }
    pub fn get_sha256(&self) -> &HashSha256 {
        &self.sha256
    }
    pub fn features(&self) -> &IncludeFeatures {
        &self.include_features
    }
}
