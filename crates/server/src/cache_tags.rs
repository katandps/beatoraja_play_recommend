#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SongsTag {
    pub tag: String,
    pub table_tag: Option<String>,
}
impl SongsTag {
    pub fn is_saved(&self, tag: &Option<String>, table_tag: &Option<String>) -> bool {
        match &tag {
            Some(s) => &self.tag == s && &self.table_tag == table_tag,
            None => false,
        }
    }
}
