mod config;

use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context};
use config::{config, TableSetting};
use futures::stream::StreamExt;
use model::*;
use rand::distributions::{Alphanumeric, DistString};
use reqwest::header::{HeaderValue, USER_AGENT};
use reqwest::Client;
use scraper::{Html, Selector};
use thiserror::Error;
use url::Url;
use TableParseError::*;

pub async fn from_web(tables_info: &mut TablesInfo) {
    futures::stream::iter(config().tables.clone())
        .then(make_table)
        .enumerate()
        .for_each(|(i, t)| {
            match t {
                Ok(t) => {
                    tables_info.tables.update(i, t);
                    let mut rng = rand::thread_rng();
                    let random_code = Alphanumeric.sample_string(&mut rng, 24);
                    tables_info.update_tag(random_code);
                }
                Err(e) => {
                    log::error!("Failed:{} {:?}", config().tables[i].title, e)
                }
            };
            futures::future::ready(())
        })
        .await;
}

pub async fn from_with_cache(tables_info: &mut TablesInfo) {
    futures::stream::iter(config().tables.clone())
        .then(read_table)
        .enumerate()
        .for_each(|(i, t)| {
            match t {
                Ok(t) => {
                    tables_info.tables.update(i, t);
                    let mut rng = rand::thread_rng();
                    let random_code = Alphanumeric.sample_string(&mut rng, 24);
                    tables_info.update_tag(random_code);
                }
                Err(e) => {
                    log::error!("Failed:{} {:?}", config().tables[i].title, e)
                }
            };
            futures::future::ready(())
        })
        .await;
}

async fn read_table(setting: TableSetting) -> anyhow::Result<Table> {
    let cache_path = create_cache_dir(&setting)?;
    log::info!("cache_path: {:?}", cache_path);
    if let Ok(table) = read_cache(&cache_path) {
        Ok(table)
    } else {
        let table = fetch(&setting).await?;
        save_cache(&cache_path, &table)?;
        Ok(table)
    }
}

async fn make_table(setting: TableSetting) -> anyhow::Result<Table> {
    let cache_path = create_cache_dir(&setting)?;
    let table = fetch(&setting).await?;
    save_cache(&cache_path, &table)?;
    Ok(table)
}

async fn fetch(setting: &TableSetting) -> anyhow::Result<Table> {
    let header_url = get_header_url(&setting.url).await?;
    let header = get_header(&header_url).await?;
    let data_url = make_data_url(&header_url, &header)?;
    let charts = get_charts(&data_url).await?;
    let levels = make_levels(&header, charts);
    Ok(Table::make(header.name, header.symbol, levels))
}

fn create_cache_dir(setting: &TableSetting) -> anyhow::Result<PathBuf> {
    let mut cache_path = dirs::cache_dir().unwrap_or_else(std::env::temp_dir);
    cache_path.push("beatoraja_play_recommend");
    cache_path.push("tables");
    create_dir_all(&cache_path).with_context(|| "could not create table cache directory")?;
    cache_path.push(format!("{}.json", setting.id.to_string()));
    Ok(cache_path)
}

fn read_cache(cache_path: &Path) -> anyhow::Result<Table> {
    Ok(serde_json::from_str(&std::fs::read_to_string(cache_path)?)?)
}

