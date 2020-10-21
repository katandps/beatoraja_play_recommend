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

    pub fn run(&mut self) {
        println!(
            "{}",
            Command::all()
                .iter()
                .map(|c| format!("{}\n", self.out(c).to_text()))
                .collect::<String>()
        )
    }

    pub fn out(&mut self, command: &Command) -> CommandResult {
        command.func()(
            &self.songs,
            &self.table,
            &self.scores,
            &crate::UpdatedAt::from_timestamp(config().timestamp()),
        )
    }
}
