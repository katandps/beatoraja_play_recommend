mod file;
mod db;

pub mod schema;
pub mod model;
pub mod table;
pub mod whole_score;

#[macro_use]
extern crate diesel;

pub use diesel::prelude::*;
use crate::whole_score::scores::score::song_id::SongId;

fn main() {
    let table = file::get_table();
    let ws = db::score();
    println!("{}", ws);
    let score = ws.get_score(&SongId::new("cda2f3ff3b6f39b7096dc1b250c6262ba26fb80c88c2a671c3e9612f3718dffe".to_string(), 0));

    if score.is_some() {
        println!("{}", score.unwrap());
    }
}