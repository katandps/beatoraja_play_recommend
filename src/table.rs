use crate::file;
use crate::song::artist::Artist;
use crate::song::hash::HashMd5;
use crate::song::level::Level;
use crate::song::title::Title;
use scraper::{Html, Selector};
use std::collections::HashSet;
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
    title: Title,
    artist: Artist,
    pub md5: HashMd5,
    level: Level,
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
    pub fn level_specified(&self, level: &Level) -> Table {
        Table::make(&self.name, &self.symbol, self.charts.level_specified(level))
    }

    pub fn get_levels(&self) -> Vec<Level> {
        self.charts.get_levels()
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
    pub fn level_specified(&self, level: &Level) -> Charts {
        let charts = self
            .charts
            .iter()
            .filter_map(|c| if &c.level == level { Some(c) } else { None })
            .cloned()
            .collect();
        Charts::new(charts)
    }

    pub fn get_levels(&self) -> Vec<Level> {
        let mut set = HashSet::new();
        for level in self.charts.iter().map(|c| c.level.clone()) {
            set.insert(level);
        }
        let mut vec: Vec<Level> = set.iter().cloned().collect();
        vec.sort_unstable();
        vec
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
            title: Title::make(title),
            artist: Artist::make(artist),
            md5,
            level: Level::make(level),
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

    let header_text: String = reqwest::get(&header_url.to_string()).await?.text().await?;
    let header: file::Header =
        serde_json::from_str(header_text.trim_start_matches('\u{feff}')).unwrap();

    let data_url = header_url.join(header.data_url.as_ref()).unwrap();
    let data_text = reqwest::get(&data_url.to_string()).await?.text().await?;
    let data: Vec<file::Chart> =
        serde_json::from_str(data_text.trim_start_matches('\u{feff}')).unwrap();

    let table = Table::make(
        header.name,
        header.symbol,
        Charts::new(data.iter().map(|c| c.to_chart()).collect()),
    );
    Ok(table)
}

#[cfg(test)]
mod text {
    use crate::table::make_table;

//    #[test]
    fn test() {
    let table = make_table(
        "http://walkure.net/hakkyou/for_glassist/bms/?lamp=fc"
            .parse()
            .unwrap(),
    ).unwrap();
    println!("{}", table.name())
}
}
