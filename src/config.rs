use std::env;
use std::str::FromStr;

pub struct Config {
    pub timestamp: i32,
}

pub fn config() -> Config {
    dotenv::dotenv().ok();
    let timestamp = i32::from_str(env::var("TIMESTAMP").unwrap().as_ref()).unwrap();
    Config { timestamp }
}
