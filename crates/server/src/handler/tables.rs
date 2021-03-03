use crate::filter::with_table;
use model::{Tables, TablesFormat};
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn tables_route(tables: &Tables) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("tables"))
        .and(with_table(tables))
        .and_then(table_handler)
        .boxed()
}

async fn table_handler(tables: Tables) -> std::result::Result<impl Reply, Rejection> {
    Ok(serde_json::to_string(&TablesFormat::from(tables)).unwrap())
}
