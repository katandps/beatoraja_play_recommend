use crate::*;
use chrono::Duration;
use std::collections::BTreeSet;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct SnapShots(pub BTreeSet<SnapShot>);

impl SnapShots {
    pub fn create_by_snaps(snapshots: Vec<SnapShot>) -> SnapShots {
        SnapShots(snapshots.iter().cloned().collect())
    }

    pub fn default() -> SnapShots {
        SnapShots::create_by_snaps(Vec::new())
    }

    pub fn add(&mut self, snapshot: SnapShot) {
        self.0.insert(snapshot);
    }

    /// deprecated
    pub fn get_snap(&self, date: &UpdatedAt) -> SnapShot {
        self.snap(date).unwrap_or_default()
    }

    pub fn snap(&self, date: &UpdatedAt) -> Option<SnapShot> {
        self.0
            .iter()
            .filter(|&s| s.updated_at.le(date))
            .last()
            .cloned()
    }

    pub fn score_snap(&self, date: &UpdatedAt) -> Option<ScoreSnap> {
        match self.snap(date) {
            Some(last) => {
                let mut last_date = &last.updated_at;
                let mut s = None;
                for snap in self.0.iter().rev() {
                    if snap.score >= last.score {
                        last_date = &snap.updated_at;
                        continue;
                    }
                    let one_day_before = self.get_snap(&(last_date - Duration::days(1)));
                    s = Some(ScoreSnap::new(
                        last.score,
                        last_date.clone(),
                        one_day_before.score,
                    ));
                    break;
                }
                match s {
                    Some(s) => Some(s),
                    None => Some(ScoreSnap::new(
                        last.score,
                        last_date.clone(),
                        Default::default(),
                    )),
                }
            }
            None => None,
        }
    }

    pub fn min_bp_snap(&self, date: &UpdatedAt) -> Option<MinBPSnap> {
        match self.snap(date) {
            Some(last) => {
                let mut last_date = &last.updated_at;

                let mut s = None;
                for snap in self.0.iter().rev() {
                    if snap.min_bp <= last.min_bp {
                        last_date = &snap.updated_at;
                        continue;
                    }
                    let one_day_before = self.get_snap(&(last_date - Duration::days(1)));
                    s = Some(MinBPSnap::new(
                        last.min_bp,
                        last_date.clone(),
                        one_day_before.min_bp,
                    ));
                    break;
                }
                match s {
                    Some(s) => Some(s),
                    None => Some(MinBPSnap::new(
                        last.min_bp,
                        last_date.clone(),
                        Default::default(),
                    )),
                }
            }
            None => None,
        }
    }

    pub fn clear_type_snap(&self, date: &UpdatedAt) -> Option<ClearTypeSnap> {
        match self.snap(date) {
            Some(last) => {
                let mut last_date = &last.updated_at;

                let mut s = None;
                for snap in self.0.iter().rev() {
                    if snap.clear_type >= last.clear_type {
                        last_date = &snap.updated_at;
                        continue;
                    }
                    let one_day_before = self.get_snap(&(last_date - Duration::days(1)));
                    s = Some(ClearTypeSnap::new(
                        last.clear_type,
                        last_date.clone(),
                        one_day_before.clear_type,
                    ));
                    break;
                }
                match s {
                    Some(s) => Some(s),
                    None => Some(ClearTypeSnap::new(
                        last.clear_type,
                        last_date.clone(),
                        Default::default(),
                    )),
                }
            }
            None => None,
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

        let shots = SnapShots::create_by_snaps(vec![
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

    #[test]
    pub fn clear() {
        const DAY: i64 = 86400;

        fn asrt(snapshots: &SnapShots, current: ClearType, before: ClearType, timestamp: i64) {
            let snap = snapshots
                .clear_type_snap(&UpdatedAt::from_timestamp(timestamp))
                .unwrap_or_default();
            assert_eq!(current, snap.current);
            assert_eq!(before, snap.before);
        }

        //10日目 failed
        let shot_failed = SnapShot::from_data(1, 2, 3, 4, DAY * 10);
        //15日目 failed継続
        let shot_failed2 = SnapShot::from_data(1, 2, 3, 4, DAY * 15);
        //17日目 assist + la
        let shot_assist = SnapShot::from_data(2, 2, 3, 4, DAY * 17);
        let shot_la = SnapShot::from_data(3, 2, 3, 4, DAY * 17 + 1);
        //20日目 assist継続
        let shot_la2 = SnapShot::from_data(3, 2, 3, 4, DAY * 20);
        //22日目 easy
        let shot_easy = SnapShot::from_data(4, 2, 3, 4, DAY * 22);
        //25日目 normal+hard
        let shot_normal = SnapShot::from_data(5, 2, 3, 4, DAY * 25);
        let shot_hard = SnapShot::from_data(6, 2, 3, 4, DAY * 25 + DAY - 1);
        //30日目 exhard
        let shot_exhard = SnapShot::from_data(7, 2, 3, 4, DAY * 30);

        let shots = SnapShots::create_by_snaps(vec![
            shot_failed.clone(),
            shot_failed2.clone(),
            shot_assist.clone(),
            shot_la.clone(),
            shot_la2.clone(),
            shot_easy.clone(),
            shot_normal.clone(),
            shot_hard.clone(),
            shot_exhard.clone(),
        ]);

        use ClearType::*;
        asrt(&shots, NoPlay, NoPlay, 0);
        asrt(&shots, NoPlay, NoPlay, DAY * 9);
        asrt(&shots, Failed, NoPlay, DAY * 10);
        asrt(&shots, Failed, NoPlay, DAY * 15);
        asrt(&shots, AssistEasy, Failed, DAY * 17);
        asrt(&shots, LightAssistEasy, Failed, DAY * 17 + 1);
        asrt(&shots, LightAssistEasy, Failed, DAY * 20);
        asrt(&shots, Easy, LightAssistEasy, DAY * 22);
        asrt(&shots, Normal, Easy, DAY * 25);
        asrt(&shots, Hard, Easy, DAY * 25 + DAY - 1);
        asrt(&shots, Hard, Easy, DAY * 26);
        asrt(&shots, ExHard, Hard, DAY * 30);
    }
}
