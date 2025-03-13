use crate::error::HandleError;
use crate::filter::*;
use bytes::Buf;
use chrono::Utc;
use futures::lock::Mutex;
use futures::TryStreamExt;
use model::*;
use mysql::MySqlPool;
use repository::{
    AccountByGoogleId, AccountByUserId, RegisterUpload, SavePlayerStateData, SaveScoreData,
    SaveSongData,
};
use service::songs::SongsTag;
use session::Claims;
use sqlite::SqliteClient;
use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;
use tempfile::NamedTempFile;
use warp::filters::multipart::FormData;
use warp::filters::BoxedFilter;
use warp::http::StatusCode;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn play_data_upload_route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path!("upload" / "play_data"))
        .and(with_db(db_pool))
        .and(with_login())
        .and(receive_sqlite_file())
        .and_then(play_data_upload_handler)
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
        .and_then(upload_song_data_handler)
        .boxed()
}

async fn play_data_upload_handler<
    C: SaveScoreData + SavePlayerStateData + AccountByGoogleId + RegisterUpload + AccountByUserId,
>(
    mut repository: C,
    claims: Claims,
    form: FormData,
) -> Result<impl Reply, Rejection> {
    let account = AccountByUserId::user(&mut repository, claims.user_id)
        .await
        .map_err(HandleError::from)?;
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
    let upload = repository
        .register_upload(account.user_id, UploadAt(Utc::now()))
        .await
        .map_err(HandleError::from)?;
    log::info!("read score file");
    let scores = sqlite_client.score().map_err(HandleError::from)?;
    log::info!("read player stat");
    let player_states = sqlite_client.player().map_err(HandleError::from)?;
    log::info!("save score");
    repository
        .save_score(&account, &scores, &upload)
        .await
        .map_err(HandleError::from)?;
    log::info!("save player stat");
    repository
        .save_player_states(&account, &player_states, &upload)
        .await
        .map_err(HandleError::from)?;
    Ok(StatusCode::OK)
}

async fn upload_song_data_handler<C: SaveSongData>(
    mut client: C,
    songs_tag: Arc<Mutex<SongsTag>>,
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
        .await
        .map_err(HandleError::from)?;
    let mut songs_tag: futures::lock::MutexGuard<'_, SongsTag> = songs_tag.lock().await;
    *songs_tag = SongsTag::new();

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
