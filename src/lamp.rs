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
        let mut ret = String::new();
        for c in ClearType::vec() {
            if !self.sum.contains_key(&c) {
                ret.push_str(c.coloring(format!("[   ]")).as_str());
            } else {
                ret.push_str(c.coloring(format!("[{:>3}]", self.sum[&c])).as_str());
            }
            ret.push_str(" ");
        }
        ret
    }
}

pub trait Lamp {
    fn clear_type(&self) -> &ClearType;
}
