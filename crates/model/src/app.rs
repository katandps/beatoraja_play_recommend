use crate::command::Command;
use crate::score::prelude::*;
use crate::score_log::prelude::*;
use crate::song::prelude::*;
use crate::table::prelude::*;

pub struct App<'a> {
    pub table: &'a Table,
    pub songs: &'a Songs,
    pub score_log: &'a ScoreLog,
}

impl<'a> App<'a> {
    pub fn run(&mut self) {
        let mut ret = String::new();
        let levels = self.table.ls();
        let updated_at = UpdatedAt::from_timestamp(crate::config::config().timestamp());

        let all = Command::all();
        for command in &all {
            ret = format!(
                "{}{}\n",
                ret,
                command.func()(
                    &self.songs,
                    &self.table,
                    &self.score_log,
                    &updated_at,
                    &levels
                )
                .to_string()
            )
        }
        println!("{}", ret)
    }

    pub fn out(&mut self, command: &Command) -> String {
        let updated_at = UpdatedAt::from_timestamp(crate::config::config().timestamp());
        format!(
            "{}\n",
            command.func()(
                &self.songs,
                &self.table,
                &self.score_log,
                &updated_at,
                &self.table.ls()
            )
            .to_string()
        )
    }
}
