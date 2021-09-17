/// 各種特殊ノートを含む譜面かどうか
#[derive(Clone, Debug)]
pub struct IncludeFeatures {
    undefined_ln: UndefinedLn,
    mine_note: MineNote,
    random: Random,
    long_note: LongNote,
    charge_note: ChargeNote,
    hell_charge_note: HellChargeNote,
    stop_sequence: StopSequence,
}

impl From<i32> for IncludeFeatures {
    fn from(i: i32) -> Self {
        let undefined_ln = UndefinedLn(i & 1 != 0);
        let mine_note = MineNote(i & 2 != 0);
        let random = Random(i & 4 != 0);
        let long_note = LongNote(i & 8 != 0);
        let charge_note = ChargeNote(i & 16 != 0);
        let hell_charge_note = HellChargeNote(i & 32 != 0);
        let stop_sequence = StopSequence(i & 64 != 0);
        IncludeFeatures {
            undefined_ln,
            mine_note,
            random,
            long_note,
            charge_note,
            hell_charge_note,
            stop_sequence,
        }
    }
}

impl From<IncludeFeatures> for i32 {
    fn from(feature: IncludeFeatures) -> Self {
        (if feature.undefined_ln.0 { 1 } else { 0 })
            + if feature.mine_note.0 { 2 } else { 0 }
            + if feature.random.0 { 4 } else { 0 }
            + if feature.long_note.0 { 8 } else { 0 }
            + if feature.charge_note.0 { 16 } else { 0 }
            + if feature.hell_charge_note.0 { 32 } else { 0 }
            + if feature.stop_sequence.0 { 64 } else { 0 }
    }
}

#[derive(Clone, Debug)]
pub struct UndefinedLn(bool);
#[derive(Clone, Debug)]
pub struct MineNote(bool);
#[derive(Clone, Debug)]
pub struct Random(bool);
#[derive(Clone, Debug)]
pub struct LongNote(bool);
#[derive(Clone, Debug)]
pub struct ChargeNote(bool);
#[derive(Clone, Debug)]
pub struct HellChargeNote(bool);
#[derive(Clone, Debug)]
pub struct StopSequence(bool);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        for i in 0..128 {
            let i_f = IncludeFeatures::from(i);
            assert_eq!(i, i32::from(i_f));
        }
    }
}
