use crate::error::HandleError;
use crate::filter::*;
use model::*;
use mysql::MySqlPool;
use repository::ResetScore;
use warp::filters::BoxedFilter;
use warp::http::StatusCode;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path!("reset"))
        .and(with_db(db_pool))
        .and(account_by_session(db_pool))
        .and_then(reset_handler)
        .boxed()
}

async fn reset_handler<C: ResetScore>(
    mut repository: C,
    account: Account,
) -> Result<impl Reply, Rejection> {
    repository
        .reset_score(&account)
        .await
        .map_err(HandleError::from)?;
    Ok(StatusCode::OK)
}
