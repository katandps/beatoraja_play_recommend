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
        let sha256 = HashSha256::new("cda2f3ff3b6f39b7096dc1b250c6262ba26fb80c88c2a671c3e9612f3718dffe".parse().unwrap());
        let score = self.whole_score.get_score(&SongId::new(sha256, 0));

        if score.is_some() {
            println!("{}", score.unwrap());
        }
    }
}