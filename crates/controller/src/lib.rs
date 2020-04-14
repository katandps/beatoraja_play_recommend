use crate::input::{Input, Table};
use crate::out::Out;
use crate::output::Output;
use model::*;

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
            output: Output::STDOUT,
            input: Input::Parameters(Table { index: 1 }, Command::Recommend),
        }
    }

    pub fn run(&self) -> Out {
        let initial = self.input.out();
        self.output.convert(initial)
    }
}
