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
    let response = DetailResponse {
        user_id: account.user_id(),
        user_name: account.user_name(),
        score: scores.out(&tables, &songs, query.date),
    };
    Ok(serde_json::to_string(&response).unwrap())
}

#[derive(Debug, Clone, Serialize)]
struct DetailResponse {
    user_id: i32,
    user_name: String,
    score: HashMap<HashMd5, SongDetail>,
}
