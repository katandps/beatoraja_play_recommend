use crate::lamp::LampSum;
use crate::rank::RankSum;
use crate::score::scores::Scores;
use crate::score::updated_at::UpdatedAt;
use crate::score_log::ScoreLog;
use crate::song::{SongWithSnap, Songs};
use crate::table::Table;
pub use diesel::prelude::*;
use std::borrow::Borrow;

pub struct App<'a> {
    pub table: &'a crate::table::Table,
    pub scores: &'a crate::score::scores::Scores,
    pub songs: &'a crate::song::Songs,
    pub score_log: &'a crate::score_log::ScoreLog,
}

impl<'a> App<'a> {
    pub fn run(&mut self) {
        println!("{}", self.table.name());

        let levels = self.table.ls();

        let updated_at = UpdatedAt::from_timestamp(crate::config::config().timestamp);
        let recommend = levels.format(
            Self::recommend(),
            &self.scores,
            &self.songs,
            &self.table,
            &self.score_log,
            &updated_at,
        );
        println!("Recommend\n{}", recommend);

        let lamp_chart = levels.format(
            Self::lamp(),
            &self.scores,
            &self.songs,
            &self.table,
            &self.score_log,
            &updated_at,
        );
        println!("Lamp chart\n{}", lamp_chart);

        let rank_chart = levels.format(
            Self::rank(),
            &self.scores,
            &self.songs,
            &self.table,
            &self.score_log,
            &updated_at,
        );
        println!("Rank chart\n{}", rank_chart);
    }

    fn recommend() -> impl Fn(&Scores, &Songs, &Table, &ScoreLog, &UpdatedAt) -> String {
        |scores, songs, table, _, _| {
            format!("{}\n", table.merge_score(scores, songs).recent_updated())
        }
    }

    fn lamp() -> impl Fn(&Scores, &Songs, &Table, &ScoreLog, &UpdatedAt) -> String {
        |_, songs, table, score_log, updated_at| {
            let mut lamp_sum = LampSum::new();
            for s in table.get_song(songs).iter() {
                lamp_sum.push(score_log.get_snap(&s.song_id(), &updated_at).borrow())
            }
            format!("{}", lamp_sum)
        }
    }

    fn rank() -> impl Fn(&Scores, &Songs, &Table, &ScoreLog, &UpdatedAt) -> String {
        |_, songs, table, score_log, updated_at| {
            let mut rank_sum = RankSum::new();
            for song in table.get_song(songs).iter() {
                rank_sum.push(
                    SongWithSnap::make(
                        &song,
                        score_log.get_snap(&song.song_id(), &updated_at).borrow(),
                    )
                        .borrow(),
                )
            }
            format!("{}", rank_sum)
        }
    }
}
