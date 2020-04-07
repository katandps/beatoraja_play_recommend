mod app;
mod db;
mod file;

pub mod command;
pub mod config;
pub mod lamp;
pub mod rank;
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

fn main() {
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
        let selected: usize = input.trim().parse().ok().unwrap_or(tables.len() + 1);

        if selected == 0 {
            break;
        }

        match tables.iter().nth(selected - 1) {
            Some(table) => App {
                table,
                scores: &whole_score,
                songs: &song_data,
                score_log: &score_log,
            }
                .run(),
            _ => (),
        }
    }
}

fn get_tables() -> Vec<Table> {
    config::table_urls()
        .iter()
        .flat_map(|url| table::make_table(url.parse().unwrap()))
        .collect()
}