fn save_cache(cache_path: &Path, table: &Table) -> anyhow::Result<()> {
    let content = serde_json::to_string(table)?;
    std::fs::write(cache_path, content)?;
    Ok(())
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
        .get(url.to_string())
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
        .get(url.to_string())
        .header(USER_AGENT, HeaderValue::from_static(""))
        .send()
        .await
        .map_err(FailedToAccessDataURL)?
        .text()
        .await
        .map_err(FailedToGetDataURL)?;
    let data_text = data_text.trim_start_matches('\u{feff}');
    serde_json::from_str(data_text).map_err(FailedToParseData)
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
    // grade: Option<Vec<Grade>>,
    // course: Option<Vec<Vec<Course>>>,
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

#[cfg(test)]
mod test {
    use crate::Header;

    #[test]
    fn test_header() {
        let header_text = r#"{
        "name": "Pastoral難易度表",
        "symbol": "Ps",
        "data_url": "https://script.google.com/macros/s/AKfycbxtccaKedbEmOvw5IKxoPcHAy6FA7JvH1fxVWptSvP5CH1bkJNZPqt_5FvPJ_ntezQ8/exec",
        "level_order": ["-20","-19","-18","-17","-16","-15","-14","-13","-12","-11","-10","-9","-8","-7","-6","-5","-4","-3","-2","-1",0,1,2,3,4,5,6,7,8,9,10]}"#;
        let header: Header =
            serde_json::from_str(header_text.trim_start_matches('\u{feff}')).unwrap();
        dbg!(header);
    }

    #[test]
    fn test_header2() {
        let header_text = r#"{
    "name": "発狂PMS難易度表",
    "symbol": "●",
    "data_url": "https://script.google.com/macros/s/AKfycbzob0GvajfzDm_IwppcW9RH-wRIwZh19kUfhZomyaU_Kwcvq12iJ2nhrG7fPD1yaAvV/exec",
    "level_order": [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,"!i"],
"course":[
{"name":"PMS段位認定 2024 発狂初段","constraint":["grade_random","gauge_lr2","ln",""],"md5": ["ebd15232e653757f26159e33cf257381","6605d690699319f6539babf6e9f5619b","e0c3733087942a435daa90a92519ce03","d8da787d7830fd703ad7648646e8db0d"],"trophy": [{"missrate":3.0,"name":"bronzemedal","scorerate":89.5},{"missrate":2.0,"name":"silvermedal","scorerate":94.5},{"missrate":1.0,"name":"goldmedal","scorerate":97.5}]},
{"name":"PMS段位認定 2024 発狂二段","constraint":["grade_random","gauge_lr2","ln",""],"md5": ["5287ff3bee73665913cdafa79f3e21ca","215239f1cac8368a3c75a47d961e8d0f","7fde7c269011d7f4c403d2b5ee099ae5","30ded05ce08b4bf15e72af26ed5c8700"],"trophy": [{"missrate":3.0,"name":"bronzemedal","scorerate":89.5},{"missrate":2.0,"name":"silvermedal","scorerate":94.5},{"missrate":1.0,"name":"goldmedal","scorerate":97.5}]},
{"name":"PMS段位認定 2024 発狂三段","constraint":["grade_random","gauge_lr2","ln",""],"md5": ["f8bf024bfcb2737d701a7ec309bacb42","c3985223159b14a42cf7c5d4418e47b6","d8a06f5e97485e10220fd7b72fd7dfdc","3dace5dfc3d2004c77cdedb9f8ffaa67"],"trophy": [{"missrate":3.0,"name":"bronzemedal","scorerate":89.5},{"missrate":2.0,"name":"silvermedal","scorerate":94.5},{"missrate":1.0,"name":"goldmedal","scorerate":97.5}]},
{"name":"PMS段位認定 2024 発狂四段","constraint":["grade_random","gauge_lr2","ln",""],"md5": ["ddd80b760636085c261bda3c0e0dcbeb","fe0ee38640dbde10df3c07a1a8ada416","217300d971af65d055960c739e6a4fe6","42e6e4c9fdd8fb97dd3893f80dc2c1d4"],"trophy": [{"missrate":3.0,"name":"bronzemedal","scorerate":89.5},{"missrate":2.0,"name":"silvermedal","scorerate":94.5},{"missrate":1.0,"name":"goldmedal","scorerate":97.5}]},
{"name":"PMS段位認定 2024 発狂五段","constraint":["grade_random","gauge_lr2","ln",""],"md5": ["36b5a08ffe47ff52c4185ad6b9bb099e","f4e7a902afc8f93cf7260124ed9ee7b7","bfb46d0789f82e31811a6e6c6852d5d1","19899adf6b16e73984ae85ec7388577f"],"trophy": [{"missrate":3.0,"name":"bronzemedal","scorerate":89.5},{"missrate":2.0,"name":"silvermedal","scorerate":94.5},{"missrate":1.0,"name":"goldmedal","scorerate":97.5}]},
{"name":"PMS段位認定 2024 発狂六段","constraint":["grade_random","gauge_lr2","ln",""],"md5": ["a2b83f7d7e76350409e9f793f8abc1f0","ce2ac318710b4e01fe9562b39626e9ed","6c28f8ff4925053b7f0ffeb28e309079","2a4db5f54b50f0ab26061bcb552f9284"],"trophy": [{"missrate":3.0,"name":"bronzemedal","scorerate":89.5},{"missrate":2.0,"name":"silvermedal","scorerate":94.5},{"missrate":1.0,"name":"goldmedal","scorerate":97.5}]},
{"name":"PMS段位認定 2024 発狂七段","constraint":["grade_random","gauge_lr2","ln",""],"md5": ["c71c7ee543723ced2f67706a59a91cdb","c2b9b77078a499ae1d5cd5c5bda3917a","cfbe5e24b6e236da273cf4589dbe850f","ce019d4053ecd877053c149740ab7fb8"],"trophy": [{"missrate":3.0,"name":"bronzemedal","scorerate":89.5},{"missrate":2.0,"name":"silvermedal","scorerate":94.5},{"missrate":1.0,"name":"goldmedal","scorerate":97.5}]},
{"name":"PMS段位認定 2024 発狂八段","constraint":["grade_random","gauge_lr2","ln",""],"md5": ["810903b9f1c292d5df82d9a8c7763d41","64fc36e58c67dafae944c0a8e70b68c9","e28062db1d2a19da0d8ea1e197bac6c8","4fd819f39b4372939aeb420eb3a3e501"],"trophy": [{"missrate":3.0,"name":"bronzemedal","scorerate":89.5},{"missrate":2.0,"name":"silvermedal","scorerate":94.5},{"missrate":1.0,"name":"goldmedal","scorerate":97.5}]},
{"name":"PMS段位認定 2024 発狂九段","constraint":["grade_random","gauge_lr2","ln",""],"md5": ["50a7786ad5f3a064bf46fd5423973089","5ab80f1ccfe579445bbf85a64f8fa866","d142e382b77459d238c1c78032579683","2cb6c23aee35e1f02fcf25e1976ba3b0"],"trophy": [{"missrate":3.0,"name":"bronzemedal","scorerate":89.5},{"missrate":2.0,"name":"silvermedal","scorerate":94.5},{"missrate":1.0,"name":"goldmedal","scorerate":97.5}]},
{"name":"PMS段位認定 2024 発狂十段","constraint":["grade_random","gauge_lr2","ln",""],"md5": ["3368cfa7d3a764c36236f980b12104c7","d75c2a2272baf4bae2ca194eca9877df","2b9f52a444e939b7a5ab881b75952499","fe8d94a35e91b9ce56241a7c539ee19d"],"trophy": [{"missrate":3.0,"name":"bronzemedal","scorerate":89.5},{"missrate":2.0,"name":"silvermedal","scorerate":94.5},{"missrate":1.0,"name":"goldmedal","scorerate":97.5}]},
{"name":"PMS段位認定 2024 発狂中伝","constraint":["grade_random","gauge_lr2","ln",""],"md5": ["e0f9cecc358de519502a88178f67927b","2cd337acbf97911d41a20288cec60bfc","e57b19972be4d3020bb57f677c777a14","5d3309cd6e0a9b39238683fac033681e"],"trophy": [{"missrate":3.0,"name":"bronzemedal","scorerate":89.5},{"missrate":2.0,"name":"silvermedal","scorerate":94.5},{"missrate":1.0,"name":"goldmedal","scorerate":97.5}]},
{"name":"PMS段位認定 2024 発狂皆伝","constraint":["grade_random","gauge_lr2","ln",""],"md5": ["0c12123c37587fc6ff507a41697f08b1","bfd12f281b7d2e5d432e9fccbaaffc38","1676a09d06d39015f5da8d15de96e6ec","eacc01b65b14c8ceb54b55cd6f9bbfda"],"trophy": [{"missrate":3.0,"name":"bronzemedal","scorerate":89.5},{"missrate":2.0,"name":"silvermedal","scorerate":94.5},{"missrate":1.0,"name":"goldmedal","scorerate":97.5}]},
{"name":"PMS段位認定 2024 Overjoy","constraint":["grade_random","gauge_lr2","ln",""],"md5": ["a55e669bcb5943fb800cc19a4c67adb2","04b7dbbf9f816b29830be8fc37cab2b2","4e1fa3a614c8c90bd28eb6f105901b78","a24b92a298b13322a407e1f9d946e443"],"trophy": [{"missrate":3.0,"name":"bronzemedal","scorerate":89.5},{"missrate":2.0,"name":"silvermedal","scorerate":94.5},{"missrate":1.0,"name":"goldmedal","scorerate":97.5}]},
{"name":"PMS段位認定 2024 Uberjoy","constraint":["grade_random","gauge_lr2","ln",""],"md5": ["f28dea87dd4faf8c65ee4ae79fd32bce","d8813b9213460afbe361cf8686cdcdcd","4bf1e3cfeafc540237ea1a74042d9c98","760d9d362aeb393839a3097c5ccc163e"],"trophy": [{"missrate":3.0,"name":"bronzemedal","scorerate":89.5},{"missrate":2.0,"name":"silvermedal","scorerate":94.5},{"missrate":1.0,"name":"goldmedal","scorerate":97.5}]}
]}
"#;
        let header: Header =
            serde_json::from_str(header_text.trim_start_matches('\u{feff}')).unwrap();
        dbg!(header);
    }
}
