pub use diesel::prelude::*;

pub struct App {
    pub table: crate::table::Table,
    pub whole_score: crate::whole_score::WholeScore,
    pub song_data: crate::song_data::SongData,
}

impl App {
    pub fn run(&self) {
        let mut levels = Vec::new();
        levels.push("0");
        levels.push("1");
        levels.push("2");
        levels.push("3");
        levels.push("4");
        levels.push("5");
        levels.push("6");
        levels.push("7");
        levels.push("8");
        levels.push("9");
        levels.push("10");
        levels.push("11");
        levels.push("12");

        for level in levels {
            let spec = self.table.level_specified(level.parse().unwrap());
            let scored = self.whole_score.merge_score(&spec, &self.song_data);
            println!("{}", scored.recent_updated())
        }
    }
}