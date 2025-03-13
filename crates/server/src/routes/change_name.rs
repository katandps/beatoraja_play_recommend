use crate::filter::{changed_name_by_query, with_db, with_login};
use crate::json;
use mysql::MySqlPool;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Reply};

pub fn route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path("user"))
        .and(path("name"))
        .and(with_db(db_pool))
        .and(with_login())
        .and(changed_name_by_query())
        .then(service::users::change_name)
        .then(json)
        .boxed()
}
