use crate::error::HandleError;
use bytes::Buf;
use futures::lock::Mutex;
use model::*;
use mysql::{MySQLClient, MySqlPool};
use repository::GetTables;
use service::songs::SongsTag;
use session::Claims;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use table::TableClient;

use warp::filters::multipart::FormData;
use warp::{Filter, Rejection};

pub fn with_db(
    db_pool: &MySqlPool,
) -> impl Filter<Extract = (MySQLClient,), Error = Infallible> + Clone {
    let db_pool = db_pool.clone();
    warp::any().map(move || MySQLClient::new(db_pool.get().unwrap()))
}

pub fn with_table(
    tables: &TableClient,
) -> impl Filter<Extract = (TablesInfo,), Error = Infallible> + Clone {
    let tables = tables.get().clone();
    warp::any().map(move || tables.clone())
}

pub fn with_songs_tag(
    songs_tag: &Arc<Mutex<SongsTag>>,
) -> impl Filter<Extract = (Arc<Mutex<SongsTag>>,), Error = Infallible> + Clone {
    let songs_tag = Arc::clone(songs_tag);
    warp::any().map(move || songs_tag.clone())
}

pub fn with_cache_tag() -> impl Filter<Extract = (Option<String>,), Error = Rejection> + Clone {
    warp::header::optional::<String>("If-None-Match")
}

pub fn receive_sqlite_file(
) -> impl Filter<Extract = (HashMap<String, Vec<u8>>,), Error = Rejection> + Clone {
    async fn parse(form: FormData) -> HashMap<String, Vec<u8>> {
        use futures::TryStreamExt;
        let res = <FormData as TryStreamExt>::and_then(form, |mut part| async move {
            let name = part.name().to_string();
            log::info!("{name}");
            let mut data: Vec<u8> = Vec::new();
            while let Some(content) = part.data().await {
                data.extend_from_slice(content.unwrap().chunk());
            }
            Ok((name, data))
        })
        .try_collect()
        .await
        .unwrap();
        res
    }

    warp::multipart::form()
        .max_length(100 * 1024 * 1024)
        .then(parse)
}

pub fn with_login() -> impl Filter<Extract = (Claims,), Error = Rejection> + Clone {
    async fn parse(jwt: String) -> Claims {
        session::verify_session_jwt(&jwt).unwrap()
    }
    warp::header::<String>(crate::SESSION_KEY).then(parse)
}

pub fn changed_name_by_query() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    warp::body::json().and_then(get_changed_name_query)
}

pub fn changed_visibility_by_query() -> impl Filter<Extract = (bool,), Error = Rejection> + Clone {
    warp::body::json().and_then(get_changed_visibility_query)
}

async fn get_changed_name_query(body: HashMap<String, String>) -> Result<String, Rejection> {
    let changed_name = body
        .get("changed_name")
        .ok_or(HandleError::ChangedNameNotFound)?;
    Ok(changed_name.clone())
}

async fn get_changed_visibility_query(body: HashMap<String, String>) -> Result<bool, Rejection> {
    let changed_visibility = body
        .get("visibility")
        .ok_or(HandleError::ChangedVisibilityNotFound)?;
    Ok(changed_visibility == &"true".to_string())
}
