use crate::*;

pub struct App<'a> {
    pub table: &'a Table,
    pub songs: &'a Songs,
    pub score_log: &'a ScoreLog,
}

impl<'a> App<'a> {
    pub fn new(table: &'a Table, songs: &'a Songs, score_log: &'a ScoreLog) -> App<'a> {
        App {
            table,
            songs,
            score_log,
        }
    }
}

pub trait AppTrait: AppRunTrait + AppOutTrait {}

pub trait AppRunTrait {
    fn run(&mut self);
}

pub trait AppOutTrait {
    fn out(&mut self, command: &Command) -> CommandResult;
}

impl AppRunTrait for App<'_> {
    fn run(&mut self) {
        println!(
            "{}",
            Command::all()
                .iter()
                .map(|c| format!("{}\n", self.out(c).to_string()))
                .collect::<String>()
        )
    }
}

impl AppOutTrait for App<'_> {
    fn out(&mut self, command: &Command) -> CommandResult {
        command.func()(
            &self.songs,
            &self.table,
            &self.score_log,
            &crate::UpdatedAt::from_timestamp(config().timestamp()),
            &self.table.levels(),
        )
    }
}
