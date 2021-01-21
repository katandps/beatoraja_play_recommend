use crate::error::HandleError;
use crate::SongData;
use bytes::BufMut;
use futures::TryStreamExt;
use model::*;
use mysql::MySQLClient;
use sqlite::SqliteClient;
use std::sync::Arc;
use warp::filters::multipart::{FormData, Part};
use warp::{Rejection, Reply};

pub async fn upload_score_handler(
    mysql_client: MySQLClient,
    form: FormData,
    session_key: String,
) -> std::result::Result<impl Reply, Rejection> {
    let user_id = crate::session::get_user_id(&session_key)?;
    let account = get_account(&mysql_client, user_id)?;
    let dir_name = account.google_id();
    save_sqlite_file(form, dir_name.clone(), "score".into()).await?;
    update_score_data(mysql_client, account, dir_name).await
}

pub async fn upload_score_log_handler(
    mysql_client: MySQLClient,
    form: FormData,
    session_key: String,
) -> std::result::Result<impl Reply, Rejection> {
    let user_id = crate::session::get_user_id(&session_key)?;
    let account = get_account(&mysql_client, user_id)?;
    let dir_name = account.google_id();
    save_sqlite_file(form, dir_name.clone(), "scorelog".into()).await?;
    update_score_data(mysql_client, account, dir_name).await
}

fn get_account(mysql_client: &MySQLClient, user_id: GoogleId) -> Result<Account, HandleError> {
    Ok(mysql_client.account_by_id(user_id)?)
}

async fn update_score_data(
    mysql_client: MySQLClient,
    account: Account,
    dir_name: String,
) -> Result<String, Rejection> {
    let score_file_name = format!("./files/{}/score.db", dir_name);
    let scorelog_file_name = format!("./files/{}/scorelog.db", dir_name);

    let _score_file = tokio::fs::read(&score_file_name)
        .await
        .map_err(|_| HandleError::FileIsNotFound.rejection())?;
    let _scorelog_file = tokio::fs::read(&scorelog_file_name)
        .await
        .map_err(|_| HandleError::FileIsNotFound.rejection())?;

    let sqlite_client = SqliteClient::new(
        scorelog_file_name.clone(),
        "".into(),
        score_file_name.clone(),
    );

    let scores = sqlite_client.score();
    mysql_client
        .save_score(account, scores)
        .map_err(|_| HandleError::SaveIsNotComplete.rejection())?;

    let _remove_score = tokio::fs::remove_file(&score_file_name)
        .await
        .map_err(|_| HandleError::FileIsNotDeleted.rejection());
    let _remove_score_log = tokio::fs::remove_file(&scorelog_file_name)
        .await
        .map_err(|_| HandleError::FileIsNotDeleted.rejection());
    Ok("Score Is Updated.".into())
}

pub async fn upload_song_data_handler(
    mysql_client: MySQLClient,
    song_data: SongData,
    form: FormData,
    session_key: String,
) -> std::result::Result<String, Rejection> {
    save_song(mysql_client, song_data, form, session_key).await?;
    Ok("SongData Is Updated.".into())
}
async fn save_song(
    mysql_client: MySQLClient,
    song_data: SongData,
    form: FormData,
    session_key: String,
) -> Result<(), HandleError> {
    let user_id = crate::session::get_user_id(&session_key)?;
    let account = mysql_client.account_by_id(user_id)?;
    let dir_name = account.google_id();
    save_sqlite_file(form, dir_name.clone(), "songdata".into()).await?;

    let song_file_name = format!("./files/{}/songdata.db", dir_name);
    let _song_file = tokio::fs::read(&song_file_name).await?;

    let sqlite_client = SqliteClient::new("".into(), song_file_name.clone(), "".into());

    let songs = sqlite_client.song_data()?;

    mysql_client.save_song(&songs)?;
    let song_db = Arc::clone(&song_data);
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
