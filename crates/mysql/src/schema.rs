// @generated automatically by Diesel CLI.

diesel::table! {
    admins (id) {
        id -> Integer,
        user_id -> Integer,
    }
}

diesel::table! {
    hashes (sha256) {
        #[max_length = 64]
        sha256 -> Char,
        #[max_length = 32]
        md5 -> Char,
    }
}

diesel::table! {
    player_stats (id) {
        id -> Integer,
        user_id -> Integer,
        date -> Datetime,
        playcount -> Integer,
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
        playtime -> Integer,
    }
}

diesel::table! {
    rename_logs (id) {
        id -> Integer,
        user_id -> Integer,
        #[max_length = 255]
        old_name -> Varchar,
        #[max_length = 255]
        new_name -> Varchar,
        date -> Datetime,
    }
}

diesel::table! {
    score_snaps (id) {
        id -> Integer,
        user_id -> Integer,
        #[max_length = 64]
        sha256 -> Char,
        mode -> Integer,
        date -> Datetime,
        clear -> Integer,
        score -> Integer,
        combo -> Integer,
        min_bp -> Integer,
    }
}

diesel::table! {
    score_upload_logs (id) {
        id -> Integer,
        user_id -> Integer,
        date -> Datetime,
    }
}

diesel::table! {
    scores (id) {
        id -> Integer,
        user_id -> Integer,
        #[max_length = 64]
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

diesel::table! {
    songs (sha256) {
        #[max_length = 64]
        sha256 -> Char,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        subtitle -> Varchar,
        #[max_length = 255]
        artist -> Varchar,
        #[max_length = 255]
        sub_artist -> Varchar,
        notes -> Integer,
        length -> Integer,
        features -> Integer,
    }
}

diesel::table! {
    user_statuses (id) {
        id -> Integer,
        user_id -> Integer,
        visible -> Bool,
        score_updated_at -> Datetime,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 64]
        google_id -> Varchar,
        #[max_length = 255]
        gmail_address -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        registered_date -> Datetime,
    }
}

diesel::joinable!(admins -> users (user_id));
diesel::joinable!(player_stats -> users (user_id));
diesel::joinable!(score_snaps -> hashes (sha256));
diesel::joinable!(score_snaps -> users (user_id));
diesel::joinable!(scores -> hashes (sha256));
diesel::joinable!(scores -> users (user_id));
diesel::joinable!(songs -> hashes (sha256));
diesel::joinable!(user_statuses -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    hashes,
    player_stats,
    rename_logs,
    score_snaps,
    score_upload_logs,
    scores,
    songs,
    user_statuses,
    users,
);
