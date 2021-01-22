use crate::SongData;
use model::Tables;
use warp::{Rejection, Reply};

pub async fn songs_handler(
    tables: Tables,
    song_data: SongData,
) -> std::result::Result<impl Reply, Rejection> {
    let songs = song_data.lock().await;
    Ok(serde_json::to_string(&songs.song.get_list(&tables.get_charts())).unwrap())
}
