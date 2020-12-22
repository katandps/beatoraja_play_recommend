#[macro_use]
extern crate lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Config {
    Config(Cfg),
    Dummy,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cfg {
    pub timestamp: Option<i32>,
    pub local_cache_url: Option<String>,
    pub mysql_url: Option<String>,
    pub song_db_url: Option<String>,
    pub score_db_url: Option<String>,
    pub songdata_db_url: Option<String>,
    pub scorelog_db_url: Option<String>,
    pub table_urls: Option<Vec<String>>,
    pub table_index: Option<usize>,
    pub coloring_table: Option<bool>,
    pub google_oauth_client_id: Option<String>,
    pub slack_bot_token: Option<String>,
    pub slack_channel: Option<String>,
    pub slack_file_name: Option<String>,
    pub output_type: Option<String>,
}

macro_rules! string_config {
    ($name:ident, $default:expr) => {
        pub fn $name(&self) -> String {
            match self {
                Self::Config(cfg) => cfg.$name.clone().unwrap_or(($default).into()),
                _ => ($default).into(),
            }
        }
    };
}

impl Config {
    string_config!(mysql_url, "mysql://root:root@127.0.0.1/user_data");
    string_config!(score_db_url, "score_db_url");
    string_config!(song_db_url, "song_db_url");
    string_config!(scorelog_db_url, "scorelog_db_url");
    string_config!(local_cache_url, "local_cache_url");
    string_config!(slack_bot_token, "Slack bot token is not configure.");
    string_config!(slack_channel, "Slack channel is not configure.");
    string_config!(slack_file_name, "Slack file name is not configure");
    string_config!(output_type, "STDOUT");
    string_config!(
        google_oauth_client_id,
        "hogehoge.apps.googleusercontent.com"
    );

    pub fn timestamp(&self) -> i32 {
        match self {
            Config::Config(cfg) => cfg.timestamp.unwrap_or(1800000000),
            _ => 1800000000,
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
}

pub fn config() -> Config {
    (*self::CONFIG).clone()
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let file = fs::read_to_string("config.toml").unwrap_or("".to_string());
        Config::Config(toml::from_str(&file).unwrap())
    };
}
