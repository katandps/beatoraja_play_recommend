use crate::*;

#[derive(Clone)]
pub struct App<T> {
    table: T,
    songs: Songs,
    scores: Scores,
}

impl<T: TableTrait> App<T> {
    pub fn new(table: T, songs: Songs, scores: Scores) -> App<T> {
        App {
            table,
            songs,
            scores,
        }
    }
}

pub trait AppTrait: AppRunTrait + AppOutTrait + Clone {}

pub trait AppRunTrait {
    fn run(&mut self);
}

pub trait AppOutTrait {
    fn out(&mut self, command: &Command) -> CommandResult;
}

impl<T: TableTrait + Clone> AppTrait for App<T> {}

impl<T: TableTrait> AppRunTrait for App<T> {
    fn run(&mut self) {
        println!(
            "{}",
            Command::all()
                .iter()
                .map(|c| format!("{}\n", self.out(c).to_text()))
                .collect::<String>()
        )
    }
}

impl<T: TableTrait> AppOutTrait for App<T> {
    fn out(&mut self, command: &Command) -> CommandResult {
        command.func()(
            &self.songs,
            &self.table,
            &self.scores,
            &crate::UpdatedAt::from_timestamp(config().timestamp()),
        )
    }
}
