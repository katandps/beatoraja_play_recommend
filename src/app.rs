use crate::file;
use crate::db;

pub use diesel::prelude::*;
use crate::whole_score::scores::score::song_id::SongId;
use crate::song::HashSha256;

pub struct App {
    pub table: crate::table::Table,
    pub whole_score: crate::whole_score::WholeScore,
    pub song_data: crate::song_data::SongData,
}

impl App {
    pub fn run(&self) {
        let level = "12";
        println!("{}", self.table.level_specified(level.parse().unwrap()));
        //println!("{}", ws);
        let scored_table = self.whole_score.merge_score(&self.table, &self.song_data);
        println!("{}", scored_table)
    }
}