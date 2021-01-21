use crate::filter::*;
use crate::handler::*;
use crate::SongData;
use mysql::MySqlPool;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Reply};

pub fn uploads(db_pool: &MySqlPool, song_data: &SongData) -> BoxedFilter<(impl Reply,)> {
    let score_upload_route = score_upload(db_pool);
    let score_log_upload_route = score_log_upload(db_pool);
    let song_data_upload_route = song_data_upload_route(db_pool, song_data);
    score_upload_route
        .or(score_log_upload_route)
        .or(song_data_upload_route)
        .boxed()
}

fn score_upload(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path("upload"))
        .and(path("score"))
        .and(with_db(&db_pool))
        .and(receive_sqlite_file())
        .and(receive_session_key())
        .and_then(upload::upload_score_handler)
        .boxed()
}

fn score_log_upload(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path("upload"))
        .and(path("score_log"))
        .and(with_db(&db_pool))
        .and(receive_sqlite_file())
        .and(receive_session_key())
        .and_then(upload::upload_score_log_handler)
        .boxed()
}

fn song_data_upload_route(db_pool: &MySqlPool, song_data: &SongData) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path("upload"))
        .and(path("song_data"))
        .and(with_db(&db_pool))
        .and(with_song_data(song_data))
        .and(receive_sqlite_file())
        .and(receive_session_key())
        .and_then(upload::upload_song_data_handler)
        .boxed()
}
