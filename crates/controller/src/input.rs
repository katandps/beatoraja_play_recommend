use crate::out::Out;
use model::*;
use play_data::SqliteClient;
use table::get_tables;

#[derive(Eq, PartialEq)]
pub enum Input {
    Interactive,
    Parameters(Table, Command),
    ReloadTable,
}

impl Input {
    pub fn out(&self) -> Out {
        match self {
            Self::Interactive => interactive(),
            Self::Parameters(table, command) => parameters(table, command),
            Self::ReloadTable => reload_table(),
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct Table {
    pub index: usize,
}

fn interactive() -> Out {
    let repository = SqliteClient::new();
    play_data::player();

    let mut tables = get_tables(true);
    let song_data = repository.song_data();
    let score_log = repository.score_log();

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
            Some(table) => App::new(table, &song_data, &score_log).run(),

            _ => (),
        }
    }
    Out::None
}

fn parameters(table: &Table, command: &Command) -> Out {
    let repository = SqliteClient::new();
    let tables = get_tables(true);
    let song_data = repository.song_data();
    let score_log = repository.score_log();

    let table_index = table.index;
    let res = match tables.iter().nth(table_index) {
        Some(table) => Some(App::new(table, &song_data, &score_log).out(command)),
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
