use std::sync::Arc;

use anyhow::Result;
use futures::lock::Mutex;
use model::RankingQuery;
use model::RankingResponse;
use model::SongFormat;
use repository::GetTables;
use repository::PublishedUsers;
use repository::ScoresBySha256;
use repository::SongDataForTables;

use crate::Response;

pub async fn list<C: SongDataForTables, T: GetTables>(
    mut repos: C,
    tables: T,
    saved_tag: Arc<Mutex<SongsTag>>,
    tag: Option<String>,
) -> Result<Response<Vec<SongFormat>>> {
    let tables = tables.get().await;
    let saved_tag: futures::lock::MutexGuard<'_, SongsTag> = saved_tag.lock().await;

    if saved_tag.is_saved(&tag) {
        // 変更がない場合、ステータスコードだけを返す
        log::info!("songs_handler ETag matched: {:?}", tag);
        Ok(Response::Cached {
            tag: saved_tag.tag.clone(),
        })
    } else {
        let songs = repos.song_data(&tables.tables).await?;
        log::info!("songs_handler ETag unmatched: {:?}", tag);
        Ok(Response::Ok {
            tag: Some(saved_tag.tag.clone()),
            body: songs.get_list(tables.tables.get_charts()),
        })
    }
}
use rand::distr::{Alphanumeric, SampleString};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SongsTag {
    pub tag: String,
}
impl SongsTag {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let random_code = Alphanumeric.sample_string(&mut rng, 24);
        Self { tag: random_code }
    }

    pub fn is_saved(&self, tag: &Option<String>) -> bool {
        match &tag {
            Some(s) => &self.tag == s,
            None => false,
        }
    }
}

pub async fn ranking<C: ScoresBySha256 + PublishedUsers + SongDataForTables, T: GetTables>(
    mut repos: C,
    tables: T,
    query: RankingQuery,
) -> Result<Response<RankingResponse>> {
    let tables = tables.get().await;
    let songs = repos.song_data(&tables.tables).await?;
    let scores = repos.score(&query.sha256).await?;
    let users = repos.fetch_users().await?;
    let response = scores.for_response(&songs, &query.date, &query.sha256, &users);
    Ok(Response::Ok {
        tag: None,
        body: response.unwrap_or_default(),
    })
}
