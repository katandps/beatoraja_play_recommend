use crate::filter::with_table;
use crate::TableData;
use model::TablesFormat;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn route(tables: &TableData) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("tables"))
        .and(with_table(tables))
        .and_then(table_handler)
        .boxed()
}

async fn table_handler(tables: TableData) -> Result<impl Reply, Rejection> {
    let tables = tables.lock().await;
    Ok(serde_json::to_string(&TablesFormat::format(&tables)).unwrap())
}
