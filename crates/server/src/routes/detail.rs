use crate::filter::{account_id_query, with_db, with_table};
use crate::{json, query};
use model::*;
use mysql::MySqlPool;
use table::TableClient;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Reply};

pub fn route(db_pool: &MySqlPool, tables: &TableClient) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("detail"))
        .and(with_db(db_pool))
        .and(with_table(tables))
        .and(
            warp::query()
                .then(DetailQuery::parse)
                .and_then(query)
                .boxed(),
        )
        .and(account_id_query(db_pool))
        .then(service::scores::list)
        .then(json)
        .boxed()
}
