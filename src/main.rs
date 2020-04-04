mod app;
mod db;
mod file;

pub mod lamp;
pub mod schema;
pub mod score;
pub mod score_log;
pub mod scored_table;
pub mod song;
pub mod song_data;
pub mod table;

#[macro_use]
extern crate diesel;
extern crate scraper;

use crate::app::App;
use crate::file::{Chart, Header};
use crate::table::Table;
use scraper::{Html, Selector};
use std::cell::RefCell;
use std::default::Default;
use std::env;
use std::io::{self, Write};
use std::rc::Rc;
use url::Url;

fn main() {
    env_logger::init();

    //    get_tables();
    let whole_score = db::score();
    let song_data = db::song_data();
    let mut score_log = db::score_log();
    for table in get_tables() {
        let mut app = App {
            table,
            whole_score: &whole_score,
            song_data: &song_data,
            score_log: &mut score_log,
        };
        app.run();
    }
}

fn get_tables() -> Vec<Table> {
    dotenv::dotenv().ok();
    let mut ret = (1..10)
        .flat_map(|i| {
            let url_key = format!("TABLE_URL{}", i);
            let url = env::var(&url_key);
            if url.is_ok() {
                let url = url.unwrap();
                let res = make_table(url);
                Some(res.unwrap())
            } else {
                None
            }
        })
        .collect();
    ret
}

#[tokio::main]
async fn make_table(table_url: String) -> Result<Table, reqwest::Error> {
    let res = reqwest::get(&table_url).await?;
    let body = res.text().await?;

    let selector = Selector::parse(r#"meta[name="bmstable"]"#).unwrap();
    let document = Html::parse_document(&body);
    let mut header_url = Url::parse(&table_url).unwrap();
    for element in document.select(&selector) {
        let header_url_fragment = element.value().attr("content").unwrap();
        header_url = header_url.join(header_url_fragment).unwrap();
    }

    println!("{}", &header_url.to_string());
    let header = reqwest::get(&header_url.to_string())
        .await?
        .json::<Header>()
        .await?;
    let data_url = header_url.join(header.data_url.as_ref()).unwrap();
    println!("{}", &data_url.to_string());
    let data_res = reqwest::get(&data_url.to_string()).await?;
    let data = data_res.json::<Vec<Chart>>().await?;

    let table = table::Table::make(
        header.name,
        header.symbol,
        table::Charts::new(data.iter().map(|c| c.to_chart()).collect()),
    );
    Ok(table)
}
