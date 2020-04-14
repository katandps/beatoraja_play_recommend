use crate::*;

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
            &crate::UpdatedAt::from_timestamp(config::config().timestamp()),
            &self.table.ls(),
        )
    }
}
