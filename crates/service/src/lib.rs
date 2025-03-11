pub mod songs;
pub mod tables;

#[derive(Clone, Debug)]
pub enum Response<T> {
    Ok { tag: Option<String>, body: T },
    Cached { tag: String },
}
