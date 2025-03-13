use crate::filter::with_db;
use crate::json;
use mysql::MySqlPool;
use warp::filters::BoxedFilter;
use warp::*;

pub fn route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    get()
        .and(path!("stats" / i32))
        .and(with_db(db_pool))
        .then(service::status::by_user)
        .then(json)
        .boxed()
}
