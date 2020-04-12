use crate::command::{Command, CommandResult};
use crate::controller::Out::Json;

pub struct Controller {
    output: Output,
    input: Input,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            output: Output::STDOUT,
            input: Input::Parameters(Table { index: 1 }, Command::RankGraph),
        }
    }

    pub fn run(&self) -> Out {
        let initial = self.input.out();
        self.output.convert(initial)
    }
}

#[derive(Eq, PartialEq)]
pub enum Input {
    Interactive,
    Parameters(Table, Command),
    ReloadTable,
}

impl Input {
    pub fn out(&self) -> Out {
        match self {
            Self::Interactive => crate::interactive(),
            Self::Parameters(table, command) => crate::parameters(table, command),
            Self::ReloadTable => crate::reload_table(),
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct Table {
    pub index: usize,
}

pub enum Output {
    JSON,
    STDOUT,
}
impl Output {
    fn convert(&self, initial: Out) -> Out {
        match self {
            Self::JSON => match initial {
                Out::Result(r) => match serde_json::to_string(&r) {
                    Ok(j) => Json(j),
                    Err(_) => Out::None,
                },
                Out::Json(j) => Out::Json(j),
                _ => Out::None,
            },
            Self::STDOUT => match initial {
                Out::Result(r) => {
                    println!("{}", r.to_string());
                    Out::None
                }
                _ => Out::None,
            },
        }
    }
}

pub enum Out {
    None,
    Json(String),
    Result(CommandResult),
}

impl Out {
    pub fn to_string(&self) -> String {
        match self {
            Out::Json(s) => s.into(),
            Out::Result(cr) => cr.to_string(),
            _ => "".into(),
        }
    }
}
