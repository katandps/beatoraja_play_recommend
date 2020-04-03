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
    pub fn get_snap(&mut self, song_id: SongId, date: &UpdatedAt) -> SnapShot {
        if !self.log.contains_key(&song_id) {
            self.log
                .insert(song_id.clone(), SnapShots::new(song_id.clone()));
        }
        self.log.get(&song_id).unwrap().get_snap(date)
    }
}

struct SnapShots {
    song_id: SongId,
    snapshots: Vec<SnapShot>,
}

impl SnapShots {
    fn new(song_id: SongId) -> SnapShots {
        let mut snapshots = Vec::new();
        snapshots.push(SnapShot::new(song_id.clone()));
        SnapShots { song_id, snapshots }
    }

    fn add(&mut self, snapshot: SnapShot) {
        self.snapshots.push(snapshot)
    }

    fn get_snap(&self, date: &UpdatedAt) -> SnapShot {
        let mut ret = SnapShot::new(self.song_id.clone());
        for ss in self.snapshots.iter() {
            if ss.updated_at.gt(date) {
                break;
            }
            ret = ss.clone();
        }
        ret
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
        clear_type: ClearType,
        score: ExScore,
        max_combo: MaxCombo,
        min_bp: MinBP,
        updated_at: UpdatedAt,
    ) -> SnapShot {
        SnapShot {
            song_id,
            clear_type,
            score,
            max_combo,
            min_bp,
            updated_at,
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
    pub fn new() -> Builder {
        Builder {
            log: HashMap::new(),
        }
    }

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

    pub fn build(builder: Self) -> ScoreLog {
        ScoreLog { log: builder.log }
    }
}
