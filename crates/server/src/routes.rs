use crate::filter::*;
use crate::handler::*;
use model::Tables;
use mysql::MySqlPool;
use std::collections::HashMap;
use warp::filters::BoxedFilter;
use warp::path;
use warp::{Filter, Reply};

pub fn all_routes(db_pool: &MySqlPool, t: &Tables) -> BoxedFilter<(impl Reply,)> {
    let tables_route = tables(&t);
    let health_route = health(&db_pool);
    let account_route = account(&db_pool);
    let change_name_route = change_name(&db_pool);
    let logout_route = logout();
    let my_detail_route = my_detail(&db_pool, &t);
    let detail_route = detail(&db_pool, &t);
    let score_upload_route = score_upload(&db_pool);
    let score_log_upload_route = score_log_upload(&db_pool);
    let song_data_upload_route = song_data_upload_route(&db_pool);
    let oauth_redirect_route = oauth_redirect_route(&db_pool);
    health_route
        .or(account_route)
        .or(change_name_route)
        .or(logout_route)
        .or(tables_route)
        .or(detail_route)
        .or(my_detail_route)
        .or(score_upload_route)
        .or(score_log_upload_route)
        .or(song_data_upload_route)
        .or(oauth_redirect_route)
        .boxed()
}

fn tables(tables: &Tables) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("tables"))
        .and(with_table(tables.clone()))
        .and_then(table_handler)
        .boxed()
}

fn health(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("health"))
        .and(with_db(db_pool.clone()))
        .and_then(health::health_handler)
        .boxed()
}

fn account(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("account"))
        .and(with_db(db_pool.clone()))
        .and(receive_session_key())
        .and_then(account_handler)
        .boxed()
}

fn change_name(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path("update"))
        .and(path("name"))
        .and(with_db(db_pool.clone()))
        .and(receive_session_key())
        .and(warp::body::json())
        .and_then(change_name::change_name_handler)
        .boxed()
}

fn logout() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("logout"))
        .and(receive_session_key())
        .and_then(logout_handler)
        .boxed()
}

fn my_detail(db_pool: &MySqlPool, tables: &Tables) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("my_detail"))
        .and(with_db(db_pool.clone()))
        .and(with_table(tables.clone()))
        .and(receive_session_key())
        .and(warp::query::<HashMap<String, String>>())
        .and_then(detail::my_detail_handler)
        .boxed()
}

fn detail(db_pool: &MySqlPool, tables: &Tables) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("detail"))
        .and(with_db(db_pool.clone()))
        .and(with_table(tables.clone()))
        .and(warp::query::<HashMap<String, String>>())
        .and_then(detail::detail_handler)
        .boxed()
}

fn score_upload(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path("upload"))
        .and(path("score"))
        .and(with_db(db_pool.clone()))
        .and(receive_sqlite_file())
        .and(receive_session_key())
        .and_then(upload::upload_score_handler)
        .boxed()
}

fn score_log_upload(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path("upload"))
        .and(path("score_log"))
        .and(with_db(db_pool.clone()))
        .and(receive_sqlite_file())
        .and(receive_session_key())
        .and_then(upload::upload_score_log_handler)
        .boxed()
}

fn song_data_upload_route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path("upload"))
        .and(path("song_data"))
        .and(with_db(db_pool.clone()))
        .and(receive_sqlite_file())
        .and(receive_session_key())
        .and_then(upload::upload_song_data_handler)
        .boxed()
}

fn oauth_redirect_route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("oauth"))
        .and(with_db(db_pool.clone()))
        .and(google_oauth_code())
        .and_then(oauth_redirect::oauth_handler)
        .boxed()
}
