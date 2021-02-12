use crate::filter::DetailQuery;
use crate::SongData;
use model::*;
use mysql::MySQLClient;
use repository::ScoresByAccount;
use std::collections::HashMap;
use warp::{Rejection, Reply};

macro_rules! log_duration {
    ($name:expr, $x:expr) => {{
        let start = std::time::Instant::now();
        let result = $x;
        let end = start.elapsed();
        log::info!("{} {}ms", stringify!($name), end.as_millis());
        result
    }};
}

/// 詳細表示ハンドラ
/// user_idをQueryParameterより取得する
pub async fn detail_handler(
    repos: MySQLClient,
    tables: Tables,
    query: DetailQuery,
    account: Account,
    song_data: SongData,
) -> Result<impl Reply, Rejection> {
    let songs = log_duration!(GetSongs, song_data.lock().await);
    let scores = log_duration!(
        GetScores,
        repos
            .score(&account)
            .unwrap_or(Scores::create_by_map(HashMap::new()))
    );
    let response = log_duration!(
        MakeResponse,
        scores.out(&tables, &songs.song, &query.date, &account)
    );
    log_duration!(Serialize, Ok(serde_json::to_string(&response).unwrap()))
}
