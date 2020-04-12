use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    timestamp: Option<i32>,
    local_cache_url: String,
    score_db_url: String,
    songdata_db_url: String,
    scorelog_db_url: String,
    table_urls: Option<Vec<String>>,
    recommend_song_number: Option<usize>,
    coloring_table: Option<bool>,
}

impl Config {
    pub fn score_db_url(&self) -> String {
        self.score_db_url.clone()
    }
    pub fn song_db_url(&self) -> String {
        self.songdata_db_url.clone()
    }
    pub fn scorelog_db_url(&self) -> String {
        self.scorelog_db_url.clone()
    }
    pub fn local_cache_url(&self) -> String {
        self.local_cache_url.clone()
    }
    pub fn timestamp(&self) -> i32 {
        self.timestamp.unwrap_or(1800000000)
    }
    pub fn recommend_song_number(&self) -> usize {
        self.recommend_song_number.unwrap_or(3)
    }
    pub fn table_urls(&self) -> Vec<String> {
        self.table_urls.clone().unwrap_or(Vec::new())
    }
    pub fn coloring_table(&self) -> bool {
        self.coloring_table.unwrap_or(true)
    }
}

pub fn config() -> Config {
    let file = fs::read_to_string("./config.toml").unwrap();
    toml::from_str(&file).unwrap()
}
