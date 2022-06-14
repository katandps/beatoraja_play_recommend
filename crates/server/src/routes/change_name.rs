use crate::error::HandleError;
use crate::filter::{account_by_session, changed_name_by_query, with_db};
use model::Account;
use mysql::MySqlPool;
use repository::RenameAccount;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path("user"))
        .and(path("name"))
        .and(with_db(db_pool))
        .and(account_by_session(db_pool))
        .and(changed_name_by_query())
        .and_then(change_name_handler)
        .boxed()
}

async fn change_name_handler<C: RenameAccount>(
    repos: C,
    mut account: Account,
    changed_name: String,
) -> Result<impl Reply, Rejection> {
    account.set_name(&changed_name);
    rename_account(&repos, &account)?;
    Ok(serde_json::to_string(&account).unwrap())
}

fn rename_account<C: RenameAccount>(repos: &C, account: &Account) -> Result<(), HandleError> {
    Ok(repos.rename(account)?)
}
