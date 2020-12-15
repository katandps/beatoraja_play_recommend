mod models;
mod schema;

use diesel::prelude::*;
use model::*;
use std::collections::HashMap;

#[macro_use]
extern crate diesel;
extern crate anyhow;

pub struct MySQLClient {
    connection: MysqlConnection,
}

impl MySQLClient {
    pub fn new() -> Self {
        Self {
            connection: Self::establish_connection(config().mysql_url()),
        }
    }

    fn establish_connection(url: String) -> MysqlConnection {
        MysqlConnection::establish(&url).expect(&format!("Error connecting to {}", url))
    }

    fn songs(&self) -> Vec<models::Song> {
        schema::songs::table
            .load(&self.connection)
            .expect("Error loading schema")
    }

    fn hash(&self) -> Vec<models::Hash> {
        schema::hashes::table
            .load(&self.connection)
            .expect("Error loading schema")
    }
}

impl SongRepository for MySQLClient {
    fn song_data(&self) -> Songs {
        let record = self.songs();
        let hash: HashMap<String, String> = self
            .hash()
            .iter()
            .map(|hash| (hash.sha256.clone(), hash.md5.clone()))
            .collect();

        record
            .iter()
            .fold(SongsBuilder::new(), |mut builder, row| {
                let md5 = HashMd5::new(hash.get(&row.sha256).unwrap().clone());
                builder.push(
                    md5,
                    HashSha256::new(row.sha256.clone()),
                    Title::new(format!("{}{}", row.title, row.subtitle)),
                    Artist::new(row.artist.clone()),
                    row.notes,
                );
                builder
            })
            .build()
    }

    fn save_song(&self, songs: &Songs) {
        let exist_hashes = self.hash();
        let mut hashmap = songs.converter.sha256_to_md5.clone();
        for row in exist_hashes {
            hashmap.remove(&HashSha256::new(row.sha256));
        }
        let new_hashes = hashmap
            .iter()
            .map(|(sha256, md5)| models::Hash {
                sha256: sha256.to_string(),
                md5: md5.to_string(),
            })
            .collect::<Vec<_>>();

        let mut index = 0;
        loop {
            let mut records = Vec::new();
            while index < new_hashes.len() && records.len() < 100 {
                records.push(new_hashes[index].clone());
                index += 1;
            }
            if records.is_empty() {
                break;
            }
            println!("Insert {} hashes.", records.len());
            diesel::insert_into(schema::hashes::table)
                .values(records)
                .execute(&self.connection)
                .expect("Error saving new hashes");
        }

        let exist_songs = self.songs();
        let mut songs = songs.songs.clone();
        for row in exist_songs {
            songs.remove(&HashSha256::new(row.sha256));
        }
        let new_songs = songs
            .iter()
            .map(|(_, song)| models::Song {
                sha256: song.hash.to_string(),
                title: song.title.to_string(),
                subtitle: "".into(),
                artist: song.artist.to_string(),
                sub_artist: "".into(),
                notes: song.notes,
                length: 0,
            })
            .collect::<Vec<_>>();
        let mut index = 0;
        loop {
            let mut records = Vec::new();
            while index < new_songs.len() && records.len() < 3 {
                records.push(new_songs[index].clone());
                index += 1;
            }
            if records.is_empty() {
                break;
            }
            println!("Insert {} songs.", records.len());
            diesel::insert_into(schema::songs::table)
                .values(records)
                .execute(&self.connection)
                .expect("Error saving new songs");
        }
    }
}

fn config() -> config::Config {
    if cfg!(test) {
        config::Config::Dummy
    } else {
        config::config()
    }
}
