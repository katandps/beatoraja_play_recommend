use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Visibility(bool);

impl Visibility {
    pub fn new(v: bool) -> Visibility {
        Visibility(v)
    }
}
