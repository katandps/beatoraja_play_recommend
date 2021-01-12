use model::{Tables, TablesFormat};
use warp::{Rejection, Reply};

pub async fn table_handler(tables: Tables) -> std::result::Result<impl Reply, Rejection> {
    Ok(serde_json::to_string(&TablesFormat::from(tables)).unwrap())
}
