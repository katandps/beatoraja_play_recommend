use std::env;
use std::str::FromStr;

pub struct Config {
    pub timestamp: i32,
    pub score_db_url: String,
    pub song_db_url: String,
    pub scorelog_db_url: String,
}

pub fn config() -> Config {
    dotenv::dotenv().ok();
    let timestamp = i32::from_str(env::var("TIMESTAMP").unwrap().as_ref()).unwrap();
    let score_db_url = env::var("SCORE_DATABASE_URL").unwrap();
    let song_db_url = env::var("SONG_DATABASE_URL").unwrap();
    let scorelog_db_url = env::var("SCORELOG_DATABASE_URL").unwrap();
    Config {
        timestamp,
        score_db_url,
        song_db_url,
        scorelog_db_url,
    }
}

pub fn table_urls() -> Vec<String> {
    dotenv::dotenv().ok();
    (1..10)
        .flat_map(|i| env::var(format!("TABLE_URL{}", i)))
        .collect()
}
