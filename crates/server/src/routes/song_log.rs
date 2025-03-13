use crate::filter::{account_id_query, with_db};
use crate::json;
use model::SongLogQuery;
use mysql::MySqlPool;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Reply};

pub fn route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("score"))
        .and(with_db(db_pool))
        .and(account_id_query(db_pool))
        .and(warp::query::<SongLogQuery>())
        .then(service::scores::log)
        .then(json)
        .boxed()
}
