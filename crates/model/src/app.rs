use crate::*;

pub struct App<'a, T: TableTrait> {
    pub table: &'a T,
    pub songs: &'a Songs,
    pub score_log: &'a ScoreLog,
}

impl<'a, T> App<'a, T>
where
    T: TableTrait,
{
    pub fn new(table: &'a T, songs: &'a Songs, score_log: &'a ScoreLog) -> App<'a, T> {
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

impl<'a, T> AppRunTrait for App<'a, T>
where
    T: TableTrait,
{
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

impl<'a, T> AppOutTrait for App<'a, T>
where
    T: TableTrait,
{
    fn out(&mut self, command: &Command) -> CommandResult {
        command.func()(
            self.songs,
            self.table,
            self.score_log,
            &crate::UpdatedAt::from_timestamp(config().timestamp()),
        )
    }
}
