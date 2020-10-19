use crate::*;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[derive(Deserialize, Serialize)]
pub struct Graph<T: Countable> {
    table: String,
    levels: Vec<CountByLevel<T>>,
}

#[derive(Deserialize, Serialize)]
pub struct CountByLevel<T: Countable>(HashMap<T, CountByType>);

#[derive(Deserialize, Serialize)]
pub struct CountByType(i32);

impl<T: Countable> Graph<T> {
    pub fn make(table: String, count: Vec<CountByLevel<T>>) -> Self {
        Graph {
            table,
            levels: count,
        }
    }
}

impl<T: Countable + Display> Display for Graph<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}\n{}",
            self.table,
            self.levels
                .iter()
                .map(ToString::to_string)
                .collect::<String>()
        )
    }
}

impl<T: Countable> CountByLevel<T> {
    pub fn make(summary: Summary<T>) -> CountByLevel<T> {
        CountByLevel(
            T::vec()
                .iter()
                .map(|c| (c.clone(), CountByType(*summary.count(c).unwrap_or(&0i32))))
                .collect(),
        )
    }
}

impl<T: Countable + Display> Display for CountByLevel<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}\n",
            T::vec()
                .iter()
                .map(|l| self.0[l].to_string())
                .collect::<String>()
        )
    }
}

impl Display for CountByType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.0 {
            0 => write!(f, "[   ]"),
            _ => write!(f, "[{:>3}]", self.0),
        }
    }
}
