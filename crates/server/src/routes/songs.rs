use crate::filter::{with_song_data, with_table};
use crate::{SongData, TableData};
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn songs_route(tables: &TableData, songs: &SongData) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("songs"))
        .and(with_table(tables))
        .and(with_song_data(songs))
        .and_then(songs_handler)
        .boxed()
}

async fn songs_handler(
    tables: TableData,
    song_data: SongData,
) -> std::result::Result<impl Reply, Rejection> {
    let tables = tables.lock().await;
    let songs = song_data.lock().await;
    Ok(serde_json::to_string(&songs.song.get_list(&tables.get_charts())).unwrap())
}
