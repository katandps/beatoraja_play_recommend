use crate::error::HandleError;
use crate::filter::{account_by_session, changed_visibility_by_query, with_db};
use model::Account;
use mysql::MySqlPool;
use repository::ChangeAccountVisibility;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path("user"))
        .and(path("visibility"))
        .and(with_db(db_pool))
        .and(account_by_session(db_pool))
        .and(changed_visibility_by_query())
        .and_then(change_visibility_handler)
        .boxed()
}

async fn change_visibility_handler<C: ChangeAccountVisibility>(
    repos: C,
    mut account: Account,
    changed_visibility: bool,
) -> Result<impl Reply, Rejection> {
    account.set_visibility(changed_visibility);
    change_visibility(repos, &account).await?;
    Ok(serde_json::to_string(&account).unwrap())
}

async fn change_visibility<C: ChangeAccountVisibility>(
    mut repos: C,
    account: &Account,
) -> Result<(), HandleError> {
    Ok(repos.change_visibility(account).await?)
}
