use crate::config;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

pub struct Summary<T> {
    sum: HashMap<T, i32>,
    subjects: Vec<T>,
}

pub trait SummaryTrait<T>: MakeSummary + SummaryCount<T> + fmt::Display {}

pub trait MakeSummary {
    fn new() -> Self;
}

impl<T: Countable + Sized> MakeSummary for Summary<T> {
    fn new() -> Self {
        Summary {
            sum: HashMap::new(),
            subjects: T::vec(),
        }
    }
}

pub trait SummaryCount<T> {
    fn tally(self, c: T) -> Self;
    fn count(&self, key: &T) -> Option<&i32>;
}

impl<T: Countable> SummaryCount<T> for Summary<T> {
    fn tally(self, c: T) -> Self {
        let mut s = self;
        let count = s.sum.entry(c.clone()).or_insert(0);
        *count += 1;
        s
    }

    fn count(&self, key: &T) -> Option<&i32> {
        self.sum.get(key)
    }
}

impl<T> fmt::Display for Summary<T>
where
    T: Countable,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let should_coloring = config().coloring_table();
        write!(
            f,
            "{}\n",
            self.subjects
                .iter()
                .map(|countable| {
                    let format = format!(
                        "[{:>3}]",
                        match self.sum.get(&countable) {
                            Some(i) => i.to_string(),
                            None => "".to_string(),
                        }
                    );
                    if should_coloring {
                        countable.coloring(format)
                    } else {
                        format
                    }
                })
                .collect::<String>()
        )
    }
}

pub trait Countable: Hash + Eq + PartialEq + Clone {
    fn coloring(&self, s: String) -> String;
    fn vec() -> Vec<Self>;
}
