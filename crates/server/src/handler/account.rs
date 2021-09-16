use crate::filter::account_by_session;
use model::Account;
use mysql::MySqlPool;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn account_route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("account"))
        .and(account_by_session(db_pool))
        .and_then(account_handler)
        .boxed()
}

async fn account_handler(account: Account) -> Result<impl Reply, Rejection> {
    Ok(serde_json::to_string(&account).unwrap())
}
