use crate::filter::RankingQuery;
use crate::filter::{with_db, with_song_data};
use crate::SongData;
use chrono::Duration;
use model::*;
use mysql::MySqlPool;
use repository::ScoresBySha256;
use std::collections::HashMap;
use std::str::FromStr;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn ranking_route(db_pool: &MySqlPool, song_data: &SongData) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("ranking"))
        .and(with_db(db_pool))
        .and(warp::query::<HashMap<String, String>>().and_then(parse_ranking_query))
        .and(with_song_data(song_data))
        .and_then(ranking_handler)
        .boxed()
}

async fn parse_ranking_query(query: HashMap<String, String>) -> Result<RankingQuery, Rejection> {
    let date = query
        .get("date".into())
        .map(UpdatedAt::from_string)
        .map(|u| &u - Duration::days(-1))
        .unwrap_or(UpdatedAt::now());
    let play_mode = if let Some(mode) = query.get("mode".into()) {
        match mode.parse::<i32>() {
            Ok(mode) => PlayMode::from(mode),
            Err(_) => PlayMode::default(),
        }
    } else {
        PlayMode::default()
    };
    let sha256 = query
        .get("sha256".into())
        .map(|s| HashSha256::from_str(s).unwrap())
        .unwrap();
    Ok(RankingQuery {
        date,
        play_mode,
        sha256,
    })
}

/// 詳細表示ハンドラ
/// user_idをQueryParameterより取得する
async fn ranking_handler<C: ScoresBySha256>(
    repos: C,
    query: RankingQuery,
    song_data: SongData,
) -> Result<impl Reply, Rejection> {
    let songs = song_data.lock().await;
    let scores = repos.score(&query.sha256).unwrap();
    let response = scores.for_response(&songs.song, &query.date, &query.sha256);
    match response {
        Some(res) => Ok(serde_json::to_string(&res).unwrap()),
        None => Ok(serde_json::to_string(&RankingResponse::default()).unwrap()),
    }
}
