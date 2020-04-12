use crate::command::Command;

pub struct Controller {
    pub output: Output,
    pub input: Input,
}
pub enum Output {
    JSON,
    STDOUT,
}
#[derive(Eq, PartialEq)]
pub enum Input {
    Interactive,
    Parameters(Table, Command),
    ReloadTable,
}

#[derive(Eq, PartialEq)]
pub struct Table {
    pub index: usize,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            output: Output::JSON,
            input: Input::Parameters(Table { index: 1 }, Command::Recommend),
        }
    }
}
