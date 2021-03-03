use crate::filter::with_db;
use mysql::MySqlPool;
use repository::HealthCheck;
use warp::filters::BoxedFilter;
use warp::http::StatusCode;
use warp::path;
use warp::{Filter, Rejection, Reply};

pub fn health_route(db_pool: &MySqlPool) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(path("health"))
        .and(with_db(&db_pool))
        .and_then(health_handler)
        .boxed()
}

async fn health_handler<C: HealthCheck>(client: C) -> std::result::Result<impl Reply, Rejection> {
    match client.health() {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
