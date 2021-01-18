use crate::error::HandleError;
use model::*;
use mysql::MySQLClient;
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
    Ok(serde_json::to_string(&CustomTableHeader::from(tables.get())).unwrap())
}

pub async fn body_handler(
    user_id: i32,
    tables: Tables,
    repos: MySQLClient,
) -> Result<impl Reply, Rejection> {
    body(user_id, repos)?;
    Ok(serde_json::to_string(&tables.get().get_charts()).unwrap())
}

fn body(user_id: i32, repos: MySQLClient) -> Result<impl Reply, HandleError> {
    let account = repos.account_by_increments(user_id)?;
    let _score = repos.score(&account)?;
    Ok(serde_json::to_string("")?)
}
