use rand::distributions::{Alphanumeric, DistString};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SongsTag {
    pub tag: String,
}
impl SongsTag {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let random_code = Alphanumeric.sample_string(&mut rng, 24);
        Self { tag: random_code }
    }

    pub fn is_saved(&self, tag: &Option<String>) -> bool {
        match &tag {
            Some(s) => &self.tag == s,
            None => false,
        }
    }
}
