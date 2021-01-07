use diesel::r2d2::ConnectionManager;
use diesel::{Connection, MysqlConnection};
use r2d2::Pool;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

pub async fn health_handler(
    db_pool: Pool<ConnectionManager<MysqlConnection>>,
) -> std::result::Result<impl Reply, Rejection> {
    match db_pool.get() {
        Ok(db) => match db.execute("SELECT 1") {
            Ok(_) => Ok(StatusCode::OK),
            Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
