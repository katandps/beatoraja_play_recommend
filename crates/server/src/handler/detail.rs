use crate::filter::DetailQuery;
use model::*;
use mysql::MySQLClient;
use serde::Serialize;
use std::collections::HashMap;
use warp::{Rejection, Reply};

/// 詳細表示ハンドラ
/// user_idをQueryParameterより取得する
pub async fn detail_handler(
    repos: MySQLClient,
    tables: Tables,
    query: DetailQuery,
    account: Account,
) -> Result<impl Reply, Rejection> {
    let songs = repos.song_data().unwrap_or(SongsBuilder::new().build());
    let scores = repos.score(&account).unwrap_or(Scores::new(HashMap::new()));
    let date = query.date;
    let response = DetailResponse {
        user_id: account.user_id(),
        user_name: account.user_name(),
        score: tables.make_detail(&songs, &scores, &date),
    };
    Ok(serde_json::to_string(&response).unwrap())
}

pub async fn my_detail_handler(
    repos: MySQLClient,
    tables: Tables,
    account: Account,
    query: DetailQuery,
) -> Result<impl Reply, Rejection> {
    let songs = repos.song_data().unwrap_or(SongsBuilder::new().build());
    let scores = repos.score(&account).unwrap_or(Scores::new(HashMap::new()));
    let date = query.date;
    let response = DetailResponse {
        user_id: account.user_id(),
        user_name: account.user_name(),
        score: tables.make_detail(&songs, &scores, &date),
    };
    Ok(serde_json::to_string(&response).unwrap())
}

#[derive(Debug, Clone, Serialize)]
struct DetailResponse {
    user_id: i32,
    user_name: String,
    score: Vec<DetailResult>,
}
