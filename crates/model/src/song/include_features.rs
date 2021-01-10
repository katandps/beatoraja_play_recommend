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
//
// 	public static final int FEATURE_UNDEFINEDLN = 1;
// 	public static final int FEATURE_MINENOTE = 2;
// 	public static final int FEATURE_RANDOM = 4;
// 	public static final int FEATURE_LONGNOTE = 8;
// 	public static final int FEATURE_CHARGENOTE = 16;
// 	public static final int FEATURE_HELLCHARGENOTE = 32;
// 	public static final int FEATURE_STOPSEQUENCE = 64;
