use crate::*;
use std::collections::BTreeSet;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SnapShots(BTreeSet<SnapShot>);

impl SnapShots {
    pub fn new(snapshots: Vec<SnapShot>) -> SnapShots {
        SnapShots(snapshots.iter().cloned().collect())
    }

    pub fn default() -> SnapShots {
        SnapShots::new(Vec::new())
    }

    pub fn add(&mut self, snapshot: SnapShot) {
        self.0.insert(snapshot);
    }

    pub fn get_snap(&self, date: &UpdatedAt) -> SnapShot {
        let snap = self.0.iter().filter(|&s| s.updated_at.le(date)).last();
        match snap {
            Some(s) => s.clone(),
            _ => SnapShot::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test() {
        let shot1 = SnapShot::from_data(1, 2, 3, 4, 11);
        let shot2 = SnapShot::from_data(1, 2, 3, 4, 22);
        let shot3 = SnapShot::from_data(1, 2, 3, 4, 33);
        let shot4 = SnapShot::from_data(1, 2, 3, 4, 44);

        let shots = SnapShots::new(vec![
            shot1.clone(),
            shot2.clone(),
            shot3.clone(),
            shot4.clone(),
        ]);

        assert_eq!(shot1, shots.get_snap(&UpdatedAt::from_timestamp(21)));
        assert_eq!(shot2, shots.get_snap(&UpdatedAt::from_timestamp(22)));
        assert_eq!(shot2, shots.get_snap(&UpdatedAt::from_timestamp(23)));
        assert_eq!(shot2, shots.get_snap(&UpdatedAt::from_timestamp(32)));
        assert_eq!(shot3, shots.get_snap(&UpdatedAt::from_timestamp(33)));
    }
}
