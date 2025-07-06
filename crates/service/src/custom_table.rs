use crate::Response;
use anyhow::Result;
use model::{Chart, CustomTableHeader, TableId, UserId};
use repository::{AccountByUserId, GetTables, ScoresByAccount, SongDataForTables};

pub async fn header<T: GetTables>(
    _user_id: i32,
    table_id: TableId,
    tables: T,
) -> Result<Response<CustomTableHeader>> {
    let tables = tables.get().await;
    let table = tables.get_by_id(table_id).unwrap();
    let header: CustomTableHeader =
        CustomTableHeader::from(table).set_name(format!("おすすめ譜面表: {}", table.title()));
    Ok(Response::Ok {
        tag: None,
        body: header,
    })
}

pub async fn body<C: AccountByUserId + ScoresByAccount + SongDataForTables, T: GetTables>(
    user_id: i32,
    table_id: TableId,
    tables: T,
    mut repos: C,
) -> Result<Response<Vec<Chart>>> {
    let tables = tables.get().await;
    let songs = repos.song_data(&tables.tables).await?;

    let account = repos.user(UserId::new(user_id)).await?;
    let score = repos.score(&account).await?;
    let table = tables.get_by_id(table_id).unwrap();
    let charts = table
        .filter_score(&score, &songs)
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    Ok(Response::Ok {
        tag: None,
        body: charts,
    })
}
