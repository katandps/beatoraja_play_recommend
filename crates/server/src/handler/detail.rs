use crate::error::HandleError::{
    AccountIsNotFound, AccountIsNotSelected, AccountSelectionIsInvalid, OtherError,
};
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
    query: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    let user_id = query
        .get(&"user_id".to_string())
        .ok_or(AccountIsNotSelected.rejection())?;
    let user_id = user_id
        .parse::<i32>()
        .map_err(|e| AccountSelectionIsInvalid(e).rejection())?;
    let account = repos
        .account_by_increments(user_id)
        .map_err(|e| AccountIsNotFound(e).rejection())?;
    let songs = repos.song_data().unwrap_or(SongsBuilder::new().build());
    let scores = repos.score(&account).unwrap_or(Scores::new(HashMap::new()));
    let date = super::date(&query);
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
    session_key: String,
    query: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    let account = crate::session::get_account_by_session(&repos, &session_key)
        .map_err(|e| OtherError(e).rejection())?;
    let songs = repos.song_data().unwrap_or(SongsBuilder::new().build());
    let scores = repos.score(&account).unwrap_or(Scores::new(HashMap::new()));
    let date = super::date(&query);
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
