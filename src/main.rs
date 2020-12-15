use model::*;
use mysql::*;
use sqlite::*;

#[tokio::main]
pub async fn main() {
    let sqlite_client = SqliteClient::new();
    let songs = sqlite_client.song_data();

    let repository = MySQLClient::new();
    repository.save_song(&songs);
    let songs = repository.song_data();
    println!("Finished. {} records", songs.songs.len());
}
