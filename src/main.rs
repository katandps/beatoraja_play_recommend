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
use std::process::exit;

fn main() {
    env_logger::init();

    db::player();

    let whole_score = db::score();
    let song_data = db::song_data();
    let score_log = db::score_log();
    let tables = get_tables();

    loop {
        println!("Select table to display!");
        println!("0: Exit");
        for i in 0..tables.len() {
            println!("{}: {}", i + 1, tables.iter().nth(i).unwrap().name());
        }

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        let selected: usize = input.trim().parse().ok().unwrap();

        if selected == 0 {
            break;
        }

        match tables.iter().nth(selected - 1) {
            Some(table) => App {
                table,
                whole_score: &whole_score,
                song_data: &song_data,
                score_log: &score_log,
            }
                .run(),
            _ => (),
        }
    }
}

fn get_tables() -> Vec<Table> {
    dotenv::dotenv().ok();
    (1..10)
        .flat_map(|i| {
            let url = env::var(format!("TABLE_URL{}", i));
            match url {
                Ok(s) => Some(table::make_table(s).unwrap()),
                _ => None,
            }
        })
        .collect()
}
