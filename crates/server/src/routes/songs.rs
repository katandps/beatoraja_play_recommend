use std::sync::Arc;

use crate::filter::{with_db, with_songs_tag, with_table, with_tag};
use crate::map_response;
use futures::lock::Mutex;
use mysql::MySqlPool;
use service::songs::SongsTag;
use table::TableClient;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Reply};

pub fn route(
    db_pool: &MySqlPool,
    tables: &TableClient,
    songs_tag: &Arc<Mutex<SongsTag>>,
) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("songs"))
        .and(with_db(db_pool))
        .and(with_table(tables))
        .and(with_songs_tag(songs_tag))
        .and(with_tag())
        .then(service::songs::list)
        .then(map_response)
        .boxed()
}
