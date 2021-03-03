use crate::error::HandleError;
use crate::filter::with_db;
use mysql::MySqlPool;
use repository::{PublishedUsers, VisibleAccount};
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
    let users = fetch_users(repos)?;
    Ok(serde_json::to_string(&users).unwrap())
}

fn fetch_users<C: PublishedUsers>(repos: C) -> Result<Vec<VisibleAccount>, HandleError> {
    let users = repos.fetch_users()?;
    Ok(users)
}
