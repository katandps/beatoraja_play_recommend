mod app;
mod db;
mod file;

pub mod lamp;
pub mod schema;
pub mod score;
pub mod score_log;
pub mod scored_table;
pub mod song;
pub mod table;

#[macro_use]
extern crate diesel;
extern crate scraper;

use crate::app::App;
use crate::table::Table;
use std::env;

fn main() {
    env_logger::init();

    db::player();

    let whole_score = db::score();
    let song_data = db::song_data();
    let mut score_log = db::score_log();
    for table in get_tables() {
        App {
            table,
            whole_score: &whole_score,
            song_data: &song_data,
            score_log: &mut score_log,
        }
            .run();
    }
}

fn get_tables() -> Vec<Table> {
    dotenv::dotenv().ok();
    (1..10)
        .flat_map(|i| {
            let url = env::var(format!("TABLE_URL{}", i)).unwrap();
            table::make_table(url)
        })
        .collect()
}
