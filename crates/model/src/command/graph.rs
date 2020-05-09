use crate::*;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[derive(Deserialize, Serialize)]
pub struct Graph<T: Countable> {
    table: String,
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
    pub fn make(table: String, count: Vec<CountByLevel<T>>) -> Self {
        Graph { table, count }
    }
}

impl<T: Countable + Display> Display for Graph<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}\n{}",
            self.table,
            self.count
                .iter()
                .map(ToString::to_string)
                .collect::<String>()
        )
    }
}

impl<T: Countable> CountByLevel<T> {
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

impl<T: Countable + Display> Display for CountByLevel<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}\n",
            T::vec()
                .iter()
                .map(|l| self.count[l].to_string())
                .collect::<String>()
        )
    }
}

impl CountByType {
    pub fn new(count: i32) -> CountByType {
        CountByType { count }
    }
}

impl Display for CountByType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.count {
            0 => write!(f, "[   ]"),
            _ => write!(f, "[{:>3}]", self.count),
        }
    }
}
