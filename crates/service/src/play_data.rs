use crate::Response;
use anyhow::Result;
use chrono::Utc;
use model::UploadAt;
use repository::{
    AccountByGoogleId, AccountByUserId, RegisterUpload, SavePlayerStateData, SaveScoreData,
};
use session::Claims;
use sqlite::SqliteClient;
use std::collections::HashMap;
use std::io::Write;
use tempfile::NamedTempFile;

pub async fn upload<
    C: SaveScoreData + SavePlayerStateData + AccountByGoogleId + RegisterUpload + AccountByUserId,
>(
    mut repository: C,
    claims: Claims,
    form: HashMap<String, Vec<u8>>,
) -> Result<Response<()>> {
    let account = AccountByUserId::user(&mut repository, claims.user_id).await?;
    log::info!("start to upload");
    let mut score_db = NamedTempFile::new()?;
    let mut scorelog_db = NamedTempFile::new()?;
    log::info!("ready named temp file to save");
    score_db.write_all(form.get("score").unwrap())?;
    scorelog_db.write_all(form.get("scorelog").unwrap())?;
    log::info!("saved uploading file");
    let sqlite_client = SqliteClient::for_score(
        score_db.path().to_str().unwrap(),
        scorelog_db.path().to_str().unwrap(),
    );
    let upload = repository
        .register_upload(account.user_id, UploadAt(Utc::now()))
        .await?;
    log::info!("read score file");
    let scores = sqlite_client.score()?;
    log::info!("read player stat");
    let player_states = sqlite_client.player()?;
    log::info!("save score");
    repository.save_score(&account, &scores, &upload).await?;
    log::info!("save player stat");
    repository
        .save_player_states(&account, &player_states, &upload)
        .await?;
    Ok(Response::Ok {
        tag: None,
        body: (),
    })
}
