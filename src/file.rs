use serde::{Deserialize, Deserializer};
use std::fs::File;
use std::io::Read;

use super::table;

use serde;
use serde_json;
use serde_json::Value;

pub fn get_table() -> table::Table {
    let header = get_header();
    let chart = get_charts();
    table::Table::make(
        header.name,
        header.symbol,
        table::Charts::new(chart.iter().map(|c| c.to_chart()).collect()),
    )
}

fn get_header() -> Header {
    let mut file = File::open("./files/satellite/header.json").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents);

    serde_json::from_str::<Header>(&contents).unwrap()
}

fn get_charts() -> Vec<Chart> {
    let mut f = File::open("./files/satellite/score.json").unwrap();

    let mut c = String::new();
    f.read_to_string(&mut c);

    serde_json::from_str::<Vec<Chart>>(&c).unwrap()
}

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
