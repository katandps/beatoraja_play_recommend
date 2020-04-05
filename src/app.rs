use crate::lamp::LampSum;
use crate::score::song_id::{PlayMode, SongId};
use crate::score::updated_at::UpdatedAt;
pub use diesel::prelude::*;

pub struct App<'a> {
    pub table: crate::table::Table,
    pub whole_score: &'a crate::score::scores::Scores,
    pub song_data: &'a crate::song::hash_converter::Converter,
    pub score_log: &'a mut crate::score_log::ScoreLog,
}

impl<'a> App<'a> {
    pub fn run(&mut self) {
        println!("{}", self.table.name());

        let levels = [
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12",
        ];
        for level in levels.iter() {
            let spec = self.table.level_specified(level.parse().unwrap());
            let scored = self.whole_score.merge_score(&spec, &self.song_data);
            println!("{}", scored.recent_updated());

            //let level = "1";
            let spec = self.table.level_specified(level.parse().unwrap());
            let updated_at = UpdatedAt::from_timestamp(2000000000);
            let mut lamp_sum = LampSum::new();
            for c in spec.get_charts().iter() {
                let sha256 = self.song_data.get_sha256(&c.md5);
                if sha256.is_none() {
                    continue;
                }
                let song_id = SongId::new(sha256.unwrap(), PlayMode::new(0));

                let snap = self.score_log.get_snap(song_id, &updated_at);
                lamp_sum.push(&snap);
                //println!("{:?}", snap)
            }
            //println!("{}", lamp_sum.format());
        }
        println!();
    }
}
