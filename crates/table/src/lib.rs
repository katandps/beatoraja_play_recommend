mod config;

use anyhow::anyhow;
use config::config;
use futures::stream::StreamExt;
use model::*;
use reqwest::header::{HeaderValue, USER_AGENT};
use reqwest::Client;
use scraper::{Html, Selector};
use thiserror::Error;
use url::Url;
use TableParseError::*;

pub async fn from_web(table: &mut Tables) {
    futures::stream::iter(config().table_urls.clone())
        .then(make_table)
        .enumerate()
        .for_each(|(i, t)| {
            match t {
                Ok(t) => table.update(i, t),
                Err(e) => {
                    log::error!("Failed:{} {:?}", config().table_urls[i], e)
                }
            };
            futures::future::ready(())
        })
        .await;
}

async fn make_table(url: String) -> Result<Table, TableParseError> {
    let header_url = get_header_url(&url).await?;
    let header = get_header(&header_url).await?;
    let data_url = make_data_url(&header_url, &header)?;
    let charts = get_charts(&data_url).await?;
    let levels = make_levels(&header, charts);
    Ok(Table::make(header.name, header.symbol, levels))
}

async fn get_header_url(url: &str) -> Result<Url, TableParseError> {
    let res = Client::default()
        .get(url)
        .header(USER_AGENT, HeaderValue::from_static(""))
        .send()
        .await
        .map_err(FailedToAccessTableURL)?;
    let body = res.text().await.map_err(FailedToGetTableURL)?;

    let selector = Selector::parse(r#"meta[name="bmstable"]"#).expect("Selector is invalid.");
    let document = Html::parse_document(&body);
    if let Some(element) = document.select(&selector).next() {
        let mut header_url = Url::parse(url).map_err(InvalidURL)?;

        let header_url_fragment = element.value().attr("content").ok_or(NotFoundHeaderURL)?;
        header_url = header_url
            .join(header_url_fragment)
            .map_err(InvalidHeaderURL)?;
        Ok(header_url)
    } else {
        Err(NotFoundCSS(anyhow!("Not Found 'bmstable' meta selector")))
    }
}

async fn get_header(url: &Url) -> Result<Header, TableParseError> {
    let header_text: String = Client::default()
        .get(&url.to_string())
        .header(USER_AGENT, HeaderValue::from_static(""))
        .send()
        .await
        .map_err(FailedToAccessHeaderURL)?
        .text()
        .await
        .map_err(FailedToAccessHeaderURL)?;
    serde_json::from_str(header_text.trim_start_matches('\u{feff}')).map_err(FailedToParseHeader)
}

fn make_data_url(header_url: &Url, header: &Header) -> Result<Url, TableParseError> {
    header_url
        .join(header.data_url.as_ref())
        .map_err(InvalidDataURL)
}

async fn get_charts(url: &Url) -> Result<Vec<Chart>, TableParseError> {
    let data_text = Client::default()
        .get(&url.to_string())
        .header(USER_AGENT, HeaderValue::from_static(""))
        .send()
        .await
        .map_err(FailedToAccessDataURL)?
        .text()
        .await
        .map_err(FailedToGetDataURL)?;
    let data_text = data_text.trim_start_matches('\u{feff}');
    Ok(serde_json::from_str(data_text).map_err(FailedToParseData)?)
}

fn make_levels(header: &Header, charts: Vec<Chart>) -> TableLevels {
    let charts = Charts::make(charts.into_iter().map(|c| c.into()).collect());
    let order = match &header.level_order {
        Some(s) => s.clone(),
        None => charts
            .get_levels()
            .iter()
            .map(|level| LevelVariant::Str(level.to_string()))
            .collect(),
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

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Header {
    pub data_url: String,
    pub name: String,
    pub symbol: String,
    grade: Option<Vec<Grade>>,
    course: Option<Vec<Vec<Course>>>,
    pub level_order: Option<Vec<LevelVariant>>,
    tag: Option<String>,
    update: Option<i64>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Grade {
    name: Option<String>,
    style: Option<String>,
    md5: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Course {
    name: Option<String>,
    constraint: Option<Vec<String>>,
    trophy: Option<Vec<Trophy>>,
    style: Option<String>,
    md5: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Trophy {
    name: Option<String>,
    missrate: Option<f32>,
    scorerate: Option<f32>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Chart {
    title: String,
    artist: Option<String>,
    md5: HashMd5,
    level: Value,
    url: Option<String>,
    url_diff: Option<String>,
    comment: Option<String>,
}

impl From<Chart> for model::Chart {
    fn from(chart: Chart) -> Self {
        model::Chart::new(
            chart.title,
            chart.artist,
            chart.md5,
            match chart.level {
                Value::String(s) => s,
                p => p.to_string(),
            },
            chart.url,
            chart.url_diff,
            chart.comment,
        )
    }
}
