use std::fs::File;
use std::io::Read;
use serde::Deserialize;

extern crate serde;
extern crate serde_json;

pub fn run() {
    let mut file = File::open("./files/satellite/header.json").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let header = serde_json::from_str::<Header>(&contents).unwrap();

    println!("{}", header.to_string())
}

#[derive(Deserialize)]
struct Header {
    name: String,
    symbol: String,
    data_url: String,
}

impl Header {
    fn to_string(&self) -> String {
        format!("{} {} {} ", self.name, self.symbol, self.data_url)
    }
}
