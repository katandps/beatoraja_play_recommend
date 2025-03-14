use std::sync::Arc;

use crate::filter::{with_cache_tag, with_db, with_songs_tag, with_table};
use crate::json;
use futures::lock::Mutex;
use mysql::MySqlPool;
use repository::HealthCheck;
use service::songs::SongsTag;
use table::TableClient;
use warp::filters::BoxedFilter;
use warp::http::StatusCode;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn routes(
    db_pool: &MySqlPool,
    t: &TableClient,
    songs_tag: &Arc<Mutex<SongsTag>>,
) -> BoxedFilter<(impl Reply,)> {
    health(db_pool)
        .or(users(db_pool))
        .or(tables(t))
        .or(stats(db_pool))
        .or(songs(db_pool, t, songs_tag))
        .or(ranking(db_pool, t))
        .or(detail(db_pool, t))
        .or(song_log(db_pool))
        .with(warp::compression::gzip())
        .boxed()
}

fn health(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    async fn health_handler<C: HealthCheck>(mut client: C) -> Result<impl Reply, Rejection> {
        match client.health().await {
            Ok(_) => Ok(StatusCode::OK),
            Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
    warp::get()
        .and(path("health"))
        .and(with_db(db_pool))
        .and_then(health_handler)
        .boxed()
}
fn users(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("users"))
        .and(with_db(db_pool))
        .then(service::users::list)
        .then(json)
        .boxed()
}

fn tables(tables: &TableClient) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("tables"))
        .and(with_table(tables))
        .and(with_cache_tag())
        .then(service::tables::get)
        .then(json)
        .boxed()
}

fn stats(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path!("stats" / i32))
        .and(with_db(db_pool))
        .then(service::status::by_user)
        .then(json)
        .boxed()
}

fn songs(
    db_pool: &MySqlPool,
    tables: &TableClient,
    songs_tag: &Arc<Mutex<SongsTag>>,
) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("songs"))
        .and(with_db(db_pool))
        .and(with_table(tables))
        .and(with_songs_tag(songs_tag))
        .and(with_cache_tag())
        .then(service::songs::list)
        .then(json)
        .boxed()
}

fn ranking(db_pool: &MySqlPool, tables: &TableClient) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("ranking"))
        .and(with_db(db_pool))
        .and(with_table(tables))
        .and(warp::query())
        .then(service::songs::ranking)
        .then(json)
        .boxed()
}

fn detail(db_pool: &MySqlPool, tables: &TableClient) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("detail"))
        .and(with_db(db_pool))
        .and(with_table(tables))
        .and(warp::query())
        .then(service::scores::list)
        .then(json)
        .boxed()
}

fn song_log(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("score"))
        .and(with_db(db_pool))
        .and(warp::query())
        .then(service::scores::log)
        .then(json)
        .boxed()
}
