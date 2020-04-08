use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

pub struct Summary<T>
where
    T: Countable,
{
    sum: HashMap<T, i32>,
    subjects: Vec<T>,
}

impl<T> Summary<T>
where
    T: Countable,
{
    pub fn new(subjects: Vec<T>) -> Summary<T> {
        Summary {
            sum: HashMap::new(),
            subjects,
        }
    }

    pub fn push(&mut self, c: &T)
    where
        T: Countable,
    {
        let count = self.sum.entry(c.clone()).or_insert(0);
        *count += 1;
    }
}

impl<T> fmt::Display for Summary<T>
where
    T: Countable,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n",
            self.subjects
                .iter()
                .map(|countable| {
                    countable.coloring(format!(
                        "[{:>3}]",
                        match self.sum.get(&countable) {
                            Some(i) => i.to_string(),
                            None => "".to_string(),
                        }
                    ))
                })
                .collect::<String>()
        )
    }
}

pub trait Countable: Hash + Eq + PartialEq + Clone {
    fn coloring(&self, s: String) -> String;
}
