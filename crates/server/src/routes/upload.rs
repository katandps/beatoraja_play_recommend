use crate::error::HandleError;
use crate::filter::*;
use crate::SongData;
use bytes::Buf;
use futures::TryStreamExt;
use http::StatusCode;
use model::*;
use mysql::MySqlPool;
use repository::{
    AccountByGoogleId, AllSongData, SavePlayerStateData, SaveScoreData, SaveSongData,
};
use sqlite::SqliteClient;
use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;
use tempfile::NamedTempFile;
use warp::filters::multipart::FormData;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn play_data_upload_route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path!("upload" / "play_data"))
        .and(with_db(db_pool))
        .and(receive_sqlite_file())
        .and(account_by_session(db_pool))
        .and_then(play_data_upload_handler)
        .boxed()
}

pub fn song_data_upload_route(
    db_pool: &MySqlPool,
    song_data: &SongData,
) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path!("upload" / "song_data"))
        .and(with_db(db_pool))
        .and(with_song_data(song_data))
        .and(receive_sqlite_file())
        .and_then(upload_song_data_handler)
        .boxed()
}

async fn play_data_upload_handler<C: SaveScoreData + SavePlayerStateData + AccountByGoogleId>(
    mut repository: C,
    form: FormData,
    account: Account,
) -> Result<impl Reply, Rejection> {
    log::info!("start to upload");
    let mut score_db = NamedTempFile::new().map_err(HandleError::from)?;
    let mut scorelog_db = NamedTempFile::new().map_err(HandleError::from)?;
    log::info!("ready named temp file to save");
    let map = form_into_map(form).await?;
    score_db
        .write_all(map.get("score").ok_or(HandleError::FormIsIncomplete)?)
        .map_err(HandleError::from)?;
    scorelog_db
        .write_all(map.get("scorelog").ok_or(HandleError::FormIsIncomplete)?)
        .map_err(HandleError::from)?;
    log::info!("saved uploading file");
    let sqlite_client = SqliteClient::for_score(
        score_db.path().to_str().unwrap(),
        scorelog_db.path().to_str().unwrap(),
    );

    let scores = sqlite_client.score().map_err(HandleError::from)?;
    repository
        .save_score(&account, &scores)
        .map_err(HandleError::from)?;
    let player_states = sqlite_client.player().map_err(HandleError::from)?;
    repository
        .save_player_states(&account, &player_states)
        .map_err(HandleError::from)?;
    Ok(StatusCode::OK)
}

async fn upload_song_data_handler<C: SaveSongData + AllSongData>(
    mut client: C,
    song_data: SongData,
    form: FormData,
) -> Result<impl Reply, Rejection> {
    let mut songdata_db = NamedTempFile::new().unwrap();
    let map = form_into_map(form).await?;
    songdata_db
        .write_all(map.get("songdata").ok_or(HandleError::FormIsIncomplete)?)
        .map_err(HandleError::from)?;
    let sqlite_client = SqliteClient::for_song(songdata_db.path().to_str().unwrap());

    client
        .save_song(&sqlite_client.song_data().map_err(HandleError::from)?)
        .map_err(HandleError::from)?;

    let song_db = Arc::clone(&song_data);
    let songs = client.song_data().unwrap();
    song_db.lock().await.update(songs);
    Ok("SongData Is Updated.")
}

async fn form_into_map(form: FormData) -> Result<HashMap<String, Vec<u8>>, HandleError> {
    let res = form
        .and_then(|mut part| async move {
            let name = part.name().to_string();
            log::info!("{name}");
            let mut data: Vec<u8> = Vec::new();
            while let Some(content) = part.data().await {
                data.extend_from_slice(content.unwrap().chunk());
            }
            Ok((name, data))
        })
        .try_collect()
        .await?;
    Ok(res)
}
