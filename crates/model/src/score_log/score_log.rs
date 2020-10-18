use crate::*;
use itertools::Itertools;
use serde::Serializer;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ScoreLog(HashMap<SongId, SnapShots>);

impl ScoreLog {
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
                    SnapShots::new(vec![self.get_snap(&song_id, date)]),
                )
            })
            .collect();
        ScoreLog(log)
    }

    pub fn get_snaps(&self, song_id: &SongId) -> SnapShots {
        self.0.get(song_id).unwrap().clone()
    }

    pub fn get_snap(&self, song_id: &SongId, date: &UpdatedAt) -> SnapShot {
        match self.0.get(&song_id) {
            Some(s) => s.get_snap(date),
            _ => SnapShot::new(),
        }
    }

    /// 更新が古い順に設定された件数だけ取得する
    fn for_recommend(&self, date: &UpdatedAt) -> Vec<(SongId, SnapShot)> {
        self.0
            .iter()
            .map(|(id, snaps)| (id.clone(), snaps.get_snap(date)))
            .sorted_by(|a, b| a.1.updated_at.cmp(&b.1.updated_at))
            .take(config().recommend_song_number())
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
            .map(|(song_id, snap)| snap.recommend_song(songs, &song_id))
            .collect()
    }
}

impl Serialize for ScoreLog {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let l: Vec<SnapShots> = self.0.iter().map(|(_, snap)| snap.clone()).collect();
        l.serialize(serializer)
    }
}
#[derive(Serialize)]
struct Id(SongId);

pub struct ScoreLogBuilder(HashMap<SongId, SnapShots>);

impl ScoreLogBuilder {
    pub fn push(&mut self, song_id: SongId, snapshot: SnapShot) {
        self.0
            .entry(song_id.clone())
            .or_insert(SnapShots::default())
            .add(snapshot);
    }

    pub fn builder() -> ScoreLogBuilder {
        ScoreLogBuilder(HashMap::new())
    }

    pub fn build(self) -> ScoreLog {
        ScoreLog(self.0)
    }
}

pub trait ScoreLogRepository {
    fn score_log(&self) -> ScoreLog;
}
