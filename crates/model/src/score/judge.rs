use crate::score::ex_score::ExScore;

#[derive(Clone, Debug)]
pub struct Judge {
    early_pgreat: i32,
    late_pgreat: i32,
    early_great: i32,
    late_great: i32,
    early_good: i32,
    late_good: i32,
    early_bad: i32,
    late_bad: i32,
    early_poor: i32,
    late_poor: i32,
    early_miss: i32,
    late_miss: i32,
}

impl Judge {
    pub fn new(
        early_pgreat: i32,
        late_pgreat: i32,
        early_great: i32,
        late_great: i32,
        early_good: i32,
        late_good: i32,
        early_bad: i32,
        late_bad: i32,
        early_poor: i32,
        late_poor: i32,
        early_miss: i32,
        late_miss: i32,
    ) -> Judge {
        Judge {
            early_pgreat,
            late_pgreat,
            early_great,
            late_great,
            early_good,
            late_good,
            early_bad,
            late_bad,
            early_poor,
            late_poor,
            early_miss,
            late_miss,
        }
    }
    pub fn ex_score(&self) -> ExScore {
        ExScore::from_judge(
            self.early_pgreat,
            self.late_pgreat,
            self.early_great,
            self.late_great,
        )
    }
}
