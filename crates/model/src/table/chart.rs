use crate::*;
use serde::{Deserialize, Serialize};

///
/// 難易度表上の楽曲データ
/// 楽曲データベース上に存在するとは限らない
///
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chart {
    title: Title,
    artist: Artist,
    md5: HashMd5,
    pub(super) level: Level,
}

impl Chart {
    pub fn new(title: String, artist: String, md5: HashMd5, level: String) -> Chart {
        Chart {
            title: Title::new(title),
            artist: Artist::new(artist),
            md5,
            level: Level::make(level),
        }
    }

    pub fn matched_song(&self, songs: &Songs) -> Song {
        match songs.song(&self.md5) {
            Some(s) => s.clone(),
            None => Song::make_from_chart(&self),
        }
    }

    pub fn title(&self) -> Title {
        self.title.clone()
    }
    pub fn artist(&self) -> Artist {
        self.artist.clone()
    }
    pub fn level(&self) -> Level {
        self.level.clone()
    }
}
