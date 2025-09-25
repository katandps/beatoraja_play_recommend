use std::sync::Arc;

use crate::filter::{with_cache_tag, with_db, with_login, with_songs_tag, with_table};
use crate::json;
use futures::lock::Mutex;
use mysql::MySqlPool;
use service::songs::SongsTag;
use table::TableClient;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Reply};

pub fn routes(
    db_pool: &MySqlPool,
    t: TableClient,
    songs_tag: &Arc<Mutex<SongsTag>>,
) -> BoxedFilter<(impl Reply,)> {
    health(db_pool)
        .or(users(db_pool))
        .or(tables(t.clone()))
        .or(stats(db_pool))
        .or(songs(db_pool, t.clone(), songs_tag))
        .or(ranking(db_pool, t.clone()))
        .or(detail(db_pool, t))
        .or(song_log(db_pool))
        .or(my_song_log(db_pool))
        .with(warp::compression::gzip())
        .boxed()
}

fn health(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("health"))
        .and(with_db(db_pool))
        .then(service::health_check)
        .then(json)
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

fn tables(tables: TableClient) -> BoxedFilter<(impl Reply,)> {
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
        .and(path!("stats" / "my"))
        .and(with_db(db_pool))
        .and(with_login())
        .then(service::status::by_user)
        .then(json)
        .boxed()
}

fn songs(
    db_pool: &MySqlPool,
    tables: TableClient,
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

fn ranking(db_pool: &MySqlPool, tables: TableClient) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("ranking"))
        .and(with_db(db_pool))
        .and(with_table(tables))
        .and(warp::query())
        .then(service::songs::ranking)
        .then(json)
        .boxed()
}

fn detail(db_pool: &MySqlPool, tables: TableClient) -> BoxedFilter<(impl Reply,)> {
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

fn my_song_log(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("score"))
        .and(with_db(db_pool))
        .and(with_login())
        .and(warp::query())
        .then(service::scores::my_log)
        .then(json)
        .boxed()
}
