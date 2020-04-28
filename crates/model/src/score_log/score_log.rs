use crate::*;
use std::collections::HashMap;

pub struct ScoreLog {
    log: HashMap<SongId, SnapShots>,
}

impl ScoreLog {
    pub fn new() -> ScoreLog {
        ScoreLog {
            log: HashMap::new(),
        }
    }

    /// Tableに存在する曲ログに絞り込む ログが存在しない曲はダミーで補完される
    fn filter_by_table<T: TableTrait>(
        &self,
        table: &T,
        songs: &Songs,
        date: &UpdatedAt,
    ) -> ScoreLog {
        let song_ids: Vec<SongId> = table
            .get_song(songs)
            .iter()
            .map(|song| song.song_id())
            .collect();
        let log: HashMap<SongId, SnapShots> = song_ids
            .iter()
            .map(|song_id| {
                (
                    song_id.clone(),
                    SnapShots {
                        song_id: song_id.clone(),
                        snapshots: vec![self.get_snap(&song_id, date)],
                    },
                )
            })
            .collect();
        ScoreLog { log }
    }

    pub fn get_snap(&self, song_id: &SongId, date: &UpdatedAt) -> SnapShot {
        match self.log.get(&song_id) {
            Some(s) => s.get_snap(date),
            _ => SnapShot::new(song_id.clone()),
        }
    }

    /// 更新が古い順に設定された件数だけ取得する
    fn for_recommend(&self, date: &UpdatedAt) -> Vec<SnapShot> {
        let mut vec: Vec<SnapShot> = self
            .log
            .iter()
            .map(|(_id, snaps)| snaps.get_snap(date))
            .collect();
        vec.sort_by(|a, b| a.updated_at.cmp(&b.updated_at));
        vec.iter()
            .take(config().recommend_song_number())
            .cloned()
            .collect()
    }

    /// リコメンドのVectorを返す
    pub fn get_recommend<T: TableTrait>(
        &self,
        table: &T,
        songs: &Songs,
        date: &UpdatedAt,
    ) -> Vec<RecommendSong> {
        self.filter_by_table(table, songs, date)
            .for_recommend(date)
            .iter()
            .flat_map(|snap| snap.recommend_song(songs))
            .collect()
    }
}

pub struct ScoreLogBuilder {
    log: HashMap<SongId, SnapShots>,
}

impl ScoreLogBuilder {
    pub fn push(&mut self, song_id: SongId, snapshot: SnapShot) {
        if !self.log.contains_key(&song_id) {
            self.log.insert(
                song_id.clone(),
                SnapShots {
                    song_id: song_id.clone(),
                    snapshots: Vec::new(),
                },
            );
        }
        let snapshots = self.log.get_mut(&song_id).unwrap();
        snapshots.add(snapshot);
    }

    pub fn builder() -> ScoreLogBuilder {
        ScoreLogBuilder {
            log: HashMap::new(),
        }
    }

    pub fn build(builder: ScoreLogBuilder) -> ScoreLog {
        ScoreLog { log: builder.log }
    }
}
