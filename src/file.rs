use std::fs::File;
use std::io::Read;
use serde::Deserialize;

use super::table;
use crate::song::HashMd5;

extern crate serde;
extern crate serde_json;

pub fn get_table() -> table::Table {
    let header = get_header();
    let chart = get_charts();
    table::Table::new(
        header.name,
        header.symbol,
        table::Charts::new(
            chart.iter().map(|c| c.to_chart()).collect(),
        ),
    )
}

fn get_header() -> Header {
    let mut file = File::open("./files/satellite/header.json").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    serde_json::from_str::<Header>(&contents).unwrap()
}

fn get_charts() -> Vec<Chart> {
    let mut f = File::open("./files/satellite/score.json").unwrap();

    let mut c = String::new();
    f.read_to_string(&mut c).unwrap();

    serde_json::from_str::<Vec<Chart>>(&c).unwrap()
}

#[derive(Deserialize)]
struct Header {
    name: String,
    symbol: String,
    //data_url: String,
}

#[derive(Deserialize)]
struct Chart {
    title: String,
    artist: String,
    md5: String,
    level: String,
}

impl Chart {
    fn to_chart(&self) -> table::Chart {
        table::Chart::new(
            (&self.title).parse().unwrap(),
            (&self.artist).parse().unwrap(),
            HashMd5::new((&self.md5).parse().unwrap()),
            (&self.level).parse().unwrap(),
        )
    }
}