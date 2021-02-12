use crate::error::HandleError;
use crate::SongData;
use model::*;
use mysql::MySQLClient;
use repository::{AccountByIncrement, ScoresByAccount};
use warp::{Rejection, Reply};

pub async fn table_handler(_user_id: i32) -> Result<impl Reply, Rejection> {
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

pub async fn header_handler(_user_id: i32, tables: Tables) -> Result<impl Reply, Rejection> {
    Ok(serde_json::to_string(
        &CustomTableHeader::from(tables.get()).set_name("おすすめ譜面表".into()),
    )
    .unwrap())
}

pub async fn body_handler(
    user_id: i32,
    tables: Tables,
    repos: MySQLClient,
    song_data: SongData,
) -> Result<impl Reply, Rejection> {
    Ok(body(user_id, repos, tables.get(), song_data).await?)
}

async fn body(
    user_id: i32,
    repos: MySQLClient,
    table: &Table,
    song_data: SongData,
) -> Result<impl Reply, HandleError> {
    let account = repos.user(user_id)?;
    let score = repos.score(&account)?;
    let songs = song_data.lock().await;
    Ok(serde_json::to_string(&table.filter_score(&score, &songs.song)).unwrap())
}
