use super::table;
use serde::Deserialize;

use serde;

#[derive(Deserialize, Debug)]
pub struct Header {
    pub data_url: String,
    pub name: String,
    pub symbol: String,
    grade: Option<Vec<Grade>>,
    course: Option<Vec<Course>>,
    level_order: Option<Vec<String>>,
    tag: Option<String>,
    update: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct Chart {
    title: String,
    artist: String,
    md5: String,
    level: String,
}

impl Chart {
    pub fn to_chart(&self) -> table::Chart {
        table::Chart::new(
            self.title.clone(),
            self.artist.clone(),
            (&self.md5).parse().unwrap(),
            self.level.clone(),
        )
    }
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
