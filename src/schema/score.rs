table! {
    score (sha256) {
        sha256 -> Text,
        mode -> Integer,
        clear -> Integer,
        epg -> Integer,
        lpg -> Integer,
        egr -> Integer,
        lgr -> Integer,
        egd -> Integer,
        lgd -> Integer,
        ebd -> Integer,
        lbd -> Integer,
        epr -> Integer,
        lpr -> Integer,
        ems -> Integer,
        lms -> Integer,
        notes -> Integer,
        combo -> Integer,
        minbp -> Integer,
        playcount -> Integer,
        clearcount -> Integer,
        //history -> Integer, nullが入っていたりする
        scorehash -> Text,
        option -> Integer,
        random -> Integer,
        date -> Integer,
        state -> Integer,
        //trophy -> Text,
        //ghost -> Text,
    }
}
