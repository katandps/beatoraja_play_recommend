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

        let recommend: String = levels
            .iter()
            .map(|level| self.recommend(&self.table.level_specified(&level)))
            .collect();
        println!("Recommend\n{}", recommend);

        let updated_at = UpdatedAt::from_timestamp(crate::config::config().timestamp);

        let lamp_chart: String = levels
            .iter()
            .map(|level| self.lamp_chart(&self.table.level_specified(&level), &updated_at))
            .collect();
        println!("Lamp chart\n{}", lamp_chart);

        let rank_chart: String = levels
            .iter()
            .map(|level| self.rank_chart(&self.table.level_specified(&level), &updated_at))
            .collect();
        println!("Rank chart\n{}", rank_chart);
    }

    fn recommend(&self, table: &crate::table::Table) -> String {
        let scored = table.merge_score(&self.whole_score, &self.song_data);
        format!("{}\n", scored.recent_updated())
    }

    fn lamp_chart(&self, table: &crate::table::Table, updated_at: &UpdatedAt) -> String {
        let mut lamp_sum = LampSum::new();
        for s in table.get_song(&self.song_data).iter() {
            lamp_sum.push(self.score_log.get_snap(&s.song_id(), &updated_at).borrow())
        }
        lamp_sum.format() + "\n"
    }

    fn rank_chart(&self, table: &crate::table::Table, updated_at: &UpdatedAt) -> String {
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
        rank_sum.format() + "\n"
    }
}
