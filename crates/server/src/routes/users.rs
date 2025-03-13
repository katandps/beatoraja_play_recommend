use crate::filter::with_db;
use crate::json;
use mysql::MySqlPool;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Reply};

pub fn route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("users"))
        .and(with_db(db_pool))
        .then(service::user::list)
        .then(json)
        .boxed()
}
