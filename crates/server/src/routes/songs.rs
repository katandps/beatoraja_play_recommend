use crate::error::HandleError;
use crate::filter::{with_db, with_table};
use crate::TableData;
use mysql::MySqlPool;
use repository::SongDataForTables;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn route(db_pool: &MySqlPool, tables: &TableData) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("songs"))
        .and(with_db(db_pool))
        .and(with_table(tables))
        .and_then(songs_handler)
        .boxed()
}

async fn songs_handler<C: SongDataForTables>(
    mut repos: C,
    tables: TableData,
) -> Result<impl Reply, Rejection> {
    let tables = tables.lock().await;
    let songs = repos.song_data(&tables).await.map_err(HandleError::from)?;
    Ok(serde_json::to_string(&songs.get_list(tables.get_charts())).unwrap())
}
