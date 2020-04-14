use config::config;
use model::command::{Command, CommandResult};
use model::App;
use send_slack::send;
use table::*;

pub fn interactive() -> Out {
    play_data::player();

    let mut tables = get_tables(true);
    let song_data = play_data::song_data();
    let score_log = play_data::score_log();

    loop {
        println!("Select table to display!\n");
        println!("q: Exit");
        print!("r: Reload tables\n\n");

        for i in 0..tables.len() {
            println!("{}: {}", i, tables.iter().nth(i).unwrap().name());
        }

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        let selected: &str = input.trim();

        if selected == "q" {
            break;
        }
        if selected == "r" {
            tables = table::get_tables(false);
            continue;
        }

        let index: usize = selected.parse().ok().unwrap_or(tables.len() + 1);
        match tables.iter().nth(index) {
            Some(table) => App {
                table,
                songs: &song_data,
                score_log: &score_log,
            }
            .run(),

            _ => (),
        }
    }
    Out::None
}

pub fn parameters(table: &Table, command: &Command) -> Out {
    let tables = get_tables(true);
    let song_data = play_data::song_data();
    let score_log = play_data::score_log();

    let table_index = table.index;
    let res = match tables.iter().nth(table_index) {
        Some(table) => Some(
            App {
                table,
                songs: &song_data,
                score_log: &score_log,
            }
            .out(command),
        ),
        _ => None,
    };
    match res {
        Some(command_result) => Out::Result(command_result),
        _ => Out::None,
    }
}

pub fn reload_table() -> Out {
    let _ = get_tables(false);
    Out::None
}

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
                    Ok(j) => Out::Json(j),
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
