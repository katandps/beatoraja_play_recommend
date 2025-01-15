use crate::error::HandleError;
use crate::filter::DetailQuery;
use crate::filter::{account_id_query, with_db, with_table};
use crate::TableData;
use chrono::Duration;
use model::Account;
use model::*;
use mysql::MySqlPool;
use repository::{ScoresByAccount, SongDataForTables};
use std::collections::HashMap;
use std::str::FromStr;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn route(db_pool: &MySqlPool, tables: &TableData) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("detail"))
        .and(with_db(db_pool))
        .and(with_table(tables))
        .and(warp::query::<HashMap<String, String>>().and_then(parse_detail_query))
        .and(account_id_query(db_pool))
        .and_then(handler)
        .boxed()
}

async fn parse_detail_query(query: HashMap<String, String>) -> Result<DetailQuery, Rejection> {
    let date = query
        .get("date")
        .map(|u| {
            UpdatedAt::from_str(u)
                .map(|u| &u - Duration::days(-1))
                .unwrap_or_else(|_| UpdatedAt::default())
        })
        .unwrap_or_default();
    let play_mode = if let Some(mode) = query.get("mode") {
        match mode.parse::<i32>() {
            Ok(mode) => PlayMode::from(mode),
            Err(_) => PlayMode::default(),
        }
    } else {
        PlayMode::default()
    };
    Ok(DetailQuery { date, play_mode })
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
async fn handler<C: ScoresByAccount + SongDataForTables>(
    mut repos: C,
    tables: TableData,
    query: DetailQuery,
    account: Account,
) -> Result<impl Reply, Rejection> {
    let tables = tables.lock().await;
    let songs = repos
        .song_data(&tables.tables)
        .await
        .map_err(HandleError::from)?;
    let scores = log_duration!(
        GetScores,
        repos
            .score(&account)
            .await
            .unwrap_or_else(|_| Scores::create_by_map(HashMap::new()))
    );
    let response = log_duration!(
        MakeResponse,
        scores.table_scores(&tables.tables, &songs, &query.date, &account)
    );
    log_duration!(Serialize, Ok(serde_json::to_string(&response).unwrap()))
}
