mod schema;

use model::*;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::{Read, Write};
use url::Url;

#[macro_use]
extern crate anyhow;

pub async fn get_tables(is_local: bool) -> Tables {
    match local(is_local) {
        Ok(t) => t,
        _ => from_web().await,
    }
}

async fn from_web() -> Tables {
    let mut tables = Vec::new();
    for url in config().table_urls() {
        match make_table(url.parse().unwrap()).await {
            Ok(r) => tables.push(r),
            Err(e) => eprintln!("{}", e),
        }
    }
    let mut file = File::create(config().local_cache_url()).unwrap();
    let _ = file.write(serde_json::to_string(&tables).unwrap().as_ref());
    Tables::new(tables)
}

fn local(is_local: bool) -> anyhow::Result<Tables> {
    fn load() -> anyhow::Result<Tables> {
        let mut file = File::open(config().local_cache_url())?;
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        let vec = serde_json::from_str(&contents)?;
        Ok(Tables::new(vec))
    }

    match is_local {
        true => load(),
        false => Err(anyhow!("No Local Access")),
    }
}

async fn make_table(table_url: String) -> anyhow::Result<Table> {
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

    Ok(Table::make(
        header.name,
        header.symbol,
        Charts::make(data.iter().map(|c| c.to_chart()).collect()),
        header.level_order,
    ))
}

fn config() -> config::Config {
    if cfg!(test) {
        config::Config::Dummy
    } else {
        config::config()
    }
}
