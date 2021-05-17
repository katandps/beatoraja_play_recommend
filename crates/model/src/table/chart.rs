use crate::*;

///
/// 難易度表上の楽曲データ
/// 楽曲データベース上に存在するとは限らない
///
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Chart {
    title: String,
    artist: Option<String>,
    md5: HashMd5,
    level: String,
    url: Option<String>,
    url_diff: Option<String>,
    comment: Option<String>,
}

impl Chart {
    pub fn new(
        title: String,
        artist: Option<String>,
        md5: HashMd5,
        level: String,
        url: Option<String>,
        url_diff: Option<String>,
        comment: Option<String>,
    ) -> Self {
        Self {
            title,
            artist,
            md5,
            level,
            url,
            url_diff,
            comment,
        }
    }

    pub fn title(&self) -> Title {
        Title::new(self.title.clone())
    }
    pub fn artist(&self) -> Artist {
        Artist::new(self.artist.clone().unwrap_or("".to_string()))
    }
    pub fn level(&self) -> Level {
        Level::make(self.level.clone())
    }
    pub fn md5(&self) -> &HashMd5 {
        &self.md5
    }
}
