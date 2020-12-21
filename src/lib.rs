pub use config::*;
pub use model::*;
pub use sqlite::*;

pub fn take(
    table: Table<Charts>,
    songs: Songs,
    scores: Scores,
    command: Command,
    date: UpdatedAt,
) -> String {
    controller::Controller::for_server(table, songs, scores, command)
        .run(date)
        .to_string()
}

pub async fn get_tables() -> Vec<Table<Charts>> {
    table::get_tables(false).await
}
