mod config;
mod schema;

use config::config;
use model::*;
use scraper::{Html, Selector};
use url::Url;

pub async fn from_web() -> Tables {
    let mut tables = Vec::new();
    for url in config().table_urls {
        match make_table(url.parse().unwrap()).await {
            Ok(r) => tables.push(r),
            Err(e) => eprintln!("{}", e),
        }
    }
    Tables::make(tables)
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

    let charts = Charts::make(data.iter().map(|c| c.to_chart()).collect());
    let order = match header.level_order {
        Some(s) => s,
        None => charts.get_levels().iter().map(Level::to_string).collect(),
    };
    let levels = charts.make_levels(&order);

    Ok(Table::make(header.name, header.symbol, levels))
}
