use std::{collections::HashMap, io::Write, sync::Arc};

use anyhow::Result;
use futures::lock::Mutex;
use repository::SaveSongData;
use sqlite::SqliteClient;
use tempfile::NamedTempFile;

use crate::{songs::SongsTag, Response};

pub async fn upload<C: SaveSongData>(
    mut client: C,
    songs_tag: Arc<Mutex<SongsTag>>,
    form: HashMap<String, Vec<u8>>,
) -> Result<Response<()>> {
    let mut songdata_db = NamedTempFile::new().unwrap();
    songdata_db.write_all(form.get("songdata").unwrap())?;
    let sqlite_client = SqliteClient::for_song(songdata_db.path().to_str().unwrap());

    client.save_song(&sqlite_client.song_data()?).await?;
    let mut songs_tag: futures::lock::MutexGuard<'_, SongsTag> = songs_tag.lock().await;
    *songs_tag = SongsTag::new();

    Ok(Response::Ok {
        tag: None,
        body: (),
    })
}
