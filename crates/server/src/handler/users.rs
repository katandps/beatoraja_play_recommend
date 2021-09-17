use crate::error::HandleError;
use crate::filter::with_db;
use mysql::MySqlPool;
use repository::PublishedUsers;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn users_route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("users"))
        .and(with_db(db_pool))
        .and_then(users_handler)
        .boxed()
}

async fn users_handler<C: PublishedUsers>(repos: C) -> std::result::Result<impl Reply, Rejection> {
    let users = repos.fetch_users().map_err(HandleError::from)?;
    Ok(serde_json::to_string(&users).unwrap())
}
