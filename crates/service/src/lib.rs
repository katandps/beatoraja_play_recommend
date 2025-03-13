pub mod custom_table;
pub mod play_data;
pub mod scores;
pub mod song_data;
pub mod songs;
pub mod status;
pub mod tables;
pub mod users;

#[derive(Clone, Debug)]
pub enum Response<T> {
    Ok { tag: Option<String>, body: T },
    Cached { tag: String },
}
