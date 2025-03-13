use std::collections::HashMap;

use anyhow::Result;
use model::{
    Account, DetailQuery, DetailResponse, Score, ScoreId, Scores, SongLogQuery, TablesInfo,
};
use repository::{ScoreByAccountAndSha256, ScoresByAccount, SongDataForTables};

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

pub async fn list<C: ScoresByAccount + SongDataForTables>(
    mut repos: C,
    tables: TablesInfo,
    query: DetailQuery,
    account: Account,
) -> Result<crate::Response<DetailResponse>> {
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
        body: scores.table_scores(&tables.tables, &songs, &query.date, &account),
    })
}

pub async fn log<C: ScoreByAccountAndSha256>(
    mut repos: C,
    account: Account,
    query: SongLogQuery,
) -> Result<Response<Score>> {
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
