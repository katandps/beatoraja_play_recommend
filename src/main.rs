use model::*;
use mysql::*;
use sqlite::*;

use anyhow::Result;

#[tokio::main]
pub async fn main() {
    update_songs();
    change_name();
    match save_score() {
        Err(e) => panic!("{:?}", e),
        _ => (),
    }
    dbg!(get_score().unwrap().to_string());
}

fn update_songs() {
    let sqlite_client = SqliteClient::by_config();
    let songs = sqlite_client.song_data();
    let repository = MySQLClient::new();
    repository.save_song(&songs);
    let songs = repository.song_data();
    println!("Finished. {} records", songs.songs.len());
}

fn save_score() -> Result<()> {
    let repository = MySQLClient::new();
    let account = repository.account("katandps@gmail.com".into())?;
    let sqlite = SqliteClient::by_config();
    let score = sqlite.score();

    repository.save_score(account, score)
}

fn change_name() -> Result<()> {
    let repository = MySQLClient::new();
    let mut account = repository.account("katandps@gmail.com".into())?;
    account.set_name("katand".into());
    repository.save_account(account)?;
    Ok(())
}

fn get_score() -> Result<Scores> {
    let repository = MySQLClient::new();
    let account = repository.account("katandps@gmail.com".into())?;
    repository.score(account)
}
