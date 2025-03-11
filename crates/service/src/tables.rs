use crate::Response;
use anyhow::Result;
use model::{TablesFormat, TablesInfo};

pub async fn get(tables: TablesInfo, tag: Option<String>) -> Result<Response<TablesFormat>> {
    if tables.tag == tag {
        // 変更がない場合、ステータスコードだけを返す
        log::info!("table_handler ETag matched: {:?}", tag);
        Ok(Response::Cached {
            tag: tables.get_tag().to_string(),
        })
    } else {
        log::info!("table_handler ETag unmatched: {:?}", tag);
        // テーブル情報をJSONとして返す
        Ok(Response::Ok {
            tag: Some(tables.get_tag().to_string()),
            body: TablesFormat::format(&tables.tables),
        })
    }
}
