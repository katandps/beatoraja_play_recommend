mod schema;

use model::*;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::{Read, Write};
use url::Url;

#[macro_use]
extern crate anyhow;

pub fn get_tables(is_local: bool) -> Vec<Table<Charts>> {
    match local(is_local) {
        Ok(t) => t,
        _ => get_from_internet(),
    }
}

fn get_from_internet() -> Vec<Table<Charts>> {
    let tables = config()
        .table_urls()
        .iter()
        .flat_map(|url| make_table(url.parse().unwrap()))
        .collect();
    let mut file = File::create(config().local_cache_url()).unwrap();
    let _ = file.write(serde_json::to_string(&tables).unwrap().as_ref());
    tables
}

fn local(is_local: bool) -> anyhow::Result<Vec<Table<Charts>>> {
    fn load() -> anyhow::Result<Vec<Table<Charts>>> {
        let mut file = File::open(config().local_cache_url())?;
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        let vec = serde_json::from_str(&contents)?;
        Ok(vec)
    }

    match is_local {
        true => load(),
        false => Err(anyhow!("No Local Access")),
    }
}

#[tokio::main]
async fn make_table(table_url: String) -> anyhow::Result<Table<Charts>> {
    let res = reqwest::get(&table_url).await?;
    let body = res.text().await?;

    let selector = Selector::parse(r#"meta[name="bmstable"]"#).unwrap();
    let document = Html::parse_document(&body);
    let mut header_url = Url::parse(&table_url).unwrap();
    for element in document.select(&selector) {
        let header_url_fragment = element.value().attr("content").unwrap();
        header_url = header_url.join(header_url_fragment).unwrap();
    }

    let header_text: String = reqwest::get(&header_url.to_string()).await?.text().await?;
    let header: crate::schema::Header =
        serde_json::from_str(header_text.trim_start_matches('\u{feff}'))?;

    let data_url = header_url.join(header.data_url.as_ref()).unwrap();
    let data_text = reqwest::get(&data_url.to_string()).await?.text().await?;
    let data: Vec<crate::schema::Chart> =
        serde_json::from_str(data_text.trim_start_matches('\u{feff}')).unwrap();

    let table = Table::make(
        header.name,
        header.symbol,
        Charts::make(data.iter().map(|c| c.to_chart()).collect()),
        header.level_order,
    );
    Ok(table)
}

fn config() -> config::Config {
    if cfg!(test) {
        config::Config::Dummy
    } else {
        config::config()
    }
}
