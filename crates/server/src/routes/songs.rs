use std::sync::Arc;

use crate::cache_tags::SongsTag;
use crate::error::HandleError;
use crate::filter::{with_db, with_songs_tag, with_table, with_tag};
use futures::lock::Mutex;
use model::TablesInfo;
use mysql::MySqlPool;
use repository::SongDataForTables;
use table::TableClient;
use warp::filters::BoxedFilter;
use warp::http;
use warp::path;
use warp::{Filter, Rejection, Reply};

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
        .and_then(songs_handler)
        .boxed()
}

async fn songs_handler<C: SongDataForTables>(
    mut repos: C,
    tables: TablesInfo,
    saved_tag: Arc<Mutex<SongsTag>>,
    tag: Option<String>,
) -> Result<impl Reply, Rejection> {
    let saved_tag: futures::lock::MutexGuard<'_, SongsTag> = saved_tag.lock().await;

    if saved_tag.is_saved(&tag) {
        // 変更がない場合、ステータスコードだけを返す
        log::info!("songs_handler ETag matched: {:?}", tag);
        Ok(http::Response::builder()
            .status(http::StatusCode::NOT_MODIFIED)
            .header("ETag", &saved_tag.tag)
            .header("Content-type", "application/json; charset=utf-8")
            .body("".to_string())
            .unwrap())
    } else {
        let songs = repos
            .song_data(&tables.tables)
            .await
            .map_err(HandleError::from)?;

        log::info!("songs_handler ETag unmatched: {:?}", tag);
        // テーブル情報をJSONとして返す
        Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .header("ETag", &saved_tag.tag)
            .header("Content-type", "application/json; charset=utf-8")
            .body(serde_json::to_string(&songs.get_list(tables.tables.get_charts())).unwrap())
            .unwrap())
    }
}
