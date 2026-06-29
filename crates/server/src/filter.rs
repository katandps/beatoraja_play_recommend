use bytes::Buf;
use futures::lock::Mutex;
use mysql::{MySQLClient, MySqlPool};
use service::songs::SongsTag;
use session::Claims;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use table::TableClient;
use warp::filters::BoxedFilter;
use warp::reject;

use warp::filters::multipart::FormData;
use warp::{Filter, Rejection};

#[derive(Debug)]
pub struct Unauthorized;
impl reject::Reject for Unauthorized {}

#[derive(Debug)]
pub struct DbUnavailable;
impl reject::Reject for DbUnavailable {}

#[derive(Debug)]
pub struct InvalidMultipart;
impl reject::Reject for InvalidMultipart {}

pub fn with_db(
    db_pool: &MySqlPool,
) -> impl Filter<Extract = (MySQLClient,), Error = Rejection> + Clone {
    let db_pool = db_pool.clone();
    warp::any().and_then(move || {
        let db_pool = db_pool.clone();
        async move {
            db_pool.get().map(MySQLClient::new).map_err(|e| {
                log::error!("db connection error: {e}");
                reject::custom(DbUnavailable)
            })
        }
    })
}

pub fn with_table(tables: TableClient) -> BoxedFilter<(TableClient,)> {
    warp::any().map(move || tables.clone()).boxed()
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
    async fn parse(form: FormData) -> Result<HashMap<String, Vec<u8>>, Rejection> {
        use futures::TryStreamExt;
        <FormData as TryStreamExt>::and_then(form, |mut part| async move {
            let name = part.name().to_string();
            log::info!("{name}");
            let mut data: Vec<u8> = Vec::new();
            while let Some(content) = part.data().await {
                let content = content?;
                data.extend_from_slice(content.chunk());
            }
            Ok((name, data))
        })
        .try_collect()
        .await
        .map_err(|e| {
            log::error!("multipart parse error: {e}");
            reject::custom(InvalidMultipart)
        })
    }

    warp::multipart::form()
        .max_length(100 * 1024 * 1024)
        .and_then(parse)
}

pub fn with_login() -> impl Filter<Extract = (Claims,), Error = Rejection> + Clone {
    async fn parse(jwt: String) -> Result<Claims, Rejection> {
        session::verify_session_jwt(&jwt).map_err(|e| {
            log::warn!("invalid session token: {e}");
            reject::custom(Unauthorized)
        })
    }
    warp::header::<String>(crate::SESSION_KEY).and_then(parse)
}
