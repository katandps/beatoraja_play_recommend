use std::sync::Arc;

use crate::filter::{
    changed_name_by_query, changed_visibility_by_query, receive_sqlite_file, with_db, with_login,
    with_songs_tag,
};
use crate::json;
use futures::lock::Mutex;
use mysql::MySqlPool;
use service::songs::SongsTag;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Reply};

pub fn routes(db_pool: &MySqlPool, songs_tag: &Arc<Mutex<SongsTag>>) -> BoxedFilter<(impl Reply,)> {
    account(db_pool)
        .or(change_name(db_pool))
        .or(change_visibility(db_pool))
        .or(play_data_upload(db_pool))
        .or(song_data_upload(db_pool, songs_tag))
        .or(reset_score(db_pool))
        .with(warp::compression::gzip())
        .boxed()
}

fn account(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("account"))
        .and(with_db(db_pool))
        .and(with_login())
        .then(service::users::my)
        .then(json)
        .boxed()
}

fn change_visibility(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path("user"))
        .and(path("visibility"))
        .and(with_db(db_pool))
        .and(with_login())
        .and(changed_visibility_by_query())
        .then(service::users::change_visibility)
        .then(json)
        .boxed()
}

fn change_name(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path("user"))
        .and(path("name"))
        .and(with_db(db_pool))
        .and(with_login())
        .and(changed_name_by_query())
        .then(service::users::change_name)
        .then(json)
        .boxed()
}

fn play_data_upload(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path!("upload" / "play_data"))
        .and(with_db(db_pool))
        .and(with_login())
        .and(receive_sqlite_file())
        .then(service::play_data::upload)
        .then(json)
        .boxed()
}

fn song_data_upload(
    db_pool: &MySqlPool,
    songs_tag: &Arc<Mutex<SongsTag>>,
) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path!("upload" / "song_data"))
        .and(with_db(db_pool))
        .and(with_songs_tag(songs_tag))
        .and(receive_sqlite_file())
        .then(service::song_data::upload)
        .then(json)
        .boxed()
}

fn reset_score(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path!("reset"))
        .and(with_db(db_pool))
        .and(with_login())
        .then(service::scores::reset_all)
        .then(json)
        .boxed()
}
