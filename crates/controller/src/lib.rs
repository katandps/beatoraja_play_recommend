use crate::input::Input;
use crate::out::Out;
use crate::output::Output;
use model::*;
use std::str::FromStr;

mod input;
mod out;
mod output;

pub struct Controller {
    output: Output,
    input: Input,
}

impl Controller {
    pub async fn local() -> Self {
        let repository = sqlite::SqliteClient::new();
        let tables = table::get_tables(true).await;
        let table_index = config().table_index();

        Self::new(
            Output::from_str(config().output_type().as_ref()).unwrap(),
            tables[table_index].clone(),
            repository.song_data(),
            repository.score(),
            Command::Detail,
        )
    }

    pub fn for_server(
        table: Table<Charts>,
        songs: Songs,
        scores: Scores,
        command: Command,
    ) -> Self {
        Self::new(Output::JSON, table, songs, scores, command)
    }

    fn new(
        output: Output,
        table: Table<Charts>,
        songs: Songs,
        scores: Scores,
        command: Command,
    ) -> Self {
        let input = Input::Parameters(App::new(table, songs, scores), command);
        Controller { output, input }
    }

    pub fn run(self) -> Out {
        self.input.out().convert(self.output)
    }

    pub async fn run_async(self) -> Out {
        self.input
            .out_async()
            .await
            .convert_async(self.output)
            .await
    }
}

fn config() -> config::Config {
    if cfg!(test) {
        config::Config::Dummy
    } else {
        config::config()
    }
}
