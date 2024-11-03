use crate::error::HandleError;
use crate::filter::*;
use crate::SongData;
use crate::TableData;
use model::*;
use mysql::MySqlPool;
use repository::{AccountByUserId, ScoresByAccount};
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

pub fn header_route(tables: &TableData) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path!("recommend_table" / i32 / usize / "header.json"))
        .and(with_table(tables))
        .and_then(header_handler)
        .boxed()
}

async fn header_handler(
    _user_id: i32,
    table_index: usize,
    tables: TableData,
) -> Result<impl Reply, Rejection> {
    let tables = tables.lock().await;
    let table = tables.get(table_index).unwrap();
    let header =
        &CustomTableHeader::from(table).set_name(format!("おすすめ譜面表: {}", table.title()));
    Ok(serde_json::to_string(&header).unwrap())
}

pub fn body_route(
    db_pool: &MySqlPool,
    tables: &TableData,
    song_data: &SongData,
) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path!("recommend_table" / i32 / usize / "score.json"))
        .and(with_table(tables))
        .and(with_db(db_pool))
        .and(with_song_data(song_data))
        .and_then(body_handler)
        .boxed()
}

async fn body_handler<C: AccountByUserId + ScoresByAccount>(
    user_id: i32,
    table_index: usize,
    tables: TableData,
    repos: C,
    song_data: SongData,
) -> Result<impl Reply, Rejection> {
    let tables = tables.lock().await;
    Ok(body(user_id, repos, tables.get(table_index).unwrap(), song_data).await?)
}

async fn body<C: AccountByUserId + ScoresByAccount>(
    user_id: i32,
    mut repos: C,
    table: &Table,
    song_data: SongData,
) -> Result<impl Reply, HandleError> {
    let account = repos.user(user_id).await?;
    let score = repos.score(&account).await?;
    let songs = song_data.lock().await;
    Ok(serde_json::to_string(&table.filter_score(&score, &songs.song)).unwrap())
}
