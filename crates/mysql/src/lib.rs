use model::*;

pub struct MySQLClient {}

impl MySQLClient {
    pub fn new() -> MySQLClient {
        MySQLClient {}
    }
}

impl ScoreRepository for MySQLClient {
    fn score(&self) -> Scores {
        unimplemented!()
    }
}
impl SongRepository for MySQLClient {
    fn song_data(&self) -> Songs {
        unimplemented!()
    }
}
impl ScoreLogRepository for MySQLClient {
    fn score_log(&self) -> ScoreLog {
        unimplemented!()
    }
}

fn config() -> config::Config {
    if cfg!(test) {
        config::Config::Dummy
    } else {
        config::config()
    }
}
