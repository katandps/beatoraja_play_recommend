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
    Parameters(Table),
    ReloadTable,
}

#[derive(Eq, PartialEq)]
pub struct Table {
    pub index: usize,
}

#[derive(Eq, PartialEq)]
pub enum Mode {
    PlayerStat,
    Recommend,
    RampGraph,
    RankGraph,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            output: Output::JSON,
            input: Input::Parameters(Table { index: 3 }),
        }
    }
}
