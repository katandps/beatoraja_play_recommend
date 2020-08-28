use crate::out::Out;
use model::*;
use sqlite::SqliteClient;

#[derive(Eq, PartialEq)]
pub enum Input<T> {
    Interactive,
    Parameters(T, Command),
    ReloadTable,
}

impl<T> Input<T>
where
    T: AppTrait,
{
    pub fn out(self) -> Out {
        match self {
            Self::Interactive => interactive(),
            Self::Parameters(mut app, command) => Out::Result(app.out(&command)),
            Self::ReloadTable => reload_table(),
        }
    }
}

fn interactive() -> Out {
    let repository = SqliteClient::new();
    sqlite::player();

    let mut tables = table::get_tables(true);
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
            Some(table) => App::new(table.clone(), song_data.clone(), score_log.clone()).run(),

            _ => (),
        }
    }
    Out::None
}

pub fn reload_table() -> Out {
    let _ = table::get_tables(false);
    Out::None
}
