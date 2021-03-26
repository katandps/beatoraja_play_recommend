use crate::error::HandleError;
use crate::filter::*;
use crate::SongData;
use bytes::BufMut;
use futures::TryStreamExt;
use model::*;
use mysql::MySqlPool;
use repository::{AccountByGoogleId, AllSongData, SaveScoreData, SaveSongData};
use sqlite::SqliteClient;
use std::sync::Arc;
use warp::filters::multipart::{FormData, Part};
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn score_upload_route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path!("upload" / "score"))
        .and(with_db(db_pool))
        .and(receive_sqlite_file())
        .and(receive_session_key())
        .and_then(upload_score_handler)
        .boxed()
}

pub fn score_log_upload_route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path!("upload" / "score_log"))
        .and(with_db(db_pool))
        .and(receive_sqlite_file())
        .and(receive_session_key())
        .and_then(upload_score_log_handler)
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
        .and(receive_session_key())
        .and_then(upload_song_data_handler)
        .boxed()
}

async fn upload_score_handler<C: SaveScoreData + AccountByGoogleId>(
    repository: C,
    form: FormData,
    session_key: String,
) -> std::result::Result<impl Reply, Rejection> {
    let user_id = crate::session::get_user_id(&session_key)?;
    let account = get_account(&repository, user_id)?;
    let dir_name = account.google_id();
    save_sqlite_file(form, dir_name.clone(), "score".into()).await?;
    update_score_data(repository, account, dir_name).await
}

async fn upload_score_log_handler<C: SaveScoreData + AccountByGoogleId>(
    repository: C,
    form: FormData,
    session_key: String,
) -> std::result::Result<impl Reply, Rejection> {
    let user_id = crate::session::get_user_id(&session_key)?;
    let account = get_account(&repository, user_id)?;
    let dir_name = account.google_id();
    save_sqlite_file(form, dir_name.clone(), "scorelog".into()).await?;
    update_score_data(repository, account, dir_name).await
}

fn get_account<C: AccountByGoogleId>(
    mysql_client: &C,
    user_id: GoogleId,
) -> Result<Account, HandleError> {
    Ok(mysql_client.user(&user_id)?)
}

async fn update_score_data<C: SaveScoreData>(
    repository: C,
    account: Account,
    dir_name: String,
) -> Result<String, Rejection> {
    let score_file_name = format!("./files/{}/score.db", dir_name);
    let score_log_file_name = format!("./files/{}/scorelog.db", dir_name);

    let score_file = tokio::fs::read(&score_file_name).await;
    let score_log_files = tokio::fs::read(&score_log_file_name).await;
    if let (Ok(_), Ok(_)) = (score_file, score_log_files) {
        let sqlite_client = SqliteClient::new(
            score_log_file_name.clone(),
            "".into(),
            score_file_name.clone(),
        );

        let scores = get_score(&sqlite_client)?;
        repository
            .save_score(&account, &scores)
            .map_err(|_| HandleError::SaveIsNotComplete.rejection())?;

        let _remove_score = tokio::fs::remove_file(&score_file_name)
            .await
            .map_err(|_| HandleError::FileIsNotDeleted.rejection());
        let _remove_score_log = tokio::fs::remove_file(&score_log_file_name)
            .await
            .map_err(|_| HandleError::FileIsNotDeleted.rejection());
        Ok("Score Is Updated.".into())
    } else {
        Ok("Score Is Not Updated.".into())
    }
}

fn get_score(client: &SqliteClient) -> Result<Scores, HandleError> {
    Ok(client.score()?)
}

async fn upload_song_data_handler<C: SaveSongData + AccountByGoogleId + AllSongData>(
    mysql_client: C,
    song_data: SongData,
    form: FormData,
    session_key: String,
) -> std::result::Result<String, Rejection> {
    save_song(mysql_client, song_data, form, session_key).await?;
    Ok("SongData Is Updated.".into())
}
async fn save_song<C: SaveSongData + AccountByGoogleId + AllSongData>(
    client: C,
    song_data: SongData,
    form: FormData,
    session_key: String,
) -> Result<(), HandleError> {
    let user_id = crate::session::get_user_id(&session_key)?;
    let account = client.user(&user_id)?;
    let dir_name = account.google_id();
    save_sqlite_file(form, dir_name.clone(), "songdata".into()).await?;

    let song_file_name = format!("./files/{}/songdata.db", dir_name);
    let _song_file = tokio::fs::read(&song_file_name).await?;

    let sqlite_client = SqliteClient::new("".into(), song_file_name.clone(), "".into());

    let songs = sqlite_client.song_data()?;

    client.save_song(&songs)?;
    let song_db = Arc::clone(&song_data);
    let songs = client.song_data().unwrap();
    song_db.lock().await.update(songs);

    remove_file(&song_file_name).await
}

async fn remove_file(file_name: &String) -> Result<(), HandleError> {
    let _remove_data = tokio::fs::remove_file(&file_name).await?;
    Ok(())
}

async fn save_sqlite_file(
    form: FormData,
    dir_name: String,
    file_name: String,
) -> std::result::Result<String, HandleError> {
    let parts: Vec<Part> = get_parts(form).await?;
    for part in parts {
        return if part.name() == "file" {
            let file = get_stream(part).await?;
            create_dir(format!("./files/{}", dir_name)).await?;
            let file_name = format!("./files/{}/{}.db", dir_name, file_name);
            write_file(&file_name, &file).await?;
            Ok(file_name)
        } else {
            Err(HandleError::FileIsNotFound)
        };
    }
    return Ok("".into());
}

async fn get_parts(form: FormData) -> Result<Vec<Part>, HandleError> {
    Ok(form.try_collect().await?)
}

async fn get_stream(part: Part) -> Result<Vec<u8>, HandleError> {
    let value = part
        .stream()
        .try_fold(Vec::new(), |mut vec, data| {
            vec.put(data);
            async move { Ok(vec) }
        })
        .await?;
    Ok(value)
}

async fn create_dir(dir_name: String) -> Result<(), HandleError> {
    tokio::fs::create_dir_all(dir_name).await?;
    Ok(())
}

async fn write_file(file_name: &String, file: &Vec<u8>) -> Result<(), HandleError> {
    tokio::fs::write(&file_name, file).await?;
    Ok(())
}
