/**
 * # 難易度表系ルート
 * beatorajaはgzip compressionされたresponseに対応していない
 */
use crate::filter::{with_db, with_table};
use crate::json;
use mysql::MySqlPool;
use table::TableClient;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn routes(db_pool: &MySqlPool, tables: &TableClient) -> BoxedFilter<(impl Reply,)> {
    header_route(tables)
        .or(body_route(db_pool, tables))
        .or(table_route())
        .with(warp::log("table_access"))
        .boxed()
}

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
        .then(json)
        .boxed()
}

pub fn body_route(db_pool: &MySqlPool, tables: &TableClient) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path!("recommend_table" / i32 / usize / "score.json"))
        .and(with_table(tables))
        .and(with_db(db_pool))
        .then(service::custom_table::body)
        .then(json)
        .boxed()
}
