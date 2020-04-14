use crate::command::{Command, CommandResult};
use crate::controller::Out::Json;
use config::config;
use send_slack::send;

pub struct Controller {
    output: Output,
    input: Input,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            output: Output::SLACK,
            input: Input::Parameters(Table { index: 1 }, Command::Recommend),
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
    SLACK,
}
impl Output {
    fn convert(&self, initial: Out) -> Out {
        match initial {
            Out::Result(r) => match self {
                Self::JSON => match serde_json::to_string(&r) {
                    Ok(j) => Json(j),
                    Err(_) => Out::None,
                },
                Self::STDOUT => {
                    println!("{}", r.to_string());
                    Out::None
                }
                Self::SLACK => {
                    let config = config();
                    let _ = send(
                        config.slack_channel(),
                        config.slack_file_name(),
                        format!("{}", r.to_string()),
                    );
                    Out::None
                }
            },
            _ => Out::None,
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
