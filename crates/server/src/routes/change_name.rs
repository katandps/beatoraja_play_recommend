use crate::filter::{account_by_session, changed_name_by_query, with_db};
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
        .and(account_by_session(db_pool))
        .and(changed_name_by_query())
        .then(service::user::change_name)
        .then(json)
        .boxed()
}
