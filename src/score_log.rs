use crate::lamp::Lamp;
use crate::score::clear_type::ClearType;
use crate::score::ex_score::ExScore;
use crate::score::max_combo::MaxCombo;
use crate::score::min_bp::MinBP;
use crate::score::song_id::SongId;
use crate::score::updated_at::UpdatedAt;
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
    pub fn get_snap(&self, song_id: SongId, date: &UpdatedAt) -> SnapShot {
        match self.log.get(&song_id) {
            Some(s) => s.get_snap(date),
            _ => SnapShot::new(song_id.clone()),
        }
    }
}

struct SnapShots {
    song_id: SongId,
    snapshots: Vec<SnapShot>,
}

impl SnapShots {
    fn add(&mut self, snapshot: SnapShot) {
        self.snapshots.push(snapshot)
    }

    fn get_snap(&self, date: &UpdatedAt) -> SnapShot {
        let snap = self
            .snapshots
            .iter()
            .filter(|s| s.updated_at.lt(date))
            .map(|s| s.clone())
            .nth(0);
        match snap {
            Some(s) => s,
            _ => SnapShot::new(self.song_id.clone()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SnapShot {
    song_id: SongId,
    clear_type: ClearType,
    score: ExScore,
    max_combo: MaxCombo,
    min_bp: MinBP,
    updated_at: UpdatedAt,
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
}

impl Lamp for SnapShot {
    fn clear_type(&self) -> &ClearType {
        &self.clear_type
    }
}

pub struct Builder {
    log: HashMap<SongId, SnapShots>,
}

impl Builder {
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

    pub fn builder() -> Builder {
        Builder {
            log: HashMap::new(),
        }
    }

    pub fn build(builder: Builder) -> ScoreLog {
        ScoreLog { log: builder.log }
    }
}
