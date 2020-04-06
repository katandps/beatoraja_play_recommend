use crate::lamp::LampSum;
use crate::rank::RankSum;
use crate::score::updated_at::UpdatedAt;
use crate::song::SongWithSnap;
pub use diesel::prelude::*;
use std::borrow::Borrow;

pub struct App<'a> {
    pub table: &'a crate::table::Table,
    pub whole_score: &'a crate::score::scores::Scores,
    pub song_data: &'a crate::song::Songs,
    pub score_log: &'a crate::score_log::ScoreLog,
}

impl<'a> App<'a> {
    pub fn run(&mut self) {
        println!("{}", self.table.name());

        let levels = self.table.get_levels();
        for level in levels.iter() {
            let spec = &self.table.level_specified(&level);
            self.recommend(spec);
        }
        println!("\nLamp chart");

        let updated_at = UpdatedAt::from_timestamp(crate::config::config().timestamp);

        for level in levels.iter() {
            let spec = &self.table.level_specified(&level);
            self.lamp_chart(spec, &updated_at);
        }
        println!("\nRank chart");
        for level in levels.iter() {
            let spec = &self.table.level_specified(&level);
            self.rank_chart(spec, &updated_at);
        }
    }
    fn recommend(&self, table: &crate::table::Table) {
        let scored = self.whole_score.merge_score(table, &self.song_data);
        println!("{}", scored.recent_updated());
    }

    fn lamp_chart(&self, table: &crate::table::Table, updated_at: &UpdatedAt) {
        let mut lamp_sum = LampSum::new();
        for s in table.get_song(&self.song_data).iter() {
            lamp_sum.push(self.score_log.get_snap(&s.song_id(), &updated_at).borrow())
        }
        println!("{}", lamp_sum.format());
    }

    fn rank_chart(&self, table: &crate::table::Table, updated_at: &UpdatedAt) {
        let mut rank_sum = RankSum::new();
        for s in table.get_song(&self.song_data).iter() {
            rank_sum.push(
                SongWithSnap::make(
                    &s,
                    self.score_log.get_snap(&s.song_id(), &updated_at).borrow(),
                )
                    .borrow(),
            )
        }
        println!("{}", rank_sum.format());
    }
}
