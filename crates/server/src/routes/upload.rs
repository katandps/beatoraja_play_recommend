use crate::{filter::*, json};
use futures::lock::Mutex;
use mysql::MySqlPool;
use service::songs::SongsTag;
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Reply};

pub fn play_data_upload_route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path!("upload" / "play_data"))
        .and(with_db(db_pool))
        .and(with_login())
        .and(receive_sqlite_file())
        .then(service::play_data::upload)
        .then(json)
        .boxed()
}

pub fn song_data_upload_route(
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
