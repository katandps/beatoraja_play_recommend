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
    pub fn local() -> Self {
        let repository = sqlite::SqliteClient::new();
        let tables = table::get_tables(true);
        let table_index = config().table_index();

        Controller {
            output: Output::from_str(config().output_type().as_ref()).unwrap(),
            input: Input::Parameters(
                App::new(
                    tables[table_index].clone(),
                    repository.song_data(),
                    repository.score_log(),
                ),
                Command::Recommend,
            ),
        }
    }

    pub fn for_server(
        table: Table<Charts>,
        songs: Songs,
        score_log: ScoreLog,
        command: Command,
    ) -> Self {
        Controller {
            output: Output::TEXT,
            input: Input::Parameters(App::new(table, songs, score_log), command),
        }
    }

    pub fn run(&self) -> Out {
        self.output.convert(self.input.out())
    }
}

fn config() -> config::Config {
    if cfg!(test) {
        config::Config::Dummy
    } else {
        config::config()
    }
}
