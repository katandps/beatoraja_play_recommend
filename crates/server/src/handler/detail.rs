use crate::filter::DetailQuery;
use crate::filter::{account_id_query, detail_query, with_db, with_song_data, with_table};
use crate::SongData;
use model::Account;
use model::*;
use mysql::MySqlPool;
use repository::ScoresByAccount;
use std::collections::HashMap;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn detail_route(
    db_pool: &MySqlPool,
    tables: &Tables,
    song_data: &SongData,
) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("detail"))
        .and(with_db(db_pool))
        .and(with_table(tables))
        .and(detail_query())
        .and(account_id_query(db_pool))
        .and(with_song_data(song_data))
        .and_then(detail_handler)
        .boxed()
}

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
async fn detail_handler<C: ScoresByAccount>(
    repos: C,
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
