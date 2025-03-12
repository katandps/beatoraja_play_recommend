use crate::error::HandleError;
use crate::{filter::*, map_response};
use model::*;
use mysql::MySqlPool;
use repository::SongDataForTables;
use repository::{AccountByUserId, ScoresByAccount};
use table::TableClient;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn table_route() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path!("recommend_table" / i32 / usize / "table.html"))
        .and_then(table_handler)
        .boxed()
}

async fn table_handler(_user_id: i32, _table_index: usize) -> Result<impl Reply, Rejection> {
    let body = r#"
        <html>
          <head>
            <meta name="bmstable" content="header.json">
            <meta http-equiv="Content-Type" content="text/html; charset=rtf-8">
          </head>
          <body>
          おすすめ譜面表
          </body>
        </html>"#;
    Ok(warp::reply::html(body))
}

pub fn header_route(tables: &TableClient) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path!("recommend_table" / i32 / usize / "header.json"))
        .and(with_table(tables))
        .then(service::custom_table::header)
        .then(map_response)
        .boxed()
}

pub fn body_route(db_pool: &MySqlPool, tables: &TableClient) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path!("recommend_table" / i32 / usize / "score.json"))
        .and(with_table(tables))
        .and(with_db(db_pool))
        .and_then(body_handler)
        .boxed()
}

async fn body_handler<C: AccountByUserId + ScoresByAccount + SongDataForTables>(
    user_id: i32,
    table_index: usize,
    tables: TablesInfo,
    mut repos: C,
) -> Result<impl Reply, Rejection> {
    let songs = repos
        .song_data(&tables.tables)
        .await
        .map_err(HandleError::from)?;

    let account = repos.user(user_id).await.map_err(HandleError::from)?;
    let score = repos.score(&account).await.map_err(HandleError::from)?;
    let table = tables.tables.get(table_index).unwrap();
    Ok(serde_json::to_string(&table.filter_score(&score, &songs)).map_err(HandleError::from)?)
}
