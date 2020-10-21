use crate::out::Out;
use model::*;
use sqlite::SqliteClient;

pub enum Input {
    Interactive,
    Parameters(App<Table<Charts>>, Command),
    ReloadTable,
}

impl Input {
    pub fn out(self) -> Out {
        match self {
            Self::Parameters(mut app, command) => Out::Result(app.out(&command)),
            _ => unreachable!(
                "Interactiveモード及びテーブルの再読み込みは非同期実行の場合のみ可能です"
            ),
        }
    }

    pub async fn out_async(self) -> Out {
        match self {
            Self::Interactive => interactive().await,
            Self::Parameters(mut app, command) => Out::Result(app.out(&command)),
            Self::ReloadTable => reload_table().await,
        }
    }
}

async fn interactive() -> Out {
    let repository = SqliteClient::new();
    sqlite::player();

    let mut tables = table::get_tables(true).await;
    let song_data = repository.song_data();
    let scores = repository.score();

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
            tables = table::get_tables(false).await;
            continue;
        }

        let index: usize = selected.parse().ok().unwrap_or(tables.len() + 1);
        match tables.iter().nth(index) {
            Some(table) => App::new(table.clone(), song_data.clone(), scores.clone()).run(),

            _ => (),
        }
    }
    Out::None
}

pub async fn reload_table() -> Out {
    let _ = table::get_tables(false).await;
    Out::None
}
