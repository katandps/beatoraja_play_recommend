use crate::filter::{with_table, with_tag};
use crate::TableData;
use model::TablesFormat;
use warp::filters::BoxedFilter;
use warp::http;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn route(tables: &TableData) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("tables"))
        .and(with_table(tables))
        .and(with_tag())
        .and_then(table_handler)
        .boxed()
}

async fn table_handler(tables: TableData, tag: Option<String>) -> Result<impl Reply, Rejection> {
    let tables_info = tables.lock().await;
    if tables_info.tag == tag {
        // 変更がない場合、ステータスコードだけを返す
        log::info!("table_handler ETag matched: {:?}", tag);
        Ok(http::Response::builder()
            .status(http::StatusCode::NOT_MODIFIED)
            .header("ETag", tables_info.get_tag())
            .header("Content-type", "application/json; charset=utf-8")
            .body("".to_string())
            .unwrap())
    } else {
        log::info!("table_handler ETag unmatched: {:?}", tag);
        // テーブル情報をJSONとして返す
        Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .header("ETag", tables_info.get_tag())
            .header("Content-type", "application/json; charset=utf-8")
            .body(serde_json::to_string(&TablesFormat::format(&tables_info.tables)).unwrap())
            .unwrap())
    }
}
