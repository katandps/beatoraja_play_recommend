use crate::*;
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct Graph<T: Countable> {
    table: String,
    vec: Vec<T>,
    count: Vec<CountByLevel<T>>,
}

#[derive(Deserialize, Serialize)]
pub struct CountByLevel<T: Countable> {
    count: HashMap<T, CountByType>,
}

#[derive(Deserialize, Serialize)]
pub struct CountByType {
    count: i32,
}

impl<T: Countable> Graph<T> {
    pub fn to_string(&self) -> String {
        self.table.clone()
            + "\n"
            + self
                .count
                .iter()
                .map(|l| l.to_string(&self.vec) + "\n")
                .collect::<String>()
                .as_str()
    }

    pub fn make(table: String, count: Vec<CountByLevel<T>>) -> Self {
        Graph {
            table,
            vec: T::vec(),
            count,
        }
    }
}

impl<T: Countable> CountByLevel<T> {
    pub fn to_string(&self, vec: &Vec<T>) -> String {
        vec.iter()
            .flat_map(|c| match self.count.get(c) {
                Some(c) => Some(format!("[{:>3}]", c.to_string())),
                _ => None,
            })
            .collect::<String>()
    }
    pub fn make(summary: Summary<T>) -> CountByLevel<T> {
        CountByLevel {
            count: T::vec()
                .iter()
                .map(|c| {
                    (
                        c.clone(),
                        CountByType::new(*summary.count(c).unwrap_or(&0i32)),
                    )
                })
                .collect(),
        }
    }
}

impl CountByType {
    pub fn to_string(&self) -> String {
        match self.count {
            0 => "".into(),
            _ => self.count.to_string(),
        }
    }
    pub fn new(count: i32) -> CountByType {
        CountByType { count }
    }
}
