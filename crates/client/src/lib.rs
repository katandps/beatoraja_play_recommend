use model::Tables;
use wasm_bindgen::prelude::*;

pub struct Data {
    tables: Tables,
}

#[wasm_bindgen]
pub fn test() -> String {
    "hello".to_string()
}
