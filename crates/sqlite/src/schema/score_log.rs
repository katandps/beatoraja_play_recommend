table! {
    scorelog (sha256, mode) {
        sha256 -> Text,
        mode -> Integer,
        clear -> Integer,
        oldclear -> Integer,
        score -> Integer,
        oldscore -> Integer,
        combo -> Integer,
        oldcombo -> Integer,
        minbp -> Integer,
        oldminbp -> Integer,
        date -> Integer,
    }
}

#[derive(Queryable)]
#[allow(dead_code)]
pub struct ScoreLog {
    pub sha256: String,
    pub mode: i32,
    pub clear: i32,
    pub oldclear: i32,
    pub score: i32,
    pub oldscore: i32,
    pub combo: i32,
    pub oldcombo: i32,
    pub minbp: i32,
    pub oldminbp: i32,
    pub date: i32,
}
