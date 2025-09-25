use std::collections::HashMap;

use anyhow::Result;
use model::{DetailQuery, DetailResponse, Score, ScoreId, Scores, SongLogQuery, SongMyLogQuery};
use repository::{
    AccountByUserId, GetTables, ResetScore, ScoreByAccountAndSha256, ScoresByAccount,
    SongDataForTables,
};
use session::Claims;

use crate::Response;

macro_rules! log_duration {
    ($name:expr, $x:expr) => {{
        let start = std::time::Instant::now();
        let result = $x;
        let end = start.elapsed();
        log::info!("{} {}ms", stringify!($name), end.as_millis());
        result
    }};
}

pub async fn list<C: ScoresByAccount + SongDataForTables + AccountByUserId, T: GetTables>(
    mut repos: C,
    tables: T,
    query: DetailQuery,
) -> Result<crate::Response<DetailResponse>> {
    let tables = tables.get().await;
    let account = repos.user(query.user_id).await?;
    let songs = repos.song_data(&tables.tables).await?;
    let scores = log_duration!(
        GetScores,
        repos
            .score(&account)
            .await
            .unwrap_or_else(|_| Scores::create_by_map(HashMap::new()))
    );
    Ok(Response::Ok {
        tag: None,
        body: DetailResponse::new(&tables.tables, &songs, scores, &query.period, &account),
    })
}

pub async fn log<C: ScoreByAccountAndSha256 + AccountByUserId>(
    mut repos: C,
    query: SongLogQuery,
) -> Result<Response<Score>> {
    let account = repos.user(query.user_id).await?;
    let score_id = ScoreId::new(query.sha256, query.play_mode);
    log::info!("account: {:?}, score_id: {:?}", account, score_id);
    let score_with_log = repos
        .score_with_log(&account, &score_id)
        .await
        .unwrap_or(Score::default());
    log::debug!("log: {:?}", score_with_log);
    Ok(Response::Ok {
        tag: None,
        body: score_with_log,
    })
}

pub async fn my_log<C: ScoreByAccountAndSha256 + AccountByUserId>(
    mut repos: C,
    claims: Claims,
    query: SongMyLogQuery,
) -> Result<Response<Score>> {
    let account = repos.user(claims.user_id).await?;
    let score_id = ScoreId::new(query.sha256, query.play_mode);
    log::info!("account: {:?}, score_id: {:?}", account, score_id);
    let score_with_log = repos
        .score_with_log(&account, &score_id)
        .await
        .unwrap_or(Score::default());
    log::debug!("log: {:?}", score_with_log);
    Ok(Response::Ok {
        tag: None,
        body: score_with_log,
    })
}

pub async fn reset_all<R: ResetScore + AccountByUserId>(
    mut repository: R,
    claims: Claims,
) -> Result<Response<()>> {
    let account = repository.user(claims.user_id).await?;
    repository.reset_score(&account).await?;
    Ok(Response::Ok {
        tag: None,
        body: (),
    })
}
