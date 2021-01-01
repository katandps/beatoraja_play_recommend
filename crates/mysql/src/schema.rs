table! {
    hashes (sha256) {
        sha256 -> Char,
        md5 -> Char,
    }
}

table! {
    scores (id) {
        id -> Integer,
        user_id -> Integer,
        sha256 -> Char,
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
        combo -> Integer,
        min_bp -> Integer,
        play_count -> Integer,
        clear_count -> Integer,
        date -> Datetime,
    }
}

table! {
    score_snaps (id) {
        id -> Integer,
        user_id -> Integer,
        sha256 -> Char,
        mode -> Integer,
        date -> Datetime,
        clear -> Integer,
        score -> Integer,
        combo -> Integer,
        min_bp -> Integer,
    }
}

table! {
    songs (sha256) {
        sha256 -> Char,
        title -> Varchar,
        subtitle -> Varchar,
        artist -> Varchar,
        sub_artist -> Varchar,
        notes -> Integer,
        length -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        google_id -> Varchar,
        gmail_address -> Varchar,
        name -> Varchar,
        registered_date -> Datetime,
    }
}

table! {
    rename_logs(id) {
        id -> Integer,
        user_id -> Integer,
        old_name -> Varchar,
        new_name -> Varchar,
        date -> Datetime,
    }
}

joinable!(score_snaps -> hashes (sha256));
joinable!(score_snaps -> users (user_id));
joinable!(scores -> hashes (sha256));
joinable!(scores -> users (user_id));
joinable!(songs -> hashes (sha256));

allow_tables_to_appear_in_same_query!(hashes, scores, score_snaps, songs, users,);
