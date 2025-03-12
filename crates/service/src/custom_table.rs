use crate::Response;
use anyhow::Result;
use model::{CustomTableHeader, TablesInfo};

pub async fn header(
    _user_id: i32,
    table_index: usize,
    tables: TablesInfo,
) -> Result<Response<CustomTableHeader>> {
    let table = tables.tables.get(table_index).unwrap();
    let header =
        CustomTableHeader::from(table).set_name(format!("おすすめ譜面表: {}", table.title()));
    Ok(Response::Ok {
        tag: None,
        body: header,
    })
}
