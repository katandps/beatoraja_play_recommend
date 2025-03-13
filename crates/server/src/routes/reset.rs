use crate::filter::{with_db, with_login};
use crate::json;
use mysql::MySqlPool;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Reply};

pub fn route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path!("reset"))
        .and(with_db(db_pool))
        .and(with_login())
        .then(service::scores::reset_all)
        .then(json)
        .boxed()
}
