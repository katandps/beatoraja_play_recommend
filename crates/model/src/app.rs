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
        println!("{}", self.out());
    }

    pub fn out(&mut self) -> String {
        let mut ret = String::new();

        ret = format!("{}{}", ret, self.table.name());

        let levels = self.table.ls();

        let updated_at = UpdatedAt::from_timestamp(crate::config::config().timestamp());

        let all = Command::all();
        for command in &all {
            ret = format!(
                "{}{}\n",
                ret,
                levels.format(
                    command,
                    &self.songs,
                    &self.table,
                    &self.score_log,
                    &updated_at,
                )
            )
        }
        ret
    }
}
