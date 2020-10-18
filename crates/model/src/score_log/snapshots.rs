use crate::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SnapShots(Vec<SnapShot>);

impl SnapShots {
    pub fn new(snapshots: Vec<SnapShot>) -> SnapShots {
        SnapShots(snapshots)
    }

    pub fn default() -> SnapShots {
        SnapShots::new(Vec::new())
    }

    pub(super) fn add(&mut self, snapshot: SnapShot) {
        self.0.push(snapshot)
    }

    pub fn get_snap(&self, date: &UpdatedAt) -> SnapShot {
        let snap = self
            .0
            .iter()
            .filter(|s| s.updated_at.le(date))
            .map(|s| s.clone())
            .last();
        match snap {
            Some(s) => s,
            _ => SnapShot::new(),
        }
    }
}
