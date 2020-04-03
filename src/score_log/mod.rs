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
    pub fn get_snap() -> SnapShot {
        unimplemented!()
    }
}

struct SnapShots {
    snapshots: Vec<SnapShot>,
}

impl SnapShots {
    fn add(&mut self, snapshot: SnapShot) {
        self.snapshots.push(snapshot)
    }
}

pub struct SnapShot {
    song_id: SongId,
    clear_type: ClearType,
    score: ExScore,
    max_combo: MaxCombo,
    min_bp: MinBP,
    updated_at: UpdatedAt,
}

impl SnapShot {
    pub fn new(
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
                    snapshots: Vec::new(),
                },
            );
        }
        let mut snapshots = self.log.get_mut(&song_id).unwrap();
        snapshots.add(snapshot);
    }

    pub fn build(builder: Self) -> ScoreLog {
        ScoreLog { log: builder.log }
    }
}
