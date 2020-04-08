mod app;
mod db;
mod file;

pub mod command;
pub mod config;
pub mod rank;
pub mod schema;
pub mod score;
pub mod score_log;
pub mod scored_table;
pub mod song;
pub mod summary;
pub mod table;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate anyhow;
extern crate scraper;

use crate::app::App;

fn main() {
    db::player();

    let mut tables = table::repository::get_tables(true);
    let whole_score = db::score();
    let song_data = db::song_data();
    let score_log = db::score_log();

    loop {
        println!("Select table to display!\n");
        println!("q: Exit");
        print!("r: Reload tables\n\n");

        for i in 0..tables.len() {
            println!("{}: {}", i, tables.iter().nth(i).unwrap().name());
        }

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        let selected: &str = input.trim();

        if selected == "q" {
            break;
        }
        if selected == "r" {
            tables = table::repository::get_tables(false);
            continue;
        }

        let index: usize = selected.parse().ok().unwrap_or(tables.len() + 1);
        match tables.iter().nth(index) {
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
