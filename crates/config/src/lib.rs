#[macro_use]
extern crate lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Config {
    Config(Cfg),
    Dummy,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cfg {
    pub timestamp: Option<i32>,
    pub local_cache_url: String,
    pub score_db_url: String,
    pub songdata_db_url: String,
    pub scorelog_db_url: String,
    pub table_urls: Option<Vec<String>>,
    pub table_index: Option<usize>,
    pub recommend_song_number: Option<usize>,
    pub coloring_table: Option<bool>,
    pub slack_bot_token: Option<String>,
    pub slack_channel: Option<String>,
    pub slack_file_name: Option<String>,
}

impl Config {
    pub fn score_db_url(&self) -> String {
        match self {
            Config::Config(cfg) => cfg.score_db_url.clone(),
            _ => "score_db_url".into(),
        }
    }
    pub fn song_db_url(&self) -> String {
        match self {
            Config::Config(cfg) => cfg.songdata_db_url.clone(),
            _ => "song_db_url".into(),
        }
    }
    pub fn scorelog_db_url(&self) -> String {
        match self {
            Config::Config(cfg) => cfg.scorelog_db_url.clone(),
            _ => "scorelog_db_url".into(),
        }
    }
    pub fn local_cache_url(&self) -> String {
        match self {
            Config::Config(cfg) => cfg.local_cache_url.clone(),
            _ => "local_cache_url".into(),
        }
    }
    pub fn timestamp(&self) -> i32 {
        match self {
            Config::Config(cfg) => cfg.timestamp.unwrap_or(1800000000),
            _ => 1800000000,
        }
    }
    pub fn recommend_song_number(&self) -> usize {
        match self {
            Config::Config(cfg) => cfg.recommend_song_number.unwrap_or(3),
            _ => 3,
        }
    }
    pub fn table_urls(&self) -> Vec<String> {
        match self {
            Config::Config(cfg) => cfg.table_urls.clone().unwrap_or(Vec::new()),
            _ => vec!["table_url".into()],
        }
    }
    pub fn table_index(&self) -> usize {
        match self {
            Config::Config(cfg) => cfg.table_index.unwrap_or(0),
            _ => 0,
        }
    }
    pub fn coloring_table(&self) -> bool {
        match self {
            Config::Config(cfg) => cfg.coloring_table.unwrap_or(true),
            _ => true,
        }
    }
    pub fn slack_bot_token(&self) -> String {
        match self {
            Config::Config(cfg) => cfg
                .slack_bot_token
                .clone()
                .expect("Slack bot token is not configure."),
            _ => "slack_bot_token bot token".into(),
        }
    }
    pub fn slack_channel(&self) -> String {
        match self {
            Config::Config(cfg) => cfg
                .slack_channel
                .clone()
                .expect("Slack channel is not configure."),
            _ => "slack_channel".into(),
        }
    }
    pub fn slack_file_name(&self) -> String {
        match self {
            Config::Config(cfg) => cfg
                .slack_file_name
                .clone()
                .expect("Slack file name is not configure"),
            _ => "slack_file_name".into(),
        }
    }
}

pub fn config() -> Config {
    (*self::CONFIG).clone()
}

lazy_static! {
    pub static ref CONFIG: Config = cfg();
}

#[cfg(not(test))]
use std::fs;
#[cfg(not(test))]
fn cfg() -> Config {
    let file = fs::read_to_string("config.toml").unwrap();
    Config::Config(toml::from_str(&file).unwrap())
}

#[cfg(test)]
fn cfg() -> Config {
    Config::Dummy
}
