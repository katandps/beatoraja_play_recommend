use crate::filter::{with_song_data, with_table};
use crate::SongData;
use model::Tables;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn songs_route(tables: &Tables, songs: &SongData) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("songs"))
        .and(with_table(tables))
        .and(with_song_data(songs))
        .and_then(songs_handler)
        .boxed()
}

async fn songs_handler(
    tables: Tables,
    song_data: SongData,
) -> std::result::Result<impl Reply, Rejection> {
    let songs = song_data.lock().await;
    Ok(serde_json::to_string(&songs.song.get_list(&tables.get_charts())).unwrap())
}
