use crate::filter::{account_by_session, changed_visibility_by_query, with_db};
use crate::json;
use mysql::MySqlPool;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Reply};

pub fn route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path("user"))
        .and(path("visibility"))
        .and(with_db(db_pool))
        .and(account_by_session(db_pool))
        .and(changed_visibility_by_query())
        .then(service::user::change_visibility)
        .then(json)
        .boxed()
}
