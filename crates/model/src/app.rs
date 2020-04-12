use crate::command::{Command, CommandResult};
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
        println!(
            "{}",
            Command::all()
                .iter()
                .map(|c| format!("{}\n", self.out(c).to_string()))
                .collect::<String>()
        )
    }

    pub fn out(&mut self, command: &Command) -> CommandResult {
        command.func()(
            &self.songs,
            &self.table,
            &self.score_log,
            &UpdatedAt::from_timestamp(crate::config::config().timestamp()),
            &self.table.ls(),
        )
    }
}
