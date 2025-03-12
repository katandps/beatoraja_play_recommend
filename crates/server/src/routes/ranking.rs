use crate::filter::{with_db, with_table};
use crate::{json, query};
use model::RankingQuery;
use mysql::MySqlPool;
use table::TableClient;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Reply};

pub fn route(db_pool: &MySqlPool, tables: &TableClient) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("ranking"))
        .and(with_db(db_pool))
        .and(with_table(tables))
        .and(warp::query().then(RankingQuery::parse).and_then(query))
        .then(service::songs::ranking)
        .then(json)
        .boxed()
}
