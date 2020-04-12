use super::*;

#[derive(Clone, Debug)]
pub struct SnapShot {
    pub(super) song_id: SongId,
    pub(super) clear_type: ClearType,
    pub(super) score: ExScore,
    pub(super) max_combo: MaxCombo,
    pub(super) min_bp: MinBP,
    pub(super) updated_at: UpdatedAt,
}

impl SnapShot {
    pub fn new(song_id: SongId) -> SnapShot {
        SnapShot {
            song_id,
            clear_type: ClearType::NoPlay,
            score: ExScore::new(),
            max_combo: MaxCombo::new(),
            min_bp: MinBP::new(),
            updated_at: UpdatedAt::new(),
        }
    }
    pub fn from_data(
        song_id: SongId,
        clear_type: i32,
        score: i32,
        combo: i32,
        minbp: i32,
        timestamp: i32,
    ) -> SnapShot {
        SnapShot {
            song_id,
            clear_type: ClearType::from_integer(clear_type),
            score: ExScore::from_score(score),
            max_combo: MaxCombo::from_combo(combo),
            min_bp: MinBP::from_bp(minbp),
            updated_at: UpdatedAt::from_timestamp(timestamp),
        }
    }

    pub fn score(&self) -> ExScore {
        self.score.clone()
    }
    pub fn clear_type(&self) -> &ClearType {
        &self.clear_type
    }
    pub fn recommend_song(&self, songs: &Songs) -> Option<RecommendSong> {
        match songs.song_by_sha256(&self.song_id.sha256()) {
            Some(s) => Some(RecommendSong::new(self.format(s.title()))),
            _ => None,
        }
    }
    pub(crate) fn format(&self, title: String) -> String {
        format!(
            "{}\n{} {} score:{} bp:{} combo:{}",
            title, self.updated_at, self.clear_type, self.score, self.min_bp, self.max_combo
        )
    }
}