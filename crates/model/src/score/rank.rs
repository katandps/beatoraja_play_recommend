use crate::*;
use std::fmt;

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum ClearRank {
    F,
    E,
    D,
    C,
    B,
    A,
    AA,
    AAA,
    Unknown,
}

impl ClearRank {
    pub fn from_notes_score(notes: i32, score: ExScore) -> ClearRank {
        let max = notes * 2;
        if max == 0 {
            return ClearRank::F;
        }
        match score.ex_score() {
            x if x >= max * 8 / 9 => ClearRank::AAA,
            x if x >= max * 7 / 9 => ClearRank::AA,
            x if x >= max * 6 / 9 => ClearRank::A,
            x if x >= max * 5 / 9 => ClearRank::B,
            x if x >= max * 4 / 9 => ClearRank::C,
            x if x >= max * 3 / 9 => ClearRank::D,
            x if x >= max * 2 / 9 => ClearRank::E,
            _ => ClearRank::F,
        }
    }

    pub fn from_integer(int: i32) -> ClearRank {
        match int {
            0 => ClearRank::F,
            1 => ClearRank::E,
            2 => ClearRank::D,
            3 => ClearRank::C,
            4 => ClearRank::B,
            5 => ClearRank::A,
            6 => ClearRank::AA,
            7 => ClearRank::AAA,
            _ => ClearRank::Unknown,
        }
    }
}

impl fmt::Display for ClearRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ClearRank::F => write!(f, "F"),
            ClearRank::E => write!(f, "E"),
            ClearRank::D => write!(f, "D"),
            ClearRank::C => write!(f, "C"),
            ClearRank::B => write!(f, "B"),
            ClearRank::A => write!(f, "A"),
            ClearRank::AA => write!(f, "AA"),
            ClearRank::AAA => write!(f, "AAA"),
            ClearRank::Unknown => write!(f, "Unknown"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let obj = ClearRank::from_notes_score(450, ExScore::from_score(0));
        assert_eq!(format!("{}", obj), "F");

        let obj = ClearRank::from_notes_score(450, ExScore::from_score(199));
        assert_eq!(format!("{}", obj), "F");

        let obj = ClearRank::from_notes_score(450, ExScore::from_score(200));
        assert_eq!(format!("{}", obj), "E");
        let obj = ClearRank::from_notes_score(450, ExScore::from_score(299));
        assert_eq!(format!("{}", obj), "E");

        let obj = ClearRank::from_notes_score(450, ExScore::from_score(300));
        assert_eq!(format!("{}", obj), "D");
        let obj = ClearRank::from_notes_score(450, ExScore::from_score(399));
        assert_eq!(format!("{}", obj), "D");

        let obj = ClearRank::from_notes_score(450, ExScore::from_score(400));
        assert_eq!(format!("{}", obj), "C");
        let obj = ClearRank::from_notes_score(450, ExScore::from_score(499));
        assert_eq!(format!("{}", obj), "C");

        let obj = ClearRank::from_notes_score(450, ExScore::from_score(500));
        assert_eq!(format!("{}", obj), "B");
        let obj = ClearRank::from_notes_score(450, ExScore::from_score(599));
        assert_eq!(format!("{}", obj), "B");

        let obj = ClearRank::from_notes_score(450, ExScore::from_score(600));
        assert_eq!(format!("{}", obj), "A");
        let obj = ClearRank::from_notes_score(450, ExScore::from_score(699));
        assert_eq!(format!("{}", obj), "A");

        let obj = ClearRank::from_notes_score(450, ExScore::from_score(700));
        assert_eq!(format!("{}", obj), "AA");
        let obj = ClearRank::from_notes_score(450, ExScore::from_score(799));
        assert_eq!(format!("{}", obj), "AA");

        let obj = ClearRank::from_notes_score(450, ExScore::from_score(800));
        assert_eq!(format!("{}", obj), "AAA");
        let obj = ClearRank::from_notes_score(450, ExScore::from_score(900));
        assert_eq!(format!("{}", obj), "AAA");
    }
}
