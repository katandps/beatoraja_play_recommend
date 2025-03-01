use std::sync::Arc;

use anyhow::Result;
use model::{TablesFormat, TablesInfo};
use repository::GetTables;
use tokio::sync::{Mutex, OnceCell};

pub async fn tables() -> &'static Arc<Mutex<TablesInfo>> {
    static INSTANCE: OnceCell<Arc<Mutex<TablesInfo>>> = OnceCell::const_new();
    INSTANCE.get_or_init(init).await
}

async fn init() -> Arc<Mutex<TablesInfo>> {
    let tables = Arc::new(Mutex::new(TablesInfo::default()));
    {
        let mut m = tables.lock().await;
        table::from_with_cache(&mut m).await;
    }
    tables
}

pub async fn table_handler<T: GetTables, A>(
    mut tables: T,
    tag: Option<String>,
) -> Result<Response<TablesFormat>> {
    let tables_info = tables.get().await;
    if tables_info.tag == tag {
        // 変更がない場合、ステータスコードだけを返す
        log::info!("table_handler ETag matched: {:?}", tag);
        Ok(Response::Cached {
            tag: tables_info.get_tag().to_string(),
        })
    } else {
        log::info!("table_handler ETag unmatched: {:?}", tag);
        // テーブル情報をJSONとして返す
        Ok(Response::Ok {
            tag: Some(tables_info.get_tag().to_string()),
            body: TablesFormat::format(&tables_info.tables),
        })
    }
}

#[derive(Clone, Debug)]
pub enum Response<T> {
    Ok { tag: Option<String>, body: T },
    Cached { tag: String },
}
