pub use diesel::prelude::*;

pub struct App {
    pub table: crate::table::Table,
    pub whole_score: crate::score::scores::Scores,
    pub song_data: crate::song_data::SongData,
}

impl App {
    pub fn run(&self) {
        let levels = [
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12",
        ];
        for level in levels.iter() {
            let spec = self.table.level_specified(level.parse().unwrap());
            let scored = self.whole_score.merge_score(&spec, &self.song_data);
            println!("{}", scored.recent_updated())
        }
    }
}
