mod app;
mod db;
mod file;

pub(crate) mod command;
pub(crate) mod config;
pub(crate) mod controller;
pub(crate) mod rank;
pub(crate) mod schema;
pub(crate) mod score;
pub(crate) mod score_log;
pub(crate) mod scored_table;
pub(crate) mod song;
pub(crate) mod summary;
pub(crate) mod table;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate anyhow;
extern crate scraper;

use crate::app::App;
pub use crate::controller::Controller;
use crate::controller::{Input, Table};

pub fn run(controller: Controller) -> String {
    match controller.input {
        Input::Interactive => interactive(),
        Input::Parameters(t) => parameters(t),
        Input::ReloadTable => reload_table(),
    }
}

fn interactive() -> String {
    db::player();

    let mut tables = table::repository::get_tables(true);
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
                songs: &song_data,
                score_log: &score_log,
            }
            .run(),

            _ => (),
        }
    }
    "done".into()
}

fn parameters(table: Table) -> String {
    let mut tables = table::repository::get_tables(true);
    let song_data = db::song_data();
    let score_log = db::score_log();

    let table_index = table.index;
    match tables.iter().nth(table_index) {
        Some(table) => App {
            table,
            songs: &song_data,
            score_log: &score_log,
        }
        .out(),

        _ => "".into(),
    }
}

fn reload_table() -> String {
    let _ = table::repository::get_tables(false);
    "done".into()
}
