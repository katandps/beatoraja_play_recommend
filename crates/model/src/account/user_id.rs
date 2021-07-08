use serde::Serialize;

#[derive(Clone, Debug, Serialize, Eq, PartialEq, Hash)]
pub struct UserId(i32);

impl UserId {
    pub fn new(id: i32) -> Self {
        UserId(id)
    }

    pub fn get(&self) -> i32 {
        self.0
    }
}
