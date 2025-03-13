pub mod custom_table;
pub mod scores;
pub mod songs;
pub mod tables;
pub mod users;

#[derive(Clone, Debug)]
pub enum Response<T> {
    Ok { tag: Option<String>, body: T },
    Cached { tag: String },
}
