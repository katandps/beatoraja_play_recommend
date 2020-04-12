use crate::command::Command;
use crate::score::updated_at::UpdatedAt;
pub use diesel::prelude::*;

pub struct App<'a> {
    pub table: &'a crate::table::Table,
    pub songs: &'a crate::song::Songs,
    pub score_log: &'a crate::score_log::ScoreLog,
}

impl<'a> App<'a> {
    pub fn run(&mut self) {
        println!("{}", self.table.name());

        let levels = self.table.ls();

        let updated_at = UpdatedAt::from_timestamp(crate::config::config().timestamp());

        let all = Command::all();
        for command in &all {
            println!(
                "{}\n",
                levels.format(
                    command,
                    &self.songs,
                    &self.table,
                    &self.score_log,
                    &updated_at,
                )
            )
        }
    }
}
