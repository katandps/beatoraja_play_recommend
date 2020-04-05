use crate::file;
use crate::song::hash::HashMd5;
use scraper::{Html, Selector};
use std::fmt;
use url::Url;

pub struct Table {
    name: String,
    symbol: String,
    charts: Charts,
}

pub struct Charts {
    pub charts: Vec<Chart>,
}

#[derive(Clone, PartialEq)]
pub struct Chart {
    title: String,
    artist: String,
    pub md5: HashMd5,
    level: String,
}

impl Table {
    pub fn new() -> Table {
        Table {
            name: "Not Loaded".to_string(),
            symbol: "".to_string(),
            charts: Charts { charts: Vec::new() },
        }
    }
    pub fn make(name: impl Into<String>, symbol: impl Into<String>, charts: Charts) -> Table {
        Table {
            name: name.into(),
            symbol: symbol.into(),
            charts,
        }
    }
    pub fn level_specified(&self, level: String) -> Table {
        Table::make(&self.name, &self.symbol, self.charts.level_specified(level))
    }

    pub fn get_charts(&self) -> &Vec<Chart> {
        &self.charts.charts
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}] {}", self.name, self.symbol, self.charts)
    }
}

impl Charts {
    pub fn new(charts: Vec<Chart>) -> Charts {
        Charts { charts }
    }
    pub fn level_specified(&self, level: String) -> Charts {
        let charts = self
            .charts
            .iter()
            .filter_map(|c| if c.level == level { Some(c) } else { None })
            .cloned()
            .collect();
        Charts::new(charts)
    }
}

impl Eq for Chart {}

impl fmt::Display for Charts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec: Vec<String> = self.charts.iter().map(|c| c.string()).collect();
        write!(f, "{}", vec.join("\n"))
    }
}

impl Chart {
    pub fn new(title: String, artist: String, md5: HashMd5, level: String) -> Chart {
        Chart {
            title,
            artist,
            md5,
            level,
        }
    }

    pub fn string(&self) -> String {
        format!("{}: {}, {}", self.title, self.artist, self.md5)
    }
}

impl fmt::Display for Chart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.level, self.title)
    }
}

#[tokio::main]
pub async fn make_table(table_url: String) -> Result<Table, reqwest::Error> {
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
        .json::<file::Header>()
        .await?;
    let data_url = header_url.join(header.data_url.as_ref()).unwrap();
    println!("{}", &data_url.to_string());
    let data_res = reqwest::get(&data_url.to_string()).await?;
    let data = data_res.json::<Vec<file::Chart>>().await?;

    let table = Table::make(
        header.name,
        header.symbol,
        Charts::new(data.iter().map(|c| c.to_chart()).collect()),
    );
    Ok(table)
}
