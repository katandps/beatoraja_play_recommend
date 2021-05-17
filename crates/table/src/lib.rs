mod config;

use anyhow::anyhow;
use config::config;
use model::*;
use scraper::{Html, Selector};
use thiserror::Error;
use url::Url;
use TableParseError::*;

pub async fn from_web() -> Tables {
    let mut tables = Vec::new();
    for url in config().table_urls {
        match make_table(url.parse().unwrap()).await {
            Ok(r) => tables.push(r),
            Err(e) => eprintln!("{}", e.to_string()),
        }
    }
    Tables::make(tables)
}

async fn make_table(url: String) -> Result<Table, TableParseError> {
    let header_url = get_header_url(&url).await?;
    let header = get_header(&header_url).await?;
    let data_url = make_data_url(&header_url, &header)?;
    let charts = get_charts(&data_url).await?;
    let levels = make_levels(&header, charts);
    Ok(Table::make(header.name, header.symbol, levels))
}

async fn get_header_url(url: &String) -> Result<Url, TableParseError> {
    let res = reqwest::get(url)
        .await
        .map_err(|e| FailedToAccessTableURL(e))?;
    let body = res.text().await.map_err(|e| FailedToGetTableURL(e))?;

    let selector =
        Selector::parse(r#"meta[name="bmstable"]"#).map_err(|e| NotFoundCSS(anyhow!("{:?}", e)))?;
    let document = Html::parse_document(&body);
    let mut header_url = Url::parse(&url).map_err(|e| InvalidURL(e))?;
    for element in document.select(&selector) {
        let header_url_fragment = element.value().attr("content").ok_or(NotFoundHeaderURL)?;
        header_url = header_url
            .join(header_url_fragment)
            .map_err(|e| InvalidHeaderURL(e))?;
    }
    Ok(header_url)
}

async fn get_header(url: &Url) -> Result<Header, TableParseError> {
    let header_text: String = reqwest::get(&url.to_string())
        .await
        .map_err(|e| FailedToAccessHeaderURL(e))?
        .text()
        .await
        .map_err(|e| FailedToAccessHeaderURL(e))?;
    serde_json::from_str(header_text.trim_start_matches('\u{feff}'))
        .map_err(|e| FailedToParseHeader(e))
}

fn make_data_url(header_url: &Url, header: &Header) -> Result<Url, TableParseError> {
    header_url
        .join(header.data_url.as_ref())
        .map_err(|e| InvalidDataURL(e))
}

async fn get_charts(url: &Url) -> Result<Vec<Chart>, TableParseError> {
    let data_text = reqwest::get(&url.to_string())
        .await
        .map_err(|e| FailedToAccessDataURL(e))?
        .text()
        .await
        .map_err(|e| FailedToGetDataURL(e))?;
    let data_text = data_text.trim_start_matches('\u{feff}');
    Ok(serde_json::from_str(data_text).map_err(|e| FailedToParseData(e))?)
}

fn make_levels(header: &Header, charts: Vec<Chart>) -> TableLevels {
    let charts = Charts::make(charts.into_iter().map(|c| c.into()).collect());
    let order = match &header.level_order {
        Some(s) => s.clone(),
        None => charts.get_levels().iter().map(Level::to_string).collect(),
    };
    charts.make_levels(&order)
}

#[derive(Debug, Error)]
pub enum TableParseError {
    #[error("指定された難易度表URLにアクセスできませんでした: {0:?}")]
    FailedToAccessTableURL(reqwest::Error),
    #[error("指定された難易度表URLからのレスポンスが取得できませんでした: {0:?}")]
    FailedToGetTableURL(reqwest::Error),
    #[error("指定された難易度表に有効なmetaタグが設定されていません: {0:?}")]
    NotFoundCSS(anyhow::Error),
    #[error("難易度表URLが不正です: {0:?}")]
    InvalidURL(url::ParseError),
    #[error("難易度表にヘッダURLがありません")]
    NotFoundHeaderURL,
    #[error("ヘッダURLが不正です: {0:?}")]
    InvalidHeaderURL(url::ParseError),
    #[error("ヘッダURLにアクセスできませんでした: {0:?}")]
    FailedToAccessHeaderURL(reqwest::Error),
    #[error("ヘッダURLからのレスポンスが取得できませんでした: {0:?}")]
    FailedToGetAccessURL(reqwest::Error),
    #[error("ヘッダのレスポンスJSONが解釈できませんでした: {0:?}")]
    FailedToParseHeader(serde_json::Error),
    #[error("データURLが不正です: {0:?}")]
    InvalidDataURL(url::ParseError),
    #[error("データURLにアクセスできませんでした: {0:?}")]
    FailedToAccessDataURL(reqwest::Error),
    #[error("データURLからのレスポンスが取得できませんでした: {0:?}")]
    FailedToGetDataURL(reqwest::Error),
    #[error("データのレスポンスJSONが解釈できませんでした: {0:?}")]
    FailedToParseData(serde_json::Error),
}

use serde;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Header {
    pub data_url: String,
    pub name: String,
    pub symbol: String,
    grade: Option<Vec<Grade>>,
    course: Option<Vec<Vec<Course>>>,
    pub level_order: Option<Vec<String>>,
    tag: Option<String>,
    update: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct Grade {
    name: String,
    style: String,
    md5: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Course {
    name: String,
    constraint: Vec<String>,
    trophy: Vec<Trophy>,
    style: String,
    md5: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Trophy {
    name: String,
    missrate: f32,
    scorerate: f32,
}

#[derive(Deserialize, Debug)]
pub struct Chart {
    title: String,
    artist: Option<String>,
    md5: HashMd5,
    level: Value,
    url: Option<String>,
    url_diff: Option<String>,
    comment: Option<String>,
}

impl Into<model::Chart> for Chart {
    fn into(self) -> model::Chart {
        model::Chart::new(
            self.title,
            self.artist,
            self.md5,
            match self.level {
                Value::String(s) => s,
                p => p.to_string(),
            },
            self.url,
            self.url_diff,
            self.comment,
        )
    }
}
