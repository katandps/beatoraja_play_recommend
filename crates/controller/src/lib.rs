use crate::input::{Input, Table};
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
    pub fn new() -> Self {
        Controller {
            output: Output::from_str(config().output_type().as_ref()).unwrap(),
            input: Input::Parameters(
                Table {
                    index: config().table_index(),
                },
                Command::Recommend,
            ),
        }
    }

    pub fn run(&self) -> Out {
        let initial = self.input.out();
        self.output.convert(initial)
    }
}

fn config() -> config::Config {
    if cfg!(test) {
        config::Config::Dummy
    } else {
        config::config()
    }
}
