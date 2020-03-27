mod file;
mod db;

pub mod schema;
pub mod model;
pub mod table;
pub mod whole_score;

#[macro_use]
extern crate diesel;

pub use diesel::prelude::*;

fn main() {
    let table = file::get_table();
    println!("{}", table.string());
    let ws = db::score();
    println!("{}", ws.count())
}