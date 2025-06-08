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

use warp::filters::multipart::FormData;
use warp::{Filter, Rejection};

pub fn with_db(
    db_pool: &MySqlPool,
) -> impl Filter<Extract = (MySQLClient,), Error = Infallible> + Clone {
    let db_pool = db_pool.clone();
    warp::any().map(move || MySQLClient::new(db_pool.get().unwrap()))
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
    async fn parse(form: FormData) -> HashMap<String, Vec<u8>> {
        use futures::TryStreamExt;
        <FormData as TryStreamExt>::and_then(form, |mut part| async move {
            let name = part.name().to_string();
            log::info!("{name}");
            let mut data: Vec<u8> = Vec::new();
            while let Some(content) = part.data().await {
                match content {
                    Ok(content) => {
                        log::info!("Received part data of length: {}", content.remaining());
                        data.extend_from_slice(content.chunk());
                    }
                    Err(e) => {
                        log::warn!("Error reading {} part data: {:?}", name, e);
                        continue;
                    }
                }
            }
            Ok((name, data))
        })
        .try_collect()
        .await
        .unwrap()
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
