use crate::error::HandleError;
use crate::filter::{with_db, with_table, RankingQuery};
use crate::TableData;
use chrono::Duration;
use model::*;
use mysql::MySqlPool;
use repository::{PublishedUsers, ScoresBySha256, SongDataForTables};
use std::collections::HashMap;
use std::str::FromStr;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn route(db_pool: &MySqlPool, tables: &TableData) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("ranking"))
        .and(with_db(db_pool))
        .and(with_table(tables))
        .and(warp::query::<HashMap<String, String>>().and_then(parse_ranking_query))
        .and_then(ranking_handler)
        .boxed()
}

async fn parse_ranking_query(query: HashMap<String, String>) -> Result<RankingQuery, Rejection> {
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
    let sha256 = query
        .get("sha256")
        .map(|s| HashSha256::from_str(s).unwrap())
        .unwrap();
    Ok(RankingQuery {
        date,
        play_mode,
        sha256,
    })
}

/// 詳細表示ハンドラ
/// user_idをQueryParameterより取得する
async fn ranking_handler<C: ScoresBySha256 + PublishedUsers + SongDataForTables>(
    mut repos: C,
    tables: TableData,
    query: RankingQuery,
) -> Result<impl Reply, Rejection> {
    let tables_info = tables.lock().await;
    let songs = repos
        .song_data(&tables_info.tables)
        .await
        .map_err(HandleError::from)?;
    let scores = repos
        .score(&query.sha256)
        .await
        .map_err(HandleError::from)?;
    let users = repos.fetch_users().await.map_err(HandleError::from)?;
    let response = scores.for_response(&songs, &query.date, &query.sha256, &users);
    match response {
        Some(res) => Ok(serde_json::to_string(&res).unwrap()),
        None => Ok(serde_json::to_string(&RankingResponse::default()).unwrap()),
    }
}
