use crate::filter::with_table;
use crate::TableData;
use model::TablesFormat;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn tables_route(tables: &TableData) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("tables"))
        .and(with_table(tables))
        .and_then(table_handler)
        .boxed()
}

async fn table_handler(tables: TableData) -> std::result::Result<impl Reply, Rejection> {
    let tables = tables.lock().await;
    Ok(serde_json::to_string(&TablesFormat::format(&tables)).unwrap())
}
