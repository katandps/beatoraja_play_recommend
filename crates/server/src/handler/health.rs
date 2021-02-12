use repository::HealthCheck;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

pub async fn health_handler<C: HealthCheck>(
    client: C,
) -> std::result::Result<impl Reply, Rejection> {
    match client.health() {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
