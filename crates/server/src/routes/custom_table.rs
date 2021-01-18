use crate::filter::*;
use crate::handler::*;
use model::Tables;
use mysql::MySqlPool;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Reply};

pub fn custom_tables(db_pool: &MySqlPool, tables: &Tables) -> BoxedFilter<(impl Reply,)> {
    custom_table_header(tables)
        .or(custom_table_body(db_pool, tables))
        .or(custom_table())
        .boxed()
}

fn custom_table() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("recommend_table"))
        .and(path::param())
        .and(path("table.html"))
        .and_then(custom_table::table_handler)
        .boxed()
}

fn custom_table_header(tables: &Tables) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("recommend_table"))
        .and(path::param())
        .and(path("header.json"))
        .and(with_table(tables))
        .and_then(custom_table::header_handler)
        .boxed()
}

fn custom_table_body(db_pool: &MySqlPool, tables: &Tables) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("recommend_table"))
        .and(path::param())
        .and(path("score.json"))
        .and(with_table(tables))
        .and(with_db(&db_pool))
        .and_then(custom_table::body_handler)
        .boxed()
}
