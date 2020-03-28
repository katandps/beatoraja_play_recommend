mod file;
mod db;

pub mod schema;
pub mod model;
pub mod table;
pub mod whole_score;
pub mod song_data;
pub mod song;

#[macro_use]
extern crate diesel;

pub use diesel::prelude::*;
use crate::whole_score::scores::score::song_id::SongId;
use crate::song::HashSha256;

fn main() {
    let table = file::get_table();
    let ws = db::score();
    let level = "12";
    println!("{}", table.level_specified(level.parse().unwrap()));
    //println!("{}", ws);
    let sha256 = HashSha256::new("cda2f3ff3b6f39b7096dc1b250c6262ba26fb80c88c2a671c3e9612f3718dffe".parse().unwrap());
    let score = ws.get_score(&SongId::new(sha256, 0));

    if score.is_some() {
        println!("{}", score.unwrap());
    }
}