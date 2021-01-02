use crate::*;
use std::collections::BTreeSet;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SnapShots(pub BTreeSet<SnapShot>);

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

    pub fn score_snap(&self, date: &UpdatedAt) -> ScoreSnap {
        let last = self.get_snap(date);
        let mut last_date = &last.updated_at;
        for snap in self.0.iter().rev() {
            if snap.score >= last.score {
                last_date = &snap.updated_at;
                continue;
            }
            let one_day_before = self.get_snap(&last_date.sub(1));
            return ScoreSnap::new(last.score, last_date.clone(), one_day_before.score);
        }
        ScoreSnap::new(last.score, last_date.clone(), ExScore::new())
    }

    pub fn min_bp_snap(&self, date: &UpdatedAt) -> MinBPSnap {
        let last = self.get_snap(date);
        let mut last_date = &last.updated_at;
        for snap in self.0.iter().rev() {
            if snap.min_bp <= last.min_bp {
                last_date = &snap.updated_at;
                continue;
            }
            let one_day_before = self.get_snap(&last_date.sub(1));
            return MinBPSnap::new(last.min_bp, last_date.clone(), one_day_before.min_bp);
        }
        MinBPSnap::new(last.min_bp, last_date.clone(), MinBP::new())
    }

    pub fn clear_type_snap(&self, date: &UpdatedAt) -> ClearTypeSnap {
        let last = self.get_snap(date);
        let mut last_date = &last.updated_at;
        for snap in self.0.iter().rev() {
            if snap.clear_type >= last.clear_type {
                last_date = &snap.updated_at;
                continue;
            }
            let one_day_before = self.get_snap(&last_date.sub(1));
            return ClearTypeSnap::new(
                last.clear_type,
                last_date.clone(),
                one_day_before.clear_type,
            );
        }
        ClearTypeSnap::new(last.clear_type, last_date.clone(), ClearType::NoPlay)
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

    #[test]
    pub fn clear() {
        const DAY: i64 = 86400;

        fn asrt(snapshots: &SnapShots, current: ClearType, before: ClearType, timestamp: i64) {
            let snap = snapshots.clear_type_snap(&UpdatedAt::from_timestamp(timestamp));
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

        let shots = SnapShots::new(vec![
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
