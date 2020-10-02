pub fn take(
    table: Table<Charts>,
    songs: Songs,
    scores: Scores,
    score_log: ScoreLog,
    command: Command,
) -> String {
    controller::Controller::for_server(table, songs, scores, score_log, command)
        .run()
        .to_string()
}

pub async fn get_tables() -> Vec<Table<Charts>> {
    table::get_tables(false).await
}

pub use model::*;
pub use sqlite::*;
