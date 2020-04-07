use crate::score::clear_type::ClearType;
use std::collections::HashMap;

#[derive(Debug)]
pub struct LampSum {
    sum: HashMap<ClearType, i32>,
}

impl LampSum {
    pub fn new() -> LampSum {
        LampSum {
            sum: HashMap::new(),
        }
    }

    pub fn push(&mut self, lamp: &dyn Lamp) {
        let count = self.sum.entry(lamp.clear_type().clone()).or_insert(0);
        *count += 1;
    }

    pub fn format(&self) -> String {
        ClearType::vec()
            .iter()
            .map(|lamp| {
                lamp.coloring(format!(
                    "[{:>3}]",
                    match self.sum.get(&lamp) {
                        Some(i) => i.to_string(),
                        None => "".to_string(),
                    }
                ))
            })
            .collect()
    }
}

pub trait Lamp {
    fn clear_type(&self) -> &ClearType;
}
