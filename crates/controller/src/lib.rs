use crate::input::Input;
use crate::out::Out;
use crate::output::Output;
use model::*;
use std::str::FromStr;

mod input;
mod out;
mod output;

pub struct Controller<T> {
    output: Output,
    input: Input<T>,
}

impl Controller<App<Table<Charts>>> {
    pub async fn local() -> Self {
        let repository = sqlite::SqliteClient::new();
        let tables = table::get_tables(true).await;
        let table_index = config().table_index();

        Self::new(
            Output::from_str(config().output_type().as_ref()).unwrap(),
            tables[table_index].clone(),
            repository.song_data(),
            repository.score_log(),
            Command::Recommend,
        )
    }

    pub fn for_server(
        table: Table<Charts>,
        songs: Songs,
        score_log: ScoreLog,
        command: Command,
    ) -> Self {
        Self::new(Output::TEXT, table, songs, score_log, command)
    }

    fn new(
        output: Output,
        table: Table<Charts>,
        songs: Songs,
        score_log: ScoreLog,
        command: Command,
    ) -> Self {
        let input = Input::Parameters(App::new(table, songs, score_log), command);
        Controller { output, input }
    }

    pub fn run(self) -> Out {
        self.output.convert(self.input.out())
    }

    pub async fn run_async(self) -> Out {
        self.output
            .convert_async(self.input.out_async().await)
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
