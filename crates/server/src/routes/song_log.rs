use crate::filter::{account_id_query, with_db, SongLogQuery};
use model::{Account, ScoreId};
use mysql::MySqlPool;
use repository::ScoreByAccountAndSha256;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("score"))
        .and(with_db(db_pool))
        .and(account_id_query(db_pool))
        .and(warp::query::<SongLogQuery>())
        .and_then(handler)
        .boxed()
}

async fn handler<C: ScoreByAccountAndSha256>(
    repos: C,
    account: Account,
    query: SongLogQuery,
) -> Result<impl Reply, Rejection> {
    Ok(serde_json::to_string(
        &repos
            .score_with_log(&account, &ScoreId::new(query.sha256, query.play_mode))
            .unwrap(),
    )
    .unwrap())
}
