use crate::error::HandleError;
use crate::filter::with_db;
use mysql::MySqlPool;
use repository::{AccountByUserId, StatsByAccount};
use warp::filters::BoxedFilter;
use warp::*;

pub fn route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    get()
        .and(path!("stats" / i32))
        .and(with_db(db_pool))
        .and_then(stats_handler)
        .boxed()
}

async fn stats_handler<C: AccountByUserId + StatsByAccount>(
    user_id: i32,
    mut repos: C,
) -> Result<impl Reply, Rejection> {
    let account = repos.user(user_id).await.map_err(HandleError::from)?;
    let stats = repos.stats(&account).await.map_err(HandleError::from)?;
    Ok(serde_json::to_string(&stats).unwrap())
}
